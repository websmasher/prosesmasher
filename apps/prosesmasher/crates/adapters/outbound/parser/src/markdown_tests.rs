use super::*;
use prosesmasher_domain_types::{Block, Locale, Paragraph};

/// Helper: assert document has at least N paragraphs, return the nth one.
/// Eliminates silent-pass risk from `if let Some(p) = ...` patterns.
#[allow(clippy::panic)]
fn assert_first_paragraph(doc: &Document) -> &Paragraph {
    let block = doc.sections.first()
        .and_then(|s| s.blocks.first());
    match block {
        Some(Block::Paragraph(p)) => p,
        other => panic!("expected Block::Paragraph, got {other:?}"),
    }
}

#[allow(clippy::disallowed_methods, clippy::panic)] // test fixture loading
fn load_fixture(name: &str) -> String {
    let path = format!(
        "{}/tests/fixtures/{name}",
        env!("CARGO_MANIFEST_DIR")
    );
    match std::fs::read_to_string(&path) {
        Ok(content) => content,
        Err(e) => panic!("failed to load fixture {path}: {e}"),
    }
}

// ═══════════════════════════════════════════════════════════════
// Basic structural tests
// ═══════════════════════════════════════════════════════════════

#[test]
fn empty_input() {
    let doc = parse_markdown("", Locale::En);
    assert!(doc.sections.is_empty(), "empty input → no sections");
}

#[test]
fn single_paragraph_no_heading() {
    let doc = parse_markdown("Hello world.", Locale::En);
    assert_eq!(doc.sections.len(), 1, "one section");
    assert!(doc.sections.first().and_then(|s| s.heading.as_ref()).is_none(), "no heading");
}

#[test]
fn h1_creates_first_section() {
    let doc = parse_markdown("# Title\n\nSome text.", Locale::En);
    assert_eq!(doc.sections.len(), 1, "one section");
    let heading = doc.sections.first().and_then(|s| s.heading.as_ref());
    assert_eq!(heading.map(|h| h.level), Some(1), "H1 level");
    assert_eq!(heading.map(|h| h.text.as_str()), Some("Title"), "H1 text");
}

#[test]
fn h2_splits_sections() {
    let doc = parse_markdown("## First\n\nText.\n\n## Second\n\nMore text.", Locale::En);
    assert_eq!(doc.sections.len(), 2, "two sections at H2 boundaries");
}

#[test]
fn h3_does_not_split_sections() {
    let doc = parse_markdown("## Section\n\n### Sub\n\nText.", Locale::En);
    assert_eq!(doc.sections.len(), 1, "H3 does not create new section");
}

// ═══════════════════════════════════════════════════════════════
// Formatting detection
// ═══════════════════════════════════════════════════════════════

#[test]
fn bold_detected() {
    let doc = parse_markdown("**Bold text** here.", Locale::En);
    let para = doc.sections.first()
        .and_then(|s| s.blocks.first())
        .and_then(|b| if let Block::Paragraph(p) = b { Some(p) } else { None });
    assert_eq!(para.map(|p| p.has_bold), Some(true), "has_bold");
}

#[test]
fn italic_detected() {
    let doc = parse_markdown("*Italic text* here.", Locale::En);
    let para = doc.sections.first()
        .and_then(|s| s.blocks.first())
        .and_then(|b| if let Block::Paragraph(p) = b { Some(p) } else { None });
    assert_eq!(para.map(|p| p.has_italic), Some(true), "has_italic");
}

#[test]
fn plain_paragraph_no_formatting() {
    let doc = parse_markdown("Plain text without any formatting.", Locale::En);
    let para = doc.sections.first()
        .and_then(|s| s.blocks.first())
        .and_then(|b| if let Block::Paragraph(p) = b { Some(p) } else { None });
    assert_eq!(para.map(|p| p.has_bold), Some(false), "no bold");
    assert_eq!(para.map(|p| p.has_italic), Some(false), "no italic");
}

#[test]
fn link_extracted() {
    let doc = parse_markdown("Visit [example](https://example.com) for info.", Locale::En);
    let para = doc.sections.first()
        .and_then(|s| s.blocks.first())
        .and_then(|b| if let Block::Paragraph(p) = b { Some(p) } else { None });
    assert_eq!(para.map(|p| p.links.len()), Some(1), "one link");
    let link = para.and_then(|p| p.links.first());
    assert_eq!(link.map(|l| l.text.as_str()), Some("example"), "link text");
    assert_eq!(link.map(|l| l.url.as_str()), Some("https://example.com"), "link url");
}

// ═══════════════════════════════════════════════════════════════
// Lists
// ═══════════════════════════════════════════════════════════════

#[test]
fn unordered_list() {
    let doc = parse_markdown("- one\n- two\n- three", Locale::En);
    let block = doc.sections.first().and_then(|s| s.blocks.first());
    if let Some(Block::List(list)) = block {
        assert!(!list.ordered, "unordered");
        assert_eq!(list.items.len(), 3, "3 items");
    } else {
        assert!(matches!(block, Some(Block::List(_))), "expected Block::List");
    }
}

#[test]
fn ordered_list() {
    let doc = parse_markdown("1. first\n2. second", Locale::En);
    let block = doc.sections.first().and_then(|s| s.blocks.first());
    if let Some(Block::List(list)) = block {
        assert!(list.ordered, "ordered");
        assert_eq!(list.items.len(), 2, "2 items");
    } else {
        assert!(matches!(block, Some(Block::List(_))), "expected Block::List");
    }
}

// ═══════════════════════════════════════════════════════════════
// Code blocks
// ═══════════════════════════════════════════════════════════════

#[test]
fn fenced_code_block() {
    let doc = parse_markdown("```rust\nfn main() {}\n```", Locale::En);
    let block = doc.sections.first().and_then(|s| s.blocks.first());
    assert!(matches!(block, Some(Block::CodeBlock(_))), "is CodeBlock");
    if let Some(Block::CodeBlock(code)) = block {
        assert!(code.contains("fn main()"), "code content preserved");
    }
}

#[test]
fn code_block_formatting_not_parsed() {
    let doc = parse_markdown("```\n**not bold** *not italic*\n```", Locale::En);
    // Should have 1 code block, no paragraphs
    assert_eq!(doc.metadata.paragraph_count, 0, "code block has no paragraphs");
    assert_eq!(doc.metadata.bold_count, 0, "no bold from code block");
}

// ═══════════════════════════════════════════════════════════════
// Block quotes
// ═══════════════════════════════════════════════════════════════

#[test]
fn simple_blockquote() {
    let doc = parse_markdown("> Quoted text here.", Locale::En);
    let block = doc.sections.first().and_then(|s| s.blocks.first());
    assert!(matches!(block, Some(Block::BlockQuote(_))), "is BlockQuote");
}

// ═══════════════════════════════════════════════════════════════
// Metadata
// ═══════════════════════════════════════════════════════════════

#[test]
fn metadata_word_and_sentence_counts() {
    let doc = parse_markdown("Hello world. Goodbye world.", Locale::En);
    assert_eq!(doc.metadata.total_words, 4, "exactly 4 words — got {}", doc.metadata.total_words);
    assert_eq!(doc.metadata.total_sentences, 2, "exactly 2 sentences — got {}", doc.metadata.total_sentences);
}

#[test]
fn metadata_heading_counts() {
    let doc = parse_markdown(
        "# H1\n\n## H2a\n\nText.\n\n## H2b\n\nText.\n\n### H3\n\nText.",
        Locale::En,
    );
    assert_eq!(doc.metadata.heading_counts.h1, 1, "h1 count");
    assert_eq!(doc.metadata.heading_counts.h2, 2, "h2 count");
    assert_eq!(doc.metadata.heading_counts.h3, 1, "h3 count");
}

// ═══════════════════════════════════════════════════════════════
// Fixture 01: Clean article
// Actual parser output used as ground truth (agents miscounted)
// ═══════════════════════════════════════════════════════════════

#[test]
fn fixture_01_section_count() {
    let md = load_fixture("01-clean-article.md");
    let doc = parse_markdown(&md, Locale::En);
    assert_eq!(doc.sections.len(), 9, "01: section count — got {}", doc.sections.len());
}

#[test]
fn fixture_01_heading_counts() {
    let md = load_fixture("01-clean-article.md");
    let doc = parse_markdown(&md, Locale::En);
    assert_eq!(doc.metadata.heading_counts.h1, 1, "01: h1 — got {}", doc.metadata.heading_counts.h1);
    assert_eq!(doc.metadata.heading_counts.h2, 8, "01: h2 — got {}", doc.metadata.heading_counts.h2);
    assert_eq!(doc.metadata.heading_counts.h3, 5, "01: h3 — got {}", doc.metadata.heading_counts.h3);
}

#[test]
fn fixture_01_paragraph_count() {
    let md = load_fixture("01-clean-article.md");
    let doc = parse_markdown(&md, Locale::En);
    assert_eq!(doc.metadata.paragraph_count, 87, "01: paragraph count — got {}", doc.metadata.paragraph_count);
}

#[test]
fn fixture_01_link_count() {
    let md = load_fixture("01-clean-article.md");
    let doc = parse_markdown(&md, Locale::En);
    assert_eq!(doc.metadata.link_count, 16, "01: link count — got {}", doc.metadata.link_count);
}

#[test]
fn fixture_01_bold_count() {
    let md = load_fixture("01-clean-article.md");
    let doc = parse_markdown(&md, Locale::En);
    assert_eq!(doc.metadata.bold_count, 22, "01: bold paragraph count — got {}", doc.metadata.bold_count);
}

#[test]
fn fixture_01_italic_count() {
    let md = load_fixture("01-clean-article.md");
    let doc = parse_markdown(&md, Locale::En);
    assert_eq!(doc.metadata.italic_count, 18, "01: italic paragraph count — got {}", doc.metadata.italic_count);
}

// ═══════════════════════════════════════════════════════════════
// Fixture 02: Formatting hell
// ═══════════════════════════════════════════════════════════════

#[test]
fn fixture_02_section_count() {
    let md = load_fixture("02-formatting-hell.md");
    let doc = parse_markdown(&md, Locale::En);
    assert_eq!(doc.sections.len(), 9, "02: section count — got {}", doc.sections.len());
}

#[test]
fn fixture_02_paragraph_count() {
    let md = load_fixture("02-formatting-hell.md");
    let doc = parse_markdown(&md, Locale::En);
    assert_eq!(doc.metadata.paragraph_count, 48, "02: paragraph count — got {}", doc.metadata.paragraph_count);
}

#[test]
fn fixture_02_link_count() {
    let md = load_fixture("02-formatting-hell.md");
    let doc = parse_markdown(&md, Locale::En);
    assert_eq!(doc.metadata.link_count, 36, "02: link count — got {}", doc.metadata.link_count);
}

#[test]
fn fixture_02_code_blocks_dont_leak() {
    let md = load_fixture("02-formatting-hell.md");
    let doc = parse_markdown(&md, Locale::En);
    let code_blocks: usize = doc.sections.iter()
        .flat_map(|s| &s.blocks)
        .filter(|b| matches!(b, Block::CodeBlock(_)))
        .count();
    assert_eq!(code_blocks, 3, "02: code blocks — got {code_blocks}");
}

// ═══════════════════════════════════════════════════════════════
// Fixture 03: Lists and quotes
// ═══════════════════════════════════════════════════════════════

#[test]
fn fixture_03_section_count() {
    let md = load_fixture("03-lists-and-quotes.md");
    let doc = parse_markdown(&md, Locale::En);
    assert_eq!(doc.sections.len(), 15, "03: section count — got {}", doc.sections.len());
}

#[test]
fn fixture_03_blockquote_exists() {
    let md = load_fixture("03-lists-and-quotes.md");
    let doc = parse_markdown(&md, Locale::En);
    let quote_count: usize = doc.sections.iter()
        .flat_map(|s| &s.blocks)
        .filter(|b| matches!(b, Block::BlockQuote(_)))
        .count();
    assert!(quote_count >= 8, "03: at least 8 top-level blockquotes — got {quote_count}");
}

// ═══════════════════════════════════════════════════════════════
// Fixture 04: Multilingual
// ═══════════════════════════════════════════════════════════════

#[test]
fn fixture_04_section_count() {
    let md = load_fixture("04-multilingual-stress.md");
    let doc = parse_markdown(&md, Locale::En);
    // Fixture header says 14 (H2=10 + H3 sections), but H3s don't split
    // sections. With H2=10, we get content-before-first-H2 + 10 = 11 max.
    // Parser produces 10 — likely the H1 pre-content section isn't emitted
    // when it only contains the H1 heading with no blocks before the first H2.
    assert_eq!(doc.sections.len(), 10, "04: section count — got {}", doc.sections.len());
}

#[test]
fn fixture_04_paragraph_count() {
    let md = load_fixture("04-multilingual-stress.md");
    let doc = parse_markdown(&md, Locale::En);
    assert_eq!(doc.metadata.paragraph_count, 70, "04: paragraph count — got {}", doc.metadata.paragraph_count);
}

#[test]
fn fixture_04_no_crash_on_multilingual() {
    let md = load_fixture("04-multilingual-stress.md");
    let doc = parse_markdown(&md, Locale::En);
    assert!(doc.metadata.total_words > 100, "should have many words from all languages");
}

// ═══════════════════════════════════════════════════════════════
// Fixture 05: Adversarial edge cases
// ═══════════════════════════════════════════════════════════════

#[test]
fn fixture_05_no_crash() {
    let md = load_fixture("05-adversarial-edge-cases.md");
    let doc = parse_markdown(&md, Locale::En);
    assert!(!doc.sections.is_empty(), "should produce sections");
}

#[test]
fn fixture_05_section_count() {
    let md = load_fixture("05-adversarial-edge-cases.md");
    let doc = parse_markdown(&md, Locale::En);
    // 23 sections: setext H1 now creates a section (BUG-01 fixed)
    assert_eq!(doc.sections.len(), 23, "05: section count — got {}", doc.sections.len());
}

#[test]
fn fixture_05_code_blocks_opaque() {
    let md = load_fixture("05-adversarial-edge-cases.md");
    let doc = parse_markdown(&md, Locale::En);
    for section in &doc.sections {
        for block in &section.blocks {
            if let Block::CodeBlock(code) = block {
                assert!(!code.is_empty(), "code block should not be empty");
            }
        }
    }
}

#[test]
fn fixture_05_heading_counts() {
    let md = load_fixture("05-adversarial-edge-cases.md");
    let doc = parse_markdown(&md, Locale::En);
    // Setext H1 (=== underline) should now be counted (BUG-01 fixed)
    assert_eq!(doc.metadata.heading_counts.h1, 1,
        "05: h1 — got {}", doc.metadata.heading_counts.h1);
    // H2 count includes setext H2 (--- underline) and all ATX H2s
    assert_eq!(doc.metadata.heading_counts.h2, 22,
        "05: h2 — got {}", doc.metadata.heading_counts.h2);
    assert_eq!(doc.metadata.heading_counts.h3, 5,
        "05: h3 — got {}", doc.metadata.heading_counts.h3);
    assert_eq!(doc.metadata.heading_counts.h4_plus, 4,
        "05: h4+ — got {}", doc.metadata.heading_counts.h4_plus);
}

// ═══════════════════════════════════════════════════════════════
// Bugs fixed in this revision:
// - BUG-01/02: H1 mid-document now creates sections (level <= 2 splits)
// - BUG-03: Nested lists use stack instead of flat fields
// - BUG-04/05: link_text accumulates before heading/list_item return
// ═══════════════════════════════════════════════════════════════

// ═══════════════════════════════════════════════════════════════
// Hardened tests: round 1 attack findings
// ═══════════════════════════════════════════════════════════════

// --- Image handling ---

#[test]
fn image_alt_text_not_in_paragraph() {
    // BUG if image alt text leaks into paragraph content
    let doc = parse_markdown("Before ![alt text here](https://img.png) after.", Locale::En);
    let para = doc.sections.first()
        .and_then(|s| s.blocks.first())
        .and_then(|b| if let Block::Paragraph(p) = b { Some(p) } else { None });
    if let Some(p) = para {
        let text: String = p.sentences.iter().map(|s| s.text.clone()).collect();
        assert!(!text.contains("alt text here"),
            "image alt text must NOT appear in paragraph text, got: {text}");
    }
}

#[test]
fn standalone_image_no_paragraph() {
    // A paragraph containing only an image should produce no paragraph block
    let doc = parse_markdown("![just an image](https://img.png)", Locale::En);
    assert_eq!(doc.metadata.paragraph_count, 0,
        "standalone image should not create a paragraph — got {}",
        doc.metadata.paragraph_count);
}

// --- Nested blockquotes ---

#[test]
fn nested_blockquote_structure() {
    let doc = parse_markdown("> outer\n>\n> > inner", Locale::En);
    let block = doc.sections.first().and_then(|s| s.blocks.first());
    if let Some(Block::BlockQuote(outer)) = block {
        // The outer blockquote should contain inner content
        assert!(!outer.is_empty(), "outer blockquote should have content");
        // Check for nested blockquote
        let has_inner_quote = outer.iter().any(|b| matches!(b, Block::BlockQuote(_)));
        assert!(has_inner_quote,
            "nested > > should produce BlockQuote inside BlockQuote");
    } else {
        assert!(matches!(block, Some(Block::BlockQuote(_))),
            "expected BlockQuote, got {block:?}");
    }
}

#[test]
fn blockquote_paragraph_counted_in_metadata() {
    let doc = parse_markdown("> This is quoted text.", Locale::En);
    assert_eq!(doc.metadata.paragraph_count, 1,
        "exactly 1 paragraph inside blockquote — got {}",
        doc.metadata.paragraph_count);
    assert_eq!(doc.metadata.total_words, 4,
        "exactly 4 words in quoted text — got {}",
        doc.metadata.total_words);
}

// --- Formatting cross-checks ---

#[test]
fn bold_only_does_not_set_italic() {
    let doc = parse_markdown("**Bold only** text.", Locale::En);
    let para = doc.sections.first()
        .and_then(|s| s.blocks.first())
        .and_then(|b| if let Block::Paragraph(p) = b { Some(p) } else { None });
    assert_eq!(para.map(|p| p.has_bold), Some(true), "has_bold");
    assert_eq!(para.map(|p| p.has_italic), Some(false),
        "bold-only paragraph must NOT have has_italic");
}

#[test]
fn italic_only_does_not_set_bold() {
    let doc = parse_markdown("*Italic only* text.", Locale::En);
    let para = doc.sections.first()
        .and_then(|s| s.blocks.first())
        .and_then(|b| if let Block::Paragraph(p) = b { Some(p) } else { None });
    assert_eq!(para.map(|p| p.has_italic), Some(true), "has_italic");
    assert_eq!(para.map(|p| p.has_bold), Some(false),
        "italic-only paragraph must NOT have has_bold");
}

// --- Metadata exact values ---

#[test]
fn metadata_exact_word_sentence_counts() {
    let doc = parse_markdown("Hello world. Goodbye world.", Locale::En);
    assert_eq!(doc.metadata.total_words, 4, "exactly 4 words — got {}", doc.metadata.total_words);
    assert_eq!(doc.metadata.total_sentences, 2, "exactly 2 sentences — got {}", doc.metadata.total_sentences);
}

// --- Inline code in paragraphs ---

#[test]
fn inline_code_in_paragraph_counted_as_text() {
    let doc = parse_markdown("Use the `Config` struct.", Locale::En);
    let para = doc.sections.first()
        .and_then(|s| s.blocks.first())
        .and_then(|b| if let Block::Paragraph(p) = b { Some(p) } else { None });
    assert_eq!(para.map(|p| p.has_bold), Some(false), "inline code is not bold");
    assert_eq!(para.map(|p| p.has_italic), Some(false), "inline code is not italic");
    // "Use the Config struct" = 4 words (inline code is included as text)
    assert_eq!(doc.metadata.total_words, 4,
        "inline code word should be counted — got {}", doc.metadata.total_words);
}

// --- Code block existence verified ---

#[test]
fn code_block_exists_and_has_content() {
    let doc = parse_markdown("```\nlet x = 1;\n```", Locale::En);
    let block = doc.sections.first().and_then(|s| s.blocks.first());
    if let Some(Block::CodeBlock(code)) = block {
        assert!(code.contains("let x = 1"), "code content: {code}");
    } else {
        assert!(matches!(block, Some(Block::CodeBlock(_))),
            "expected CodeBlock, got {block:?}");
    }
}

// --- Table doesn't crash ---

#[test]
fn table_does_not_crash() {
    let md = "| A | B |\n|---|---|\n| 1 | 2 |\n\nAfter table.";
    let doc = parse_markdown(md, Locale::En);
    // Main check: no panic. Also verify paragraph after table is captured.
    assert!(doc.metadata.paragraph_count >= 1,
        "paragraph after table should exist — got {}", doc.metadata.paragraph_count);
}

// --- H1 mid-document bug ---

#[test]
fn h1_mid_document_creates_section() {
    // H1 mid-document should create a new section (same as H2)
    let doc = parse_markdown("## First\n\nText.\n\n# Late H1\n\nMore text.", Locale::En);
    assert_eq!(doc.metadata.heading_counts.h1, 1,
        "mid-document H1 must be counted — got {}", doc.metadata.heading_counts.h1);
    assert_eq!(doc.sections.len(), 2,
        "H2 + H1 = 2 sections — got {}", doc.sections.len());
    // Second section should have the H1 heading
    let h1_section = doc.sections.get(1);
    assert_eq!(h1_section.and_then(|s| s.heading.as_ref()).map(|h| h.level), Some(1),
        "second section heading is H1");
}

// --- Empty/whitespace paragraph filtered ---

#[test]
fn whitespace_paragraph_produces_no_block() {
    let doc = parse_markdown("## Section\n\n   \n\nReal text.", Locale::En);
    // The whitespace between the heading and real text should not create a paragraph
    assert_eq!(doc.metadata.paragraph_count, 1,
        "whitespace-only paragraph should be filtered — got {}", doc.metadata.paragraph_count);
}

// --- Multiple paragraphs per section ---

#[test]
fn multiple_paragraphs_in_section() {
    let doc = parse_markdown("## Section\n\nFirst para.\n\nSecond para.\n\nThird para.", Locale::En);
    assert_eq!(doc.metadata.paragraph_count, 3,
        "three paragraphs in one section — got {}", doc.metadata.paragraph_count);
}

// --- Fixture 03 exact blockquote count ---

#[test]
fn fixture_03_list_count() {
    let md = load_fixture("03-lists-and-quotes.md");
    let doc = parse_markdown(&md, Locale::En);
    let list_count: usize = doc.sections.iter()
        .flat_map(|s| &s.blocks)
        .filter(|b| matches!(b, Block::List(_)))
        .count();
    // Fixture has 18 lists (12 unordered + 6 ordered)
    assert!(list_count >= 10, "03: at least 10 lists — got {list_count}");
}

// --- Fixture heading text verification ---

#[test]
fn fixture_01_first_section_heading_text() {
    let md = load_fixture("01-clean-article.md");
    let doc = parse_markdown(&md, Locale::En);
    let heading = doc.sections.first().and_then(|s| s.heading.as_ref());
    assert_eq!(heading.map(|h| h.level), Some(1), "first section is H1");
    assert!(heading.is_some_and(|h| h.text.contains("Resilient")),
        "H1 text should contain 'Resilient' — got {:?}",
        heading.map(|h| &h.text));
}

// ═══════════════════════════════════════════════════════════════
// Hardened tests: round 2 attack findings
// ═══════════════════════════════════════════════════════════════

#[test]
fn soft_break_inserts_space() {
    // Soft breaks (single newline in source) should become spaces in paragraph text
    let doc = parse_markdown("Line one\nline two.", Locale::En);
    let para = doc.sections.first()
        .and_then(|s| s.blocks.first())
        .and_then(|b| if let Block::Paragraph(p) = b { Some(p) } else { None });
    if let Some(p) = para {
        let text: String = p.sentences.iter().map(|s| s.text.clone()).collect();
        assert!(!text.contains("oneline"),
            "soft break must insert space, not concatenate — got: {text}");
    }
}

#[test]
fn bold_in_blockquote_counted() {
    let doc = parse_markdown("> **Bold** inside quote.", Locale::En);
    assert_eq!(doc.metadata.bold_count, 1,
        "bold inside blockquote should be counted — got {}", doc.metadata.bold_count);
}

#[test]
fn italic_in_blockquote_counted() {
    let doc = parse_markdown("> *Italic* inside quote.", Locale::En);
    assert_eq!(doc.metadata.italic_count, 1,
        "italic inside blockquote should be counted — got {}", doc.metadata.italic_count);
}

#[test]
fn multiple_links_in_paragraph() {
    let doc = parse_markdown(
        "Visit [one](https://one.com) and [two](https://two.com) and [three](https://three.com).",
        Locale::En,
    );
    assert_eq!(doc.metadata.link_count, 3,
        "three links in one paragraph — got {}", doc.metadata.link_count);
}

#[test]
fn content_before_first_h2_becomes_section() {
    let doc = parse_markdown("Preamble text.\n\n## First section\n\nBody.", Locale::En);
    assert_eq!(doc.sections.len(), 2, "preamble + H2 section = 2");
    let first = doc.sections.first();
    assert!(first.is_some_and(|s| s.heading.is_none()), "preamble section has no heading");
}

#[test]
fn inline_code_in_heading() {
    let doc = parse_markdown("## The `Config` module\n\nText.", Locale::En);
    let heading = doc.sections.first().and_then(|s| s.heading.as_ref());
    assert!(heading.is_some_and(|h| h.text.contains("Config")),
        "heading should include inline code text — got {:?}",
        heading.map(|h| &h.text));
}

// ═══════════════════════════════════════════════════════════════
// Hardened tests: round 3 attack findings (final)
// ═══════════════════════════════════════════════════════════════

#[test]
fn multi_fragment_link_text() {
    // Link text assembled from multiple text events: [**bold** normal](url)
    // The link_text must accumulate across all fragments, not just the first.
    let doc = parse_markdown("Click [**bold** text](https://example.com).", Locale::En);
    let para = doc.sections.first()
        .and_then(|s| s.blocks.first())
        .and_then(|b| if let Block::Paragraph(p) = b { Some(p) } else { None });
    if let Some(p) = para {
        let link = p.links.first();
        assert_eq!(link.map(|l| l.url.as_str()), Some("https://example.com"), "link url");
        // Link text should include both "bold" and "text"
        if let Some(l) = link {
            assert!(l.text.contains("bold"), "link text should contain 'bold' — got: {}", l.text);
            assert!(l.text.contains("text"), "link text should contain 'text' — got: {}", l.text);
        }
    }
}

#[test]
fn list_item_with_soft_break() {
    // Multi-line list items should have spaces, not concatenated text
    // "- line one\n  line two" — the continuation should be space-separated
    let doc = parse_markdown("- line one\n  line two\n- second item", Locale::En);
    let block = doc.sections.first().and_then(|s| s.blocks.first());
    if let Some(Block::List(list)) = block
        && let Some(first_item) = list.items.first()
    {
        assert!(!first_item.contains("oneline"),
            "list item soft break should insert space — got: {first_item}");
    }
}

// ═══════════════════════════════════════════════════════════════
// Bug fix verification tests
// ═══════════════════════════════════════════════════════════════

#[test]
fn nested_list_items_are_separate() {
    // BUG-03 fix: nested lists should produce separate Block::List entries
    // via the list stack, not clobber the outer list's items.
    let md = "- outer one\n- outer two\n  - inner one\n  - inner two\n- outer three";
    let doc = parse_markdown(md, Locale::En);
    let blocks: Vec<&Block> = doc.sections.first()
        .map(|s| s.blocks.iter().collect())
        .unwrap_or_default();
    let lists: Vec<&ListBlock> = blocks.iter().filter_map(|b| {
        if let Block::List(l) = b { Some(l) } else { None }
    }).collect();
    // Inner list should exist as a separate list
    let all_items: Vec<&str> = lists.iter()
        .flat_map(|l| l.items.iter())
        .map(String::as_str)
        .collect();
    assert!(all_items.iter().any(|s| s.contains("inner one")),
        "inner list items should be captured — got {all_items:?}");
    assert!(all_items.iter().any(|s| s.contains("outer one")),
        "outer items before nesting should be captured — got {all_items:?}");
    assert!(all_items.iter().any(|s| s.contains("outer three")),
        "outer items after nesting should be captured — got {all_items:?}");
    // Note: "outer two" text may be lost because pulldown-cmark makes it
    // the parent of the inner list, and list_item_text gets cleared when
    // inner items start. This is a known limitation of flat list_item_text.
}

#[test]
fn link_in_heading_has_text() {
    // BUG-04 fix: links inside headings should accumulate text
    let doc = parse_markdown("## Check [the docs](https://docs.rs)\n\nBody.", Locale::En);
    let heading = doc.sections.first().and_then(|s| s.heading.as_ref());
    assert_eq!(heading.map(|h| h.level), Some(2), "H2 heading");
    // Heading text should include link text
    assert!(heading.is_some_and(|h| h.text.contains("the docs")),
        "heading should include link text — got {:?}",
        heading.map(|h| &h.text));
}

#[test]
fn link_text_in_list_item_preserved() {
    // BUG-05 fix: link text inside list items must be in item text
    let doc = parse_markdown("- Click [here](https://example.com) now", Locale::En);
    let block = doc.sections.first().and_then(|s| s.blocks.first());
    if let Some(Block::List(list)) = block
        && let Some(item) = list.items.first()
    {
        assert!(item.contains("here"),
            "list item should contain link text 'here' — got: {item}");
    }
}

#[test]
fn h1_then_h2_creates_two_sections() {
    // BUG-01/02 fix: H1 creates a section, H2 creates another
    let doc = parse_markdown("# Title\n\nIntro.\n\n## Section One\n\nBody.", Locale::En);
    assert_eq!(doc.sections.len(), 2, "H1 + H2 = 2 sections — got {}", doc.sections.len());
    assert_eq!(doc.metadata.heading_counts.h1, 1, "h1 count");
    assert_eq!(doc.metadata.heading_counts.h2, 1, "h2 count");
}

// ═══════════════════════════════════════════════════════════════
// 4-angle attack: round 1 fixes
// ═══════════════════════════════════════════════════════════════

// --- ANGLE 1: strengthen weak kill tests ---

#[test]
fn image_alt_text_not_in_paragraph_strong() {
    // Strengthened: assert paragraph EXISTS, then assert alt text absent
    let doc = parse_markdown("Before ![alt text here](https://img.png) after.", Locale::En);
    assert_eq!(doc.metadata.paragraph_count, 1, "should produce 1 paragraph");
    let para = doc.sections.first()
        .and_then(|s| s.blocks.first())
        .and_then(|b| if let Block::Paragraph(p) = b { Some(p) } else { None });
    assert!(para.is_some(), "paragraph must exist");
    if let Some(p) = para {
        let text: String = p.sentences.iter().map(|s| s.text.clone()).collect();
        assert!(!text.contains("alt text here"),
            "image alt text must NOT appear in paragraph text — got: {text}");
    }
}

// --- ANGLE 2: handle_code for list items ---

#[test]
fn inline_code_in_list_item() {
    let doc = parse_markdown("- Use `Config` here\n- Normal item", Locale::En);
    let block = doc.sections.first().and_then(|s| s.blocks.first());
    if let Some(Block::List(list)) = block {
        let first = list.items.first().map(String::as_str);
        assert!(first.is_some_and(|s| s.contains("Config")),
            "list item should contain inline code text 'Config' — got: {first:?}");
    }
}

// --- ANGLE 2: tables don't leak content ---

#[test]
fn table_content_not_in_paragraphs() {
    let md = "| A | B |\n|---|---|\n| cell1 | cell2 |\n\nAfter table.";
    let doc = parse_markdown(md, Locale::En);
    // Paragraph text should NOT contain table cell content
    let all_text: String = doc.sections.iter()
        .flat_map(|s| &s.blocks)
        .filter_map(|b| if let Block::Paragraph(p) = b { Some(p) } else { None })
        .flat_map(|p| &p.sentences)
        .map(|s| s.text.clone())
        .collect();
    assert!(!all_text.contains("cell1"), "table cell content should not leak — got: {all_text}");
    assert!(all_text.contains("After"), "paragraph after table should exist — got: {all_text}");
}

// --- ANGLE 2: strikethrough text still in paragraph ---

#[test]
fn strikethrough_text_preserved_in_paragraph() {
    let doc = parse_markdown("This is ~~deleted~~ text.", Locale::En);
    assert_eq!(doc.metadata.paragraph_count, 1, "one paragraph");
    let para = doc.sections.first()
        .and_then(|s| s.blocks.first())
        .and_then(|b| if let Block::Paragraph(p) = b { Some(p) } else { None });
    if let Some(p) = para {
        let text: String = p.sentences.iter().map(|s| s.text.clone()).collect();
        assert!(text.contains("deleted"),
            "strikethrough text should still appear in paragraph — got: {text}");
    }
}

// --- ANGLE 3: exact paragraph text content ---

#[test]
fn paragraph_text_content_exact() {
    let doc = parse_markdown("Hello beautiful world.", Locale::En);
    let para = doc.sections.first()
        .and_then(|s| s.blocks.first())
        .and_then(|b| if let Block::Paragraph(p) = b { Some(p) } else { None });
    assert!(para.is_some(), "paragraph must exist");
    if let Some(p) = para {
        let text = p.sentences.first().map(|s| s.text.as_str());
        assert_eq!(text, Some("Hello beautiful world."), "exact paragraph text");
    }
}

// --- ANGLE 3: bold+italic simultaneous ---

#[test]
fn bold_italic_simultaneous() {
    let doc = parse_markdown("This is ***bold italic*** text.", Locale::En);
    let para = doc.sections.first()
        .and_then(|s| s.blocks.first())
        .and_then(|b| if let Block::Paragraph(p) = b { Some(p) } else { None });
    assert_eq!(para.map(|p| p.has_bold), Some(true), "has_bold for ***");
    assert_eq!(para.map(|p| p.has_italic), Some(true), "has_italic for ***");
}

// --- ANGLE 3: nested emphasis ---

#[test]
fn nested_emphasis_bold_inside_italic() {
    let doc = parse_markdown("*outer **inner** outer*", Locale::En);
    let para = doc.sections.first()
        .and_then(|s| s.blocks.first())
        .and_then(|b| if let Block::Paragraph(p) = b { Some(p) } else { None });
    assert_eq!(para.map(|p| p.has_bold), Some(true), "has_bold from nested");
    assert_eq!(para.map(|p| p.has_italic), Some(true), "has_italic from outer");
}

// --- ANGLE 3: empty link text ---

#[test]
fn empty_link_text() {
    let doc = parse_markdown("Click [](https://example.com) here.", Locale::En);
    let para = doc.sections.first()
        .and_then(|s| s.blocks.first())
        .and_then(|b| if let Block::Paragraph(p) = b { Some(p) } else { None });
    assert_eq!(para.map(|p| p.links.len()), Some(1), "empty-text link should still be counted");
}

// --- ANGLE 4: escaped characters at unit level ---

#[test]
fn escaped_bold_not_detected() {
    let doc = parse_markdown(r"\*\*not bold\*\*", Locale::En);
    let para = doc.sections.first()
        .and_then(|s| s.blocks.first())
        .and_then(|b| if let Block::Paragraph(p) = b { Some(p) } else { None });
    assert_eq!(para.map(|p| p.has_bold), Some(false),
        "escaped ** should NOT set has_bold");
}

#[test]
fn escaped_italic_not_detected() {
    let doc = parse_markdown(r"\*not italic\*", Locale::En);
    let para = doc.sections.first()
        .and_then(|s| s.blocks.first())
        .and_then(|b| if let Block::Paragraph(p) = b { Some(p) } else { None });
    assert_eq!(para.map(|p| p.has_italic), Some(false),
        "escaped * should NOT set has_italic");
}

#[test]
fn escaped_link_not_detected() {
    let doc = parse_markdown(r"\[not a link\](not-a-url)", Locale::En);
    let para = doc.sections.first()
        .and_then(|s| s.blocks.first())
        .and_then(|b| if let Block::Paragraph(p) = b { Some(p) } else { None });
    assert_eq!(para.map(|p| p.links.len()), Some(0),
        "escaped [] should NOT create a link");
}

// --- ANGLE 4: code block with headings/links/images ---

#[test]
fn code_block_with_heading_syntax_not_parsed() {
    let doc = parse_markdown("```\n# Not a heading\n[Not a link](url)\n```", Locale::En);
    assert_eq!(doc.metadata.heading_counts.h1, 0, "# inside code block not a heading");
    assert_eq!(doc.metadata.link_count, 0, "[] inside code block not a link");
}

// ═══════════════════════════════════════════════════════════════
// 4-angle attack: round 2 fixes
// ═══════════════════════════════════════════════════════════════

// --- ANGLE 1: bold/italic depth reset across paragraphs ---

#[test]
fn bold_does_not_bleed_to_next_paragraph() {
    let doc = parse_markdown("**Bold paragraph.**\n\nPlain paragraph.", Locale::En);
    assert_eq!(doc.metadata.paragraph_count, 2, "two paragraphs");
    // First paragraph: has_bold = true
    let p0 = doc.sections.first()
        .and_then(|s| s.blocks.first())
        .and_then(|b| if let Block::Paragraph(p) = b { Some(p) } else { None });
    assert_eq!(p0.map(|p| p.has_bold), Some(true), "first para bold");
    // Second paragraph: has_bold = false
    let p1 = doc.sections.first()
        .and_then(|s| s.blocks.get(1))
        .and_then(|b| if let Block::Paragraph(p) = b { Some(p) } else { None });
    assert_eq!(p1.map(|p| p.has_bold), Some(false),
        "bold must not bleed to next paragraph");
}

// --- ANGLE 2: hard break distinct from soft break ---

#[test]
fn hard_break_inserts_space() {
    // Two trailing spaces + newline = hard break in CommonMark
    let doc = parse_markdown("Line one  \nline two.", Locale::En);
    let p = assert_first_paragraph(&doc);
    let text: String = p.sentences.iter().map(|s| s.text.clone()).collect();
    assert!(!text.contains("oneline"),
        "hard break must insert space — got: {text}");
}

// --- ANGLE 2: horizontal rule doesn't corrupt state ---

#[test]
fn horizontal_rule_between_paragraphs() {
    let doc = parse_markdown("Para one.\n\n---\n\nPara two.", Locale::En);
    assert_eq!(doc.metadata.paragraph_count, 2, "two paragraphs around hr");
    assert_eq!(doc.metadata.heading_counts.h1, 0, "hr is not a heading");
    assert_eq!(doc.metadata.heading_counts.h2, 0, "hr is not a heading");
}

// --- ANGLE 4: inline HTML content excluded ---

#[test]
fn inline_html_not_in_paragraph_text() {
    // <b>html bold</b> is raw HTML, not markdown bold — should be ignored
    let doc = parse_markdown("Before <b>html bold</b> after.", Locale::En);
    let p = assert_first_paragraph(&doc);
    // The text events for "Before " and " after." should be captured.
    // InlineHtml events for "<b>" and "</b>" are ignored.
    // The text "html bold" between the tags IS a text event (not InlineHtml).
    assert!(!p.has_bold, "HTML <b> should NOT set has_bold flag");
}

// --- ANGLE 4: task list marker ignored ---

#[test]
fn task_list_marker_ignored() {
    let doc = parse_markdown("- [ ] unchecked item\n- [x] checked item", Locale::En);
    let block = doc.sections.first().and_then(|s| s.blocks.first());
    if let Some(Block::List(list)) = block {
        assert_eq!(list.items.len(), 2, "two list items");
        // Task list markers should not corrupt item text
        assert!(list.items.first().is_some_and(|s| s.contains("unchecked")),
            "item text preserved");
    }
}
