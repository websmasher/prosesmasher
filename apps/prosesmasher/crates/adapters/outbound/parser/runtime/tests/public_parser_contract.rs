use hyphenation as _;
use icu_segmenter as _;
use prosesmasher_adapters_outbound_parser_assertions::{
    assert_first_paragraph_formatting, assert_first_paragraph_link, assert_heading_counts,
    assert_section_headings, assert_top_level_list_counts, assert_total_sections,
};
use prosesmasher_adapters_outbound_parser_runtime::MarkdownParser;
use prosesmasher_domain_types::Locale;
use prosesmasher_ports_outbound_traits::DocumentParser;
use pulldown_cmark as _;
use scraper as _;

fn load_fixture(name: &str) -> String {
    let path = format!("{}/tests/fixtures/{name}", env!("CARGO_MANIFEST_DIR"));
    std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("failed to load fixture {path}: {error}"))
}

#[test]
fn public_parser_contract_splits_sections_on_h1_and_h2_only() {
    let parser = MarkdownParser;
    let doc = parser
        .parse(
            "# Title\n\nIntro.\n\n## Section\n\nBody.\n\n### Detail\n\nMore.",
            &Locale::En,
        )
        .unwrap_or_else(|error| panic!("parse should succeed: {error:?}"));

    assert_total_sections(&doc, 2, "public section split contract");
    assert_section_headings(
        &doc,
        &[(1, Some("Title")), (2, Some("Section"))],
        "public section headings",
    );
    assert_heading_counts(&doc, 1, 1, 1, 0, "public heading metadata");
}

#[test]
fn public_parser_contract_extracts_formatting_and_links() {
    let parser = MarkdownParser;
    let doc = parser
        .parse("**Bold** [example](https://example.com) text.", &Locale::En)
        .unwrap_or_else(|error| panic!("parse should succeed: {error:?}"));

    let paragraph = assert_first_paragraph_formatting(&doc, true, false, "public formatting");
    let _ = assert_first_paragraph_link(
        &doc,
        "example",
        "https://example.com",
        "public link extraction",
    );
    assert_eq!(
        paragraph.links.len(),
        1,
        "public paragraph still has one link"
    );
}

#[test]
fn public_fixture_03_preserves_exact_top_level_list_counts() {
    let parser = MarkdownParser;
    let md = load_fixture("03-lists-and-quotes.md");
    let doc = parser
        .parse(&md, &Locale::En)
        .unwrap_or_else(|error| panic!("parse should succeed: {error:?}"));

    assert_total_sections(&doc, 15, "fixture 03 public section count");
    assert_heading_counts(&doc, 1, 14, 0, 0, "fixture 03 public heading counts");
    assert_top_level_list_counts(&doc, 6, 12, "fixture 03 public list counts");
}

#[test]
fn public_fixture_04_counts_h3_in_metadata_but_not_sections() {
    let parser = MarkdownParser;
    let md = load_fixture("04-multilingual-stress.md");
    let doc = parser
        .parse(&md, &Locale::En)
        .unwrap_or_else(|error| panic!("parse should succeed: {error:?}"));

    assert_total_sections(&doc, 10, "fixture 04 public section count");
    assert_heading_counts(&doc, 1, 9, 7, 0, "fixture 04 public heading counts");
}
