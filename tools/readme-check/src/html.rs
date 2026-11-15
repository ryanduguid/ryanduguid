//! Just enough HTML handling to find an anchor, cut out its section and read
//! the section's visible text. Entities are not decoded, so a quoted fact that
//! a page writes with an entity fails visibly rather than passing silently.

pub fn has_anchor(html: &str, fragment: &str) -> bool {
    attribute(html, "id", fragment).is_some() || attribute(html, "name", fragment).is_some()
}

/// Where `name="value"` starts as a whole attribute, so `data-id="x"` is not `id="x"`.
fn attribute(html: &str, name: &str, value: &str) -> Option<usize> {
    let needle = format!("{name}=\"{value}\"");
    html.match_indices(&needle)
        .map(|(at, _)| at)
        .find(|&at| at > 0 && html.as_bytes()[at - 1].is_ascii_whitespace())
}

/// The element carrying `id="<id>"`: up to the next heading of the same or a
/// higher level when it is a heading, otherwise up to its matching end tag.
pub fn section<'a>(html: &'a str, id: &str) -> Option<&'a str> {
    let at = attribute(html, "id", id)?;
    let start = html[..at].rfind('<')?;
    let name: String = html[start + 1..]
        .chars()
        .take_while(char::is_ascii_alphanumeric)
        .collect::<String>()
        .to_ascii_lowercase();
    let open_end = start + html[start..].find('>')? + 1;
    let rest = &html[open_end..];
    let end = match heading_level(&name) {
        Some(level) => next_heading(rest, level),
        None => matching_end(rest, &name),
    };
    Some(&html[start..open_end + end.unwrap_or(rest.len())])
}

fn heading_level(name: &str) -> Option<u8> {
    match name.as_bytes() {
        [b'h', digit @ b'1'..=b'6'] => Some(digit - b'0'),
        _ => None,
    }
}

fn next_heading(html: &str, level: u8) -> Option<usize> {
    html.match_indices("<h").map(|(i, _)| i).find(|&i| {
        let tail = html.as_bytes().get(i + 2..i + 4);
        matches!(tail, Some([d @ b'1'..=b'6', b' ' | b'>']) if d - b'0' <= level)
    })
}

fn matching_end(html: &str, name: &str) -> Option<usize> {
    let open = format!("<{name}");
    let close = format!("</{name}>");
    let mut depth = 1;
    let mut at = 0;
    while depth > 0 {
        let next_close = at + html[at..].find(&close)?;
        let nested = html[at..next_close]
            .match_indices(&open)
            .filter(|(i, _)| {
                let after = html.as_bytes().get(at + i + open.len());
                matches!(after, Some(b' ' | b'>'))
            })
            .count();
        depth += nested;
        depth -= 1;
        at = next_close + close.len();
    }
    Some(at)
}

/// Visible text with scripts and styles removed and whitespace collapsed.
pub fn text(html: &str) -> String {
    let mut visible = String::with_capacity(html.len());
    let mut rest = html;
    while let Some(start) = rest.find('<') {
        visible.push_str(&rest[..start]);
        visible.push(' ');
        let tag = &rest[start..];
        let skip_to = ["script", "style"]
            .iter()
            .find(|name| {
                tag.get(1..=name.len())
                    .is_some_and(|t| t.eq_ignore_ascii_case(name))
            })
            .and_then(|name| tag.find(&format!("</{name}>")));
        let from = skip_to.unwrap_or(0);
        match tag[from..].find('>') {
            Some(end) => rest = &tag[from + end + 1..],
            None => rest = "",
        }
    }
    visible.push_str(rest);
    visible.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::{has_anchor, section, text};

    const PAGE: &str = r#"<body><h2 id="intro">Intro</h2><p>Before</p>
<section id="worked-example"><h2>Example</h2><section><p>inner</p></section>
<p>Extract GST from <b>$1,100</b></p></section><p>After $9</p>
<h3 id="xero-certification">Xero</h3><dl><dd>Awarded 1 July 2026</dd></dl>
<h4>Detail</h4><p>still in</p><h3>Next</h3><p>out</p>
<script>var x = "Awarded 1 July 2099";</script></body>"#;

    #[test]
    fn a_section_element_ends_at_its_own_end_tag_despite_nesting() {
        let worked = text(section(PAGE, "worked-example").unwrap());
        assert_eq!(worked, "Example inner Extract GST from $1,100");
    }

    #[test]
    fn a_heading_section_runs_to_the_next_heading_of_its_level() {
        let xero = text(section(PAGE, "xero-certification").unwrap());
        assert_eq!(xero, "Xero Awarded 1 July 2026 Detail still in");
    }

    #[test]
    fn text_drops_scripts() {
        assert!(!text(PAGE).contains("2099"));
    }

    #[test]
    fn anchors_are_found_by_id_only_when_present() {
        assert!(has_anchor(PAGE, "intro"));
        assert!(!has_anchor(PAGE, "croft-manor"));
        let decoy = r#"<div data-id="worked-example"><p>Take $1,100</p></div>"#;
        assert!(!has_anchor(decoy, "worked-example"));
        assert_eq!(section(decoy, "worked-example"), None);
        assert_eq!(section(PAGE, "croft-manor"), None);
    }
}
