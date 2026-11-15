//! The checks themselves, collecting findings into one report.

use crate::facts::{FACTS, Fact, OZZIT_LATEST_RELEASE, csv_counts, ozzit_functions_csv};
use crate::html;
use crate::net::{Outcome, Transport, fetch};
use crate::readme;
use std::collections::HashMap;
use std::path::Path;

const OWN_BLOB: &str = "https://github.com/ryanduguid/ryanduguid/blob/main/";

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

/// Runs every check on `markdown`, the README at the root of `root`.
pub fn run(markdown: &str, root: &Path, transport: &mut dyn Transport) -> Report {
    let mut checker = Checker {
        transport,
        pages: HashMap::new(),
        report: Report::default(),
    };
    checker.links(markdown, root);
    checker.ozzit(markdown);
    for fact in FACTS {
        checker.fact(markdown, fact);
    }
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

    fn links(&mut self, markdown: &str, root: &Path) {
        let targets = readme::links(markdown);
        for target in &targets {
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
        self.report.checked.push(format!("{} links", targets.len()));
    }

    fn remote_link(&mut self, base: &str, fragment: Option<&str>, root: &Path) {
        let Some(body) = self.page(base, "Dead link") else {
            return;
        };
        let Some(fragment) = fragment else { return };
        if let Some(path) = base.strip_prefix(OWN_BLOB) {
            // GitHub renders headings client-side; the anchor is checked in the
            // file itself, as this branch has it.
            let markdown = std::fs::read_to_string(root.join(path)).unwrap_or_default();
            if !readme::has_heading(&markdown, fragment) {
                self.report.drift(format!(
                    "Missing anchor: {base}#{fragment}: no such heading in {path}"
                ));
            }
        } else if !base.starts_with("https://github.com/") && !html::has_anchor(&body, fragment) {
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

    fn fact(&mut self, markdown: &str, fact: &Fact) {
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
        let place = fact
            .section
            .map_or_else(|| fact.url.to_owned(), |id| format!("{}#{id}", fact.url));
        let scope = match fact.section {
            Some(id) => html::section(&body, id),
            None => body.find("<body").map(|at| &body[at..]),
        };
        let Some(scope) = scope else {
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
    use super::{OWN_BLOB, run};
    use crate::facts::{FACTS, OZZIT_LATEST_RELEASE, ozzit_functions_csv};
    use crate::net::fake::{Fake, status};
    use crate::readme;
    use std::fmt::Write as _;
    use std::path::{Path, PathBuf};

    const BADGE: &str = "https://badgen.net/github/release/ryanduguid/Ozzit/stable";
    const LOST: &str = "https://duguid.com.au/croft-manor/";

    fn root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
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
        let mut net = Fake::default()
            .ok(OZZIT_LATEST_RELEASE, r#"{"tag_name": "v9.9.9"}"#)
            .ok(&ozzit_functions_csv("v9.9.9"), &csv(functions, help));
        for fact in FACTS {
            let id = fact.section.unwrap_or("top");
            let page = format!(
                "<body><section id=\"{id}\"><p>{}</p></section></body>",
                fact.page.join(" and ")
            );
            net = net.ok(fact.url, &page);
        }
        net
    }

    fn fixture_readme(extra: &str) -> String {
        let facts: Vec<&str> = FACTS
            .iter()
            .flat_map(|f| f.readme.iter().copied())
            .collect();
        format!(
            "[Ozzit](https://github.com/ryanduguid/Ozzit): 150 native Excel LAMBDA functions plus 5 help tables.\n\
[badge]({BADGE}) [notes]({OWN_BLOB}AGENTS.md#reusing-this-code)\n{}\n{extra}\n",
            facts.join(". ")
        )
    }

    fn drift(net: &mut Fake, markdown: &str) -> (i32, String) {
        let report = run(markdown, &root(), net);
        let messages: Vec<String> = report.findings.iter().map(|(_, m)| m.clone()).collect();
        (report.exit_code(), messages.join("\n"))
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
        let (code, messages) = drift(&mut net, &fixture_readme(&format!("[Croft Manor]({LOST})")));
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
        let cash = &FACTS[0];
        let mut net = world(150, 5).ok(cash.url, "<body><p>$35,957.55 and ($99,999)</p></body>");
        let (code, messages) = drift(&mut net, &fixture_readme(""));
        assert_eq!(code, 1);
        assert_eq!(
            messages,
            format!("{}: {} no longer shows '$25,160'", cash.name, cash.url)
        );
    }

    #[test]
    fn a_figure_dropped_from_the_readme_is_drift() {
        let markdown =
            fixture_readme("").replace("$250 creditors difference", "$300 creditors difference");
        let (code, messages) = drift(&mut world(150, 5), &markdown);
        assert_eq!(code, 1);
        assert!(messages.contains("the README no longer says '$250 creditors difference'"));
    }

    /// The real README passes offline: its local files and anchors exist, it
    /// states the Ozzit counts and every fact's wording, and links every fact page.
    #[test]
    fn the_real_readme_passes_against_a_matching_world() {
        let markdown = std::fs::read_to_string(root().join("README.md")).unwrap();
        let (functions, help) = readme::ozzit_counts(&markdown).expect("Ozzit counts");
        assert_eq!(
            drift(&mut world(functions, help), &markdown),
            (0, String::new())
        );
        let links = readme::links(&markdown);
        for fact in FACTS {
            assert!(
                links.iter().any(|link| link.starts_with(fact.url)),
                "{} not linked",
                fact.url
            );
        }
    }
}
