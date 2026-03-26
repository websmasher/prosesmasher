//! Boilerplate-framing check — flags repeated canned setup and preview scaffolding.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{Block, CheckConfig, Document, Locale};
use serde_json::{Value, json};

use crate::check::Check;
use crate::support::{normalize, sentence_evidence, strip_leading_prefixes, strip_quoted_segments};

#[derive(Debug)]
pub struct BoilerplateFramingCheck;

impl Check for BoilerplateFramingCheck {
    fn id(&self) -> &'static str {
        "boilerplate-framing"
    }

    fn label(&self) -> &'static str {
        "Boilerplate Framing"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        Some(&[Locale::En])
    }

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        if !config.quality.heuristics.boilerplate_framing.enabled {
            return;
        }
        if doc.locale != Locale::En {
            return;
        }

        let max = config.quality.heuristics.boilerplate_framing.max_per_document;
        let max_i64 = i64::try_from(max).unwrap_or(i64::MAX);
        let evidence = collect_boilerplate_framing_evidence(doc);
        let observed = i64::try_from(evidence.len()).unwrap_or(i64::MAX);
        let _result = suite
            .record_custom_values(
                "boilerplate-framing",
                observed <= max_i64,
                json!({ "max": max_i64 }),
                json!(observed),
                &evidence,
            )
            .label("Boilerplate Framing")
            .checking("repeated canned setup, preview, or list-preface framing per document");
    }
}

const LEADING_PREFIXES: &[&str] = &["however, ", "but ", "and ", "so ", "that being said, "];

const VAGUE_INTROS: &[&str] = &[
    "some",
    "common",
    "certain",
    "several",
    "following",
];

const CATEGORY_WORDS: &[&str] = &[
    "examples",
    "types",
    "reasons",
    "factors",
    "foods",
    "triggers",
    "ways",
    "steps",
    "sections",
    "parts",
];

const ENUMERATION_VERBS: &[&str] = &["include", "includes"];
const EXISTENCE_WORDS: &[&str] = &["there"];
const AUXILIARY_WORDS: &[&str] = &["are"];

const PREVIEW_OBJECTS: &[&str] = &["sections", "section", "parts", "part", "pages", "page"];
const PREVIEW_VERBS: &[&str] = &["explore", "discuss", "examine", "cover"];

fn collect_boilerplate_framing_evidence(doc: &Document) -> Vec<Value> {
    let mut evidence = Vec::new();
    let mut paragraph_index: usize = 0;

    for (section_index, section) in doc.sections.iter().enumerate() {
        for block in &section.blocks {
            collect_from_block(
                block,
                section_index,
                &mut paragraph_index,
                &mut evidence,
            );
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
            for (sentence_index, sentence) in paragraph.sentences.iter().enumerate() {
                for (pattern_kind, matched_text) in match_boilerplate_framing(&sentence.text) {
                    evidence.push(sentence_evidence(
                        section_index,
                        *paragraph_index,
                        sentence_index,
                        &[
                            ("pattern_kind", pattern_kind),
                            ("matched_text", matched_text),
                            ("sentence", &sentence.text),
                        ],
                    ));
                }
            }
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

fn match_boilerplate_framing(sentence: &str) -> Vec<(&'static str, &'static str)> {
    let normalized = normalize(sentence);
    let stripped = strip_quoted_segments(strip_leading_prefixes(&normalized, LEADING_PREFIXES));
    let tokens = token_words(&stripped);
    let mut matches = Vec::new();

    if tokens.is_empty() {
        return matches;
    }
    if matches_preview_frame(&tokens) {
        matches.push(("preview-frame", "following + explore"));
    }
    if matches_topic_frame(&stripped) {
        matches.push(("topic-frame", "when it comes to"));
    }
    if matches_existence_frame(&tokens) {
        matches.push(("existence-frame", "there are certain/common"));
    }
    if let Some(signal) = match_enumeration_preface(&tokens) {
        matches.push(("enumeration-preface", signal));
    }

    matches
}

fn token_words(text: &str) -> Vec<&str> {
    text.split_whitespace()
        .map(|token| token.trim_matches(|ch: char| !ch.is_ascii_alphanumeric() && ch != '\''))
        .filter(|token| !token.is_empty())
        .collect()
}

fn tokens_contain_in_order(tokens: &[&str], groups: &[&[&str]]) -> bool {
    let mut search_from = 0usize;

    for group in groups {
        let Some(position) = tokens[search_from..]
            .iter()
            .position(|token| group.iter().any(|candidate| token == candidate))
        else {
            return false;
        };
        search_from += position + 1;
    }

    true
}

fn matches_preview_frame(tokens: &[&str]) -> bool {
    tokens_contain_in_order(
        tokens,
        &[&["following"], PREVIEW_OBJECTS, PREVIEW_VERBS],
    )
}

fn matches_topic_frame(normalized: &str) -> bool {
    normalized.contains("when it comes to")
}

fn matches_existence_frame(tokens: &[&str]) -> bool {
    tokens_contain_in_order(tokens, &[EXISTENCE_WORDS, AUXILIARY_WORDS, &["certain", "common"], CATEGORY_WORDS])
}

fn match_enumeration_preface(tokens: &[&str]) -> Option<&'static str> {
    if tokens_contain_in_order(tokens, &[&["some"], &["examples"], ENUMERATION_VERBS]) {
        return Some("some examples + include");
    }
    if tokens_contain_in_order(tokens, &[&["some", "common"], CATEGORY_WORDS, ENUMERATION_VERBS])
    {
        return Some("some/common + include");
    }
    if tokens_contain_in_order(tokens, &[&["certain", "several"], CATEGORY_WORDS, ENUMERATION_VERBS])
    {
        return Some("certain/several + include");
    }
    if tokens_contain_in_order(tokens, &[VAGUE_INTROS, CATEGORY_WORDS, ENUMERATION_VERBS]) {
        return Some("vague intro + include");
    }
    None
}

#[cfg(test)]
#[path = "slop_04_boilerplate_framing_tests/mod.rs"]
mod tests;
