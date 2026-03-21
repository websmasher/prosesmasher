use super::*;
use prosesmasher_domain_types::Locale;
use std::path::Path;

fn fixture_path(name: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

fn write_temp(name: &str, content: &str) -> std::path::PathBuf {
    let path = std::env::temp_dir().join(format!("prosesmasher-test-{name}"));
    #[allow(clippy::disallowed_methods)] // test setup
    std::fs::write(&path, content).unwrap_or_default();
    path
}

fn cleanup(path: &Path) {
    #[allow(clippy::disallowed_methods)] // test cleanup
    let _ = std::fs::remove_file(path);
}

// ═══════════════════════════════════════════════════════════════
// Every single field — exhaustive round-trip verification
// ═══════════════════════════════════════════════════════════════

#[test]
fn all_term_fields_populated() {
    let config = FsConfigLoader.load_config(&fixture_path("all-fields.json"))
        .unwrap_or_default();

    assert_eq!(config.locale, Locale::De, "locale");

    // Every term list field — exact counts
    assert_eq!(config.terms.banned_words.len(), 3, "banned_words");
    assert_eq!(config.terms.banned_phrases.len(), 2, "banned_phrases");
    assert_eq!(config.terms.gendered_terms.len(), 2, "gendered_terms");
    assert_eq!(config.terms.forbidden_terms.len(), 2, "forbidden_terms");
    assert_eq!(config.terms.race_terms.len(), 1, "race_terms");
    assert_eq!(config.terms.hedge_words.len(), 3, "hedge_words");
    assert_eq!(config.terms.simplicity_pairs.len(), 3, "simplicity_pairs");
    assert_eq!(config.terms.negation_signals.len(), 3, "negation_signals");
    assert_eq!(config.terms.reframe_signals.len(), 3, "reframe_signals");
    assert_eq!(config.terms.llm_openers.len(), 2, "llm_openers");
    assert_eq!(config.terms.affirmation_closers.len(), 2, "affirmation_closers");
    assert_eq!(config.terms.summative_patterns.len(), 2, "summative_patterns");
    assert_eq!(config.terms.false_question_patterns.len(), 2, "false_question_patterns");
    assert_eq!(config.terms.humble_bragger_phrases.len(), 2, "humble_bragger_phrases");
    assert_eq!(config.terms.jargon_faker_phrases.len(), 2, "jargon_faker_phrases");
    assert_eq!(config.terms.stop_words.len(), 8, "stop_words");
}

#[test]
fn all_threshold_fields_populated() {
    let config = FsConfigLoader.load_config(&fixture_path("all-fields.json"))
        .unwrap_or_default();

    // Every threshold field — exact values
    let t = &config.thresholds;
    assert!(t.word_count.is_some(), "word_count");
    if let Some(wc) = t.word_count {
        assert_eq!(wc.min(), 650, "word_count.min");
        assert_eq!(wc.max(), 1000, "word_count.max");
    }
    assert!(t.h2_count.is_some(), "h2_count");
    if let Some(h2) = t.h2_count {
        assert_eq!(h2.min(), 2, "h2_count.min");
        assert_eq!(h2.max(), 6, "h2_count.max");
    }
    assert_eq!(t.h3_min, Some(1), "h3_min");
    assert_eq!(t.bold_min, Some(3), "bold_min");
    assert_eq!(t.max_paragraph_sentences, Some(4), "max_paragraph_sentences");
    assert_eq!(t.max_exclamations_per_paragraph, Some(1), "max_exclamations_per_paragraph");
    assert_eq!(t.max_hedges_per_sentence, Some(2), "max_hedges_per_sentence");
    assert_eq!(t.flesch_kincaid_min, Some(50.0), "flesch_kincaid_min");
    assert_eq!(t.gunning_fog_max, Some(14.0), "gunning_fog_max");
    assert_eq!(t.avg_sentence_length_max, Some(25), "avg_sentence_length_max");
    assert_eq!(t.word_repetition_max, Some(5), "word_repetition_max");
    assert_eq!(t.coleman_liau_max, Some(12.5), "coleman_liau_max");
}

#[test]
fn all_fields_absent_defaults_to_empty_and_none() {
    let config = FsConfigLoader.load_config(&fixture_path("minimal-config.json"))
        .unwrap_or_default();

    assert_eq!(config.locale, Locale::Ru, "locale");

    // All 16 term lists empty
    assert!(config.terms.banned_words.is_empty(), "banned_words");
    assert!(config.terms.banned_phrases.is_empty(), "banned_phrases");
    assert!(config.terms.gendered_terms.is_empty(), "gendered_terms");
    assert!(config.terms.forbidden_terms.is_empty(), "forbidden_terms");
    assert!(config.terms.race_terms.is_empty(), "race_terms");
    assert!(config.terms.hedge_words.is_empty(), "hedge_words");
    assert!(config.terms.simplicity_pairs.is_empty(), "simplicity_pairs");
    assert!(config.terms.negation_signals.is_empty(), "negation_signals");
    assert!(config.terms.reframe_signals.is_empty(), "reframe_signals");
    assert!(config.terms.llm_openers.is_empty(), "llm_openers");
    assert!(config.terms.affirmation_closers.is_empty(), "affirmation_closers");
    assert!(config.terms.summative_patterns.is_empty(), "summative_patterns");
    assert!(config.terms.false_question_patterns.is_empty(), "false_question_patterns");
    assert!(config.terms.humble_bragger_phrases.is_empty(), "humble_bragger_phrases");
    assert!(config.terms.jargon_faker_phrases.is_empty(), "jargon_faker_phrases");
    assert!(config.terms.stop_words.is_empty(), "stop_words");

    // All 12 thresholds None
    assert!(config.thresholds.word_count.is_none(), "word_count");
    assert!(config.thresholds.h2_count.is_none(), "h2_count");
    assert!(config.thresholds.h3_min.is_none(), "h3_min");
    assert!(config.thresholds.bold_min.is_none(), "bold_min");
    assert!(config.thresholds.max_paragraph_sentences.is_none(), "max_paragraph_sentences");
    assert!(config.thresholds.max_exclamations_per_paragraph.is_none(), "max_exclamations_per_paragraph");
    assert!(config.thresholds.max_hedges_per_sentence.is_none(), "max_hedges_per_sentence");
    assert!(config.thresholds.flesch_kincaid_min.is_none(), "flesch_kincaid_min");
    assert!(config.thresholds.gunning_fog_max.is_none(), "gunning_fog_max");
    assert!(config.thresholds.avg_sentence_length_max.is_none(), "avg_sentence_length_max");
    assert!(config.thresholds.word_repetition_max.is_none(), "word_repetition_max");
    assert!(config.thresholds.coleman_liau_max.is_none(), "coleman_liau_max");
}

// ═══════════════════════════════════════════════════════════════
// SimplePair conversion — order matters
// ═══════════════════════════════════════════════════════════════

#[test]
fn simplicity_pair_order_complex_then_simple() {
    let config = FsConfigLoader.load_config(&fixture_path("all-fields.json"))
        .unwrap_or_default();

    // [["utilize", "use"]] → SimplePair { complex: "utilize", simple: "use" }
    if let Some(pair) = config.terms.simplicity_pairs.first() {
        assert_eq!(pair.complex, "utilize", "first element is complex");
        assert_eq!(pair.simple, "use", "second element is simple");
    }
    // [["facilitate", "help"]] — third pair
    if let Some(pair) = config.terms.simplicity_pairs.get(2) {
        assert_eq!(pair.complex, "facilitate", "complex");
        assert_eq!(pair.simple, "help", "simple");
    }
}

// ═══════════════════════════════════════════════════════════════
// Term content verification — not just counts
// ═══════════════════════════════════════════════════════════════

#[test]
fn term_content_values_correct() {
    let config = FsConfigLoader.load_config(&fixture_path("all-fields.json"))
        .unwrap_or_default();

    assert_eq!(config.terms.banned_words.first().map(String::as_str), Some("actually"), "first banned word");
    assert_eq!(config.terms.banned_words.get(2).map(String::as_str), Some("literally"), "third banned word");
    assert_eq!(config.terms.banned_phrases.first().map(String::as_str), Some("let's dive in"), "first banned phrase");
    assert_eq!(config.terms.gendered_terms.first().map(String::as_str), Some("mama"), "first gendered term");
    assert_eq!(config.terms.negation_signals.get(1).map(String::as_str), Some("isn't"), "second negation signal");
    assert_eq!(config.terms.llm_openers.first().map(String::as_str), Some("the interesting part is"), "first llm opener");
}

// ═══════════════════════════════════════════════════════════════
// Locale handling
// ═══════════════════════════════════════════════════════════════

#[test]
fn all_seven_locales() {
    for (code, expected) in [
        ("en", Locale::En), ("ru", Locale::Ru), ("de", Locale::De),
        ("es", Locale::Es), ("pt", Locale::Pt), ("fr", Locale::Fr),
        ("id", Locale::Id),
    ] {
        let json = format!(r#"{{"locale":"{code}","terms":{{}},"thresholds":{{}}}}"#);
        let path = write_temp(&format!("locale-{code}.json"), &json);
        let result = FsConfigLoader.load_config(&path);
        assert!(result.is_ok(), "locale '{code}' should parse — got {result:?}");
        assert_eq!(result.unwrap_or_default().locale, expected, "locale '{code}'");
        cleanup(&path);
    }
}

#[test]
fn locale_case_insensitive() {
    for code in ["EN", "En", "eN"] {
        let json = format!(r#"{{"locale":"{code}","terms":{{}},"thresholds":{{}}}}"#);
        let path = write_temp(&format!("locale-case-{code}.json"), &json);
        let result = FsConfigLoader.load_config(&path);
        assert!(result.is_ok(), "locale '{code}' (case variant) should parse — got {result:?}");
        assert_eq!(result.unwrap_or_default().locale, Locale::En, "'{code}' → En");
        cleanup(&path);
    }
}

// ═══════════════════════════════════════════════════════════════
// Range edge cases
// ═══════════════════════════════════════════════════════════════

#[test]
fn range_min_equals_max_is_valid() {
    let config = FsConfigLoader.load_config(&fixture_path("range-edge-min-eq-max.json"))
        .unwrap_or_default();
    if let Some(wc) = config.thresholds.word_count {
        assert_eq!(wc.min(), 500, "min");
        assert_eq!(wc.max(), 500, "max");
    }
}

#[test]
fn range_zero_zero_is_valid() {
    let config = FsConfigLoader.load_config(&fixture_path("range-edge-zero.json"))
        .unwrap_or_default();
    if let Some(wc) = config.thresholds.word_count {
        assert_eq!(wc.min(), 0, "min");
        assert_eq!(wc.max(), 0, "max");
    }
}

#[test]
fn range_min_gt_max_is_error() {
    let result = FsConfigLoader.load_config(&fixture_path("invalid-range.json"));
    assert!(matches!(result, Err(ConfigError::ValidationFailed(_))),
        "min > max → ValidationFailed — got {result:?}");
    if let Err(ConfigError::ValidationFailed(msg)) = &result {
        assert!(msg.contains("min"), "error mentions min — got: {msg}");
        assert!(msg.contains("max"), "error mentions max — got: {msg}");
    }
}

// ═══════════════════════════════════════════════════════════════
// Unicode in term values
// ═══════════════════════════════════════════════════════════════

#[test]
fn unicode_cyrillic_terms() {
    let config = FsConfigLoader.load_config(&fixture_path("unicode-terms.json"))
        .unwrap_or_default();

    assert_eq!(config.locale, Locale::Ru, "locale");
    assert_eq!(config.terms.banned_words.first().map(String::as_str), Some("кстати"), "Cyrillic banned word");
    assert_eq!(config.terms.banned_phrases.first().map(String::as_str), Some("давайте погрузимся"), "Cyrillic banned phrase");
    if let Some(pair) = config.terms.simplicity_pairs.first() {
        assert_eq!(pair.complex, "утилизировать", "Cyrillic pair complex");
        assert_eq!(pair.simple, "использовать", "Cyrillic pair simple");
    }
}

// ═══════════════════════════════════════════════════════════════
// Extra/unknown fields — silently ignored
// ═══════════════════════════════════════════════════════════════

#[test]
fn extra_unknown_fields_ignored() {
    let result = FsConfigLoader.load_config(&fixture_path("extra-fields.json"));
    assert!(result.is_ok(), "unknown fields should be silently ignored — got {result:?}");
    let config = result.unwrap_or_default();
    assert_eq!(config.terms.banned_words.len(), 1, "known field still works");
}

// ═══════════════════════════════════════════════════════════════
// Error cases
// ═══════════════════════════════════════════════════════════════

#[test]
fn nonexistent_file() {
    let result = FsConfigLoader.load_config(Path::new("/nonexistent/config.json"));
    assert!(matches!(result, Err(ConfigError::NotFound(_))), "NotFound — got {result:?}");
}

#[test]
fn empty_file() {
    let path = write_temp("empty.json", "");
    let result = FsConfigLoader.load_config(&path);
    assert!(matches!(result, Err(ConfigError::InvalidJson(_))), "empty file → InvalidJson — got {result:?}");
    cleanup(&path);
}

#[test]
fn invalid_json_syntax() {
    let path = write_temp("bad-syntax.json", "not json {{{");
    let result = FsConfigLoader.load_config(&path);
    assert!(matches!(result, Err(ConfigError::InvalidJson(_))), "bad syntax → InvalidJson — got {result:?}");
    cleanup(&path);
}

#[test]
fn missing_required_locale() {
    let result = FsConfigLoader.load_config(&fixture_path("missing-locale.json"));
    assert!(matches!(result, Err(ConfigError::InvalidJson(_))),
        "missing locale → InvalidJson (serde required field) — got {result:?}");
}

#[test]
fn wrong_type_for_field() {
    let result = FsConfigLoader.load_config(&fixture_path("wrong-type.json"));
    assert!(matches!(result, Err(ConfigError::InvalidJson(_))),
        "string where array expected → InvalidJson — got {result:?}");
}

#[test]
fn unknown_locale() {
    let result = FsConfigLoader.load_config(&fixture_path("bad-locale.json"));
    assert!(matches!(result, Err(ConfigError::ValidationFailed(_))),
        "unknown locale → ValidationFailed — got {result:?}");
    if let Err(ConfigError::ValidationFailed(msg)) = &result {
        assert!(msg.contains("xx"), "error mentions bad locale — got: {msg}");
    }
}

// ═══════════════════════════════════════════════════════════════
// Trait interface
// ═══════════════════════════════════════════════════════════════

#[test]
fn trait_object_works() {
    let loader: &dyn ConfigLoader = &FsConfigLoader;
    let result = loader.load_config(&fixture_path("sample-config.json"));
    assert!(result.is_ok(), "should work through trait object");
}
