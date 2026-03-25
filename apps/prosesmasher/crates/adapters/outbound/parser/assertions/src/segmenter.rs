use prosesmasher_domain_types::Sentence;

pub fn assert_sentence_count(
    sentences: Vec<Sentence>,
    expected_count: usize,
    context: &str,
) -> Vec<Sentence> {
    assert_eq!(
        sentences.len(),
        expected_count,
        "{context}: expected {expected_count} sentences, got {}",
        sentences.len()
    );
    sentences
}
