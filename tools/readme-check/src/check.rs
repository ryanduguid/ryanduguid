//! The checks themselves, collecting findings into one report.

use crate::facts::{FACTS, Fact, OZZIT_LATEST_RELEASE, csv_counts, ozzit_functions_csv};
use crate::html;
use crate::llms;
use crate::net::{Outcome, Transport, fetch};
use crate::readme;
use std::collections::{HashMap, HashSet};
use std::path::{Component, Path, PathBuf};

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
        let seen: HashSet<&str> = readme_links.iter().map(String::as_str).collect();
        let extra: Vec<String> = llms_links
            .iter()
            .filter(|link| !seen.contains(link.as_str()))
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
        if let Some(path) = base
            .strip_prefix(OWN_BLOB)
            .or_else(|| base.strip_prefix(OWN_RAW))
        {
            // Check this repository's own blob/raw links against the checked-out
            // tree first so PR-introduced files validate before merge whilst
            // keeping resolution strictly inside the repository root.
            let Some(local) = resolve_repo_file(root, path) else {
                self.report
                    .drift(format!("Dead link: {base}: no such file in the repository"));
                return;
            };
            if let Some(fragment) = fragment {
                let markdown = std::fs::read_to_string(&local).unwrap_or_default();
                if !readme::has_heading(&markdown, fragment) {
                    self.report.drift(format!(
                        "Missing anchor: {base}#{fragment}: no such heading in {path}"
                    ));
                }
            }
            return;
        }
        let Some(body) = self.page(base, "Dead link") else {
            return;
        };
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

/// Resolves a repository-relative `OWN_BLOB`/`OWN_RAW` path strictly inside
/// `root`, rejecting absolute paths, query strings, `..` traversal and symlinks
/// whose targets escape the canonical repository root.
fn resolve_repo_file(root: &Path, rel: &str) -> Option<PathBuf> {
    if rel.is_empty() || rel.contains('?') || rel.starts_with(['/', '\\']) {
        return None;
    }
    let rel_path = Path::new(rel);
    if !rel_path
        .components()
        .all(|c| matches!(c, Component::Normal(_)))
    {
        return None;
    }
    let base_root = if root.as_os_str().is_empty() {
        Path::new(".")
    } else {
        root
    };
    let canon_root = std::fs::canonicalize(base_root).ok()?;
    let canon_file = std::fs::canonicalize(canon_root.join(rel_path)).ok()?;
    (canon_file.starts_with(&canon_root) && canon_file.is_file()).then_some(canon_file)
}

#[cfg(test)]
#[path = "check_tests.rs"]
mod tests;
