use prosesmasher_domain_types::Document;

pub fn assert_document_has_content(document: &Document, minimum_words: usize, context: &str) {
    assert!(
        !document.sections.is_empty(),
        "{context}: should produce sections"
    );
    assert!(
        document.metadata.total_words >= minimum_words,
        "{context}: should have at least {minimum_words} words, got {}",
        document.metadata.total_words
    );
}
