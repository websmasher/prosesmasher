use prosesmasher_domain_types::Sentence;

pub fn assert_sentence_count(sentences: &[Sentence], expected_count: usize, context: &str) {
    assert_eq!(
        sentences.len(),
        expected_count,
        "{context}: expected {expected_count} sentences, got {}",
        sentences.len()
    );
}

pub fn assert_sentence_texts(sentences: &[Sentence], expected: &[&str], context: &str) {
    let observed: Vec<&str> = sentences
        .iter()
        .map(|sentence| sentence.text.as_str())
        .collect();
    assert_eq!(
        observed, expected,
        "{context}: expected sentence texts {expected:?}, got {observed:?}"
    );
}

pub fn assert_word_texts(sentence: &Sentence, expected: &[&str], context: &str) {
    let observed: Vec<&str> = sentence
        .words
        .iter()
        .map(|word| word.text.as_str())
        .collect();
    assert_eq!(
        observed, expected,
        "{context}: expected word texts {expected:?}, got {observed:?}"
    );
}

pub fn assert_word_syllable(
    sentence: &Sentence,
    word_index: usize,
    expected_text: &str,
    expected_syllables: usize,
    context: &str,
) {
    let word = sentence
        .words
        .get(word_index)
        .unwrap_or_else(|| panic!("{context}: missing word at index {word_index}"));

    assert_eq!(word.text, expected_text, "{context}: word text");
    assert_eq!(
        word.syllable_count, expected_syllables,
        "{context}: expected {expected_syllables} syllables, got {}",
        word.syllable_count
    );
}

pub fn assert_non_zero_syllables(sentences: &[Sentence], context: &str) {
    for sentence in sentences {
        for word in &sentence.words {
            assert!(
                word.syllable_count > 0,
                "{context}: word {:?} should never have zero syllables",
                word.text
            );
        }
    }
}
