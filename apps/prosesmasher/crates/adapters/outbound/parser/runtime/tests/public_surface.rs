use hyphenation as _;
use icu_segmenter as _;
use prosesmasher_adapters_outbound_parser_assertions::{
    assert_document_has_content, assert_first_paragraph, assert_first_paragraph_text,
    assert_heading, assert_top_level_blockquote_count, assert_top_level_list_counts,
    assert_total_sections,
};
use prosesmasher_adapters_outbound_parser_runtime::MarkdownParser;
use prosesmasher_domain_types::Locale;
use prosesmasher_ports_outbound_traits::DocumentParser;
use pulldown_cmark as _;
use scraper as _;
use std::path::Path;

fn fixture(name: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read fixture {}: {err}", path.display()))
}

#[test]
fn public_parser_trait_parses_clean_article_fixture() {
    let parser: &dyn DocumentParser = &MarkdownParser;
    let doc = parser
        .parse(&fixture("01-clean-article.md"), &Locale::En)
        .unwrap_or_else(|err| panic!("fixture 01 parse failed: {err}"));

    assert_document_has_content(&doc, 100, "fixture 01 should produce meaningful content");
    assert_total_sections(&doc, 9, "fixture 01 section split");
    let first = doc
        .sections
        .first()
        .unwrap_or_else(|| panic!("fixture 01 should have a first section"));
    assert_heading(
        first,
        1,
        "Building Resilient Distributed Systems: A Practical Guide",
        "fixture 01 first heading",
    );
}

#[test]
fn public_parser_trait_preserves_top_level_list_counts_in_fixture_03() {
    let parser: &dyn DocumentParser = &MarkdownParser;
    let doc = parser
        .parse(&fixture("03-lists-and-quotes.md"), &Locale::En)
        .unwrap_or_else(|err| panic!("fixture 03 parse failed: {err}"));

    assert_document_has_content(&doc, 500, "fixture 03 should parse to nontrivial content");
    assert_total_sections(&doc, 15, "fixture 03 section split");
    assert_top_level_list_counts(&doc, 6, 12, "fixture 03 top-level list counts");
}

#[test]
fn public_parser_trait_keeps_image_alt_text_out_of_paragraphs() {
    let parser: &dyn DocumentParser = &MarkdownParser;
    let doc = parser
        .parse(
            "Before ![alt text here](https://img.png) after.",
            &Locale::En,
        )
        .unwrap_or_else(|err| panic!("image fixture parse failed: {err}"));

    let paragraph = assert_first_paragraph(&doc);
    let observed: String = paragraph
        .sentences
        .iter()
        .map(|sentence| sentence.text.as_str())
        .collect();
    assert!(
        observed.contains("Before") && observed.contains("after."),
        "image removal should preserve surrounding text, got {observed:?}"
    );
    assert!(
        !observed.contains("alt text here"),
        "image alt text should stay out of public paragraph text, got {observed:?}"
    );
    assert!(
        paragraph.links.is_empty(),
        "image-only syntax should not create paragraph links"
    );
}

#[test]
fn public_parser_trait_preserves_top_level_blockquotes() {
    let parser: &dyn DocumentParser = &MarkdownParser;
    let doc = parser
        .parse("> Quoted text here.", &Locale::En)
        .unwrap_or_else(|err| panic!("blockquote parse failed: {err}"));

    assert_total_sections(&doc, 1, "blockquote section split");
    assert_top_level_blockquote_count(&doc, 1, "blockquote public contract");
}

#[test]
fn public_parser_trait_preserves_visible_html_text_only() {
    let parser: &dyn DocumentParser = &MarkdownParser;
    let doc = parser
        .parse(
            "<aside>Visible <script>hidden()</script><style>.x{}</style>text.</aside>",
            &Locale::En,
        )
        .unwrap_or_else(|err| panic!("html block parse failed: {err}"));

    let _ = assert_first_paragraph_text(
        &doc,
        "Visible text.",
        "raw html should preserve visible text and strip non-visible tags",
    );
}
