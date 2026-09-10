use crate::compiler::model::site::Inline;

pub fn parse(input: &str) -> Vec<Inline> {
    let mut nodes: Vec<Inline> = Vec::new();
    let mut rest: &str = input;

    while !rest.is_empty() {
        if let Some(start) = rest.find('[') {
            if start > 0 {
                nodes.extend(parse_styles(&rest[..start]));
            }
            let link = &rest[start..];
            if let Some(label_end) = link.find("](") {
                if let Some(url_end) = link[label_end + 2..].find(')') {
                    let label = &link[1..label_end];
                    let url = &link[label_end + 2..label_end + 2 + url_end];
                    nodes.push(Inline::Link {
                        label: parse_styles(label),
                        href: url.to_string(),
                    });
                    rest = &link[url_end + 1..];
                    continue;
                }
            }
            nodes.push(Inline::Text("[".to_string()));
            rest = &link[1..];
        } else {
            nodes.extend(parse_styles(rest));
            break;
        }
    }

    nodes
}

fn parse_styles(input: &str) -> Vec<Inline> {
    let mut nodes: Vec<Inline> = Vec::new();
    let mut rest: &str = input;

    while !rest.is_empty() {
        let next = ['`', '*']
            .iter()
            .filter_map(|chr| rest.find(*chr).map(|index| (index, *chr)))
            .min_by_key(|(index, _)| *index);

        let Some((index, chr)) = next else {
            nodes.push(Inline::Text(rest.to_string()));
            break;
        };

        if index > 0 {
            nodes.push(Inline::Text(rest[..index].to_string()));
            rest = &rest[index..];
        }

        if chr == '`' {
            if let Some(end) = rest[1..].find('`') {
                nodes.push(Inline::Code(rest[1..end + 1].to_string()));
                rest = &rest[end + 2..];
            } else {
                nodes.push(Inline::Text("`".to_string()));
                rest = &rest[1..];
            }
        } else if rest.starts_with("**") {
            if let Some(end) = rest[2..].find("**") {
                nodes.push(Inline::Strong(vec![Inline::Text(
                    rest[2..end + 2].to_string(),
                )]));
                rest = &rest[end + 4..];
            } else {
                nodes.push(Inline::Text("**".to_string()));
                rest = &rest[2..];
            }
        } else if let Some(end) = rest[1..].find('*') {
            nodes.push(Inline::Emphasis(vec![Inline::Text(
                rest[1..end + 1].to_string(),
            )]));
            rest = &rest[end + 2..];
        } else {
            nodes.push(Inline::Text("*".to_string()));
            rest = &rest[1..];
        }
    }

    nodes
}
