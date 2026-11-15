//! Figures the README quotes from the website. Each fact names the README's
//! wording and the wording the linked page (or the section its link points
//! to) must still show. When a figure changes on purpose, change it in the
//! README, on the site and here.

pub struct Fact {
    pub name: &'static str,
    pub readme: &'static [&'static str],
    pub url: &'static str,
    pub section: Option<&'static str>,
    pub page: &'static [&'static str],
}

pub const FACTS: &[Fact] = &[
    Fact {
        name: "Cash-flow case",
        readme: &["$35,957.55 quarterly profit", "$25,160 cash shortfall"],
        url: "https://duguid.com.au/examples/profit-vs-cash-flow/",
        section: None,
        page: &["$35,957.55", "$25,160"],
    },
    Fact {
        name: "Ozzit GST example",
        readme: &["$100 GST", "$1,100 amount"],
        url: "https://duguid.com.au/tools/ozzit/",
        section: Some("worked-example"),
        page: &["Take $1,100", "The result is 100.00"],
    },
    Fact {
        name: "Monthly Close Controls example",
        readme: &["8 exceptions", "$250 creditors difference"],
        url: "https://duguid.com.au/tools/monthly-close-controls/",
        section: Some("worked-example"),
        page: &["8 exceptions", "$250"],
    },
    Fact {
        name: "Xero certificate dates",
        readme: &["awarded 1 July 2026", "valid until 1 July 2027"],
        url: "https://duguid.com.au/evidence/",
        section: Some("xero-certification"),
        page: &["Awarded 1 July 2026", "Valid until 1 July 2027"],
    },
];

pub const OZZIT_LATEST_RELEASE: &str =
    "https://api.github.com/repos/ryanduguid/Ozzit/releases/latest";

pub fn ozzit_functions_csv(tag: &str) -> String {
    format!("https://raw.githubusercontent.com/ryanduguid/Ozzit/{tag}/functions.csv")
}

/// Counts in Ozzit's `functions.csv`: (functions, help tables). Help tables are
/// the `oz.About…` rows. Quoted fields may hold commas, quotes and line breaks.
pub fn csv_counts(csv: &str) -> Result<(u32, u32), String> {
    let names = first_fields(csv);
    let Some((header, rows)) = names.split_first() else {
        return Err("functions.csv is empty".into());
    };
    if !header.eq_ignore_ascii_case("function") {
        return Err(format!(
            "functions.csv starts with '{header}', not 'function'"
        ));
    }
    let help = rows
        .iter()
        .filter(|name| name.starts_with("oz.About"))
        .count();
    let to_u32 = |n: usize| u32::try_from(n).map_err(|e| e.to_string());
    Ok((to_u32(rows.len() - help)?, to_u32(help)?))
}

/// The first field of every non-blank record.
fn first_fields(csv: &str) -> Vec<String> {
    let mut names = Vec::new();
    let mut field = String::new();
    let (mut quoted, mut first, mut blank) = (false, true, true);
    let mut chars = csv.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '"' if quoted && chars.peek() == Some(&'"') => {
                chars.next();
                field.push('"');
            }
            '"' => quoted = !quoted,
            '\n' if !quoted => {
                if !blank {
                    names.push(std::mem::take(&mut field));
                }
                field.clear();
                (first, blank) = (true, true);
                continue;
            }
            '\r' if !quoted => continue,
            ',' if !quoted => first = false,
            _ if first => field.push(c),
            _ => {}
        }
        blank = false;
    }
    if !blank {
        names.push(field);
    }
    names
}

#[cfg(test)]
mod tests {
    use super::csv_counts;

    #[test]
    fn counts_split_functions_from_help_tables() {
        let csv = "function,module,description\r\n\
oz.GSTExtractλ,GST,\"Extract GST, from a \"\"wholly\"\" taxable amount\nover lines\"\r\n\
oz.AboutGSTλ,Help,Help table\r\n\
\r\n\
\"oz.NPVλ\",Finance,NPV\r\n";
        assert_eq!(csv_counts(csv), Ok((2, 1)));
    }

    #[test]
    fn an_unexpected_header_is_an_error() {
        assert!(csv_counts("name,module\nx,y\n").is_err());
        assert!(csv_counts("").is_err());
    }
}
