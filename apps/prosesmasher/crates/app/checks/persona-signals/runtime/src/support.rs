use prosesmasher_domain_types::{Block, CheckConfig, Document, Locale};
use serde_json::{Value, json};

type SentenceMatcher = fn(&str, &str) -> bool;

pub(crate) fn collect_sentence_phrase_evidence(
    doc: &Document,
    phrases: &[String],
    predicate: SentenceMatcher,
) -> Vec<Value> {
    let mut evidence = Vec::new();
    let mut paragraph_index: usize = 0;

    for (section_index, section) in doc.sections.iter().enumerate() {
        for block in &section.blocks {
            collect_sentence_phrase_evidence_from_block(
                block,
                section_index,
                &mut paragraph_index,
                phrases,
                predicate,
                &mut evidence,
            );
        }
    }

    evidence
}

fn collect_sentence_phrase_evidence_from_block(
    block: &Block,
    section_index: usize,
    paragraph_index: &mut usize,
    phrases: &[String],
    predicate: SentenceMatcher,
    evidence: &mut Vec<Value>,
) {
    match block {
        Block::Paragraph(paragraph) => {
            for (sentence_index, sentence) in paragraph.sentences.iter().enumerate() {
                let lowered_sentence = sentence.text.to_lowercase();
                for phrase in phrases {
                    let lowered_phrase = phrase.to_lowercase();
                    if predicate(&lowered_sentence, &lowered_phrase) {
                        evidence.push(json!({
                            "section_index": section_index,
                            "paragraph_index": *paragraph_index,
                            "sentence_index": sentence_index,
                            "matched_text": phrase,
                            "sentence": sentence.text,
                        }));
                    }
                }
            }
            *paragraph_index = paragraph_index.saturating_add(1);
        }
        Block::BlockQuote(inner) => {
            for inner_block in inner {
                collect_sentence_phrase_evidence_from_block(
                    inner_block,
                    section_index,
                    paragraph_index,
                    phrases,
                    predicate,
                    evidence,
                );
            }
        }
        Block::List(_) | Block::CodeBlock(_) => {}
    }
}

pub(crate) fn sentence_contains(sentence: &str, phrase: &str) -> bool {
    sentence.contains(phrase)
}

#[must_use]
pub fn resolve_humble_bragger_phrases(config: &CheckConfig) -> Vec<String> {
    default_humble_bragger_phrases(config.locale)
        .iter()
        .map(|item| (*item).to_owned())
        .collect()
}

#[must_use]
pub fn resolve_jargon_faker_phrases(config: &CheckConfig) -> Vec<String> {
    default_jargon_faker_phrases(config.locale)
        .iter()
        .map(|item| (*item).to_owned())
        .collect()
}

fn default_humble_bragger_phrases(locale: Locale) -> &'static [&'static str] {
    if locale == Locale::En {
        &[
            "in my experience",
            "as someone who has",
            "having worked with",
        ]
    } else {
        &[]
    }
}

fn default_jargon_faker_phrases(locale: Locale) -> &'static [&'static str] {
    if locale == Locale::En {
        &[
            "debug our",
            "debug your",
            "optimizing for",
            "iterating on your",
        ]
    } else {
        &[]
    }
}
