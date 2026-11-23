//! Checks the profile README and `llms.txt` against their sources: every link
//! resolves, Ozzit's counts equal those in `functions.csv` at Ozzit's latest
//! release, the figures quoted from the website still appear there, and
//! `llms.txt` keeps valid structure, current dates and repository sync.
//!
//! Run from the repository root, where it reads README.md and llms.txt. Prints
//! each finding, then a summary.
//! Exit 0 clean, 1 on drift, 2 when a source could not be read and nothing
//! drifted. `GITHUB_TOKEN`, when set, is sent to the GitHub API only.
//!
//! `readme-check releases` instead rebuilds the README's 'Recent releases'
//! block from the GitHub API (see `releases.rs`): exit 0 whether or not the
//! block changed, 1 with README.md untouched on any failure.

mod check;
mod facts;
mod html;
mod llms;
mod net;
mod readme;
mod releases;

use check::Severity;
use std::path::{Path, PathBuf};

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
    // The argument only picks the mode; it grants nothing.
    // nosemgrep: rust.lang.security.args.args
    if std::env::args().nth(1).as_deref() == Some("releases") {
        let result = releases::update(&markdown, &mut net::Network::new(token)).and_then(|new| {
            if new == markdown {
                return Ok("the release list is current");
            }
            replace_file(&path, &new).map_err(|e| e.to_string())?;
            Ok("the release list was updated")
        });
        match result {
            Ok(message) => println!("README.md: {message}"),
            Err(why) => {
                eprintln!("readme-check releases: {why}");
                std::process::exit(1);
            }
        }
        return;
    }
    let llms_text = read_llms(&root);
    let report = check::run(
        &markdown,
        &llms_text,
        &root,
        llms::today_utc_days(),
        &mut net::Network::new(token),
    );
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

fn read_llms(root: &Path) -> String {
    let path = root.join("llms.txt");
    match std::fs::read_to_string(&path) {
        Ok(text) => text,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => String::new(),
        Err(error) => {
            eprintln!("readme-check: {}: {error}", path.display());
            std::process::exit(2);
        }
    }
}

/// Writes `contents` to a temporary file beside `path`, then renames it over
/// `path`, so a failed write leaves the original file whole.
fn replace_file(path: &Path, contents: &str) -> std::io::Result<()> {
    let mut name = path.file_name().unwrap_or_default().to_owned();
    name.push(".tmp");
    let temporary = path.with_file_name(name);
    let written =
        std::fs::write(&temporary, contents).and_then(|()| std::fs::rename(&temporary, path));
    if written.is_err() {
        let _ = std::fs::remove_file(&temporary);
    }
    written
}

#[cfg(test)]
mod tests {
    use super::replace_file;
    use std::fs;
    use std::path::Path;

    #[test]
    fn a_failed_write_leaves_the_original() {
        // Under the crate's ignored target folder, not a shared temporary one.
        let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join(format!("replace-file-test-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("README.md");
        fs::write(&path, "old").unwrap();
        replace_file(&path, "new").unwrap();
        assert_eq!(fs::read_to_string(&path).unwrap(), "new");
        // A directory where the temporary file goes makes the write fail.
        fs::create_dir(dir.join("README.md.tmp")).unwrap();
        assert!(replace_file(&path, "newer").is_err());
        assert_eq!(fs::read_to_string(&path).unwrap(), "new");
        fs::remove_dir_all(&dir).unwrap();
    }
}
