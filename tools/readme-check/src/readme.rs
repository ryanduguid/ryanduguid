//! Reading the README: its link targets, its Markdown heading anchors and the
//! Ozzit counts it states.

/// The README as a reader sees it: fenced code and HTML comments removed, so
/// a figure or link kept only there cannot satisfy a check.
pub fn visible(markdown: &str) -> String {
    let mut shown = Vec::new();
    let (mut fenced, mut commented) = (false, false);
    for line in markdown.lines() {
        if !commented && line.trim_start().starts_with("```") {
            fenced = !fenced;
            continue;
        }
        if fenced {
            continue;
        }
        let mut kept = String::new();
        let mut rest = line;
        loop {
            let marker = if commented { "-->" } else { "<!--" };
            let Some(at) = rest.find(marker) else {
                if !commented {
                    kept.push_str(rest);
                }
                break;
            };
            if !commented {
                kept.push_str(&rest[..at]);
            }
            rest = &rest[at + marker.len()..];
            commented = !commented;
        }
        shown.push(kept);
    }
    shown.join("\n")
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
            let open = format!("{name}={quote}");
            // Whole attributes only: whitespace before the name, so data-src is not src.
            let whole = |at: &usize| *at > 0 && line.as_bytes()[at - 1].is_ascii_whitespace();
            for at in line.match_indices(&open).map(|(at, _)| at).filter(whole) {
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
const BROWSER_CALC_PHRASE: &str = " browser tax and business calculators";
const AU_CALC_PHRASE: &str = " Australian calculators";

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

/// Every integer immediately preceding `phrase`, requiring a non-alphanumeric
/// boundary before the digits so substrings like `x76` do not match.
pub fn counts_before(text: &str, phrase: &str) -> Vec<u32> {
    text.match_indices(phrase)
        .filter_map(|(at, _)| trailing_u32(&text[..at]))
        .collect()
}

/// Calculator totals quoted in `README.md`, `llms.txt` or `duguid.com.au/llms.txt`.
pub fn calculator_counts(text: &str) -> Vec<u32> {
    let mut found = counts_before(text, BROWSER_CALC_PHRASE);
    found.extend(counts_before(text, AU_CALC_PHRASE));
    found
}

/// Parses `<A> business, <B> individual tax and <C> standalone`, scanning past
/// any earlier prose containing `" business, "`.
pub fn calculator_breakdown(text: &str) -> Option<(u32, u32, u32)> {
    const MID: &str = " business, ";
    const END: &str = " individual tax and ";
    const WORD: &str = "standalone";
    for (at, _) in text.match_indices(MID) {
        let Some(business) = trailing_u32(&text[..at]) else {
            continue;
        };
        let after_mid = &text[at + MID.len()..];
        let Some(end_at) = after_mid.find(END) else {
            continue;
        };
        let Some(individual) = parse_ascii_u32(&after_mid[..end_at]) else {
            continue;
        };
        let after_end = &after_mid[end_at + END.len()..];
        let Some((standalone_str, rest)) = after_end.split_once(' ') else {
            continue;
        };
        let Some(standalone) = parse_ascii_u32(standalone_str) else {
            continue;
        };
        let Some(tail) = rest.strip_prefix(WORD) else {
            continue;
        };
        if tail.chars().next().is_none_or(|c| !c.is_alphanumeric()) {
            return Some((business, individual, standalone));
        }
    }
    None
}

fn parse_ascii_u32(text: &str) -> Option<u32> {
    (!text.is_empty() && text.bytes().all(|b| b.is_ascii_digit()))
        .then(|| text.parse().ok())
        .flatten()
}

fn trailing_u32(before: &str) -> Option<u32> {
    let digits = before.len() - before.trim_end_matches(|c: char| c.is_ascii_digit()).len();
    if digits == 0 {
        return None;
    }
    let start = before.len() - digits;
    if let Some(prev) = before[..start].chars().next_back()
        && prev.is_alphanumeric()
    {
        return None;
    }
    parse_ascii_u32(&before[start..])
}

#[cfg(test)]
mod tests {
    use super::{
        calculator_breakdown, calculator_counts, has_heading, links, ozzit_counts, slug, visible,
    };

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
    fn a_tab_before_an_attribute_still_counts() {
        assert_eq!(links("<img\tsrc=\"g.png\" data-src=\"h.png\">"), ["g.png"]);
    }

    #[test]
    fn a_comment_marker_inside_code_hides_nothing_after_the_fence() {
        let text = "```html\n<!-- unclosed\n```\n150 native Excel LAMBDA functions plus 5 help tables [x](https://x.test/)\n";
        let shown = visible(text);
        assert_eq!(ozzit_counts(&shown), Some((150, 5)));
        assert_eq!(links(&shown), ["https://x.test/"]);
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
        assert_eq!(
            visible("before <!-- hidden --> after <!-- again --> end"),
            "before  after  end"
        );
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

    #[test]
    fn calculator_counts_and_breakdown_require_word_boundaries() {
        let text = "76 browser tax and business calculators. Run 76 Australian calculators. x99 browser tax and business calculators.";
        assert_eq!(calculator_counts(text), [76, 76]);
        assert_eq!(
            calculator_breakdown(
                "For your business, see: 57 business, 17 individual tax and 2 standalone Australian"
            ),
            Some((57, 17, 2))
        );
        assert_eq!(
            calculator_breakdown("x57 business, 17 individual tax and 2 standalone"),
            None
        );
        assert_eq!(
            calculator_breakdown("57 business, +17 individual tax and 2 standalone"),
            None
        );
        assert_eq!(
            calculator_breakdown("57 business, 17 individual tax and 2 standalones"),
            None
        );
    }
}
