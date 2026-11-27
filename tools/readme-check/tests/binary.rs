//! The built binary, run where there is no README.md: it must say so and exit 2
//! before reading anything else or touching the network.

use std::path::Path;
use std::process::Command;

#[test]
fn a_missing_readme_exits_2() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join(format!("binary-test-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_readme-check"))
        .current_dir(&dir)
        .env("GITHUB_TOKEN", "unused")
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    let err = String::from_utf8(output.stderr).unwrap();
    assert!(err.starts_with("readme-check: README.md: "), "{err}");
    std::fs::remove_dir_all(&dir).unwrap();
}
