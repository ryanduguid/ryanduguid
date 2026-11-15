//! Reading the README: its link targets, its Markdown heading anchors and the
//! Ozzit counts it states.

/// The README as a reader sees it: fenced code and HTML comments removed, so
/// a figure or link kept only there cannot satisfy a check.
pub fn visible(markdown: &str) -> String {
    let mut shown = String::with_capacity(markdown.len());
    let mut rest = markdown;
    while let Some(at) = rest.find("<!--") {
        shown.push_str(&rest[..at]);
        rest = rest[at..]
            .find("-->")
            .map_or("", |end| &rest[at + end + 3..]);
    }
    shown.push_str(rest);
    let mut fenced = false;
    let lines = shown.lines().filter(|line| {
        if line.trim_start().starts_with("```") {
            fenced = !fenced;
            return false;
        }
        !fenced
    });
    lines.collect::<Vec<_>>().join(
        "
",
    )
}

/// Every link target in Markdown links and images and in HTML `href`, `src`
/// and `srcset` attributes, in order of first appearance. Pass `visible` text.
pub fn links(markdown: &str) -> Vec<String> {
    let mut found: Vec<String> = Vec::new();
    for target in markdown.lines().flat_map(line_targets) {
        if !target.is_empty() && !found.contains(&target) {
            found.push(target);
        }
    }
    found
}

fn first_word(value: &str) -> String {
    value
        .split_whitespace()
        .next()
        .unwrap_or_default()
        .to_owned()
}

fn line_targets(line: &str) -> Vec<String> {
    let mut targets = Vec::new();
    for (at, _) in line.match_indices("](") {
        let value = &line[at + 2..];
        let target = match value.strip_prefix('<') {
            Some(bracketed) => bracketed.split('>').next().unwrap_or_default().to_owned(),
            // A Markdown link title follows the target after a space.
            None => first_word(value.split(')').next().unwrap_or_default()),
        };
        targets.push(target);
    }
    for name in ["href", "src", "srcset"] {
        for quote in ['"', '\''] {
            let open = format!(" {name}={quote}");
            for (at, _) in line.match_indices(&open) {
                let value = line[at + open.len()..]
                    .split(quote)
                    .next()
                    .unwrap_or_default();
                if name == "srcset" {
                    targets.extend(value.split(',').map(first_word));
                } else {
                    targets.push(value.trim().to_owned());
                }
            }
        }
    }
    targets
}

/// GitHub's anchor for a heading: lower case, punctuation dropped, spaces as hyphens.
pub fn slug(heading: &str) -> String {
    heading
        .trim()
        .to_lowercase()
        .chars()
        .filter_map(|c| match c {
            ' ' => Some('-'),
            c if c.is_alphanumeric() || c == '-' || c == '_' => Some(c),
            _ => None,
        })
        .collect()
}

/// Whether a Markdown document has a heading whose GitHub anchor is `fragment`.
pub fn has_heading(markdown: &str, fragment: &str) -> bool {
    markdown.lines().any(|line| {
        let hashes = line.chars().take_while(|&c| c == '#').count();
        (1..=6).contains(&hashes)
            && line[hashes..].starts_with(' ')
            && slug(&line[hashes..]) == fragment
    })
}

const COUNT_PHRASE: &str = " native Excel LAMBDA functions plus ";

/// The function and help-table counts in '150 native Excel LAMBDA functions
/// plus 5 help tables', which must appear exactly once.
pub fn ozzit_counts(markdown: &str) -> Option<(u32, u32)> {
    if markdown.matches(COUNT_PHRASE).count() != 1 {
        return None;
    }
    let at = markdown.find(COUNT_PHRASE)?;
    let before = &markdown[..at];
    let digits = before.len() - before.trim_end_matches(|c: char| c.is_ascii_digit()).len();
    let functions = before[before.len() - digits..].parse().ok()?;
    let after = &markdown[at + COUNT_PHRASE.len()..];
    let (help, rest) = after.split_once(' ')?;
    rest.starts_with("help tables")
        .then(|| help.parse().ok())
        .flatten()
        .map(|help| (functions, help))
}

#[cfg(test)]
mod tests {
    use super::{has_heading, links, ozzit_counts, slug, visible};

    #[test]
    fn links_cover_markdown_html_and_srcset_once_each_and_skip_code() {
        let text = "[![b](https://img.test/b)](https://a.test/) [again](https://a.test/ \"t\")\n\
<source srcset=\"one.svg 1x, two.svg 2x\"><img src=\"three.png\"> <a href=\"https://c.test/#x\">c</a>\n\
```mermaid\n[x](https://code.test/)\n```\n";
        assert_eq!(
            links(&visible(text)),
            [
                "https://img.test/b",
                "https://a.test/",
                "https://c.test/#x",
                "three.png",
                "one.svg",
                "two.svg"
            ]
        );
    }

    #[test]
    fn single_quoted_attributes_and_bracketed_destinations_are_links() {
        let text = "<a href='https://d.test/'>d</a> <img src='e.png'> [f](<https://f.test/a b>)";
        assert_eq!(
            links(text),
            ["https://f.test/a b", "https://d.test/", "e.png"]
        );
    }

    #[test]
    fn comments_and_code_hide_figures_and_links() {
        let text = "<!-- 150 native Excel LAMBDA functions plus 5 help tables -->
151 native Excel LAMBDA functions plus 5 help tables
<!--
[x](https://hidden.test/)
-->
";
        let shown = visible(text);
        assert_eq!(ozzit_counts(&shown), Some((151, 5)));
        assert!(links(&shown).is_empty());
        let twice = "150 native Excel LAMBDA functions plus 5 help tables; 151 native Excel LAMBDA functions plus 5 help tables";
        assert_eq!(ozzit_counts(twice), None);
    }

    #[test]
    fn heading_anchors_follow_github() {
        assert_eq!(slug(" Reusing this code"), "reusing-this-code");
        assert_eq!(slug("Q&A: GST (2026-27)"), "qa-gst-2026-27");
        assert!(has_heading(
            "# Notes\n\n## Reusing this code\n",
            "reusing-this-code"
        ));
        assert!(!has_heading("#Reusing this code\n", "reusing-this-code"));
    }

    #[test]
    fn ozzit_counts_are_read_from_the_sentence() {
        let line = "**[Ozzit](x):** 150 native Excel LAMBDA functions plus 5 help tables for";
        assert_eq!(ozzit_counts(line), Some((150, 5)));
        assert_eq!(
            ozzit_counts("native Excel LAMBDA functions plus 5 help tables"),
            None
        );
        assert_eq!(
            ozzit_counts("150 native Excel LAMBDA functions plus five help tables"),
            None
        );
    }
}
