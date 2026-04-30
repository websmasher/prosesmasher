//! Demonstrative-emphasis check — flags repeated short demonstrative-subject
//! emphatic sentences ("That is X.", "This is where Y.", "That difference matters.").

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{Block, CheckConfig, Document, Locale, Paragraph, Sentence};
use serde_json::{Value, json};

use crate::check::Check;

const MAX_SENTENCE_WORDS: usize = 12;

const EMPHATIC_INTRANSITIVE_VERBS: &[&str] = &[
    "matters", "counts", "helps", "works", "fails", "hurts", "sticks", "lasts", "applies",
    "holds", "wins", "dies", "burns", "stops", "ends", "begins", "remains", "follows",
];

const DEMONSTRATIVE_SUBJECTS: &[&str] = &["that", "this", "these", "those"];

const DEFINITE_DETERMINERS: &[&str] = &["the", "that", "this", "these", "those"];

const COPULAR_VERBS: &[&str] = &["is", "are", "was", "were"];

const RELATIVE_TAILS: &[&str] = &[
    "where", "how", "why", "what", "when", "who",
];

const PERCEPTION_VERBS: &[&str] = &[
    "looks", "look", "seems", "seem", "sounds", "sound", "feels", "feel", "appears", "appear",
];

const STOPWORD_NEXT_WORD: &[&str] = &[
    "the", "a", "an", "it", "he", "she", "they", "we", "you", "i", "of", "in", "on", "at", "by",
    "for", "to", "from", "with", "about", "into", "than", "as", "and", "or", "but",
];

const LEADING_PREFIXES: &[&str] = &["so ", "but ", "and ", "however, ", "however "];

#[derive(Debug)]
pub struct DemonstrativeEmphasisCheck;

impl Check for DemonstrativeEmphasisCheck {
    fn id(&self) -> &'static str {
        "demonstrative-emphasis"
    }

    fn label(&self) -> &'static str {
        "Demonstrative Emphasis"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        Some(&[Locale::En])
    }

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        if !config
            .quality
            .heuristics
            .demonstrative_emphasis
            .enabled
        {
            return;
        }
        if doc.locale != Locale::En {
            return;
        }

        let max = config
            .quality
            .heuristics
            .demonstrative_emphasis
            .max_per_document;
        let max_i64 = i64::try_from(max).unwrap_or(i64::MAX);
        let evidence = collect_demonstrative_emphasis_evidence(doc);
        let observed = i64::try_from(evidence.len()).unwrap_or(i64::MAX);
        let _result = suite
            .record_custom_values(
                "demonstrative-emphasis",
                observed <= max_i64,
                json!({ "max": max_i64 }),
                json!(observed),
                &evidence,
            )
            .label("Demonstrative Emphasis")
            .checking("repeated short demonstrative-subject emphatic sentences");
    }
}

fn collect_demonstrative_emphasis_evidence(doc: &Document) -> Vec<Value> {
    let mut evidence = Vec::new();
    for (section_index, section) in doc.sections.iter().enumerate() {
        let mut paragraph_index: usize = 0;
        for block in &section.blocks {
            collect_from_block(block, section_index, &mut paragraph_index, &mut evidence);
        }
    }
    evidence
}

fn collect_from_block(
    block: &Block,
    section_index: usize,
    paragraph_index: &mut usize,
    evidence: &mut Vec<Value>,
) {
    match block {
        Block::Paragraph(paragraph) => {
            collect_from_paragraph(paragraph, section_index, *paragraph_index, evidence);
            *paragraph_index = paragraph_index.saturating_add(1);
        }
        Block::BlockQuote(inner) => {
            for inner_block in inner {
                collect_from_block(inner_block, section_index, paragraph_index, evidence);
            }
        }
        Block::List(_) | Block::CodeBlock(_) => {}
    }
}

fn collect_from_paragraph(
    paragraph: &Paragraph,
    section_index: usize,
    paragraph_index: usize,
    evidence: &mut Vec<Value>,
) {
    for (sentence_index, sentence) in paragraph.sentences.iter().enumerate() {
        let Some(pattern_kind) = classify_demonstrative_emphasis(sentence) else {
            continue;
        };
        evidence.push(json!({
            "section_index": section_index,
            "paragraph_index": paragraph_index,
            "sentence_index": sentence_index,
            "pattern_kind": pattern_kind,
            "sentence": sentence.text,
        }));
    }
}

fn classify_demonstrative_emphasis(sentence: &Sentence) -> Option<&'static str> {
    if sentence.word_count() < 3 || sentence.word_count() > MAX_SENTENCE_WORDS {
        return None;
    }

    let normalized = normalize_text(&sentence.text);
    let stripped = strip_leading_prefix(&normalized);
    let tokens: Vec<&str> = stripped.split_whitespace().collect();
    if tokens.len() < 3 {
        return None;
    }

    if let Some(kind) = classify_demonstrative_relative(&tokens) {
        return Some(kind);
    }
    if let Some(kind) = classify_demonstrative_perception(&tokens) {
        return Some(kind);
    }
    if let Some(kind) = classify_demonstrative_copular(&tokens) {
        return Some(kind);
    }
    if let Some(kind) = classify_definite_short_copular(&tokens) {
        return Some(kind);
    }
    if let Some(kind) = classify_demonstrative_np_copular(&tokens) {
        return Some(kind);
    }
    if let Some(kind) = classify_demonstrative_emphatic_verb(&tokens) {
        return Some(kind);
    }
    None
}

/// "That difference matters.", "That part counts."
/// Demonstrative + N (1-2 modifiers) + emphatic intransitive verb.
fn classify_demonstrative_emphatic_verb(tokens: &[&str]) -> Option<&'static str> {
    if tokens.len() < 3 || tokens.len() > 5 {
        return None;
    }
    if !DEMONSTRATIVE_SUBJECTS.contains(&tokens[0]) {
        return None;
    }
    let last = *tokens.last()?;
    if !EMPHATIC_INTRANSITIVE_VERBS.contains(&last) {
        return None;
    }
    Some("demonstrative-emphatic-verb")
}

/// "That is where ...", "This is how ...", "That's why ..."
fn classify_demonstrative_relative(tokens: &[&str]) -> Option<&'static str> {
    if tokens.len() < 4 {
        return None;
    }
    if !DEMONSTRATIVE_SUBJECTS.contains(&tokens[0]) {
        return None;
    }
    if !COPULAR_VERBS.contains(&tokens[1]) {
        return None;
    }
    if RELATIVE_TAILS.contains(&tokens[2]) {
        return Some("demonstrative-relative");
    }
    None
}

/// "That looks normal.", "That seems fine."
fn classify_demonstrative_perception(tokens: &[&str]) -> Option<&'static str> {
    if !DEMONSTRATIVE_SUBJECTS.contains(&tokens[0]) {
        return None;
    }
    if !PERCEPTION_VERBS.contains(&tokens[1]) {
        return None;
    }
    Some("demonstrative-perception")
}

/// "That is X.", "That is not X.", "This is fine."
fn classify_demonstrative_copular(tokens: &[&str]) -> Option<&'static str> {
    if !DEMONSTRATIVE_SUBJECTS.contains(&tokens[0]) {
        return None;
    }
    if !COPULAR_VERBS.contains(&tokens[1]) {
        return None;
    }
    let predicate_start = tokens.get(2)?;
    if RELATIVE_TAILS.contains(predicate_start) {
        return None;
    }
    if STOPWORD_NEXT_WORD.contains(predicate_start) {
        return None;
    }
    Some("demonstrative-copular")
}

/// "The hard part is the judgment call.", "The point is plain enough."
/// Definite-NP subject (≤ 4 words) + copula + short predicate.
fn classify_definite_short_copular(tokens: &[&str]) -> Option<&'static str> {
    if !DEFINITE_DETERMINERS.contains(&tokens[0]) {
        return None;
    }
    let copula_index = tokens
        .iter()
        .position(|tok| COPULAR_VERBS.contains(tok))?;
    if !(2..=4).contains(&copula_index) {
        return None;
    }
    let predicate = &tokens[copula_index + 1..];
    if predicate.is_empty() {
        return None;
    }
    let first_pred = predicate[0];
    if first_pred == "not" {
        return None;
    }
    if !DEFINITE_DETERMINERS.contains(&first_pred) {
        return None;
    }
    Some("definite-np-copular")
}

/// "That difference matters.", "That last part is where ..."
/// Demonstrative + N (≤2 modifiers) + verb/copular.
fn classify_demonstrative_np_copular(tokens: &[&str]) -> Option<&'static str> {
    if tokens.len() < 3 {
        return None;
    }
    if !DEMONSTRATIVE_SUBJECTS.contains(&tokens[0]) {
        return None;
    }
    let max_head_window = tokens.len().min(4);
    let copula_index = tokens[1..max_head_window]
        .iter()
        .position(|tok| COPULAR_VERBS.contains(tok))
        .map(|i| i + 1);
    if let Some(copula_idx) = copula_index {
        if copula_idx >= 2 {
            let predicate = &tokens[copula_idx + 1..];
            if predicate.first() == Some(&"not") {
                return Some("demonstrative-np-negation");
            }
            if predicate
                .first()
                .is_some_and(|w| RELATIVE_TAILS.contains(w))
            {
                return Some("demonstrative-np-relative");
            }
            if !predicate
                .first()
                .is_some_and(|w| STOPWORD_NEXT_WORD.contains(w))
            {
                return Some("demonstrative-np-copular");
            }
        }
    }
    None
}

fn strip_leading_prefix(text: &str) -> &str {
    for prefix in LEADING_PREFIXES {
        if let Some(rest) = text.strip_prefix(prefix) {
            return rest;
        }
    }
    text
}

fn normalize_text(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|ch| {
            if ch.is_alphanumeric() || ch.is_whitespace() || ch == '\'' {
                ch
            } else {
                ' '
            }
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
#[path = "heur_08_demonstrative_emphasis_tests/mod.rs"]
mod tests;
