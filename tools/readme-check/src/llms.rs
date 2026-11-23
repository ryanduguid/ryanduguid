//! Deterministic freshness, structure and sync checks for `llms.txt`.
//!
//! Verifies the `llms.txt` layout (H1 title, blockquote summary, non-empty
//! unique sections, well-formed `https://` entries without duplicate targets),
//! validates Australian calendar dates and `valid until` expiries, and keeps
//! `llms.txt` aligned with `README.md` and `facts::FACTS`.

use crate::facts::FACTS;
use crate::readme;
use std::collections::HashSet;

const REQUIRED_SECTIONS: &[&str] = &["Start here", "Repositories", "Reuse", "Optional"];
const LINK_SECTIONS: &[&str] = &["Start here", "Repositories", "Reuse", "Optional"];
const OWN_REPO_PREFIX: &str = "https://github.com/ryanduguid/";
const SITE_PREFIX: &str = "https://duguid.com.au/";
const REUSE_LINK: &str =
    "https://github.com/ryanduguid/ryanduguid/blob/main/AGENTS.md#reusing-this-code";
const LLMS_BLOB: &str = "https://github.com/ryanduguid/ryanduguid/blob/main/llms.txt";

const MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

struct Section<'a> {
    heading: &'a str,
    lines: Vec<&'a str>,
}

/// Today's UTC date as days since 1970-01-01.
pub fn today_utc_days() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .ok()
        .and_then(|elapsed| i64::try_from(elapsed.as_secs() / 86_400).ok())
        .unwrap_or(0)
}

/// Every link target in `llms.txt`: Markdown links plus bare `http(s)://` URLs
/// in visible text, deduplicated in order of first appearance.
pub fn links(text: &str) -> Vec<String> {
    let shown = readme::visible(text);
    let mut found = readme::links(&shown);
    for line in shown.lines() {
        for url in bare_urls(line) {
            if !found.contains(&url) {
                found.push(url);
            }
        }
    }
    found
}

fn bare_urls(line: &str) -> Vec<String> {
    let mut urls = Vec::new();
    let mut rest = line;
    while let Some(at) = find_scheme(rest) {
        let candidate = &rest[at..];
        let end = candidate
            .find(|c: char| c.is_whitespace() || matches!(c, ')' | ']' | '>' | '"' | '\'' | '`'))
            .unwrap_or(candidate.len());
        let cleaned = candidate[..end].trim_end_matches(['.', ',', ';', ':']);
        if cleaned.len() > "https://".len() {
            urls.push(cleaned.to_owned());
        }
        rest = &candidate[end.max(1)..];
    }
    urls
}

fn find_scheme(text: &str) -> Option<usize> {
    match (text.find("https://"), text.find("http://")) {
        (Some(a), Some(b)) => Some(a.min(b)),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    }
}

/// Checks `llms.txt` structure: H1 heading, blockquote summary, required
/// unique non-empty sections and well-formed `https://` bullet entries.
pub fn check_structure(text: &str) -> Vec<String> {
    let shown = readme::visible(text);
    if shown.trim().is_empty() {
        return vec!["llms.txt: file is empty".into()];
    }
    let mut drift = Vec::new();
    let (preamble, sections) = parse_sections(&shown);
    check_preamble(&preamble, &mut drift);
    check_sections(&sections, &mut drift);
    drift
}

fn parse_sections(shown: &str) -> (Vec<&str>, Vec<Section<'_>>) {
    let mut preamble = Vec::new();
    let mut sections: Vec<Section<'_>> = Vec::new();
    for line in shown.lines() {
        if let Some(heading) = line.strip_prefix("## ") {
            sections.push(Section {
                heading: heading.trim(),
                lines: Vec::new(),
            });
        } else if line.trim() == "##" {
            sections.push(Section {
                heading: "",
                lines: Vec::new(),
            });
        } else if let Some(current) = sections.last_mut() {
            current.lines.push(line);
        } else {
            preamble.push(line);
        }
    }
    (preamble, sections)
}

fn check_preamble(preamble: &[&str], drift: &mut Vec<String>) {
    let first = preamble
        .iter()
        .map(|l| l.trim())
        .find(|l| !l.is_empty())
        .unwrap_or_default();
    if first
        .strip_prefix("# ")
        .is_none_or(|title| title.trim().is_empty())
    {
        drift.push("llms.txt: must start with an H1 '# <title>' heading".into());
    }
    let has_summary = preamble.iter().any(|l| {
        l.trim_start()
            .strip_prefix("> ")
            .is_some_and(|summary| !summary.trim().is_empty())
    });
    if !has_summary {
        drift.push("llms.txt: missing blockquote '> ...' summary before the first section".into());
    }
}

fn check_sections(sections: &[Section<'_>], drift: &mut Vec<String>) {
    let mut seen_headings = HashSet::new();
    let mut seen_entries = HashSet::new();
    for section in sections {
        if section.heading.is_empty() {
            drift.push("llms.txt: empty '##' section heading".into());
            continue;
        }
        if !seen_headings.insert(section.heading.to_ascii_lowercase()) {
            drift.push(format!(
                "llms.txt: duplicate section '## {}'",
                section.heading
            ));
        }
        if section.lines.iter().all(|l| l.trim().is_empty()) {
            drift.push(format!(
                "llms.txt: section '## {}' is empty",
                section.heading
            ));
        }
        if LINK_SECTIONS.contains(&section.heading) {
            check_link_section(section, &mut seen_entries, drift);
        }
    }
    for &required in REQUIRED_SECTIONS {
        if !sections.iter().any(|s| s.heading == required) {
            drift.push(format!(
                "llms.txt: missing required section '## {required}'"
            ));
        }
    }
}

fn check_link_section(
    section: &Section<'_>,
    seen_entries: &mut HashSet<String>,
    drift: &mut Vec<String>,
) {
    let mut bullets = 0;
    for line in &section.lines {
        let trimmed = line.trim();
        let Some(entry) = trimmed.strip_prefix("- ") else {
            continue;
        };
        bullets += 1;
        match parse_entry_link(entry) {
            Some(url) => {
                if !seen_entries.insert(url.to_owned()) {
                    drift.push(format!("llms.txt: duplicate entry link '{url}'"));
                }
            }
            None => drift.push(format!(
                "llms.txt: malformed link entry in '## {}': {trimmed}",
                section.heading
            )),
        }
    }
    if bullets == 0 {
        drift.push(format!(
            "llms.txt: section '## {}' has no link entries",
            section.heading
        ));
    }
}

/// Parses `[Title](https://...)` at the start of a bullet entry.
fn parse_entry_link(entry: &str) -> Option<&str> {
    let rest = entry.strip_prefix('[')?;
    let (title, after_title) = rest.split_once("](")?;
    if title.trim().is_empty() || title.contains('[') || title.contains(']') {
        return None;
    }
    let (url, _) = after_title.split_once(')')?;
    let valid_url = url.starts_with("https://")
        && url.len() > "https://".len()
        && !url.chars().any(char::is_whitespace);
    valid_url.then_some(url)
}

/// Validates calendar dates in `llms.txt` and flags any `valid until` date
/// earlier than `today_days`.
pub fn check_dates(text: &str, today_days: i64) -> Vec<String> {
    let shown = readme::visible(text);
    let tokens: Vec<&str> = shown.split_whitespace().collect();
    let mut drift = Vec::new();
    for i in 0..tokens.len() {
        check_date_at(&tokens, i, today_days, &mut drift);
        check_valid_until_at(&tokens, i, &mut drift);
    }
    drift
}

fn check_date_at(tokens: &[&str], i: usize, today_days: i64, drift: &mut Vec<String>) {
    if i + 2 >= tokens.len() {
        return;
    }
    let d_tok = tokens[i].trim_start_matches(['(', '[', '\'', '"']);
    let m_tok = tokens[i + 1];
    let y_tok = tokens[i + 2].trim_end_matches(['.', ',', ';', ':', ')', ']', '\'', '"']);
    let Some(month_idx) = MONTHS.iter().position(|m| *m == m_tok) else {
        return;
    };
    let is_day = (1..=2).contains(&d_tok.len()) && d_tok.bytes().all(|b| b.is_ascii_digit());
    let is_year = y_tok.len() == 4 && y_tok.bytes().all(|b| b.is_ascii_digit());
    if !is_day || !is_year {
        return;
    }
    let label = format!("{d_tok} {m_tok} {y_tok}");
    let Some(days) = parse_calendar_date(d_tok, month_idx + 1, y_tok) else {
        drift.push(format!("llms.txt: invalid date '{label}'"));
        return;
    };
    if is_valid_until_prefix(tokens, i) && days < today_days {
        drift.push(format!(
            "llms.txt: stale date 'valid until {label}' has passed"
        ));
    }
}

fn check_valid_until_at(tokens: &[&str], i: usize, drift: &mut Vec<String>) {
    if !is_valid_until_prefix(tokens, i) {
        return;
    }
    let has_date = i + 2 < tokens.len() && {
        let d_tok = tokens[i].trim_start_matches(['(', '[', '\'', '"']);
        let m_tok = tokens[i + 1];
        let y_tok = tokens[i + 2].trim_end_matches(['.', ',', ';', ':', ')', ']', '\'', '"']);
        MONTHS.contains(&m_tok)
            && (1..=2).contains(&d_tok.len())
            && d_tok.bytes().all(|b| b.is_ascii_digit())
            && y_tok.len() == 4
            && y_tok.bytes().all(|b| b.is_ascii_digit())
    };
    if !has_date {
        let snippet = tokens[i..tokens.len().min(i + 3)].join(" ");
        drift.push(format!(
            "llms.txt: unreadable 'valid until' date near '{snippet}'"
        ));
    }
}

fn is_valid_until_prefix(tokens: &[&str], i: usize) -> bool {
    if i < 2 {
        return false;
    }
    fn clean(tok: &str) -> &str {
        tok.trim_matches(|c: char| !c.is_ascii_alphabetic())
    }
    clean(tokens[i - 2]).eq_ignore_ascii_case("valid")
        && clean(tokens[i - 1]).eq_ignore_ascii_case("until")
}

fn parse_calendar_date(d_tok: &str, month: usize, y_tok: &str) -> Option<i64> {
    let d: i64 = d_tok.parse().ok()?;
    let y: i64 = y_tok.parse().ok()?;
    let m = i64::try_from(month).ok()?;
    let days = days_from_civil(y, m, d);
    (civil_from_days(days) == (y, m, d) && d_tok == d.to_string()).then_some(days)
}

/// Checks that `llms.txt` stays in sync with `README.md` and `facts::FACTS`.
pub fn check_sync(readme_links: &[String], llms_text: &str, llms_links: &[String]) -> Vec<String> {
    let shown = readme::visible(llms_text);
    let mut drift = Vec::new();
    if !readme_links
        .iter()
        .any(|l| l == LLMS_BLOB || l == "llms.txt")
    {
        drift.push(format!("llms.txt: README.md no longer links {LLMS_BLOB}"));
    }
    check_repos_sync(readme_links, &shown, &mut drift);
    check_site_pages_sync(readme_links, llms_links, &mut drift);
    if !llms_links.iter().any(|l| l == REUSE_LINK) {
        drift.push(format!("llms.txt: missing reuse link {REUSE_LINK}"));
    }
    check_facts_sync(&shown, llms_links, &mut drift);
    drift
}

fn check_repos_sync(readme_links: &[String], llms_shown: &str, drift: &mut Vec<String>) {
    let (_, sections) = parse_sections(llms_shown);
    let repo_urls: HashSet<&str> = sections
        .iter()
        .find(|s| s.heading == "Repositories")
        .map(|s| {
            s.lines
                .iter()
                .filter_map(|l| l.trim().strip_prefix("- ").and_then(parse_entry_link))
                .collect()
        })
        .unwrap_or_default();
    let mut checked = HashSet::new();
    for link in readme_links {
        let Some(rest) = link.strip_prefix(OWN_REPO_PREFIX) else {
            continue;
        };
        let repo = rest.split(['/', '#']).next().unwrap_or_default();
        if repo.is_empty() || repo == "ryanduguid" || !checked.insert(repo) {
            continue;
        }
        let expected = format!("{OWN_REPO_PREFIX}{repo}");
        if !repo_urls.contains(expected.as_str()) {
            drift.push(format!(
                "llms.txt: missing repository '{repo}' linked in README.md"
            ));
        }
    }
}

fn check_site_pages_sync(readme_links: &[String], llms_links: &[String], drift: &mut Vec<String>) {
    let llms_bases: HashSet<&str> = llms_links
        .iter()
        .map(|l| l.split('#').next().unwrap_or(l.as_str()))
        .collect();
    let mut checked = HashSet::new();
    for link in readme_links {
        let base = link.split('#').next().unwrap_or(link.as_str());
        let Some(path) = base.strip_prefix(SITE_PREFIX) else {
            continue;
        };
        if path.is_empty() || !checked.insert(base) {
            continue;
        }
        if !llms_bases.contains(base) {
            drift.push(format!(
                "llms.txt: missing site page '{base}' linked in README.md"
            ));
        }
    }
}

fn check_facts_sync(llms_shown: &str, llms_links: &[String], drift: &mut Vec<String>) {
    for fact in FACTS {
        let place = format!("{}#{}", fact.url, fact.section);
        if !llms_links.contains(&place) {
            drift.push(format!(
                "llms.txt: {}: no longer links {place}; update tools/readme-check/src/facts.rs with its source",
                fact.name
            ));
        }
        for wording in fact.readme {
            if !llms_shown.contains(wording) {
                drift.push(format!(
                    "llms.txt: {}: no longer says '{wording}'; update tools/readme-check/src/facts.rs with it",
                    fact.name
                ));
            }
        }
    }
}

/// Days since 1970-01-01 of a proleptic Gregorian date (Howard Hinnant).
pub fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let doy = (153 * ((m + 9) % 12) + 2) / 5 + d - 1;
    era * 146_097 + yoe * 365 + yoe / 4 - yoe / 100 + doy - 719_468
}

fn civil_from_days(days: i64) -> (i64, i64, i64) {
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    (yoe + era * 400 + i64::from(m <= 2), m, d)
}

#[cfg(test)]
mod tests {
    use super::{
        LLMS_BLOB, REUSE_LINK, check_dates, check_structure, check_sync, days_from_civil, links,
        today_utc_days,
    };
    use crate::facts::FACTS;
    use crate::readme;
    use std::path::Path;

    fn fixture_llms() -> String {
        let fact = &FACTS[0];
        format!(
            "# Ryan Duguid\n\n\
> Summary.\n\n\
- Credentials: {} (certificate: {}#{}).\n\n\
## Start here\n\n\
- [Tools](https://duguid.com.au/tools/): tools.\n\n\
## Repositories\n\n\
- [Ozzit](https://github.com/ryanduguid/Ozzit): Excel functions.\n\n\
## Reuse\n\n\
- [Reusing this material]({REUSE_LINK}): CC BY 4.0.\n\n\
## Optional\n\n\
- [Contribution forks](https://raw.githubusercontent.com/ryanduguid/ryanduguid/main/FORKS.md): forks.\n",
            fact.readme.join(", "),
            fact.url,
            fact.section
        )
    }

    fn fixture_readme_links() -> Vec<String> {
        vec![
            "https://duguid.com.au/".into(),
            "https://duguid.com.au/tools/".into(),
            "https://github.com/ryanduguid/Ozzit".into(),
            format!("{}#{}", FACTS[0].url, FACTS[0].section),
            LLMS_BLOB.into(),
        ]
    }

    #[test]
    fn valid_fixture_passes_structure_dates_and_sync() {
        let doc = fixture_llms();
        assert!(check_structure(&doc).is_empty());
        assert!(check_dates(&doc, days_from_civil(2026, 11, 24)).is_empty());
        assert!(check_sync(&fixture_readme_links(), &doc, &links(&doc)).is_empty());
    }

    #[test]
    fn links_extracts_markdown_and_bare_urls_without_punctuation() {
        let text = "Index: https://duguid.com.au/llms.txt.\n\
(certificate: https://duguid.com.au/evidence/#xero-certification).\n\
- [Ozzit](https://github.com/ryanduguid/Ozzit): Overview: https://duguid.com.au/tools/ozzit/.\n\
<!-- https://hidden.test/ -->\n";
        assert_eq!(
            links(text),
            [
                "https://github.com/ryanduguid/Ozzit",
                "https://duguid.com.au/llms.txt",
                "https://duguid.com.au/evidence/#xero-certification",
                "https://duguid.com.au/tools/ozzit/"
            ]
        );
    }

    #[test]
    fn structure_catches_missing_parts_duplicates_and_bad_entries() {
        assert_eq!(check_structure("   "), ["llms.txt: file is empty"]);
        let bad = "## Start here\n\n- plain bullet\n\n## Repositories\n\n## Repositories\n\n\
- [A](https://example.test/a)\n- [B](https://example.test/a)\n";
        let findings = check_structure(bad);
        assert!(findings.iter().any(|f| f.contains("must start with an H1")));
        assert!(findings.iter().any(|f| f.contains("missing blockquote")));
        assert!(findings.iter().any(|f| f.contains("malformed link entry")));
        assert!(findings.iter().any(|f| f.contains("duplicate section")));
        assert!(
            findings
                .iter()
                .any(|f| f.contains("section '## Repositories' is empty"))
        );
        assert!(findings.iter().any(|f| f.contains("duplicate entry link")));
        assert!(
            findings
                .iter()
                .any(|f| f.contains("missing required section '## Optional'"))
        );
    }

    #[test]
    fn dates_catch_invalid_calendar_and_stale_valid_until_only() {
        let doc = "Awarded 1 July 2026 and valid until 1 July 2027. Cash falls on 6 November 2026.";
        assert!(check_dates(doc, days_from_civil(2026, 11, 24)).is_empty());
        assert!(check_dates(doc, days_from_civil(2027, 7, 1)).is_empty());
        let expired = check_dates(doc, days_from_civil(2027, 7, 2));
        assert_eq!(
            expired,
            ["llms.txt: stale date 'valid until 1 July 2027' has passed"]
        );
        let invalid = check_dates(
            "On 31 November 2026 and 01 July 2026.",
            days_from_civil(2026, 11, 24),
        );
        assert_eq!(invalid.len(), 2);
        assert!(invalid[0].contains("31 November 2026"));
        assert!(invalid[1].contains("01 July 2026"));
        let unreadable = check_dates("Valid until Jul 2027.", days_from_civil(2026, 11, 24));
        assert_eq!(unreadable.len(), 1);
        assert!(unreadable[0].contains("unreadable 'valid until' date"));
    }

    #[test]
    fn sync_catches_missing_repo_site_page_fact_and_readme_backlink() {
        let doc = fixture_llms();
        let mut rlinks = fixture_readme_links();
        rlinks.push("https://github.com/ryanduguid/OzFPA".into());
        rlinks.push("https://duguid.com.au/contact/".into());
        rlinks.retain(|l| l != LLMS_BLOB);
        let findings = check_sync(&rlinks, &doc, &links(&doc));
        assert!(
            findings
                .iter()
                .any(|f| f.contains("README.md no longer links"))
        );
        assert!(
            findings
                .iter()
                .any(|f| f.contains("missing repository 'OzFPA'"))
        );
        assert!(
            findings
                .iter()
                .any(|f| f.contains("missing site page 'https://duguid.com.au/contact/'"))
        );

        let hidden_fact = doc.replace(
            FACTS[0].readme[0],
            &format!("<!-- {} -->", FACTS[0].readme[0]),
        );
        let fact_findings = check_sync(&fixture_readme_links(), &hidden_fact, &links(&hidden_fact));
        assert!(fact_findings.iter().any(|f| f.contains("no longer says")));
    }

    #[test]
    fn the_real_llms_txt_passes_offline_checks() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let readme_text = std::fs::read_to_string(root.join("README.md")).unwrap();
        let llms_text = std::fs::read_to_string(root.join("llms.txt")).unwrap();
        let readme_links = readme::links(&readme::visible(&readme_text));
        let llms_links = links(&llms_text);
        assert_eq!(check_structure(&llms_text), Vec::<String>::new());
        assert_eq!(
            check_dates(&llms_text, today_utc_days()),
            Vec::<String>::new()
        );
        assert_eq!(
            check_sync(&readme_links, &llms_text, &llms_links),
            Vec::<String>::new()
        );
    }
}
