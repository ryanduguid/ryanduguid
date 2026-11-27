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
use net::Transport;
use std::io::Write;
use std::path::Path;

fn main() {
    let token = std::env::var("GITHUB_TOKEN").ok().filter(|t| !t.is_empty());
    // The argument only picks the mode; it grants nothing.
    // nosemgrep: rust.lang.security.args.args
    let mode = std::env::args().nth(1);
    let code = run(
        mode.as_deref(),
        Path::new(""),
        &mut net::Network::new(token),
        &mut std::io::stdout(),
        &mut std::io::stderr(),
    );
    std::process::exit(code);
}

/// One run in `root`, printing findings to `out` and failures to `err`.
/// Returns the process exit code.
fn run(
    mode: Option<&str>,
    root: &Path,
    transport: &mut dyn Transport,
    out: &mut dyn Write,
    err: &mut dyn Write,
) -> i32 {
    let path = root.join("README.md");
    let markdown = match std::fs::read_to_string(&path) {
        Ok(markdown) => markdown,
        Err(error) => {
            let _ = writeln!(err, "readme-check: {}: {error}", path.display());
            return 2;
        }
    };
    if mode == Some("releases") {
        return update_releases(&path, &markdown, transport, out, err);
    }
    let llms_text = match read_llms(root) {
        Ok(text) => text,
        Err(why) => {
            let _ = writeln!(err, "readme-check: {why}");
            return 2;
        }
    };
    let report = check::run(
        &markdown,
        &llms_text,
        root,
        llms::today_utc_days(),
        transport,
    );
    for (severity, message) in &report.findings {
        let label = match severity {
            Severity::Drift => "DRIFT",
            Severity::Inconclusive => "INCONCLUSIVE",
        };
        let _ = writeln!(out, "{label}: {message}");
    }
    let _ = writeln!(out, "Checked: {}", report.checked.join("; "));
    report.exit_code()
}

fn update_releases(
    path: &Path,
    markdown: &str,
    transport: &mut dyn Transport,
    out: &mut dyn Write,
    err: &mut dyn Write,
) -> i32 {
    let result = releases::update(markdown, transport).and_then(|new| {
        if new == markdown {
            return Ok("the release list is current");
        }
        replace_file(path, &new).map_err(|e| e.to_string())?;
        Ok("the release list was updated")
    });
    match result {
        Ok(message) => {
            let _ = writeln!(out, "README.md: {message}");
            0
        }
        Err(why) => {
            let _ = writeln!(err, "readme-check releases: {why}");
            1
        }
    }
}

/// `llms.txt` beside the README, or empty when there is none.
fn read_llms(root: &Path) -> Result<String, String> {
    let path = root.join("llms.txt");
    match std::fs::read_to_string(&path) {
        Ok(text) => Ok(text),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(String::new()),
        Err(error) => Err(format!("{}: {error}", path.display())),
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
    use super::{replace_file, run};
    use crate::net::fake::{Fake, status};
    use crate::releases::{END, REPOS, START};
    use std::fs;
    use std::path::{Path, PathBuf};

    const API: &str = "https://api.github.com/repos/ryanduguid/";

    /// A fresh folder under the crate's ignored target folder, not a shared
    /// temporary one.
    fn scratch(name: &str) -> PathBuf {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join(format!("{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    /// Runs in `root`, returning the exit code, standard output and standard error.
    fn outcome(mode: Option<&str>, root: &Path, net: &mut Fake) -> (i32, String, String) {
        let (mut out, mut err) = (Vec::new(), Vec::new());
        let code = run(mode, root, net, &mut out, &mut err);
        let text = |bytes: Vec<u8>| String::from_utf8(bytes).unwrap();
        (code, text(out), text(err))
    }

    /// Every release repository unarchived with one stable release.
    fn released() -> Fake {
        let mut net = Fake::default();
        for repo in REPOS {
            let latest = r#"{"tag_name": "v1.0.0", "published_at": "2026-11-01T01:00:00Z", "draft": false, "prerelease": false}"#;
            net = net
                .ok(&format!("{API}{repo}"), r#"{"archived": false}"#)
                .ok(&format!("{API}{repo}/releases/latest"), latest);
        }
        net
    }

    #[test]
    fn a_failed_write_leaves_the_original() {
        let dir = scratch("replace-file-test");
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

    #[test]
    fn an_unreadable_readme_or_llms_txt_exits_2() {
        let dir = scratch("run-unreadable");
        let (code, out, err) = outcome(None, &dir, &mut Fake::default());
        assert_eq!((code, out.as_str()), (2, ""));
        assert!(err.starts_with("readme-check: ") && err.contains("README.md"));

        fs::write(dir.join("README.md"), "# Me\n").unwrap();
        // A folder where llms.txt should be cannot be read as text.
        fs::create_dir(dir.join("llms.txt")).unwrap();
        let (code, out, err) = outcome(None, &dir, &mut Fake::default());
        assert_eq!((code, out.as_str()), (2, ""));
        assert!(err.contains("llms.txt"));
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn findings_are_labelled_then_summarised() {
        const SLOW: &str = "https://slow.example.test/";
        let dir = scratch("run-findings");
        // No llms.txt: the check runs against an empty one.
        fs::write(dir.join("README.md"), format!("# Me\n\n[slow]({SLOW})\n")).unwrap();
        let mut net = Fake::default().script(SLOW, vec![status(503, "")]);
        let (code, out, err) = outcome(None, &dir, &mut net);
        assert_eq!((code, err.as_str()), (1, ""));
        assert!(out.contains(&format!("INCONCLUSIVE: Dead link: {SLOW}: HTTP 503")));
        assert!(out.contains("DRIFT: Ozzit: the README no longer says"));
        assert!(out.contains("DRIFT: llms.txt: file is empty"));
        assert!(out.ends_with("\n") && out.lines().last().unwrap().starts_with("Checked: 1 links"));

        // With an llms.txt, its own text is checked instead.
        fs::write(dir.join("llms.txt"), "# Me\n").unwrap();
        let (_, out, _) = outcome(None, &dir, &mut Fake::default());
        assert!(!out.contains("llms.txt: file is empty"));
        assert!(out.contains("DRIFT: llms.txt: missing blockquote"));
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn the_releases_mode_updates_reports_current_and_fails_whole() {
        let dir = scratch("run-releases");
        let readme = dir.join("README.md");
        fs::write(&readme, format!("# Me\n\n{START}\nold\n{END}\n")).unwrap();
        let (code, out, err) = outcome(Some("releases"), &dir, &mut released());
        assert_eq!(
            (code, out.as_str(), err.as_str()),
            (0, "README.md: the release list was updated\n", "")
        );
        let updated = fs::read_to_string(&readme).unwrap();
        assert!(
            updated.contains("[v1.0.0](https://github.com/ryanduguid/Ozzit/releases/tag/v1.0.0)")
        );

        let (code, out, _) = outcome(Some("releases"), &dir, &mut released());
        assert_eq!(
            (code, out.as_str()),
            (0, "README.md: the release list is current\n")
        );

        // An API failure leaves README.md as it was.
        let mut down = released().script(&format!("{API}Ozzit"), vec![status(404, "")]);
        let (code, out, err) = outcome(Some("releases"), &dir, &mut down);
        assert_eq!((code, out.as_str()), (1, ""));
        assert!(err.starts_with("readme-check releases: ") && err.contains("HTTP 404"));
        assert_eq!(fs::read_to_string(&readme).unwrap(), updated);

        // So does a failed write.
        fs::write(&readme, format!("# Me\n\n{START}\nold\n{END}\n")).unwrap();
        fs::create_dir(dir.join("README.md.tmp")).unwrap();
        let (code, _, err) = outcome(Some("releases"), &dir, &mut released());
        assert_eq!(code, 1);
        assert!(err.starts_with("readme-check releases: "));
        assert!(fs::read_to_string(&readme).unwrap().contains("old"));
        fs::remove_dir_all(&dir).unwrap();
    }
}
