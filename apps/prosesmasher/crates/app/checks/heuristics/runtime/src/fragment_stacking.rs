//! Fragment-stacking check — flags runs of clipped fragment sentences.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{Block, CheckConfig, Document, Locale, Paragraph, Sentence};
use serde_json::{Value, json};

use crate::check::Check;

const MAX_FRAGMENT_WORDS: usize = 6;
const MAX_PAYOFF_WORDS: usize = 28;
const SUBJECT_WORDS: &[&str] = &[
    "i", "you", "we", "they", "he", "she", "it", "this", "that", "these", "those",
];
const OBJECT_WORDS: &[&str] = &[
    "me", "him", "her", "us", "them", "it", "the", "a", "an", "my", "your", "our", "their",
];
const FINITE_VERBS: &[&str] = &[
    "is", "are", "was", "were", "am", "be", "been", "being", "have", "has", "had", "do", "does",
    "did", "can", "could", "will", "would", "should", "may", "might", "must", "shall",
];
const FRAGMENT_LEADS: &[&str] = &[
    "too",
    "most",
    "more",
    "less",
    "deeply",
    "completely",
    "possibly",
    "probably",
    "maybe",
    "weird",
    "strange",
    "odd",
    "pure",
    "total",
    "deeply",
    "incredibly",
];
const PAYOFF_STARTS: &[&str] = &["more like ", "most ", "then ", "instead "];
const IMPERATIVE_STARTS: &[&str] = &[
    "feed", "leave", "notice", "stop", "start", "take", "keep", "get", "look", "think", "try",
    "make", "let", "give", "accept", "hold", "reduce",
];
const SIMPLE_PAST_VERBS: &[&str] = &[
    "ran", "went", "came", "felt", "heard", "found", "made", "took", "kept", "left", "thought",
    "knew", "got", "put", "said", "told", "held", "stood", "sat", "became", "wrote", "spoke",
    "won", "lost", "paid", "met", "read", "saw", "grew", "fell", "broke",
];

#[derive(Debug)]
pub struct FragmentStackingCheck;

impl Check for FragmentStackingCheck {
    fn id(&self) -> &'static str {
        "fragment-stacking"
    }

    fn label(&self) -> &'static str {
        "Fragment Stacking"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        Some(&[Locale::En])
    }

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        if !config.quality.heuristics.fragment_stacking.enabled {
            return;
        }
        if doc.locale != Locale::En {
            return;
        }

        let evidence = collect_fragment_stacking_evidence(doc);
        let _result = suite
            .record_custom_values(
                "fragment-stacking",
                evidence.is_empty(),
                json!({ "min": 0, "max": 0 }),
                json!(evidence.len()),
                &evidence,
            )
            .label("Fragment Stacking")
            .checking("stacked clipped-fragment cadence runs");
    }
}

fn collect_fragment_stacking_evidence(doc: &Document) -> Vec<Value> {
    let mut evidence = Vec::new();

    for section in &doc.sections {
        for block in &section.blocks {
            collect_fragment_stacking_from_block(block, &mut evidence);
        }
    }

    evidence
}

fn collect_fragment_stacking_from_block(block: &Block, evidence: &mut Vec<Value>) {
    match block {
        Block::Paragraph(paragraph) => {
            collect_fragment_stacking_from_paragraph(paragraph, evidence)
        }
        Block::BlockQuote(inner) => {
            for inner_block in inner {
                collect_fragment_stacking_from_block(inner_block, evidence);
            }
        }
        Block::List(_) | Block::CodeBlock(_) => {}
    }
}

fn collect_fragment_stacking_from_paragraph(para: &Paragraph, evidence: &mut Vec<Value>) {
    if para.sentences.len() < 3 {
        return;
    }

    let classifications: Vec<Option<&'static str>> =
        para.sentences.iter().map(classify_fragment).collect();

    let mut sentence_index = 0usize;
    while sentence_index < para.sentences.len() {
        let Some(first_fragment_type) = classifications.get(sentence_index).and_then(|item| *item)
        else {
            sentence_index = sentence_index.saturating_add(1);
            continue;
        };
        let Some(first_sentence) = para.sentences.get(sentence_index) else {
            sentence_index = sentence_index.saturating_add(1);
            continue;
        };

        let mut run_sentences = vec![first_sentence.text.clone()];
        let mut fragment_types = vec![first_fragment_type];
        let mut cursor = sentence_index.saturating_add(1);

        while let Some(sentence) = para.sentences.get(cursor) {
            if let Some(next_fragment_type) = classifications.get(cursor).and_then(|item| *item) {
                run_sentences.push(sentence.text.clone());
                fragment_types.push(next_fragment_type);
                cursor = cursor.saturating_add(1);
                continue;
            }
            if looks_like_payoff_sentence(sentence) {
                run_sentences.push(sentence.text.clone());
                cursor = cursor.saturating_add(1);
            }
            break;
        }

        if fragment_types.len() >= 2 && run_sentences.len() >= 3 {
            evidence.push(json!({
                "sentences": run_sentences,
                "fragment_types": fragment_types,
            }));
        }

        sentence_index = cursor.max(sentence_index.saturating_add(1));
    }
}

fn classify_fragment(sentence: &Sentence) -> Option<&'static str> {
    if sentence.word_count() < 2 || sentence.word_count() > MAX_FRAGMENT_WORDS {
        return None;
    }

    let words: Vec<String> = sentence
        .words
        .iter()
        .map(|word| clean_word(&word.text))
        .collect();
    if words.iter().any(String::is_empty) {
        return None;
    }
    if words
        .iter()
        .any(|word| SUBJECT_WORDS.contains(&word.as_str()))
    {
        return None;
    }
    if words
        .iter()
        .any(|word| FINITE_VERBS.contains(&word.as_str()))
    {
        return None;
    }

    let first = words.first().map(String::as_str)?;
    let second = words.get(1).map(String::as_str)?;

    if FRAGMENT_LEADS.contains(&first) || looks_like_modifier_phrase(first, second) {
        return Some("modifier-fragment");
    }
    if looks_like_simple_clause(first, second) {
        return None;
    }
    if looks_like_brief_imperative(first, second) {
        return None;
    }
    if looks_like_subject_drop(first, second) {
        return Some("subject-drop");
    }

    Some("noun-fragment")
}

fn clean_word(word: &str) -> String {
    word.trim_matches(|c: char| !c.is_alphanumeric() && c != '\'')
        .to_lowercase()
}

fn looks_like_payoff_sentence(sentence: &Sentence) -> bool {
    if sentence.word_count() <= 2 || sentence.word_count() > MAX_PAYOFF_WORDS {
        return false;
    }

    let lowered = sentence.text.to_lowercase();
    if !PAYOFF_STARTS.iter().any(|start| lowered.starts_with(start)) {
        return false;
    }

    let words: Vec<String> = sentence
        .words
        .iter()
        .map(|word| clean_word(&word.text))
        .collect();
    !words
        .iter()
        .any(|word| SUBJECT_WORDS.contains(&word.as_str()))
}

fn looks_like_modifier_phrase(first: &str, second: &str) -> bool {
    first.ends_with("ly")
        || second.ends_with("ive")
        || second.ends_with("ous")
        || second.ends_with("al")
        || second.ends_with("ful")
        || second.ends_with("less")
}

fn looks_like_subject_drop(first: &str, second: &str) -> bool {
    ((first.ends_with("ed") && !first.ends_with("eed")) || first.ends_with("ing"))
        && !is_function_word(second)
}

fn looks_like_simple_clause(first: &str, second: &str) -> bool {
    !is_function_word(first) && (second.ends_with("ed") || SIMPLE_PAST_VERBS.contains(&second))
}

fn looks_like_brief_imperative(first: &str, second: &str) -> bool {
    IMPERATIVE_STARTS.contains(&first)
        && (OBJECT_WORDS.contains(&second) || second.ends_with("er") || second.ends_with("ly"))
}

fn is_function_word(word: &str) -> bool {
    matches!(
        word,
        "the" | "a" | "an" | "and" | "or" | "but" | "to" | "of" | "in" | "on" | "at" | "for"
    )
}

#[cfg(test)]
#[path = "fragment_stacking_tests/mod.rs"]
mod tests;
