//! The checks themselves, collecting findings into one report.

use crate::facts::{FACTS, Fact, OZZIT_LATEST_RELEASE, csv_counts, ozzit_functions_csv};
use crate::html;
use crate::llms;
use crate::net::{Outcome, Transport, fetch};
use crate::readme;
use std::collections::HashMap;
use std::path::Path;

const OWN_BLOB: &str = "https://github.com/ryanduguid/ryanduguid/blob/main/";
const OWN_RAW: &str = "https://raw.githubusercontent.com/ryanduguid/ryanduguid/main/";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    /// The README disagrees with its source, or a link is dead.
    Drift,
    /// A source could not be read in this run, so nothing is known.
    Inconclusive,
}

#[derive(Debug, Default)]
pub struct Report {
    pub findings: Vec<(Severity, String)>,
    pub checked: Vec<String>,
}

impl Report {
    /// 0 clean, 1 drift, 2 inconclusive without drift.
    pub fn exit_code(&self) -> i32 {
        let has = |s| self.findings.iter().any(|(severity, _)| *severity == s);
        if has(Severity::Drift) {
            1
        } else if has(Severity::Inconclusive) {
            2
        } else {
            0
        }
    }

    fn drift(&mut self, message: String) {
        self.findings.push((Severity::Drift, message));
    }
}

struct Checker<'a> {
    transport: &'a mut dyn Transport,
    pages: HashMap<String, Outcome>,
    report: Report,
}

/// Runs every check on `markdown` (`README.md`) and `llms_text` (`llms.txt`)
/// at the root of `root`.
pub fn run(
    markdown: &str,
    llms_text: &str,
    root: &Path,
    today_days: i64,
    transport: &mut dyn Transport,
) -> Report {
    let mut checker = Checker {
        transport,
        pages: HashMap::new(),
        report: Report::default(),
    };
    let shown = readme::visible(markdown);
    let links = readme::links(&shown);
    checker.links(&links, markdown, root);
    checker.ozzit(&shown);
    for fact in FACTS {
        checker.fact(&shown, &links, fact);
    }
    checker.llms(&links, llms_text, root, today_days);
    checker.report
}

impl Checker<'_> {
    /// The page body, fetched once per URL. A failure is reported on every use,
    /// naming `what` needed it.
    fn page(&mut self, url: &str, what: &str) -> Option<String> {
        let transport = &mut *self.transport;
        let outcome = self
            .pages
            .entry(url.to_owned())
            .or_insert_with(|| fetch(transport, url))
            .clone();
        let (severity, why) = match outcome {
            Outcome::Ok(body) => return Some(body),
            Outcome::Dead(why) => (Severity::Drift, why),
            Outcome::Transient(why) => (Severity::Inconclusive, why),
        };
        self.report
            .findings
            .push((severity, format!("{what}: {url}: {why}")));
        None
    }

    fn links(&mut self, targets: &[String], markdown: &str, root: &Path) {
        self.check_targets(targets, markdown, root);
        self.report.checked.push(format!("{} links", targets.len()));
    }

    fn check_targets(&mut self, targets: &[String], markdown: &str, root: &Path) {
        for target in targets {
            let (base, fragment) = match target.split_once('#') {
                Some((base, fragment)) => (base, Some(fragment)),
                None => (target.as_str(), None),
            };
            if base.starts_with("https://") || base.starts_with("http://") {
                self.remote_link(base, fragment, root);
            } else if !base.contains(':') {
                self.local_link(target, base, fragment, root, markdown);
            }
        }
    }

    fn llms(&mut self, readme_links: &[String], llms_text: &str, root: &Path, today_days: i64) {
        let llms_links = llms::links(llms_text);
        let extra: Vec<String> = llms_links
            .iter()
            .filter(|link| !readme_links.contains(*link))
            .cloned()
            .collect();
        self.check_targets(&extra, llms_text, root);
        for message in llms::check_structure(llms_text)
            .into_iter()
            .chain(llms::check_dates(llms_text, today_days))
            .chain(llms::check_sync(readme_links, llms_text, &llms_links))
        {
            self.report.drift(message);
        }
        self.report
            .checked
            .push(format!("llms.txt ({} links)", llms_links.len()));
    }

    fn remote_link(&mut self, base: &str, fragment: Option<&str>, root: &Path) {
        let Some(body) = self.page(base, "Dead link") else {
            return;
        };
        if let Some(path) = base
            .strip_prefix(OWN_BLOB)
            .or_else(|| base.strip_prefix(OWN_RAW))
        {
            let local = root.join(path);
            if !local.is_file() {
                self.report
                    .drift(format!("Dead link: {base}: no such file in the repository"));
                return;
            }
            if let Some(fragment) = fragment {
                // GitHub renders headings client-side; the anchor is checked in the
                // file itself, as this branch has it.
                let markdown = std::fs::read_to_string(&local).unwrap_or_default();
                if !readme::has_heading(&markdown, fragment) {
                    self.report.drift(format!(
                        "Missing anchor: {base}#{fragment}: no such heading in {path}"
                    ));
                }
            }
            return;
        }
        let Some(fragment) = fragment else { return };
        if !base.starts_with("https://github.com/") && !html::has_anchor(&body, fragment) {
            self.report.drift(format!(
                "Missing anchor: {base}#{fragment}: no element has that id"
            ));
        }
    }

    fn local_link(
        &mut self,
        target: &str,
        base: &str,
        fragment: Option<&str>,
        root: &Path,
        readme: &str,
    ) {
        let path = root.join(base);
        let content = if base.is_empty() {
            Some(readme.to_owned())
        } else if path.exists() {
            std::fs::read_to_string(&path).ok()
        } else {
            self.report.drift(format!(
                "Dead link: {target}: no such file in the repository"
            ));
            return;
        };
        if let Some(fragment) = fragment
            && !readme::has_heading(&content.unwrap_or_default(), fragment)
        {
            self.report
                .drift(format!("Missing anchor: {target}: no such heading"));
        }
    }

    fn ozzit(&mut self, markdown: &str) {
        let Some((functions, help)) = readme::ozzit_counts(markdown) else {
            self.report.drift("Ozzit: the README no longer says 'N native Excel LAMBDA functions plus M help tables'".into());
            return;
        };
        let Some(release) = self.page(OZZIT_LATEST_RELEASE, "Ozzit's latest release") else {
            return;
        };
        let tag = serde_json::from_str::<serde_json::Value>(&release)
            .ok()
            .and_then(|json| json["tag_name"].as_str().map(str::to_owned));
        let Some(tag) = tag else {
            self.report
                .drift("Ozzit: the latest release has no tag_name".into());
            return;
        };
        let Some(csv) = self.page(&ozzit_functions_csv(&tag), "Ozzit's functions.csv") else {
            return;
        };
        // Honest weights and measures (Proverbs 11:1, NET): the README's
        // counts must equal those the released catalogue derives.
        match csv_counts(&csv) {
            Ok(actual) if actual == (functions, help) => {
                self.report.checked.push(format!(
                    "Ozzit {tag}: {functions} functions, {help} help tables"
                ));
            }
            Ok((want_functions, want_help)) => self.report.drift(format!(
                "Ozzit: the README says {functions} functions plus {help} help tables; \
functions.csv at {tag} has {want_functions} plus {want_help}"
            )),
            Err(why) => self.report.drift(format!("Ozzit: {tag}: {why}")),
        }
    }

    fn fact(&mut self, markdown: &str, links: &[String], fact: &Fact) {
        let place = format!("{}#{}", fact.url, fact.section);
        if !links.contains(&place) {
            self.report.drift(format!(
                "{}: the README no longer links {place}; update tools/readme-check/src/facts.rs with its source",
                fact.name
            ));
        }
        for wording in fact.readme {
            if !markdown.contains(wording) {
                self.report.drift(format!(
                    "{}: the README no longer says '{wording}'; update tools/readme-check/src/facts.rs with it",
                    fact.name
                ));
            }
        }
        let Some(body) = self.page(fact.url, fact.name) else {
            return;
        };
        let Some(scope) = html::section(&body, fact.section) else {
            self.report
                .drift(format!("{}: {place} not found", fact.name));
            return;
        };
        let text = html::text(scope);
        for wording in fact.page {
            if !text.contains(wording) {
                self.report.drift(format!(
                    "{}: {place} no longer shows '{wording}'",
                    fact.name
                ));
            }
        }
        self.report.checked.push(fact.name.to_owned());
    }
}

#[cfg(test)]
mod tests {
    use super::{OWN_BLOB, OWN_RAW, run};
    use crate::facts::{FACTS, Fact, OZZIT_LATEST_RELEASE, ozzit_functions_csv};
    use crate::llms;
    use crate::net::fake::{Fake, status};
    use crate::{html, readme};
    use std::collections::HashMap;
    use std::fmt::Write as _;
    use std::path::{Path, PathBuf};

    const BADGE: &str = "https://badgen.net/github/release/ryanduguid/Ozzit/stable";
    const LOST: &str = "https://duguid.com.au/croft-manor/";

    fn root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
    }

    fn today() -> i64 {
        llms::days_from_civil(2026, 11, 24)
    }

    fn csv(functions: u32, help: u32) -> String {
        let mut csv = String::from("function,module,previous_name,signature,description\n");
        for n in 0..functions {
            writeln!(csv, "oz.Fn{n}λ,Finance,\"Does {n}, quoted\"").unwrap();
        }
        for n in 0..help {
            writeln!(csv, "oz.About{n}λ,Help,Help table").unwrap();
        }
        csv
    }

    /// A network where Ozzit's release has the given counts and every fact page
    /// shows its wording inside the section the README links to.
    fn world(functions: u32, help: u32) -> Fake {
        world_linked_from(functions, help, "", "")
    }

    /// `world`, where each page `markdown` or `llms_text` links to a fragment of
    /// also has an element with that id, so only a missing local file or heading
    /// can fail.
    fn world_linked_from(functions: u32, help: u32, markdown: &str, llms_text: &str) -> Fake {
        let mut net = Fake::default()
            .ok(OZZIT_LATEST_RELEASE, r#"{"tag_name": "v9.9.9"}"#)
            .ok(&ozzit_functions_csv("v9.9.9"), &csv(functions, help));
        let mut pages: HashMap<String, String> = HashMap::new();
        for fact in FACTS {
            let section = fact_section(fact, &fact.page.join(" and "));
            pages
                .entry(fact.url.to_owned())
                .or_default()
                .push_str(&section);
        }
        let all_links = readme::links(&readme::visible(markdown))
            .into_iter()
            .chain(llms::links(llms_text));
        for link in all_links {
            let Some((url, id)) = link.split_once('#') else {
                continue;
            };
            if url.starts_with("https://") {
                let page = pages.entry(url.to_owned()).or_default();
                if !html::has_anchor(page, id) {
                    write!(page, "<p id=\"{id}\"></p>").unwrap();
                }
            }
        }
        for (url, page) in pages {
            net = net.ok(&url, &format!("<body>{page}</body>"));
        }
        net
    }

    fn fact_section(fact: &Fact, text: &str) -> String {
        format!("<section id=\"{}\"><p>{text}</p></section>", fact.section)
    }

    fn fact_link(fact: &Fact) -> String {
        format!("{}#{}", fact.url, fact.section)
    }

    /// A README that states every fact and links each fact's source.
    fn fixture_readme(extra: &str) -> String {
        let facts: Vec<String> = FACTS
            .iter()
            .map(|f| format!("[{}]({}) {}", f.name, fact_link(f), f.readme.join(", ")))
            .collect();
        format!(
            "[Ozzit](https://github.com/ryanduguid/Ozzit): 150 native Excel LAMBDA functions plus 5 help tables.\n\
[badge]({BADGE}) [notes]({OWN_BLOB}AGENTS.md#reusing-this-code) [llms]({OWN_BLOB}llms.txt)\n{}\n{extra}\n",
            facts.join(". ")
        )
    }

    fn fixture_llms(extra_optional: &str) -> String {
        let facts: Vec<String> = FACTS
            .iter()
            .map(|f| format!("{} ({})", f.readme.join(", "), fact_link(f)))
            .collect();
        format!(
            "# Ryan Duguid\n\n\
> Summary.\n\n\
- Credentials: {}\n\n\
## Start here\n\n\
- [Evidence](https://duguid.com.au/evidence/): evidence.\n\n\
## Repositories\n\n\
- [Ozzit](https://github.com/ryanduguid/Ozzit): Excel functions.\n\n\
## Reuse\n\n\
- [Reusing this material]({OWN_BLOB}AGENTS.md#reusing-this-code): CC BY 4.0.\n\n\
## Optional\n\n\
- [Contribution forks]({OWN_RAW}FORKS.md): forks.\n\
{extra_optional}",
            facts.join("; ")
        )
    }

    fn drift_with(net: &mut Fake, markdown: &str, llms_text: &str, day: i64) -> (i32, String) {
        let report = run(markdown, llms_text, &root(), day, net);
        let messages: Vec<String> = report.findings.iter().map(|(_, m)| m.clone()).collect();
        (report.exit_code(), messages.join("\n"))
    }

    fn drift(net: &mut Fake, markdown: &str) -> (i32, String) {
        drift_with(net, markdown, &fixture_llms(""), today())
    }

    #[test]
    fn a_matching_readme_is_clean() {
        assert_eq!(
            drift(&mut world(150, 5), &fixture_readme("")),
            (0, String::new())
        );
    }

    #[test]
    fn a_wrong_function_count_is_drift() {
        let (code, messages) = drift(&mut world(151, 5), &fixture_readme(""));
        assert_eq!(code, 1);
        assert!(messages.contains(
            "README says 150 functions plus 5 help tables; functions.csv at v9.9.9 has 151 plus 5"
        ));
    }

    #[test]
    fn a_dead_link_is_drift() {
        let mut net = world(150, 5).script(LOST, vec![status(404, "")]);
        let (code, messages) = drift_with(
            &mut net,
            &fixture_readme(&format!("[Croft Manor]({LOST})")),
            &fixture_llms(&format!("- [Croft Manor]({LOST}): lost.\n")),
            today(),
        );
        assert_eq!(
            (code, messages),
            (1, format!("Dead link: {LOST}: HTTP 404"))
        );
    }

    #[test]
    fn a_missing_file_or_anchor_is_drift() {
        let extra = format!("![x](assets/croft-manor.png) [y]({OWN_BLOB}AGENTS.md#croft-manor)");
        let (code, messages) = drift(&mut world(150, 5), &fixture_readme(&extra));
        assert_eq!(code, 1);
        assert!(messages.contains("Dead link: assets/croft-manor.png"));
        assert!(messages.contains("AGENTS.md#croft-manor: no such heading"));
    }

    #[test]
    fn a_rate_limited_badge_is_inconclusive_not_drift() {
        let mut net = world(150, 5).script(BADGE, vec![status(429, "")]);
        assert_eq!(drift(&mut net, &fixture_readme("")).0, 2);
    }

    #[test]
    fn a_changed_figure_on_the_site_is_drift() {
        let fact = &FACTS[0];
        let mut net = world(150, 5).ok(
            fact.url,
            &format!("<body>{}</body>", fact_section(fact, "($99,999)")),
        );
        let (code, messages) = drift(&mut net, &fixture_readme(""));
        assert_eq!(code, 1);
        let expected: Vec<String> = fact
            .page
            .iter()
            .map(|wording| {
                format!(
                    "{}: {} no longer shows '{wording}'",
                    fact.name,
                    fact_link(fact)
                )
            })
            .collect();
        assert_eq!(messages, expected.join("\n"));
    }

    #[test]
    fn a_figure_dropped_from_the_readme_is_drift() {
        let wording = FACTS[0].readme[0];
        let markdown = fixture_readme("").replace(wording, "Croft Manor");
        let (code, messages) = drift(&mut world(150, 5), &markdown);
        assert_eq!(code, 1);
        assert!(messages.contains(&format!("the README no longer says '{wording}'")));
    }

    /// The real README and llms.txt pass offline: their local files and anchors
    /// exist, they state the Ozzit counts and every fact's wording, link every
    /// fact page, stay in sync and hold a valid release block.
    #[test]
    fn the_real_readme_passes_against_a_matching_world() {
        let markdown = std::fs::read_to_string(root().join("README.md")).unwrap();
        let llms_text = std::fs::read_to_string(root().join("llms.txt")).unwrap();
        let (functions, help) =
            readme::ozzit_counts(&readme::visible(&markdown)).expect("Ozzit counts");
        let mut net = world_linked_from(functions, help, &markdown, &llms_text);
        assert_eq!(
            drift_with(&mut net, &markdown, &llms_text, llms::today_utc_days()),
            (0, String::new())
        );
        crate::releases::check_block(&markdown).expect("the Recent releases block");
    }

    #[test]
    fn a_fact_whose_source_link_changed_is_drift() {
        let fact = &FACTS[0];
        let moved = format!("{}?croft-manor#{}", fact.url, fact.section);
        let markdown =
            fixture_readme("").replace(&format!("({})", fact_link(fact)), &format!("({moved})"));
        let (code, messages) = drift(&mut world(150, 5), &markdown);
        assert_eq!(code, 1);
        assert!(messages.contains(&format!("the README no longer links {}", fact_link(fact))));
    }

    #[test]
    fn a_figure_kept_only_in_a_comment_is_drift() {
        let wording = FACTS[0].readme[0];
        let markdown = fixture_readme("").replace(wording, &format!("<!-- {wording} -->"));
        let (code, messages) = drift(&mut world(150, 5), &markdown);
        assert_eq!(code, 1);
        assert!(messages.contains(&format!("the README no longer says '{wording}'")));
    }

    #[test]
    fn llms_txt_dead_link_missing_local_file_and_stale_date_are_drift() {
        let mut net = world(150, 5).script(LOST, vec![status(404, "")]);
        let llms_doc = fixture_llms(&format!(
            "- [Croft]({LOST}): dead.\n- [Gone]({OWN_RAW}CROFT.md): missing.\n"
        ));
        let (code, messages) = drift_with(
            &mut net,
            &fixture_readme(""),
            &llms_doc,
            llms::days_from_civil(2027, 7, 2),
        );
        assert_eq!(code, 1);
        assert!(messages.contains(&format!("Dead link: {LOST}: HTTP 404")));
        assert!(messages.contains("CROFT.md: no such file in the repository"));
        assert!(messages.contains("stale date 'valid until 1 July 2027' has passed"));
    }
}
