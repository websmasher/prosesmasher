//! Shared test helpers for building hand-crafted `Document` structs.

use prosesmasher_domain_types::{
    Block, Document, DocumentMetadata, Locale, Paragraph, Section, Sentence, Word,
};

/// Build a simple document with one paragraph containing the given text.
///
/// Words are split on whitespace. All syllable counts are set to 1.
/// Metadata `total_words` is computed from the words vec.
pub fn make_doc(text: &str, locale: Locale) -> Document {
    let words: Vec<Word> = text
        .split_whitespace()
        .map(|w| Word {
            text: w.to_owned(),
            syllable_count: 1,
        })
        .collect();
    let word_count = words.len();

    Document {
        locale,
        sections: vec![Section {
            heading: None,
            blocks: vec![Block::Paragraph(Paragraph {
                sentences: vec![Sentence {
                    text: text.to_owned(),
                    words,
                }],
                has_bold: false,
                has_italic: false,
                links: vec![],
            })],
        }],
        metadata: DocumentMetadata {
            total_words: word_count,
            total_sentences: 1,
            ..Default::default()
        },
    }
}

/// Build a document with a specific `total_words` in metadata.
///
/// Creates N dummy words. Useful for testing word count thresholds.
pub fn make_doc_with_word_count(word_count: usize, locale: Locale) -> Document {
    let words: Vec<Word> = (0..word_count)
        .map(|i| Word {
            text: format!("word{i}"),
            syllable_count: 1,
        })
        .collect();
    let text = words.iter().map(|w| w.text.as_str()).collect::<Vec<_>>().join(" ");

    Document {
        locale,
        sections: vec![Section {
            heading: None,
            blocks: vec![Block::Paragraph(Paragraph {
                sentences: vec![Sentence { text, words }],
                has_bold: false,
                has_italic: false,
                links: vec![],
            })],
        }],
        metadata: DocumentMetadata {
            total_words: word_count,
            total_sentences: 1,
            ..Default::default()
        },
    }
}

/// Build a document with text inside a blockquote.
pub fn make_doc_in_blockquote(text: &str, locale: Locale) -> Document {
    let words: Vec<Word> = text
        .split_whitespace()
        .map(|w| Word { text: w.to_owned(), syllable_count: 1 })
        .collect();
    let word_count = words.len();

    Document {
        locale,
        sections: vec![Section {
            heading: None,
            blocks: vec![Block::BlockQuote(vec![
                Block::Paragraph(Paragraph {
                    sentences: vec![Sentence { text: text.to_owned(), words }],
                    has_bold: false,
                    has_italic: false,
                    links: vec![],
                }),
            ])],
        }],
        metadata: DocumentMetadata {
            total_words: word_count,
            total_sentences: 1,
            ..Default::default()
        },
    }
}

/// Build a document with only a code block (no paragraphs).
pub fn make_doc_code_only(code: &str, locale: Locale) -> Document {
    Document {
        locale,
        sections: vec![Section {
            heading: None,
            blocks: vec![Block::CodeBlock(code.to_owned())],
        }],
        metadata: DocumentMetadata::default(),
    }
}

/// Build a document with multiple sections, each with one paragraph.
pub fn make_doc_multi_section(texts: &[&str], locale: Locale) -> Document {
    let mut total_words: usize = 0;
    let sections: Vec<Section> = texts
        .iter()
        .map(|text| {
            let words: Vec<Word> = text
                .split_whitespace()
                .map(|w| Word { text: w.to_owned(), syllable_count: 1 })
                .collect();
            total_words = total_words.saturating_add(words.len());
            Section {
                heading: None,
                blocks: vec![Block::Paragraph(Paragraph {
                    sentences: vec![Sentence { text: (*text).to_owned(), words }],
                    has_bold: false,
                    has_italic: false,
                    links: vec![],
                })],
            }
        })
        .collect();

    Document {
        locale,
        sections,
        metadata: DocumentMetadata {
            total_words,
            total_sentences: texts.len(),
            ..Default::default()
        },
    }
}
