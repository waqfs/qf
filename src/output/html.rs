use std::{fs, path::Path};

use crate::{
    compiler::model::{
        document::Site,
        site::{Block, Document, ImageAlt, Inline},
    },
    config::Config,
    output::OutputFormatter,
};

pub struct HTMLOutputFormatter;

impl OutputFormatter for HTMLOutputFormatter {
    fn name(&self) -> &'static str {
        "html"
    }

    fn write(&self, site: &Site, config: &Config, stage: &Path) -> Result<(), String> {
        for document in &site.documents {
            let dir = stage.join(document.route.trim_matches('/'));
            fs::create_dir_all(&dir)
                .map_err(|e| format!("Failed to create output directory: {e}"))?;
            let text = write_document(document, site, config);
            fs::write(dir.join("index.html"), text)
                .map_err(|e| format!("Failed to write file: {e}"))?;
        }
        Ok(())
    }
}

fn write_document(document: &Document, site: &Site, config: &Config) -> String {
    let mut body = String::new();

    write_head(document, config, &mut body);
    body.push_str("<body>\n");

    body.push_str("<a class=\"skip-link\" href=\"#content\">Skip to content</a>\n");
    body.push_str(&format!(
        "<header><a href=\"/\">{}</a></header>\n",
        &site.config.title
    ));

    body.push_str("<main id=\"content\">\n");
    body.push_str("<article>\n");
    body.push_str("<header>\n");
    body.push_str(&format!(
        "<h1>{}</h1>",
        html_escape(&document.metadata.title)
    ));
    if let Some(summary) = &document.metadata.summary {
        body.push_str(&format!("\n<p>{}</p>\n", html_escape(summary)));
    }
    if let Some(date) = &document.metadata.date {
        body.push_str(&format!(
            "<time datetime=\"{}\">{}</time>\n",
            html_escape_attribute(date),
            html_escape(date)
        ));
    }
    if let Some(author) = &document.metadata.author {
        body.push_str(&format!("<p class=\"author\">{}</p>\n", author));
    }
    body.push_str("</header>\n");
    for block in &document.blocks {
        write_block(block, &mut body);
    }

    if document.previous.is_some() || document.next.is_some() {
        body.push_str("<nav aria-label=\"navigation\">\n");
        if let Some(route) = &document.previous {
            body.push_str(&format!(
                "<a rel=\"prev\" href=\"{}\">Previous</a>\n",
                html_escape_attribute(route)
            ));
        }
        if let Some(route) = &document.next {
            body.push_str(&format!(
                "<a rel=\"next\" href=\"{}\">Next</a>\n",
                html_escape_attribute(route)
            ));
        }
        body.push_str("</nav>\n");
    }

    body.push_str("</article>\n");
    body.push_str("</main>\n");

    body.push_str("</body>\n");
    body.push_str("</html>\n");
    body
}

fn write_head(document: &Document, config: &Config, output: &mut String) {
    output.push_str("<!DOCTYPE html>");
    output.push_str(&format!("<html lang=\"{}\">\n", config.language.as_str()));
    output.push_str("<head>\n");
    output.push_str("<meta charset=\"utf-8\">\n");
    output.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n");
    output.push_str("<meta name=\"color-scheme\" content=\"light dark\">\n");
    output.push_str(&format!(
        "<title>{}</title>\n",
        html_escape(&document.metadata.title)
    ));
    output.push_str(&format!(
        "<meta name=\"description\" content=\"{}\">\n",
        html_escape_attribute(
            document
                .metadata
                .summary
                .as_deref()
                .unwrap_or(&document.metadata.title)
        )
    ));
    output.push_str(&format!(
        "<link rel=\"canonical\" href=\"{}{}\">\n",
        config.base_url, document.route
    ));
    if let Some(path) = &config.stylesheet {
        output.push_str(&format!(
            "<link rel=\"stylesheet\" href=\"{}\">\n",
            html_escape_attribute(path)
        ));
    }
    output.push_str("<link rel=\"alternate\" type=\"text/plain\" href=\"index.txt\">\n");
    if let Some(path) = &document.previous {
        output.push_str(&format!("<link rel=\"prev\" href=\"{}\">", path));
    }
    if let Some(path) = &document.next {
        output.push_str(&format!("<link rel=\"next\" href=\"{}\">", path));
    }
    output.push_str("</head>\n");
}

fn write_block(block: &Block, output: &mut String) {
    match block {
        Block::Heading { level, content } => {
            let level = level + 1;
            output.push_str(&format!("<h{level}>"));
            write_inline(content, output);
            output.push_str(&format!("</h{level}>\n"));
        }
        Block::UnorderedList(items) => write_list("ul", items, output),
        Block::OrderedList(items) => write_list("ol", items, output),
        Block::BlockQuote(inlines) => {
            output.push_str("<blockquote><p>");
            write_inline(inlines, output);
            output.push_str("</p></blockquote>\n");
        }
        Block::CodeBlock { language, code } => {
            output.push_str("<pre><code");
            if let Some(language) = language {
                output.push_str(&format!(
                    " class=\"language-{}\"",
                    html_escape_attribute(language)
                ));
            }
            output.push('>');
            output.push_str(&html_escape(code));
            output.push_str("</code></pre>\n");
        }
        Block::Image(image) => {
            output.push_str("<figure>\n");
            let alt = match &image.alt {
                ImageAlt::Description(value) => value.as_str(),
                ImageAlt::Decorative => "",
            };
            output.push_str(&format!(
                "<img src=\"{}\" alt=\"{}\" loading=\"lazy\">\n",
                html_escape_attribute(&format!("/{}", image.source.trim_start_matches('/'))),
                html_escape_attribute(alt)
            ));
            if let Some(caption) = &image.caption {
                output.push_str(&format!(
                    "<figcaption>{}</figcaption>\n",
                    html_escape(caption)
                ));
            }
            output.push_str("</figure>\n");
        }
        Block::Paragraph(inlines) => {
            output.push_str("<p>");
            write_inline(inlines, output);
            output.push_str("</p>\n");
        }
    }
}

fn write_inline(inlines: &[Inline], output: &mut String) {
    for inline in inlines {
        match inline {
            Inline::Text(text) => output.push_str(&html_escape(text)),
            Inline::Emphasis(text) => {
                output.push_str("<em>");
                write_inline(text, output);
                output.push_str("</em>");
            }
            Inline::Strong(text) => {
                output.push_str("<strong>");
                write_inline(text, output);
                output.push_str("</strong>");
            }
            Inline::Code(code) => output.push_str(&format!("<code>{}</code>", html_escape(&code))),
            Inline::Link { label, href } => {
                output.push_str(&format!("<a href=\"{}\">", html_escape_attribute(&href)));
                write_inline(label, output);
                output.push_str("</a>");
            }
        }
    }
}

fn write_list(tag: &str, items: &[Vec<Inline>], output: &mut String) {
    output.push_str(&format!("<{tag}>\n"));
    for item in items {
        output.push_str("<li>");
        write_inline(item, output);
        output.push_str("</li>\n");
    }
    output.push_str(&format!("</{tag}>\n"));
}

fn html_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn html_escape_attribute(value: &str) -> String {
    value.replace('&', "&amp;").replace('"', "&quot;")
}
