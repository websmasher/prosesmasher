pub fn assert_syllable_count(observed: usize, expected: usize, context: &str) {
    assert_eq!(
        observed,
        expected,
        "{context}: expected {expected} syllables, got {observed}"
    );
}
