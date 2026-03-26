use prosesmasher_domain_types::{Block, Document};
use serde_json::Value;

pub(crate) fn collect_sentence_evidence<F>(doc: &Document, mut matcher: F) -> Vec<Value>
where
    F: FnMut(&str, usize, usize, usize) -> Option<Value>,
{
    let mut evidence = Vec::new();
    let mut paragraph_index: usize = 0;

    for (section_index, section) in doc.sections.iter().enumerate() {
        for block in &section.blocks {
            collect_from_block(
                block,
                section_index,
                &mut paragraph_index,
                &mut evidence,
                &mut matcher,
            );
        }
    }

    evidence
}

fn collect_from_block<F>(
    block: &Block,
    section_index: usize,
    paragraph_index: &mut usize,
    evidence: &mut Vec<Value>,
    matcher: &mut F,
) where
    F: FnMut(&str, usize, usize, usize) -> Option<Value>,
{
    match block {
        Block::Paragraph(paragraph) => {
            for (sentence_index, sentence) in paragraph.sentences.iter().enumerate() {
                if let Some(item) = matcher(
                    &sentence.text,
                    section_index,
                    *paragraph_index,
                    sentence_index,
                ) {
                    evidence.push(item);
                }
            }
            *paragraph_index = paragraph_index.saturating_add(1);
        }
        Block::BlockQuote(inner) => {
            for inner_block in inner {
                collect_from_block(
                    inner_block,
                    section_index,
                    paragraph_index,
                    evidence,
                    matcher,
                );
            }
        }
        Block::List(_) | Block::CodeBlock(_) => {}
    }
}

pub(crate) fn normalize(sentence: &str) -> String {
    sentence
        .chars()
        .map(|ch| match ch {
            '\u{2018}' | '\u{2019}' => '\'',
            '\u{201C}' | '\u{201D}' => '"',
            _ => ch.to_ascii_lowercase(),
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

pub(crate) fn strip_leading_prefixes<'a>(mut text: &'a str, prefixes: &[&str]) -> &'a str {
    loop {
        let Some(prefix) = prefixes.iter().find(|prefix| text.starts_with(**prefix)) else {
            return text;
        };
        text = text.strip_prefix(*prefix).unwrap_or(text);
    }
}

pub(crate) fn contains_any<'a>(text: &str, patterns: &'a [&str]) -> Option<&'a str> {
    patterns
        .iter()
        .find(|pattern| text.contains(**pattern))
        .copied()
}

pub(crate) fn strip_quoted_segments(text: &str) -> String {
    let mut unquoted = String::new();
    let mut in_quotes = false;

    for ch in text.chars() {
        if ch == '"' {
            in_quotes = !in_quotes;
            continue;
        }
        if !in_quotes {
            unquoted.push(ch);
        }
    }

    unquoted.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub(crate) fn sentence_evidence(
    section_index: usize,
    paragraph_index: usize,
    sentence_index: usize,
    pairs: &[(&str, &str)],
) -> serde_json::Value {
    let mut map = serde_json::Map::new();
    let _ = map.insert(
        "section_index".to_string(),
        serde_json::json!(section_index),
    );
    let _ = map.insert(
        "paragraph_index".to_string(),
        serde_json::json!(paragraph_index),
    );
    let _ = map.insert(
        "sentence_index".to_string(),
        serde_json::json!(sentence_index),
    );
    for (key, value) in pairs {
        let _ = map.insert((*key).to_string(), serde_json::json!(*value));
    }
    serde_json::Value::Object(map)
}
