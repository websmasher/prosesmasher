//! Boilerplate-conclusion check — flags canned closing formulas near document endings.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{Block, CheckConfig, Document, Locale};
use serde_json::{Value, json};

use crate::check::Check;
use crate::support::{
    contains_any, normalize, sentence_evidence, strip_leading_prefixes, strip_quoted_segments,
};

#[derive(Debug)]
pub struct BoilerplateConclusionCheck;

impl Check for BoilerplateConclusionCheck {
    fn id(&self) -> &'static str {
        "boilerplate-conclusion"
    }

    fn label(&self) -> &'static str {
        "Boilerplate Conclusion"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        Some(&[Locale::En])
    }

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        if !config.quality.heuristics.boilerplate_conclusion.enabled {
            return;
        }
        if doc.locale != Locale::En {
            return;
        }

        let evidence = collect_boilerplate_conclusion_evidence(doc);
        let _result = suite
            .record_custom_values(
                "boilerplate-conclusion",
                evidence.is_empty(),
                json!({ "min": 0, "max": 0 }),
                json!(evidence.len()),
                &evidence,
            )
            .label("Boilerplate Conclusion")
            .checking("canned reassurance, authority, or key-insight conclusion formulas near the document ending");
    }
}

const LEADING_PREFIXES: &[&str] = &[
    "ultimately, ",
    "overall, ",
    "in the end, ",
    "in short, ",
    "and ",
    "but ",
];

const IMPORTANCE_CUES: &[&str] = &["most important", "single most important", "deepest"];

const SUMMARY_NOUNS: &[&str] = &["insight", "reason", "idea", "step"];

const AUTHORITY_CLOSE_PATTERNS: &[&str] = &[
    "the research is clear",
    "science is clear",
    "decades of research",
    "research leaves no doubt",
];

const ACCEPTANCE_CLOSE_PATTERNS: &[&str] = &[
    "not something you have to accept as normal",
    "not something you need to accept as normal",
    "is not a luxury",
];
const RESPONSE_CLOSE_PATTERNS: &[&str] = &["the practical response is plain"];
const BASIC_RULE_SIMPLE_PATTERN: &str = "the basic rule is simple";
const COMPRESSION_CLOSE_PATTERNS: &[&str] =
    &["the whole trick", "the core fact", "the rest is detail"];

#[derive(Clone)]
struct SentenceRef {
    section_index: usize,
    paragraph_index: usize,
    sentence_index: usize,
    sentence: String,
    is_document_tail: bool,
}

fn collect_boilerplate_conclusion_evidence(doc: &Document) -> Vec<Value> {
    let sentences = closing_candidate_sentences(doc);
    sentences
        .iter()
        .filter_map(|candidate| {
            match_boilerplate_conclusion(candidate).map(|(pattern_kind, matched_signal)| {
                sentence_evidence(
                    candidate.section_index,
                    candidate.paragraph_index,
                    candidate.sentence_index,
                    &[
                        ("pattern_kind", pattern_kind),
                        ("matched_signal", matched_signal),
                        ("sentence", &candidate.sentence),
                    ],
                )
            })
        })
        .collect()
}

fn closing_candidate_sentences(doc: &Document) -> Vec<SentenceRef> {
    let mut sentences = Vec::new();
    let mut paragraph_index = 0usize;

    for (section_index, section) in doc.sections.iter().enumerate() {
        for block in &section.blocks {
            collect_paragraph_sentences(block, section_index, &mut paragraph_index, &mut sentences);
        }
    }

    let keep = sentences.len().min(3);
    let tail_start = sentences.len().saturating_sub(keep);
    for (index, sentence) in sentences.iter_mut().enumerate() {
        sentence.is_document_tail = index >= tail_start;
    }

    sentences
}

fn collect_paragraph_sentences(
    block: &Block,
    section_index: usize,
    paragraph_index: &mut usize,
    sentences: &mut Vec<SentenceRef>,
) {
    match block {
        Block::Paragraph(paragraph) => {
            for (sentence_index, sentence) in paragraph.sentences.iter().enumerate() {
                sentences.push(SentenceRef {
                    section_index,
                    paragraph_index: *paragraph_index,
                    sentence_index,
                    sentence: sentence.text.clone(),
                    is_document_tail: false,
                });
            }
            *paragraph_index = paragraph_index.saturating_add(1);
        }
        Block::BlockQuote(_) | Block::List(_) | Block::CodeBlock(_) => {}
    }
}

fn match_boilerplate_conclusion(candidate: &SentenceRef) -> Option<(&'static str, &'static str)> {
    let normalized = normalize(&candidate.sentence);
    let stripped = strip_quoted_segments(strip_leading_prefixes(&normalized, LEADING_PREFIXES));

    if candidate.is_document_tail {
        if let Some(signal) = match_insight_close(&stripped) {
            return Some(("insight-close", signal));
        }
        if let Some(signal) = contains_any(&stripped, AUTHORITY_CLOSE_PATTERNS) {
            return Some(("authority-close", signal));
        }
        if let Some(signal) = contains_any(&stripped, ACCEPTANCE_CLOSE_PATTERNS) {
            return Some(("acceptance-close", signal));
        }
    }
    if let Some(signal) = match_response_close(&stripped) {
        return Some(("response-close", signal));
    }
    if let Some(signal) = match_compression_close(&stripped) {
        return Some(("compression-close", signal));
    }
    None
}

fn is_basic_rule_simple_close(normalized: &str) -> bool {
    let Some(remainder) = normalized.strip_prefix(BASIC_RULE_SIMPLE_PATTERN) else {
        return false;
    };

    let tail = remainder.trim();
    tail.is_empty() || tail == "."
}

fn match_response_close(normalized: &str) -> Option<&'static str> {
    if let Some(signal) = contains_any(normalized, RESPONSE_CLOSE_PATTERNS) {
        return Some(signal);
    }
    is_basic_rule_simple_close(normalized).then_some(BASIC_RULE_SIMPLE_PATTERN)
}

fn match_compression_close(normalized: &str) -> Option<&'static str> {
    contains_any(normalized, COMPRESSION_CLOSE_PATTERNS)
}

fn match_insight_close(normalized: &str) -> Option<&'static str> {
    let importance = contains_any(normalized, IMPORTANCE_CUES)?;
    if contains_any(normalized, SUMMARY_NOUNS).is_some() {
        Some(importance)
    } else {
        None
    }
}

#[cfg(test)]
#[path = "slop_08_boilerplate_conclusion_tests/mod.rs"]
mod tests;
