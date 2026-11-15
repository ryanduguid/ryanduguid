//! Checks the profile README against its sources: every link resolves, the
//! Ozzit counts equal those in `functions.csv` at Ozzit's latest release, and
//! the figures it quotes from the website still appear there.
//!
//! Run from the repository root, where it reads README.md. Prints each
//! finding, then a summary.
//! Exit 0 clean, 1 on drift, 2 when a source could not be read and nothing
//! drifted. `GITHUB_TOKEN`, when set, is sent to the GitHub API only.

mod check;
mod facts;
mod html;
mod net;
mod readme;

use check::Severity;
use std::path::PathBuf;

fn main() {
    let path = PathBuf::from("README.md");
    let markdown = match std::fs::read_to_string(&path) {
        Ok(markdown) => markdown,
        Err(error) => {
            eprintln!("readme-check: {}: {error}", path.display());
            std::process::exit(2);
        }
    };
    let root = path.parent().map(PathBuf::from).unwrap_or_default();
    let token = std::env::var("GITHUB_TOKEN").ok().filter(|t| !t.is_empty());
    let report = check::run(&markdown, &root, &mut net::Network::new(token));
    for (severity, message) in &report.findings {
        let label = match severity {
            Severity::Drift => "DRIFT",
            Severity::Inconclusive => "INCONCLUSIVE",
        };
        println!("{label}: {message}");
    }
    println!("Checked: {}", report.checked.join("; "));
    std::process::exit(report.exit_code());
}
