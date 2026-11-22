//! The README's 'Recent releases' block: the latest published release of each
//! accounting repository, newest first, dated in Sydney time.
//!
//! `readme-check releases` rebuilds the block from the GitHub REST API and
//! rewrites only the lines between its markers. Any failure, including an
//! unreadable repository or no releases at all, leaves README.md untouched
//! rather than writing a shorter list. An archived repository is left out.

use crate::net::{Outcome, Transport, fetch};
use serde_json::Value;

const OWNER: &str = "ryanduguid";

/// The accounting repositories: those the README links in its accounting
/// tasks, plus those the 14 November 2026 profile proposal listed.
pub const REPOS: &[&str] = &[
    "Ozzit",
    "au-fpa-pack",
    "accounting-review-pipeline",
    "australian-accounting",
    "australian-accounting-skills",
    "DrDebits",
    "PaulGoodman",
];

pub const START: &str = "<!-- recent-releases:start -->";
pub const END: &str = "<!-- recent-releases:end -->";

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Release {
    pub repo: &'static str,
    pub tag: String,
    /// `published_at` as the API gives it, RFC 3339 in UTC.
    pub published: String,
    pub date: String,
}

/// The README with its release block rebuilt from the API.
pub fn update(markdown: &str, transport: &mut dyn Transport) -> Result<String, String> {
    let lines: Vec<String> = latest(transport)?
        .iter()
        .map(|r| line(r.repo, &r.tag, &r.date))
        .collect();
    let new = replace(markdown, &lines)?;
    check_block(&new)?;
    Ok(new)
}

/// The latest release of each unarchived repository, newest first.
pub fn latest(transport: &mut dyn Transport) -> Result<Vec<Release>, String> {
    let mut found = Vec::new();
    for &repo in REPOS {
        let api = format!("https://api.github.com/repos/{OWNER}/{repo}");
        let archived = json(transport, &api)?["archived"].as_bool();
        match archived {
            Some(true) => continue,
            Some(false) => {}
            None => return Err(format!("{api}: no 'archived' field")),
        }
        found.push(release(
            repo,
            &json(transport, &format!("{api}/releases/latest"))?,
        )?);
    }
    if found.is_empty() {
        return Err("no repository has a published release".into());
    }
    found.sort_by(|a, b| b.published.cmp(&a.published).then(a.repo.cmp(b.repo)));
    Ok(found)
}

fn json(transport: &mut dyn Transport, url: &str) -> Result<Value, String> {
    match fetch(transport, url) {
        Outcome::Ok(body) => serde_json::from_str(&body).map_err(|e| format!("{url}: {e}")),
        Outcome::Dead(why) | Outcome::Transient(why) => Err(format!("{url}: {why}")),
    }
}

fn release(repo: &'static str, json: &Value) -> Result<Release, String> {
    let field = |name: &str| {
        json[name]
            .as_str()
            .map(str::to_owned)
            .ok_or_else(|| format!("{repo}: the latest release has no {name}"))
    };
    if json["draft"] != false || json["prerelease"] != false {
        return Err(format!(
            "{repo}: the latest release is a draft or prerelease"
        ));
    }
    let tag = field("tag_name")?;
    let safe = |c: char| c.is_ascii_alphanumeric() || "._-/+".contains(c);
    if tag.is_empty() || !tag.chars().all(safe) {
        return Err(format!("{repo}: unexpected tag name '{tag}'"));
    }
    let published = field("published_at")?;
    let date = sydney_date(&published)
        .ok_or_else(|| format!("{repo}: unexpected published_at '{published}'"))?;
    Ok(Release {
        repo,
        tag,
        published,
        date,
    })
}

fn line(repo: &str, tag: &str, date: &str) -> String {
    format!("- **{repo}** [{tag}](https://github.com/{OWNER}/{repo}/releases/tag/{tag}), {date}")
}

/// `markdown` with the lines between the markers replaced by `lines`.
fn replace(markdown: &str, lines: &[String]) -> Result<String, String> {
    let (before, _, after) = split(markdown)?;
    Ok(format!(
        "{before}{START}\n\n{}\n\n{END}{after}",
        lines.join("\n")
    ))
}

/// The text before the start marker, between the markers and after the end one.
fn split(markdown: &str) -> Result<(&str, &str, &str), String> {
    if markdown.matches(START).count() != 1 || markdown.matches(END).count() != 1 {
        return Err(format!("README.md must hold {START} and {END} once each"));
    }
    let (before, rest) = markdown.split_once(START).unwrap_or_default();
    let (inner, after) = rest
        .split_once(END)
        .ok_or_else(|| format!("{END} comes before {START}"))?;
    Ok((before, inner, after))
}

/// Checks the block as committed: each line in the generated format, each
/// repository at most once and known, dates valid and newest first.
pub fn check_block(markdown: &str) -> Result<usize, String> {
    let (_, inner, _) = split(markdown)?;
    let mut seen: Vec<&str> = Vec::new();
    let mut previous = None;
    for text in inner.lines().filter(|l| !l.trim().is_empty()) {
        let bad = || format!("release block: unexpected line '{text}'");
        let (repo, tag, date) = parse_line(text).ok_or_else(bad)?;
        let day = parse_date(date).ok_or_else(bad)?;
        if !REPOS.contains(&repo) || seen.contains(&repo) || line(repo, tag, date) != text {
            return Err(bad());
        }
        if previous.is_some_and(|p| p < day) {
            return Err(format!("release block: '{text}' is out of date order"));
        }
        seen.push(repo);
        previous = Some(day);
    }
    if seen.is_empty() {
        return Err("release block: no releases".into());
    }
    Ok(seen.len())
}

fn parse_line(line: &str) -> Option<(&str, &str, &str)> {
    let (repo, rest) = line.strip_prefix("- **")?.split_once("** [")?;
    let (tag, rest) = rest.split_once("](")?;
    let (_, date) = rest.split_once("), ")?;
    Some((repo, tag, date))
}

/// 'D Month YYYY' as days since 1970, when it names a real date.
fn parse_date(date: &str) -> Option<i64> {
    let mut parts = date.split(' ');
    let (d, month, y) = (parts.next()?, parts.next()?, parts.next()?);
    let m = MONTHS.iter().position(|name| *name == month)?;
    let (d, y): (i64, i64) = (d.parse().ok()?, y.parse().ok()?);
    let days = days_from_civil(y, i64::try_from(m).ok()? + 1, d);
    let canonical = civil_from_days(days) == (y, i64::try_from(m).ok()? + 1, d);
    (parts.next().is_none() && canonical && date == format!("{d} {month} {y}")).then_some(days)
}

/// The calendar date in Sydney of a UTC time 'YYYY-MM-DDTHH:MM:SSZ', as
/// 'D Month YYYY'. NSW daylight saving runs from 2 am standard time on the
/// first Sunday in October to 2 am standard time on the first Sunday in April.
pub fn sydney_date(utc: &str) -> Option<String> {
    let (date, time) = utc.strip_suffix('Z')?.split_once('T')?;
    let numbers =
        |text: &str, sep| -> Option<Vec<i64>> { text.split(sep).map(|n| n.parse().ok()).collect() };
    let (ymd, hms) = (numbers(date, '-')?, numbers(time, ':')?);
    let ([y, m, d], [h, mi, s]) = (
        <[i64; 3]>::try_from(ymd).ok()?,
        <[i64; 3]>::try_from(hms).ok()?,
    );
    if civil_from_days(days_from_civil(y, m, d)) != (y, m, d) || h > 23 || mi > 59 || s > 60 {
        return None;
    }
    let standard = days_from_civil(y, m, d) * 86_400 + h * 3_600 + mi * 60 + s + 10 * 3_600;
    let (year, ..) = civil_from_days(standard.div_euclid(86_400));
    let changes = |month| {
        let first = days_from_civil(year, month, 1);
        (first + (3 - first).rem_euclid(7)) * 86_400 + 2 * 3_600
    };
    let daylight = standard >= changes(10) || standard < changes(4);
    let local = standard + if daylight { 3_600 } else { 0 };
    let (y, m, d) = civil_from_days(local.div_euclid(86_400));
    Some(format!("{d} {} {y}", MONTHS[usize::try_from(m - 1).ok()?]))
}

/// Days since 1970-01-01 of a proleptic Gregorian date (Howard Hinnant).
fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
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
    use super::{END, REPOS, START, check_block, latest, sydney_date, update};
    use crate::net::fake::{Fake, status};

    const API: &str = "https://api.github.com/repos/ryanduguid/";

    fn release(tag: &str, published: &str) -> String {
        format!(
            r#"{{"tag_name": "{tag}", "published_at": "{published}", "draft": false, "prerelease": false}}"#
        )
    }

    /// Every repository unarchived, each released on day `n + 1` of November.
    fn world() -> Fake {
        let mut net = Fake::default();
        for (n, repo) in REPOS.iter().enumerate() {
            net = net
                .ok(&format!("{API}{repo}"), r#"{"archived": false}"#)
                .ok(
                    &format!("{API}{repo}/releases/latest"),
                    &release(
                        &format!("v1.{n}.0"),
                        &format!("2026-11-{:02}T01:00:00Z", n + 1),
                    ),
                );
        }
        net
    }

    fn readme(inner: &str) -> String {
        format!("# Me\n\n## Recent releases\n\n{START}\n{inner}\n{END}\n\n## Next\n")
    }

    #[test]
    fn the_block_lists_newest_first_and_changes_nothing_else() {
        let new = update(&readme("old"), &mut world()).unwrap();
        let expected = "- **PaulGoodman** [v1.6.0](https://github.com/ryanduguid/PaulGoodman/releases/tag/v1.6.0), 7 November 2026\n\
- **DrDebits** [v1.5.0](https://github.com/ryanduguid/DrDebits/releases/tag/v1.5.0), 6 November 2026";
        assert!(new.starts_with(&format!(
            "# Me\n\n## Recent releases\n\n{START}\n\n{expected}\n"
        )));
        assert!(new.ends_with("- **Ozzit** [v1.0.0](https://github.com/ryanduguid/Ozzit/releases/tag/v1.0.0), 1 November 2026\n\n<!-- recent-releases:end -->\n\n## Next\n"));
        assert_eq!(check_block(&new), Ok(REPOS.len()));
        assert_eq!(update(&new, &mut world()).unwrap(), new);
    }

    #[test]
    fn an_archived_repository_is_left_out() {
        let mut net = world().ok(&format!("{API}Ozzit"), r#"{"archived": true}"#);
        let found = latest(&mut net).unwrap();
        assert_eq!(found.len(), REPOS.len() - 1);
        assert!(found.iter().all(|r| r.repo != "Ozzit"));
        assert!(
            !net.calls
                .iter()
                .any(|url| url.ends_with("Ozzit/releases/latest"))
        );
    }

    #[test]
    fn an_api_error_fails_instead_of_shortening_the_list() {
        let missing = format!("{API}DrDebits/releases/latest");
        let mut net = world().script(&missing, vec![status(404, "")]);
        assert_eq!(
            update(&readme("old"), &mut net),
            Err(format!("{missing}: HTTP 404"))
        );
        let mut net = world().script(&format!("{API}Ozzit"), vec![status(503, "")]);
        assert!(update(&readme("old"), &mut net).is_err());
    }

    #[test]
    fn no_releases_at_all_is_an_error() {
        let mut net = Fake::default();
        for repo in REPOS {
            net = net.ok(&format!("{API}{repo}"), r#"{"archived": true}"#);
        }
        assert!(update(&readme("old"), &mut net).is_err());
    }

    #[test]
    fn odd_answers_are_errors() {
        let latest_of = format!("{API}Ozzit/releases/latest");
        for body in [
            r#"{"tag_name": "v1", "published_at": "2026-11-01T00:00:00Z", "draft": false, "prerelease": true}"#.to_owned(),
            release("v1](https://x.test)", "2026-11-01T00:00:00Z"),
            release("v1", "1 November 2026"),
            "not json".to_owned(),
        ] {
            assert!(update(&readme(""), &mut world().ok(&latest_of, &body)).is_err());
        }
        assert!(update(&readme(""), &mut world().ok(&format!("{API}Ozzit"), "{}")).is_err());
    }

    #[test]
    fn missing_or_repeated_markers_are_errors() {
        assert!(update("# Me\n", &mut world()).is_err());
        assert!(update(&format!("{}{START}", readme("")), &mut world()).is_err());
        assert!(update(&format!("{END}\n{START}\n"), &mut world()).is_err());
    }

    #[test]
    fn dates_are_sydney_calendar_dates() {
        for (utc, date) in [
            ("2026-06-30T13:59:59Z", "30 June 2026"),
            ("2026-06-30T14:00:00Z", "1 July 2026"),
            ("2026-11-30T12:59:59Z", "30 November 2026"),
            ("2026-11-30T13:00:00Z", "1 December 2026"),
            ("2026-11-20T18:19:12Z", "21 November 2026"),
            // Daylight saving starts at 16:00 UTC on Saturday 3 October 2026.
            ("2026-10-03T13:30:00Z", "3 October 2026"),
            ("2026-10-04T13:30:00Z", "5 October 2026"),
            // And ends at 16:00 UTC on Saturday 4 April 2026.
            ("2026-04-04T13:30:00Z", "5 April 2026"),
            ("2026-04-05T13:30:00Z", "5 April 2026"),
            ("2024-02-29T00:00:00Z", "29 February 2024"),
        ] {
            assert_eq!(sydney_date(utc).as_deref(), Some(date), "{utc}");
        }
        for bad in [
            "2026-02-30T00:00:00Z",
            "2026-11-20T18:19:12",
            "2026-11-20",
            "",
        ] {
            assert_eq!(sydney_date(bad), None, "{bad}");
        }
    }

    #[test]
    fn the_block_check_rejects_hand_edits() {
        let line = |repo: &str, tag: &str, date: &str| {
            format!(
                "- **{repo}** [{tag}](https://github.com/ryanduguid/{repo}/releases/tag/{tag}), {date}"
            )
        };
        let good = line("Ozzit", "v3.7.0", "18 November 2026");
        let older = line("DrDebits", "v0.7.0", "14 November 2026");
        assert_eq!(check_block(&readme(&format!("\n{good}\n{older}\n"))), Ok(2));
        for inner in [
            String::new(),
            format!("{older}\n{good}"),
            format!("{good}\n{good}"),
            line("Croft-Manor", "v1", "1 November 2026"),
            line("Ozzit", "v1", "31 November 2026"),
            line("Ozzit", "v1", "18 Nov 2026"),
            good.replace("tag/v3.7.0", "tag/v3.6.0"),
            format!("{good} and more"),
        ] {
            assert!(check_block(&readme(&inner)).is_err(), "{inner}");
        }
    }
}
