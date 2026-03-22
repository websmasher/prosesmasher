use super::*;

// ═══════════════════════════════════════════════════════════════
// Locale
// ═══════════════════════════════════════════════════════════════

#[test]
fn locale_default_is_en() {
    assert_eq!(Locale::default(), Locale::En, "default locale must be English");
}

#[test]
fn locale_equality_same() {
    assert_eq!(Locale::En, Locale::En);
    assert_eq!(Locale::Ru, Locale::Ru);
}

#[test]
fn locale_equality_different() {
    assert_ne!(Locale::En, Locale::Ru);
    assert_ne!(Locale::De, Locale::Fr);
    assert_ne!(Locale::Es, Locale::Pt);
    assert_ne!(Locale::Id, Locale::En);
}

#[test]
fn locale_all_variants_are_distinct() {
    let all = [Locale::En, Locale::Ru, Locale::De, Locale::Es, Locale::Pt, Locale::Fr, Locale::Id];
    for (i, a) in all.iter().enumerate() {
        for (j, b) in all.iter().enumerate() {
            if i != j {
                assert_ne!(a, b, "{a:?} and {b:?} must be distinct");
            }
        }
    }
}

#[test]
fn locale_copy_semantics() {
    let a = Locale::En;
    let b = a; // Copy
    let c = a; // Still valid after copy
    assert_eq!(b, c, "Locale must be Copy");
}

// ═══════════════════════════════════════════════════════════════
// DocumentMetadata + HeadingCounts — Default
// ═══════════════════════════════════════════════════════════════

#[test]
fn metadata_default_all_zeroed() {
    let meta = DocumentMetadata::default();
    assert_eq!(meta.total_words, 0, "total_words");
    assert_eq!(meta.total_sentences, 0, "total_sentences");
    assert_eq!(meta.total_syllables, 0, "total_syllables");
    assert_eq!(meta.bold_count, 0, "bold_count");
    assert_eq!(meta.italic_count, 0, "italic_count");
    assert_eq!(meta.paragraph_count, 0, "paragraph_count");
    assert_eq!(meta.link_count, 0, "link_count");
    assert_eq!(meta.heading_counts.h1, 0, "heading_counts.h1");
    assert_eq!(meta.heading_counts.h2, 0, "heading_counts.h2");
    assert_eq!(meta.heading_counts.h3, 0, "heading_counts.h3");
    assert_eq!(meta.heading_counts.h4_plus, 0, "heading_counts.h4_plus");
}

#[test]
fn heading_counts_default_zeroed() {
    let counts = HeadingCounts::default();
    assert_eq!(counts.h1, 0, "h1");
    assert_eq!(counts.h2, 0, "h2");
    assert_eq!(counts.h3, 0, "h3");
    assert_eq!(counts.h4_plus, 0, "h4_plus");
}

// ═══════════════════════════════════════════════════════════════
// CheckConfig, TermLists, Thresholds — Default
// ═══════════════════════════════════════════════════════════════

#[test]
fn check_config_default_locale_is_en() {
    let config = CheckConfig::default();
    assert_eq!(config.locale, Locale::En, "default config locale must be En");
}

#[test]
fn term_lists_default_all_empty() {
    let terms = TermLists::default();
    assert!(terms.banned_words.is_empty(), "banned_words");
    assert!(terms.banned_phrases.is_empty(), "banned_phrases");
    assert!(terms.gendered_terms.is_empty(), "gendered_terms");
    assert!(terms.forbidden_terms.is_empty(), "forbidden_terms");
    assert!(terms.race_terms.is_empty(), "race_terms");
    assert!(terms.hedge_words.is_empty(), "hedge_words");
    assert!(terms.simplicity_pairs.is_empty(), "simplicity_pairs");
    assert!(terms.negation_signals.is_empty(), "negation_signals");
    assert!(terms.reframe_signals.is_empty(), "reframe_signals");
    assert!(terms.llm_openers.is_empty(), "llm_openers");
    assert!(terms.affirmation_closers.is_empty(), "affirmation_closers");
    assert!(terms.summative_patterns.is_empty(), "summative_patterns");
    assert!(terms.false_question_patterns.is_empty(), "false_question_patterns");
    assert!(terms.humble_bragger_phrases.is_empty(), "humble_bragger_phrases");
    assert!(terms.jargon_faker_phrases.is_empty(), "jargon_faker_phrases");
    assert!(terms.stop_words.is_empty(), "stop_words");
}

#[test]
fn thresholds_default_all_none() {
    let t = Thresholds::default();
    assert!(t.word_count.is_none(), "word_count");
    assert!(t.h2_count.is_none(), "h2_count");
    assert!(t.h3_min.is_none(), "h3_min");
    assert!(t.bold_min.is_none(), "bold_min");
    assert!(t.max_paragraph_sentences.is_none(), "max_paragraph_sentences");
    assert!(t.max_exclamations_per_paragraph.is_none(), "max_exclamations_per_paragraph");
    assert!(t.max_hedges_per_sentence.is_none(), "max_hedges_per_sentence");
    assert!(t.flesch_kincaid_min.is_none(), "flesch_kincaid_min");
    assert!(t.gunning_fog_max.is_none(), "gunning_fog_max");
    assert!(t.avg_sentence_length_max.is_none(), "avg_sentence_length_max");
    assert!(t.word_repetition_max.is_none(), "word_repetition_max");
    assert!(t.coleman_liau_max.is_none(), "coleman_liau_max");
}

// ═══════════════════════════════════════════════════════════════
// Document tree construction — smoke test (split for clippy line limit)
// ═══════════════════════════════════════════════════════════════

fn build_test_document() -> Document {
    Document {
        locale: Locale::En,
        sections: vec![
            Section {
                heading: Some(Heading {
                    level: 2,
                    text: "Introduction".to_owned(),
                }),
                blocks: vec![
                    Block::Paragraph(Paragraph {
                        sentences: vec![Sentence {
                            text: "Hello world.".to_owned(),
                            words: vec![
                                Word { text: "Hello".to_owned(), syllable_count: 2 },
                                Word { text: "world".to_owned(), syllable_count: 1 },
                            ],
                        }],
                        has_bold: true,
                        has_italic: false,
                        links: vec![Link {
                            text: "example".to_owned(),
                            url: "https://example.com".to_owned(),
                        }],
                    }),
                    Block::List(ListBlock {
                        ordered: false,
                        items: vec!["item one".to_owned(), "item two".to_owned()],
                    }),
                    Block::CodeBlock("fn main() {}".to_owned()),
                    Block::BlockQuote(vec![
                        Block::Paragraph(Paragraph {
                            sentences: vec![],
                            has_bold: false,
                            has_italic: true,
                            links: vec![],
                        }),
                    ]),
                ],
            },
            Section {
                heading: None,
                blocks: vec![],
            },
        ],
        metadata: DocumentMetadata {
            total_words: 2,
            total_sentences: 1,
            total_syllables: 3,
            heading_counts: HeadingCounts { h1: 0, h2: 1, h3: 0, h4_plus: 0 },
            bold_count: 1,
            italic_count: 1,
            paragraph_count: 2,
            link_count: 1,
        },
    }
}

#[test]
fn document_construction_top_level() {
    let doc = build_test_document();
    assert_eq!(doc.locale, Locale::En, "locale");
    assert_eq!(doc.sections.len(), 2, "section count");
    assert_eq!(doc.metadata.total_words, 2, "total_words");
    assert_eq!(doc.metadata.heading_counts.h2, 1, "h2 count");
}

#[test]
fn document_construction_paragraph_block() {
    let doc = build_test_document();
    let Some(s0) = doc.sections.first() else {
        assert!(!doc.sections.is_empty(), "section 0 must exist");
        return;
    };
    let Some(heading) = s0.heading.as_ref() else {
        assert!(s0.heading.is_some(), "section 0 must have heading");
        return;
    };
    assert_eq!(heading.level, 2, "heading level");
    assert_eq!(heading.text, "Introduction", "heading text");
    assert_eq!(s0.blocks.len(), 4, "block count");

    assert!(matches!(s0.blocks.first(), Some(Block::Paragraph(_))), "blocks[0] must be Paragraph");
    if let Some(Block::Paragraph(p)) = s0.blocks.first() {
        assert!(p.has_bold, "has_bold");
        assert!(!p.has_italic, "has_italic");
        assert_eq!(p.sentences.len(), 1, "sentence count");
        assert!(!p.sentences.is_empty(), "sentence 0 must exist");
        if let Some(sentence) = p.sentences.first() {
            assert_eq!(sentence.word_count(), 2, "word_count");
        }
        assert!(!p.links.is_empty(), "link 0 must exist");
        if let Some(link) = p.links.first() {
            assert_eq!(link.url, "https://example.com", "link url");
        }
    }
}

#[test]
fn document_construction_list_block() {
    let doc = build_test_document();
    let Some(s0) = doc.sections.first() else {
        assert!(!doc.sections.is_empty(), "section 0 must exist");
        return;
    };
    assert!(matches!(s0.blocks.get(1), Some(Block::List(_))), "blocks[1] must be List");
    if let Some(Block::List(list)) = s0.blocks.get(1) {
        assert!(!list.ordered, "unordered list");
        assert_eq!(list.items.len(), 2, "list items");
    }
}

#[test]
fn document_construction_section_without_heading() {
    let doc = build_test_document();
    let Some(s1) = doc.sections.get(1) else {
        assert!(doc.sections.get(1).is_some(), "section 1 must exist");
        return;
    };
    assert!(s1.heading.is_none(), "section 1 has no heading");
}

// ═══════════════════════════════════════════════════════════════
// Word — PartialEq / Eq
// ═══════════════════════════════════════════════════════════════

#[test]
fn word_equality() {
    let a = Word { text: "hello".to_owned(), syllable_count: 2 };
    let b = Word { text: "hello".to_owned(), syllable_count: 2 };
    let c = Word { text: "world".to_owned(), syllable_count: 1 };
    let d = Word { text: "hello".to_owned(), syllable_count: 3 };
    assert_eq!(a, b, "same word = equal");
    assert_ne!(a, c, "different text = not equal");
    assert_ne!(a, d, "same text, different syllable count = not equal");
}

#[test]
fn word_clone() {
    let a = Word { text: "test".to_owned(), syllable_count: 1 };
    #[allow(clippy::redundant_clone)] // intentionally testing Clone trait
    let b = a.clone();
    assert_eq!(a, b, "Word must be Clone");
}

// ═══════════════════════════════════════════════════════════════
// SimplePair + Range — construction
// ═══════════════════════════════════════════════════════════════

#[test]
fn simple_pair_construction() {
    let pair = SimplePair {
        complex: "utilize".to_owned(),
        simple: "use".to_owned(),
    };
    assert_eq!(pair.complex, "utilize", "complex field");
    assert_eq!(pair.simple, "use", "simple field");
}

#[test]
fn range_construction_and_copy() {
    let Some(r) = Range::new(650, 1000) else {
        assert!(Range::new(650, 1000).is_some(), "valid range must construct");
        return;
    };
    let r2 = r; // Copy
    let r3 = r; // Still valid
    assert_eq!(r2.min(), 650, "min after copy");
    assert_eq!(r3.max(), 1000, "max after copy");
}

// ═══════════════════════════════════════════════════════════════
// Error Display — all variants
// ═══════════════════════════════════════════════════════════════

#[test]
fn read_error_not_found_display() {
    let err = ReadError::NotFound("/path/to/file".to_owned());
    let msg = err.to_string();
    assert!(msg.contains("/path/to/file"), "should contain path");
    assert!(msg.contains("not found"), "should contain label");
}

#[test]
fn read_error_permission_denied_display() {
    let err = ReadError::PermissionDenied("/secret".to_owned());
    let msg = err.to_string();
    assert!(msg.contains("/secret"), "should contain path");
    assert!(msg.contains("permission denied"), "should contain label");
}

#[test]
fn read_error_io_display() {
    let err = ReadError::Io("disk failure".to_owned());
    let msg = err.to_string();
    assert!(msg.contains("disk failure"), "should contain message");
    assert!(msg.contains("io error"), "should contain label");
}

#[test]
fn parse_error_invalid_markdown_display() {
    let err = ParseError::InvalidMarkdown("bad input".to_owned());
    let msg = err.to_string();
    assert!(msg.contains("bad input"), "should contain message");
    assert!(msg.contains("invalid markdown"), "should contain label");
}

#[test]
fn parse_error_segmentation_failed_display() {
    let err = ParseError::SegmentationFailed("no sentences".to_owned());
    let msg = err.to_string();
    assert!(msg.contains("no sentences"), "should contain message");
    assert!(msg.contains("segmentation failed"), "should contain label");
}

#[test]
fn config_error_not_found_display() {
    let err = ConfigError::NotFound("config.json".to_owned());
    let msg = err.to_string();
    assert!(msg.contains("config.json"), "should contain filename");
    assert!(msg.contains("not found"), "should contain label");
}

#[test]
fn config_error_invalid_json_display() {
    let err = ConfigError::InvalidJson("unexpected token".to_owned());
    let msg = err.to_string();
    assert!(msg.contains("unexpected token"), "should contain message");
    assert!(msg.contains("invalid json"), "should contain label");
}

#[test]
fn config_error_validation_failed_display() {
    let err = ConfigError::ValidationFailed("missing field".to_owned());
    let msg = err.to_string();
    assert!(msg.contains("missing field"), "should contain message");
    assert!(msg.contains("validation failed"), "should contain label");
}

// ═══════════════════════════════════════════════════════════════
// Error types as trait objects — CLI compatibility
// ═══════════════════════════════════════════════════════════════

#[test]
fn read_error_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<ReadError>();
}

#[test]
fn parse_error_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<ParseError>();
}

#[test]
fn config_error_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<ConfigError>();
}

#[test]
fn read_error_as_boxed_dyn() {
    let boxed: Box<dyn std::error::Error> = Box::new(ReadError::Io("fail".to_owned()));
    assert!(boxed.to_string().contains("fail"), "boxed ReadError Display works");
}

#[test]
fn parse_error_as_boxed_dyn() {
    let boxed: Box<dyn std::error::Error> = Box::new(ParseError::InvalidMarkdown("bad".to_owned()));
    assert!(boxed.to_string().contains("bad"), "boxed ParseError Display works");
}

#[test]
fn config_error_as_boxed_dyn() {
    let boxed: Box<dyn std::error::Error> = Box::new(ConfigError::NotFound("missing".to_owned()));
    assert!(boxed.to_string().contains("missing"), "boxed ConfigError Display works");
}

#[test]
fn read_error_source_is_none() {
    use std::error::Error;
    let read_err = ReadError::NotFound("test".to_owned());
    assert!(read_err.source().is_none(), "ReadError::source() should be None");
}

#[test]
fn parse_error_source_is_none() {
    use std::error::Error;
    let parse_err = ParseError::InvalidMarkdown("test".to_owned());
    assert!(parse_err.source().is_none(), "ParseError::source() should be None");
}

#[test]
fn config_error_source_is_none() {
    use std::error::Error;
    let config_err = ConfigError::NotFound("test".to_owned());
    assert!(config_err.source().is_none(), "ConfigError::source() should be None");
}

// ═══════════════════════════════════════════════════════════════
// Clone exercises
// ═══════════════════════════════════════════════════════════════

#[test]
fn document_metadata_clone() {
    let meta = DocumentMetadata {
        total_words: 100,
        total_sentences: 10,
        total_syllables: 150,
        heading_counts: HeadingCounts { h1: 1, h2: 3, h3: 5, h4_plus: 0 },
        bold_count: 2,
        italic_count: 1,
        paragraph_count: 8,
        link_count: 4,
    };
    #[allow(clippy::redundant_clone)] // intentionally testing Clone trait
    let cloned = meta.clone();
    assert_eq!(cloned.total_words, 100, "clone preserves total_words");
    assert_eq!(cloned.heading_counts.h2, 3, "clone preserves nested h2");
}

#[test]
fn check_config_clone() {
    let config = CheckConfig {
        locale: Locale::De,
        terms: TermLists {
            banned_words: vec!["actually".to_owned()],
            ..TermLists::default()
        },
        thresholds: Thresholds {
            word_count: Range::new(500, 1000),
            ..Thresholds::default()
        },
        ..CheckConfig::default()
    };
    #[allow(clippy::redundant_clone)] // intentionally testing Clone trait
    let cloned = config.clone();
    assert_eq!(cloned.locale, Locale::De, "clone preserves locale");
    assert_eq!(cloned.terms.banned_words.len(), 1, "clone preserves terms");
    assert!(cloned.thresholds.word_count.is_some(), "clone preserves thresholds");
}

#[test]
fn read_error_clone() {
    let read_err = ReadError::NotFound("test".to_owned());
    #[allow(clippy::redundant_clone)] // intentionally testing Clone trait
    let cloned = read_err.clone();
    assert_eq!(read_err.to_string(), cloned.to_string(), "ReadError clone preserves Display");
}

#[test]
fn parse_error_clone() {
    let parse_err = ParseError::InvalidMarkdown("test".to_owned());
    #[allow(clippy::redundant_clone)] // intentionally testing Clone trait
    let cloned = parse_err.clone();
    assert_eq!(parse_err.to_string(), cloned.to_string(), "ParseError clone preserves Display");
}

#[test]
fn config_error_clone() {
    let config_err = ConfigError::NotFound("test".to_owned());
    #[allow(clippy::redundant_clone)] // intentionally testing Clone trait
    let cloned = config_err.clone();
    assert_eq!(config_err.to_string(), cloned.to_string(), "ConfigError clone preserves Display");
}

#[test]
fn document_clone() {
    let doc = build_test_document();
    #[allow(clippy::redundant_clone)] // intentionally testing Clone trait
    let cloned = doc.clone();
    assert_eq!(cloned.locale, Locale::En, "clone preserves locale");
    assert_eq!(cloned.sections.len(), 2, "clone preserves sections");
    assert_eq!(cloned.metadata.total_words, 2, "clone preserves metadata");
}

#[test]
fn simple_pair_clone() {
    let pair = SimplePair {
        complex: "utilize".to_owned(),
        simple: "use".to_owned(),
    };
    #[allow(clippy::redundant_clone)] // intentionally testing Clone trait
    let cloned = pair.clone();
    assert_eq!(cloned.complex, "utilize", "clone preserves complex");
    assert_eq!(cloned.simple, "use", "clone preserves simple");
}

// ═══════════════════════════════════════════════════════════════
// Debug derive exercises — verify Debug output is non-empty
// ═══════════════════════════════════════════════════════════════

#[test]
fn locale_debug() {
    let dbg = format!("{:?}", Locale::En);
    assert!(dbg.contains("En"), "Locale::En debug should contain 'En'");
}

#[test]
fn document_debug_non_empty() {
    let doc = build_test_document();
    let dbg = format!("{doc:?}");
    assert!(!dbg.is_empty(), "Document debug must be non-empty");
    assert!(dbg.contains("Document"), "should contain type name");
}

#[test]
fn error_debug_contains_variant() {
    let read_dbg = format!("{:?}", ReadError::NotFound("x".to_owned()));
    assert!(read_dbg.contains("NotFound"), "ReadError debug should contain variant name");

    let parse_dbg = format!("{:?}", ParseError::InvalidMarkdown("x".to_owned()));
    assert!(parse_dbg.contains("InvalidMarkdown"), "ParseError debug should contain variant name");

    let config_dbg = format!("{:?}", ConfigError::ValidationFailed("x".to_owned()));
    assert!(config_dbg.contains("ValidationFailed"), "ConfigError debug should contain variant name");
}

#[test]
fn block_variants_debug() {
    let p = Block::Paragraph(Paragraph {
        sentences: vec![], has_bold: false, has_italic: false, links: vec![],
    });
    assert!(format!("{p:?}").contains("Paragraph"), "Block::Paragraph debug");

    let l = Block::List(ListBlock { ordered: true, items: vec![] });
    assert!(format!("{l:?}").contains("List"), "Block::List debug");

    let c = Block::CodeBlock("code".to_owned());
    assert!(format!("{c:?}").contains("CodeBlock"), "Block::CodeBlock debug");

    let q = Block::BlockQuote(vec![]);
    assert!(format!("{q:?}").contains("BlockQuote"), "Block::BlockQuote debug");
}

// ═══════════════════════════════════════════════════════════════
// Document construction — deeper value assertions
// ═══════════════════════════════════════════════════════════════

#[test]
fn document_construction_sentence_content() {
    let doc = build_test_document();
    let Some(s0) = doc.sections.first() else { return; };
    if let Some(Block::Paragraph(p)) = s0.blocks.first()
        && let Some(sentence) = p.sentences.first()
    {
        assert_eq!(sentence.text, "Hello world.", "sentence text");
        assert_eq!(sentence.words.len(), 2, "sentence words count");
        if let Some(w0) = sentence.words.first() {
            assert_eq!(w0.text, "Hello", "first word text");
            assert_eq!(w0.syllable_count, 2, "first word syllables");
        }
        if let Some(w1) = sentence.words.get(1) {
            assert_eq!(w1.text, "world", "second word text");
            assert_eq!(w1.syllable_count, 1, "second word syllables");
        }
    }
}

#[test]
fn document_construction_link_text() {
    let doc = build_test_document();
    let Some(s0) = doc.sections.first() else { return; };
    if let Some(Block::Paragraph(p)) = s0.blocks.first()
        && let Some(link) = p.links.first()
    {
        assert_eq!(link.text, "example", "link text field");
        assert_eq!(link.url, "https://example.com", "link url field");
    }
}

#[test]
fn document_construction_list_items_content() {
    let doc = build_test_document();
    let Some(s0) = doc.sections.first() else { return; };
    if let Some(Block::List(list)) = s0.blocks.get(1) {
        if let Some(item0) = list.items.first() {
            assert_eq!(item0, "item one", "first list item");
        }
        if let Some(item1) = list.items.get(1) {
            assert_eq!(item1, "item two", "second list item");
        }
    }
}

#[test]
fn document_construction_code_block_content() {
    let doc = build_test_document();
    let Some(s0) = doc.sections.first() else { return; };
    assert!(matches!(s0.blocks.get(2), Some(Block::CodeBlock(_))), "blocks[2] must be CodeBlock");
    if let Some(Block::CodeBlock(code)) = s0.blocks.get(2) {
        assert_eq!(code, "fn main() {}", "code block content");
    }
}

#[test]
fn document_construction_blockquote_content() {
    let doc = build_test_document();
    let Some(s0) = doc.sections.first() else { return; };
    assert!(matches!(s0.blocks.get(3), Some(Block::BlockQuote(_))), "blocks[3] must be BlockQuote");
    if let Some(Block::BlockQuote(inner)) = s0.blocks.get(3) {
        assert_eq!(inner.len(), 1, "blockquote has 1 inner block");
        if let Some(Block::Paragraph(p)) = inner.first() {
            assert!(p.has_italic, "inner paragraph has_italic");
            assert!(!p.has_bold, "inner paragraph not has_bold");
        }
    }
}

// ═══════════════════════════════════════════════════════════════
// TermLists with SimplePair populated
// ═══════════════════════════════════════════════════════════════

#[test]
fn term_lists_with_simplicity_pairs() {
    let terms = TermLists {
        simplicity_pairs: vec![
            SimplePair { complex: "utilize".to_owned(), simple: "use".to_owned() },
            SimplePair { complex: "implement".to_owned(), simple: "do".to_owned() },
        ],
        ..TermLists::default()
    };
    assert_eq!(terms.simplicity_pairs.len(), 2, "should have 2 pairs");
    if let Some(pair) = terms.simplicity_pairs.first() {
        assert_eq!(pair.complex, "utilize", "first pair complex");
        assert_eq!(pair.simple, "use", "first pair simple");
    }
}

// ═══════════════════════════════════════════════════════════════
// Thresholds with f64 values populated
// ═══════════════════════════════════════════════════════════════

#[test]
fn thresholds_with_float_values() {
    let t = Thresholds {
        flesch_kincaid_min: Some(50.0),
        gunning_fog_max: Some(14.0),
        coleman_liau_max: Some(12.5),
        ..Thresholds::default()
    };
    assert_eq!(t.flesch_kincaid_min, Some(50.0), "flesch_kincaid_min");
    assert_eq!(t.gunning_fog_max, Some(14.0), "gunning_fog_max");
    assert_eq!(t.coleman_liau_max, Some(12.5), "coleman_liau_max");
    // Other fields remain None
    assert!(t.word_count.is_none(), "word_count still None");
}

// ═══════════════════════════════════════════════════════════════
// Range — full field coverage
// ═══════════════════════════════════════════════════════════════

#[test]
fn range_both_fields_survive_copy() {
    let Some(r) = Range::new(10, 20) else {
        assert!(Range::new(10, 20).is_some(), "valid range must construct");
        return;
    };
    let copied = r;
    assert_eq!(copied.min(), 10, "min after copy");
    assert_eq!(copied.max(), 20, "max after copy");
}

#[test]
fn range_rejects_min_greater_than_max() {
    assert!(Range::new(100, 50).is_none(), "min > max must return None");
}

#[test]
fn range_allows_min_equals_max() {
    assert!(Range::new(42, 42).is_some(), "min == max is valid");
}

#[test]
fn range_allows_zero() {
    let Some(r) = Range::new(0, 0) else { return; };
    assert_eq!(r.min(), 0, "zero min");
    assert_eq!(r.max(), 0, "zero max");
}

// ═══════════════════════════════════════════════════════════════
// Sentence.word_count() — derived from words.len()
// ═══════════════════════════════════════════════════════════════

#[test]
fn sentence_word_count_matches_words_len() {
    let sentence = Sentence {
        text: "Hello world.".to_owned(),
        words: vec![
            Word { text: "Hello".to_owned(), syllable_count: 2 },
            Word { text: "world".to_owned(), syllable_count: 1 },
        ],
    };
    assert_eq!(sentence.word_count(), 2, "word_count() == words.len()");
}

#[test]
fn sentence_empty_words_count_zero() {
    let sentence = Sentence {
        text: String::new(),
        words: vec![],
    };
    assert_eq!(sentence.word_count(), 0, "empty words → 0");
}
