use super::{OWN_BLOB, OWN_RAW, run};
use crate::facts::{FACTS, Fact, OZZIT_LATEST_RELEASE, SITE_LLMS_TXT, ozzit_functions_csv};
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
    let calculators = readme::calculator_counts(&readme::visible(markdown))
        .first()
        .copied()
        .unwrap_or(76);
    let mut net = Fake::default()
        .ok(OZZIT_LATEST_RELEASE, r#"{"tag_name": "v9.9.9"}"#)
        .ok(&ozzit_functions_csv("v9.9.9"), &csv(functions, help))
        .ok(
            SITE_LLMS_TXT,
            &format!("including {calculators} browser tax and business calculators."),
        );
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
76 browser tax and business calculators. Run 76 Australian calculators.\n\
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
> Summary with 76 browser tax and business calculators.\n\n\
- Credentials: {}\n\n\
## Start here\n\n\
- [76 browser tax and business calculators](https://duguid.com.au/tools/business-calculators/): 57 business, 17 individual tax and 2 standalone Australian calculators.\n\
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

fn temp_repo(name: &str) -> (PathBuf, PathBuf) {
    let outer = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join(format!("{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&outer);
    let repo = outer.join("repo");
    std::fs::create_dir_all(&repo).unwrap();
    std::fs::write(repo.join("AGENTS.md"), "## Reusing this code\n").unwrap();
    std::fs::write(repo.join("llms.txt"), "placeholder\n").unwrap();
    std::fs::write(repo.join("FORKS.md"), "forks\n").unwrap();
    (outer, repo)
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
fn a_wrong_calculator_count_on_the_site_is_drift() {
    let mut net = world(150, 5).ok(
        SITE_LLMS_TXT,
        "including 79 browser tax and business calculators.",
    );
    let (code, messages) = drift(&mut net, &fixture_readme(""));
    assert_eq!(code, 1);
    assert!(messages.contains(&format!(
        "Calculators: README.md says 76; {SITE_LLMS_TXT} says 79"
    )));
}

#[test]
fn mismatched_calculator_counts_between_readme_and_llms_are_drift() {
    let stale_llms = fixture_llms("").replace("76 browser", "71 browser");
    let (code, messages) = drift_with(
        &mut world(150, 5),
        &fixture_readme(""),
        &stale_llms,
        today(),
    );
    assert_eq!(code, 1);
    assert!(messages.contains("llms.txt counts ([71, 71]) disagree with README.md (76)"));
}

#[test]
fn a_calculator_breakdown_that_does_not_sum_to_total_is_drift() {
    let bad_llms =
        fixture_llms("").replace("57 business, 17 individual", "54 business, 15 individual");
    let (code, messages) = drift_with(&mut world(150, 5), &fixture_readme(""), &bad_llms, today());
    assert_eq!(code, 1);
    assert!(messages.contains("llms.txt breakdown (54 + 15 + 2) does not equal 76"));
}

#[test]
fn overflowing_or_missing_calculator_breakdown_is_drift() {
    let overflow_llms = fixture_llms("").replace(
        "57 business, 17 individual tax and 2 standalone",
        "4294967295 business, 76 individual tax and 1 standalone",
    );
    let (code, messages) = drift_with(
        &mut world(150, 5),
        &fixture_readme(""),
        &overflow_llms,
        today(),
    );
    assert_eq!(code, 1);
    assert!(messages.contains("does not equal 76"));

    let missing_llms =
        fixture_llms("").replace("57 business, 17 individual tax and 2 standalone", "several");
    let (code, messages) = drift_with(
        &mut world(150, 5),
        &fixture_readme(""),
        &missing_llms,
        today(),
    );
    assert_eq!(code, 1);
    assert!(messages.contains("no longer shows"));
}

#[test]
fn commented_site_calculator_count_is_ignored() {
    let mut net = world(150, 5).ok(
        SITE_LLMS_TXT,
        "<!-- 71 browser tax and business calculators -->\nincluding 76 browser tax and business calculators.",
    );
    assert_eq!(drift(&mut net, &fixture_readme("")), (0, String::new()));
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

#[test]
fn pr_local_own_raw_file_passes_even_when_remote_main_is_404() {
    let (outer, repo) = temp_repo("pr-local-raw-test");
    std::fs::write(repo.join("NEW_DOC.md"), "# New\n").unwrap();
    let new_url = format!("{OWN_RAW}NEW_DOC.md");
    let mut net = world(150, 5).script(&new_url, vec![status(404, "")]);
    let llms_doc = fixture_llms(&format!("- [New]({new_url}): added in PR.\n"));
    let report = run(&fixture_readme(""), &llms_doc, &repo, today(), &mut net);
    std::fs::remove_dir_all(&outer).unwrap();
    assert_eq!(report.exit_code(), 0, "{:?}", report.findings);
    assert!(!net.calls.iter().any(|u| u == &new_url));
}

#[test]
fn own_raw_absolute_and_parent_paths_outside_repo_are_rejected() {
    let (outer, repo) = temp_repo("own-raw-abs-test");
    let outside = outer.join("outside.md");
    std::fs::write(&outside, "# Outside\n").unwrap();
    let abs_posix = outside.to_string_lossy().replace('\\', "/");
    let abs_url = if abs_posix.starts_with('/') {
        format!("{OWN_RAW}{abs_posix}")
    } else {
        format!("{OWN_RAW}/{abs_posix}")
    };
    let parent_url = format!("{OWN_RAW}../outside.md");
    let llms_doc = fixture_llms(&format!(
        "- [Abs]({abs_url}): absolute.\n- [Parent]({parent_url}): traversal.\n"
    ));
    let mut net = world(150, 5);
    let report = run(&fixture_readme(""), &llms_doc, &repo, today(), &mut net);
    std::fs::remove_dir_all(&outer).unwrap();
    assert_eq!(report.exit_code(), 1);
    let messages: String = report
        .findings
        .iter()
        .map(|(_, m)| m.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    assert!(messages.contains(&format!(
        "Dead link: {abs_url}: no such file in the repository"
    )));
    assert!(messages.contains(&format!(
        "Dead link: {parent_url}: no such file in the repository"
    )));
    assert!(!net.calls.iter().any(|u| u == &abs_url || u == &parent_url));
}

#[test]
fn own_raw_symlink_escaping_repo_is_rejected() {
    let (outer, repo) = temp_repo("own-raw-symlink-test");
    let outside = outer.join("outside.md");
    std::fs::write(&outside, "# Outside\n").unwrap();
    let link = repo.join("escape.md");
    #[cfg(unix)]
    let created = std::os::unix::fs::symlink(&outside, &link);
    #[cfg(windows)]
    let created = std::os::windows::fs::symlink_file(&outside, &link);
    if let Err(err) = created {
        std::fs::remove_dir_all(&outer).unwrap();
        #[cfg(windows)]
        if err.raw_os_error() == Some(1314) {
            return;
        }
        panic!("could not create test symlink: {err}");
    }
    let escape_url = format!("{OWN_RAW}escape.md");
    let llms_doc = fixture_llms(&format!("- [Escape]({escape_url}): symlink.\n"));
    let mut net = world(150, 5);
    let report = run(&fixture_readme(""), &llms_doc, &repo, today(), &mut net);
    std::fs::remove_dir_all(&outer).unwrap();
    assert_eq!(report.exit_code(), 1);
    assert!(report.findings.iter().any(|(_, m)| {
        m.contains(&format!(
            "Dead link: {escape_url}: no such file in the repository"
        ))
    }));
    assert!(!net.calls.iter().any(|u| u == &escape_url));
}

#[test]
fn resolve_repo_file_handles_empty_root_as_current_directory() {
    assert!(super::resolve_repo_file(Path::new(""), "Cargo.toml").is_some());
    assert!(super::resolve_repo_file(Path::new(""), "../Cargo.toml").is_none());
}
