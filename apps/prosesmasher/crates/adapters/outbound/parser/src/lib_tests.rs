use prosesmasher_domain_types::Locale;
use prosesmasher_ports_outbound_traits::DocumentParser;

use super::MarkdownParser;

#[test]
fn markdown_parser_implements_trait() {
    let parser: &dyn DocumentParser = &MarkdownParser;
    let result = parser.parse("Hello world.", &Locale::En);
    assert!(result.is_ok(), "parse must return Ok");
    let doc = result.unwrap_or_else(|_| prosesmasher_domain_types::Document {
        locale: Locale::En,
        sections: vec![],
        metadata: prosesmasher_domain_types::DocumentMetadata::default(),
    });
    assert!(!doc.sections.is_empty(), "should produce sections");
    assert!(doc.metadata.total_words >= 2, "should have words — got {}", doc.metadata.total_words);
}
