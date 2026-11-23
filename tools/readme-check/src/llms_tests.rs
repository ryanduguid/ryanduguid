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
fn fact_sync_tolerates_punctuation_and_rejects_mismatched_tokens() {
    let punctuated = fixture_llms().replace("valid until 1 July 2027", "valid until: 1 July, 2027");
    assert!(check_sync(&fixture_readme_links(), &punctuated, &links(&punctuated)).is_empty());

    let wrong_year = fixture_llms().replace("1 July 2027", "1 July 20270");
    let findings = check_sync(&fixture_readme_links(), &wrong_year, &links(&wrong_year));
    assert!(findings.iter().any(|f| f.contains("no longer says")));
}

#[test]
fn repo_sync_catches_stale_llms_repo_whilst_keeping_package_entries() {
    let doc = fixture_llms().replace(
        "- [Ozzit](https://github.com/ryanduguid/Ozzit): Excel functions.\n",
        "- [Ozzit](https://github.com/ryanduguid/Ozzit): Excel functions.\n\
- [OldRepo](https://github.com/ryanduguid/OldRepo): archived.\n\
- [aus-accounting-mcp](https://raw.githubusercontent.com/ryanduguid/australian-accounting/main/apps/aus-accounting-mcp/README.md): package.\n",
    );
    let findings = check_sync(&fixture_readme_links(), &doc, &links(&doc));
    assert_eq!(
        findings,
        ["llms.txt: repository 'OldRepo' is no longer linked in README.md"]
    );
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
