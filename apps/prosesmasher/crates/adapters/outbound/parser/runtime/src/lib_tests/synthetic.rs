use prosesmasher_adapters_outbound_parser_assertions as assertions;
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
    assertions::assert_document_has_content(&doc, 2, "markdown parser should produce content");
}
