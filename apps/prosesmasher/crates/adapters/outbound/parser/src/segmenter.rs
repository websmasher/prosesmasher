//! Text segmentation using ICU4X.
//!
//! Splits raw paragraph text into sentences, then each sentence into
//! words with syllable counts. Uses `SentenceSegmenter` and
//! `WordSegmenter` from ICU4X for locale-aware boundaries.

use icu_segmenter::{SentenceSegmenter, WordSegmenter};
use prosesmasher_domain_types::{Locale, Sentence, Word};

use crate::syllables::count_syllables;

/// Segment paragraph text into sentences with word-level detail.
///
/// Each returned `Sentence` has:
/// - `text`: the raw sentence string
/// - `words`: word-like tokens (letters/numbers, not punctuation/whitespace)
///   with syllable counts populated via the hyphenation dictionary
pub fn segment_paragraph(text: &str, locale: Locale) -> Vec<Sentence> {
    if text.is_empty() {
        return Vec::new();
    }

    let sentence_segmenter = SentenceSegmenter::new();
    let sentence_breaks: Vec<usize> = sentence_segmenter.segment_str(text).collect();

    let mut sentences = Vec::new();
    let mut prev = 0;

    for &brk in &sentence_breaks {
        if brk <= prev {
            continue;
        }

        // SAFETY: ICU4X guarantees break points are at valid UTF-8 boundaries
        // ICU4X SentenceSegmenter guarantees break points at UTF-8 char boundaries
        #[allow(clippy::string_slice)]
        let sentence_text = &text[prev..brk];
        let trimmed = sentence_text.trim();

        if !trimmed.is_empty() {
            let words = segment_words(trimmed, locale);
            if !words.is_empty() {
                sentences.push(Sentence {
                    text: trimmed.to_owned(),
                    words,
                });
            }
        }

        prev = brk;
    }

    sentences
}

/// Segment sentence text into word-like tokens with syllable counts.
///
/// Filters to word-like segments only (letters and numbers per ICU4X
/// `WordType::is_word_like`), skipping whitespace and punctuation.
fn segment_words(sentence: &str, locale: Locale) -> Vec<Word> {
    let word_segmenter = WordSegmenter::new_auto();
    let mut iter = word_segmenter.segment_str(sentence);
    let mut words = Vec::new();

    // First break is always 0 (start of string) — skip it
    let Some(mut prev) = iter.next() else {
        return words;
    };

    // Each call to next() advances the iterator. After advancing,
    // word_type() returns the type of the segment we just passed over
    // (between prev and the new break point).
    loop {
        let Some(brk) = iter.next() else {
            break;
        };
        let is_word_like = iter.word_type().is_word_like();
        // ICU4X WordSegmenter guarantees break points at UTF-8 char boundaries
        #[allow(clippy::string_slice)]
        let word_text = &sentence[prev..brk];
        let trimmed = word_text.trim();

        if is_word_like && !trimmed.is_empty() {
            let syllables = count_syllables(trimmed, locale);
            words.push(Word {
                text: trimmed.to_owned(),
                syllable_count: syllables,
            });
        }

        prev = brk;
    }

    words
}

#[cfg(test)]
#[path = "segmenter_tests.rs"]
mod tests;
