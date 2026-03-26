use super::*;
use prosesmasher_adapters_outbound_parser_assertions::{
    assert_non_zero_syllables, assert_sentence_count, assert_sentence_texts, assert_word_syllable,
    assert_word_texts,
};
use prosesmasher_domain_types::Locale;

fn sentences(
    text: &str,
    locale: Locale,
    expected_count: usize,
) -> Vec<prosesmasher_domain_types::Sentence> {
    let sentences = segment_paragraph(text, locale);
    assert_sentence_count(&sentences, expected_count, text);
    sentences
}

// ═══════════════════════════════════════════════════════════════
// segment_paragraph — sentence splitting
// ═══════════════════════════════════════════════════════════════

#[test]
fn empty_text_returns_no_sentences() {
    let _ = sentences("", Locale::En, 0);
}

#[test]
fn whitespace_only_returns_empty() {
    let _ = sentences("   \n\t  ", Locale::En, 0);
}

#[test]
fn single_sentence() {
    let result = sentences("Hello world.", Locale::En, 1);
    assert_sentence_texts(&result, &["Hello world."], "single sentence text");
}

#[test]
fn two_sentences() {
    let result = sentences("Hello world. How are you?", Locale::En, 2);
    assert_sentence_texts(
        &result,
        &["Hello world.", "How are you?"],
        "two sentence texts",
    );
}

#[test]
fn abbreviation_splits_in_icu4x() {
    // ICU4X treats "Dr." as a sentence boundary — this is ICU4X behavior,
    // not a bug in our code. Pinning the actual behavior.
    let result = sentences("Dr. Smith went home.", Locale::En, 2);
    assert_sentence_texts(
        &result,
        &["Dr.", "Smith went home."],
        "abbreviation split behavior",
    );
}

#[test]
fn trailing_whitespace_trimmed() {
    let result = sentences("Hello world.  ", Locale::En, 1);
    assert_eq!(
        result.first().map(|s| s.text.as_str()),
        Some("Hello world."),
        "trimmed exact text"
    );
}

#[test]
fn leading_whitespace_trimmed() {
    let result = sentences("  Hello world.", Locale::En, 1);
    assert_eq!(
        result.first().map(|s| s.text.as_str()),
        Some("Hello world."),
        "leading whitespace trimmed"
    );
}

#[test]
fn newline_splits_sentences() {
    // ICU4X treats newlines as sentence boundaries
    let _ = sentences("Hello\nworld.", Locale::En, 2);
}

#[test]
fn punctuation_only_returns_empty() {
    // "... !!! ???" has no word-like tokens — sentences with 0 words are filtered
    let _ = sentences("... !!! ???", Locale::En, 0);
}

#[test]
fn numbers_only_sentence() {
    let result = sentences("42 100 7.", Locale::En, 1);
    assert_eq!(
        result.first().map(Sentence::word_count),
        Some(3),
        "3 number words"
    );
}

// ═══════════════════════════════════════════════════════════════
// segment_paragraph — word extraction
// ═══════════════════════════════════════════════════════════════

#[test]
fn words_extracted_from_sentence() {
    let result = sentences("Hello world.", Locale::En, 1);
    assert_eq!(
        result.first().map(Sentence::word_count),
        Some(2),
        "two words"
    );
}

#[test]
fn punctuation_not_counted_as_words() {
    let result = sentences("Hello, world!", Locale::En, 1);
    assert_eq!(
        result.first().map(Sentence::word_count),
        Some(2),
        "punctuation excluded"
    );
}

#[test]
fn word_text_is_correct() {
    let result = sentences("Hello world.", Locale::En, 1);
    let sentence = result
        .first()
        .unwrap_or_else(|| panic!("word_text_is_correct: missing sentence"));
    assert_word_texts(sentence, &["Hello", "world"], "word texts");
}

#[test]
fn numbers_are_word_like() {
    let result = sentences("There are 42 cats.", Locale::En, 1);
    let words = result.first().map(|s| &s.words);
    assert!(
        words.is_some_and(|ws| ws.iter().any(|w| w.text == "42")),
        "42 should be a word-like token",
    );
}

#[test]
fn em_dash_splits_words() {
    // ICU4X treats em-dash as word boundary: "Hello—world" → ["Hello", "world"]
    let result = sentences("Hello\u{2014}world.", Locale::En, 1);
    assert_eq!(
        result.first().map(Sentence::word_count),
        Some(2),
        "em-dash splits into 2 words"
    );
}

#[test]
fn contraction_is_one_word() {
    // ICU4X treats "don't" as a single word-like token
    let result = sentences("I don't know.", Locale::En, 1);
    let words = result.first().map(|s| &s.words);
    assert!(
        words.is_some_and(|ws| ws.iter().any(|w| w.text == "don't")),
        "contraction 'don't' should be one word",
    );
}

#[test]
fn hyphenated_word_splits() {
    // ICU4X treats "well-known" as two words: ["well", "known"]
    let result = sentences("A well-known fact.", Locale::En, 1);
    let words = result.first().map(|s| &s.words);
    assert!(
        words.is_some_and(|ws| ws.iter().any(|w| w.text == "well")),
        "well should be a separate word",
    );
    assert!(
        words.is_some_and(|ws| ws.iter().any(|w| w.text == "known")),
        "known should be a separate word",
    );
}

#[test]
fn multiple_spaces_between_words() {
    let result = sentences("Hello    world.", Locale::En, 1);
    assert_eq!(
        result.first().map(Sentence::word_count),
        Some(2),
        "extra spaces don't create extra words"
    );
}

#[test]
fn multi_word_no_period() {
    let result = sentences("Hello world", Locale::En, 1);
    assert_eq!(
        result.first().map(Sentence::word_count),
        Some(2),
        "no period still produces words"
    );
}

#[test]
fn single_word_no_period() {
    let result = sentences("Hello", Locale::En, 1);
    assert_eq!(
        result.first().map(Sentence::word_count),
        Some(1),
        "one word"
    );
}

// ═══════════════════════════════════════════════════════════════
// segment_paragraph — syllable counts
//
// These tests assert exact syllable values and use the helper to
// guarantee sentences exist (no silent-pass risk).
// ═══════════════════════════════════════════════════════════════

#[test]
fn hello_has_two_syllables() {
    let result = sentences("Hello.", Locale::En, 1);
    let sentence = result
        .first()
        .unwrap_or_else(|| panic!("hello_has_two_syllables: missing sentence"));
    assert_word_syllable(sentence, 0, "Hello", 2, "Hello syllables");
}

#[test]
fn multi_syllable_word_exact() {
    let result = sentences("Beautiful.", Locale::En, 1);
    let sentence = result
        .first()
        .unwrap_or_else(|| panic!("multi_syllable_word_exact: missing sentence"));
    assert_word_syllable(sentence, 0, "Beautiful", 3, "Beautiful syllables");
}

#[test]
fn syllable_count_never_zero() {
    // Every word in the output must have syllable_count >= 1.
    // Test with a longer sentence to exercise multiple words.
    let result = sentences("The quick brown fox jumps over.", Locale::En, 1);
    let sentence = result
        .first()
        .unwrap_or_else(|| panic!("syllable_count_never_zero: missing sentence"));
    assert!(sentence.word_count() >= 5, "should have multiple words");
    assert_non_zero_syllables(&result, "non-zero syllables");
}

// ═══════════════════════════════════════════════════════════════
// segment_paragraph — multilingual with syllable verification
//
// Each locale test asserts both word counts AND at least one exact
// syllable count to prove the locale is actually passed through.
// ═══════════════════════════════════════════════════════════════

#[test]
fn russian_two_sentences() {
    let _ = sentences("Привет мир. Как дела?", Locale::Ru, 2);
}

#[test]
fn russian_words_and_syllables() {
    let result = sentences("молоко.", Locale::Ru, 1);
    let word = result.first().and_then(|s| s.words.first());
    assert_eq!(
        word.map(|w| w.text.as_str()),
        Some("молоко"),
        "Russian word text"
    );
    assert_eq!(
        word.map(|w| w.syllable_count),
        Some(3),
        "молоко = 3 syllables (proves Ru locale used)"
    );
}

#[test]
fn german_words_and_syllables() {
    let result = sentences("Schmetterling.", Locale::De, 1);
    let word = result.first().and_then(|s| s.words.first());
    assert_eq!(
        word.map(|w| w.text.as_str()),
        Some("Schmetterling"),
        "German word text"
    );
    assert_eq!(
        word.map(|w| w.syllable_count),
        Some(3),
        "Schmetterling = 3 syllables (proves De locale used)"
    );
}

#[test]
fn french_words_and_syllables() {
    let result = sentences("Bonjour le monde.", Locale::Fr, 1);
    assert_eq!(
        result.first().map(Sentence::word_count),
        Some(3),
        "exactly 3 French words"
    );
    let word = result.first().and_then(|s| s.words.first());
    assert_eq!(
        word.map(|w| w.text.as_str()),
        Some("Bonjour"),
        "French word text"
    );
    // Bonjour = 2 syllables: Bon-jour
    assert_eq!(
        word.map(|w| w.syllable_count),
        Some(2),
        "Bonjour = 2 syllables (proves Fr locale used)"
    );
}

#[test]
fn spanish_words_and_syllables() {
    let result = sentences("Mariposa.", Locale::Es, 1);
    let word = result.first().and_then(|s| s.words.first());
    assert_eq!(
        word.map(|w| w.text.as_str()),
        Some("Mariposa"),
        "Spanish word text"
    );
    assert_eq!(
        word.map(|w| w.syllable_count),
        Some(4),
        "Mariposa = 4 syllables (proves Es locale used)"
    );
}

#[test]
fn portuguese_words_and_syllables() {
    let result = sentences("Borboleta.", Locale::Pt, 1);
    let word = result.first().and_then(|s| s.words.first());
    assert_eq!(
        word.map(|w| w.text.as_str()),
        Some("Borboleta"),
        "Portuguese word text"
    );
    assert_eq!(
        word.map(|w| w.syllable_count),
        Some(4),
        "Borboleta = 4 syllables (proves Pt locale used)"
    );
}

#[test]
fn indonesian_words_and_syllables() {
    let result = sentences("Selamat pagi.", Locale::Id, 1);
    assert_eq!(
        result.first().map(Sentence::word_count),
        Some(2),
        "two Indonesian words"
    );
    // "Selamat" vowel clusters: e, a, a = but "a" and "a" separated by "m" = 3 clusters
    let word = result.first().and_then(|s| s.words.first());
    assert_eq!(
        word.map(|w| w.text.as_str()),
        Some("Selamat"),
        "Indonesian word text"
    );
    // Indonesian uses vowel fallback: S-e-l-a-m-a-t = e, a, a = 3 clusters
    assert_eq!(
        word.map(|w| w.syllable_count),
        Some(3),
        "Selamat = 3 syllables via vowel fallback"
    );
}

// ═══════════════════════════════════════════════════════════════
// segment_paragraph — word_count() consistency
// ═══════════════════════════════════════════════════════════════

#[test]
fn word_count_method_equals_words_len() {
    let result = sentences("The quick brown fox jumps.", Locale::En, 1);
    if let Some(sentence) = result.first() {
        assert_eq!(
            sentence.word_count(),
            sentence.words.len(),
            "word_count() == words.len()"
        );
    }
}

// ═══════════════════════════════════════════════════════════════
// 4-angle attack findings
// ═══════════════════════════════════════════════════════════════

#[test]
fn multi_sentence_last_without_period() {
    // Second sentence has no terminal punctuation — final segment must be captured
    let result = sentences("Hello world. How are you", Locale::En, 2);
    assert_eq!(
        result.get(1).map(Sentence::word_count),
        Some(3),
        "second sentence without period should have 3 words"
    );
}

#[test]
fn code_like_text_extracts_identifiers() {
    // Operators and braces should be filtered, alphabetic tokens kept
    let result = sentences("x = a + b.", Locale::En, 1);
    let words = result.first().map(|s| &s.words);
    assert!(
        words.is_some_and(|ws| ws.iter().any(|w| w.text == "x")),
        "identifier 'x' should be a word"
    );
    assert!(
        words.is_some_and(|ws| ws.iter().any(|w| w.text == "a")),
        "identifier 'a' should be a word"
    );
}

#[test]
fn long_text_100_words() {
    // Stress test: 100 words in one sentence should not panic or lose words
    let words: Vec<&str> = (0..100).map(|_| "word").collect();
    let text = format!("{}.", words.join(" "));
    let result = sentences(&text, Locale::En, 1);
    assert!(
        result.first().is_some_and(|s| s.word_count() >= 90),
        "100-word sentence should retain most words"
    );
}
