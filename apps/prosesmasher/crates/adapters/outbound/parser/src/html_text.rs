use scraper::{ElementRef, Html};

const IGNORED_TAGS: &[&str] = &[
    "canvas", "embed", "head", "iframe", "noscript", "object", "script", "style", "svg", "template",
];

const BLOCK_TAGS: &[&str] = &[
    "address",
    "article",
    "aside",
    "blockquote",
    "body",
    "dd",
    "div",
    "dl",
    "dt",
    "fieldset",
    "figcaption",
    "figure",
    "footer",
    "form",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "header",
    "li",
    "main",
    "nav",
    "ol",
    "p",
    "pre",
    "section",
    "table",
    "td",
    "th",
    "tr",
    "ul",
];

#[must_use]
pub fn extract_inline_html_text(html: &str) -> String {
    extract_html_block_paragraphs(html).join(" ")
}

#[must_use]
pub fn extract_html_block_paragraphs(html: &str) -> Vec<String> {
    let fragment = Html::parse_fragment(html);
    let mut paragraphs = Vec::new();
    let mut current = String::new();

    collect_visible_text(fragment.root_element(), &mut current, &mut paragraphs);
    flush_paragraph(&mut current, &mut paragraphs);

    paragraphs
}

fn collect_visible_text(
    element: ElementRef<'_>,
    current: &mut String,
    paragraphs: &mut Vec<String>,
) {
    let tag_name = element.value().name();
    if should_ignore_tag(tag_name) {
        return;
    }

    if tag_name == "br" || tag_name == "hr" {
        flush_paragraph(current, paragraphs);
        return;
    }

    let breaks_paragraphs = is_block_tag(tag_name);
    if breaks_paragraphs {
        flush_paragraph(current, paragraphs);
    }

    for child in element.children() {
        if let Some(text) = child.value().as_text() {
            current.push_str(text);
            continue;
        }

        if let Some(child_element) = ElementRef::wrap(child) {
            collect_visible_text(child_element, current, paragraphs);
        }
    }

    if breaks_paragraphs {
        flush_paragraph(current, paragraphs);
    }
}

fn flush_paragraph(current: &mut String, paragraphs: &mut Vec<String>) {
    let normalized = normalize_whitespace(current);
    if !normalized.is_empty() {
        paragraphs.push(normalized);
    }
    current.clear();
}

fn normalize_whitespace(text: &str) -> String {
    let mut normalized = String::new();
    let mut pending_space = false;

    for ch in text.chars() {
        if ch.is_whitespace() {
            pending_space = true;
            continue;
        }

        if pending_space && !normalized.is_empty() {
            normalized.push(' ');
        }

        normalized.push(ch);
        pending_space = false;
    }

    normalized
}

fn should_ignore_tag(tag_name: &str) -> bool {
    IGNORED_TAGS.contains(&tag_name)
}

fn is_block_tag(tag_name: &str) -> bool {
    BLOCK_TAGS.contains(&tag_name)
}
