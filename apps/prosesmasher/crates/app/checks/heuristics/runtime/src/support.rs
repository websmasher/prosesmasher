use prosesmasher_domain_types::{Block, CheckConfig, Document, Locale, Paragraph};
use serde_json::{Value, json};

type SentenceSelector = fn(&[Block]) -> Option<SentenceRef<'_>>;
type SentenceMatcher = fn(&str, &str) -> bool;
type SentenceRef<'a> = (&'a str, usize);

fn first_paragraph(blocks: &[Block]) -> Option<&Paragraph> {
    for block in blocks {
        match block {
            Block::Paragraph(paragraph) => return Some(paragraph),
            Block::BlockQuote(inner) => {
                if let Some(paragraph) = first_paragraph(inner) {
                    return Some(paragraph);
                }
            }
            Block::List(_) | Block::CodeBlock(_) => {}
        }
    }

    None
}

fn last_paragraph(blocks: &[Block]) -> Option<&Paragraph> {
    for block in blocks.iter().rev() {
        match block {
            Block::Paragraph(paragraph) => return Some(paragraph),
            Block::BlockQuote(inner) => {
                if let Some(paragraph) = last_paragraph(inner) {
                    return Some(paragraph);
                }
            }
            Block::List(_) | Block::CodeBlock(_) => {}
        }
    }

    None
}

pub(crate) fn collect_section_sentence_evidence(
    doc: &Document,
    phrases: &[String],
    select_sentence: SentenceSelector,
    matcher: SentenceMatcher,
) -> Vec<Value> {
    let mut evidence = Vec::new();

    for (section_index, section) in doc.sections.iter().enumerate() {
        let Some((sentence_text, sentence_index)) = select_sentence(&section.blocks) else {
            continue;
        };
        let lowered_sentence = sentence_text.to_lowercase();

        for phrase in phrases {
            let lowered_phrase = phrase.to_lowercase();
            if matcher(&lowered_sentence, &lowered_phrase) {
                evidence.push(json!({
                    "section_index": section_index,
                    "sentence_index": sentence_index,
                    "matched_text": phrase,
                    "sentence": sentence_text,
                }));
            }
        }
    }

    evidence
}

pub(crate) fn section_first_sentence(blocks: &[Block]) -> Option<SentenceRef<'_>> {
    let paragraph = first_paragraph(blocks)?;
    let sentence = paragraph.sentences.first()?;
    Some((sentence.text.as_str(), 0))
}

pub(crate) fn section_last_sentence(blocks: &[Block]) -> Option<SentenceRef<'_>> {
    let paragraph = last_paragraph(blocks)?;
    let sentence_index = paragraph.sentences.len().checked_sub(1)?;
    let sentence = paragraph.sentences.get(sentence_index)?;
    Some((sentence.text.as_str(), sentence_index))
}

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

pub(crate) fn sentence_starts_with(sentence: &str, phrase: &str) -> bool {
    sentence.starts_with(phrase)
}

pub(crate) fn sentence_ends_with(sentence: &str, phrase: &str) -> bool {
    sentence.ends_with(phrase)
}

pub(crate) fn sentence_contains(sentence: &str, phrase: &str) -> bool {
    sentence.contains(phrase)
}

#[must_use]
pub fn resolve_llm_openers(config: &CheckConfig) -> Vec<String> {
    default_llm_openers(config.locale)
        .iter()
        .map(|item| (*item).to_owned())
        .collect()
}

#[must_use]
pub fn resolve_affirmation_closers(config: &CheckConfig) -> Vec<String> {
    default_affirmation_closers(config.locale)
        .iter()
        .map(|item| (*item).to_owned())
        .collect()
}

#[must_use]
pub fn resolve_summative_patterns(config: &CheckConfig) -> Vec<String> {
    default_summative_patterns(config.locale)
        .iter()
        .map(|item| (*item).to_owned())
        .collect()
}

#[must_use]
pub fn resolve_false_question_patterns(config: &CheckConfig) -> Vec<String> {
    default_false_question_patterns(config.locale)
        .iter()
        .map(|item| (*item).to_owned())
        .collect()
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

fn default_llm_openers(locale: Locale) -> &'static [&'static str] {
    if locale == Locale::En {
        &["the interesting part is", "in the world of"]
    } else {
        &[]
    }
}

fn default_affirmation_closers(locale: Locale) -> &'static [&'static str] {
    if locale == Locale::En {
        &["and that's the key.", "that's what matters."]
    } else {
        &[]
    }
}

fn default_summative_patterns(locale: Locale) -> &'static [&'static str] {
    if locale == Locale::En {
        &["and that's what makes", "that's why this"]
    } else {
        &[]
    }
}

fn default_false_question_patterns(locale: Locale) -> &'static [&'static str] {
    if locale == Locale::En {
        &["isn't that what we all", "isn't that the point"]
    } else {
        &[]
    }
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
