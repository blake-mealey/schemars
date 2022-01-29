use syn::{Attribute, Lit::Str, Meta::NameValue, MetaNameValue};

pub fn get_title_and_desc_from_doc(attrs: &[Attribute]) -> (Option<String>, Option<String>) {
    let doc = match get_doc(attrs) {
        None => return (None, None),
        Some(doc) => doc,
    };

    if doc.starts_with('#') {
        let mut split = doc.splitn(2, '\n');
        let title = split
            .next()
            .unwrap()
            .trim_start_matches('#')
            .trim()
            .to_owned();
        let maybe_desc = split.next().and_then(merge_description_lines);
        (none_if_empty(title), maybe_desc)
    } else {
        (None, merge_description_lines(&doc))
    }
}

fn merge_description_lines(doc: &str) -> Option<String> {
    none_if_empty(doc.trim().to_owned())
}

fn get_doc(attrs: &[Attribute]) -> Option<String> {
    let attrs = attrs
        .iter()
        .filter_map(|attr| {
            if !attr.path.is_ident("doc") {
                return None;
            }

            let meta = attr.parse_meta().ok()?;
            if let NameValue(MetaNameValue { lit: Str(s), .. }) = meta {
                return Some(s.value());
            }

            None
        })
        .collect::<Vec<_>>();

    none_if_empty(
        attrs
            .iter()
            .flat_map(|a| a.split('\n'))
            .map(|line| {
                if line.starts_with(' ') {
                    &line[1..]
                } else {
                    line
                }
            })
            .collect::<Vec<_>>()
            .join("\n"),
    )
}

fn none_if_empty(s: String) -> Option<String> {
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}
