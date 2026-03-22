use super::*;
use prosesmasher_domain_types::{Locale, Range};
use std::path::Path;

fn fixture_path(name: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

fn preset_path(name: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../../presets")
        .join(name)
}

fn write_temp(name: &str, content: &str) -> std::path::PathBuf {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!("prosesmasher-test-{name}-{id}"));
    #[allow(clippy::disallowed_methods, clippy::panic)] // test setup
    {
        std::fs::write(&path, content).unwrap_or_else(|e| panic!("failed to write temp {}: {e}", path.display()));
    }
    path
}

fn cleanup(path: &Path) {
    #[allow(clippy::disallowed_methods)] // test cleanup
    let _ = std::fs::remove_file(path);
}

// Helper: load JSON string, assert success, return config
#[allow(clippy::panic)] // test helper
fn load_json_ok(json: &str) -> prosesmasher_domain_types::CheckConfig {
    let path = write_temp("json", json);
    let result = FsConfigLoader.load_config(&path);
    cleanup(&path);
    if let Ok(config) = result {
        config
    } else {
        panic!("expected Ok, got Err: {result:?}")
    }
}

// Helper: load JSON string, assert failure
#[allow(clippy::panic)] // test helper
fn load_json_err(json: &str) -> ConfigError {
    let path = write_temp("json", json);
    let result = FsConfigLoader.load_config(&path);
    cleanup(&path);
    if let Err(e) = result {
        e
    } else {
        panic!("expected Err, got Ok")
    }
}

// Helper: load fixture, assert success
#[allow(clippy::panic)] // test helper
fn load_fixture_ok(name: &str) -> prosesmasher_domain_types::CheckConfig {
    match FsConfigLoader.load_config(&fixture_path(name)) {
        Ok(config) => config,
        Err(e) => panic!("failed to load fixture {name}: {e}"),
    }
}

#[allow(clippy::panic)] // test helper
fn load_preset_ok(name: &str) -> prosesmasher_domain_types::CheckConfig {
    match FsConfigLoader.load_config(&preset_path(name)) {
        Ok(config) => config,
        Err(e) => panic!("failed to load preset {name}: {e}"),
    }
}

// ═══════════════════════════════════════════════════════════════
// Field swap detection — every field has a unique sentinel
// ═══════════════════════════════════════════════════════════════

#[test]
fn every_term_field_maps_correctly() {
    let c = load_fixture_ok("swap-detection.json");

    assert_eq!(c.locale, Locale::Fr, "locale");
    assert_eq!(c.terms.banned_words.first().map(String::as_str), Some("BANNED_WORD_SENTINEL"), "banned_words");
    assert_eq!(c.terms.banned_phrases.first().map(String::as_str), Some("BANNED_PHRASE_SENTINEL"), "banned_phrases");
    assert_eq!(c.terms.gendered_terms.first().map(String::as_str), Some("GENDERED_SENTINEL"), "gendered_terms");
    assert_eq!(c.terms.forbidden_terms.first().map(String::as_str), Some("FORBIDDEN_SENTINEL"), "forbidden_terms");
    assert_eq!(c.terms.race_terms.first().map(String::as_str), Some("RACE_SENTINEL"), "race_terms");
    assert_eq!(c.terms.hedge_words.first().map(String::as_str), Some("HEDGE_SENTINEL"), "hedge_words");
    assert_eq!(c.terms.negation_signals.first().map(String::as_str), Some("NEGATION_SENTINEL"), "negation_signals");
    assert_eq!(c.terms.reframe_signals.first().map(String::as_str), Some("REFRAME_SENTINEL"), "reframe_signals");
    assert_eq!(c.terms.llm_openers.first().map(String::as_str), Some("LLM_OPENER_SENTINEL"), "llm_openers");
    assert_eq!(c.terms.affirmation_closers.first().map(String::as_str), Some("AFFIRMATION_SENTINEL"), "affirmation_closers");
    assert_eq!(c.terms.summative_patterns.first().map(String::as_str), Some("SUMMATIVE_SENTINEL"), "summative_patterns");
    assert_eq!(c.terms.false_question_patterns.first().map(String::as_str), Some("FALSE_QUESTION_SENTINEL"), "false_question_patterns");
    assert_eq!(c.terms.humble_bragger_phrases.first().map(String::as_str), Some("HUMBLE_SENTINEL"), "humble_bragger_phrases");
    assert_eq!(c.terms.jargon_faker_phrases.first().map(String::as_str), Some("JARGON_SENTINEL"), "jargon_faker_phrases");
    assert_eq!(c.terms.stop_words.first().map(String::as_str), Some("STOP_SENTINEL"), "stop_words");

    // SimplePair — both fields unique
    assert_eq!(c.terms.simplicity_pairs.len(), 1, "simplicity_pairs count");
    let pair = c.terms.simplicity_pairs.first();
    assert_eq!(pair.map(|p| p.complex.as_str()), Some("COMPLEX_SENTINEL"), "pair.complex");
    assert_eq!(pair.map(|p| p.simple.as_str()), Some("SIMPLE_SENTINEL"), "pair.simple");
}

#[test]
fn every_threshold_field_maps_correctly() {
    let c = load_fixture_ok("swap-detection.json");
    let t = &c.thresholds;

    // Range fields — assert is_some AND exact values (no silent skip)
    assert!(t.word_count.is_some(), "word_count must be Some");
    assert_eq!(t.word_count.map(Range::min), Some(111), "word_count.min");
    assert_eq!(t.word_count.map(Range::max), Some(222), "word_count.max");

    assert!(t.h2_count.is_some(), "h2_count must be Some");
    assert_eq!(t.h2_count.map(Range::min), Some(333), "h2_count.min");
    assert_eq!(t.h2_count.map(Range::max), Some(444), "h2_count.max");

    // usize fields — all unique
    assert_eq!(t.h3_min, Some(555), "h3_min");
    assert_eq!(t.bold_min, Some(666), "bold_min");
    assert_eq!(t.max_paragraph_sentences, Some(777), "max_paragraph_sentences");
    assert_eq!(t.max_exclamations_per_paragraph, Some(888), "max_exclamations_per_paragraph");
    assert_eq!(t.max_hedges_per_sentence, Some(999), "max_hedges_per_sentence");
    assert_eq!(t.avg_sentence_length_max, Some(1111), "avg_sentence_length_max");
    assert_eq!(t.word_repetition_max, Some(2222), "word_repetition_max");

    // f64 fields — all unique
    assert_eq!(t.flesch_kincaid_min, Some(11.11), "flesch_kincaid_min");
    assert_eq!(t.gunning_fog_max, Some(22.22), "gunning_fog_max");
    assert_eq!(t.coleman_liau_max, Some(33.33), "coleman_liau_max");
}

// ═══════════════════════════════════════════════════════════════
// All fields absent → defaults
// ═══════════════════════════════════════════════════════════════

#[test]
fn all_absent_terms_empty() {
    let c = load_fixture_ok("minimal-config.json");
    assert_eq!(c.locale, Locale::Ru, "locale");
    assert!(c.terms.banned_words.is_empty(), "banned_words");
    assert!(c.terms.banned_phrases.is_empty(), "banned_phrases");
    assert!(c.terms.gendered_terms.is_empty(), "gendered_terms");
    assert!(c.terms.forbidden_terms.is_empty(), "forbidden_terms");
    assert!(c.terms.race_terms.is_empty(), "race_terms");
    assert!(c.terms.hedge_words.is_empty(), "hedge_words");
    assert!(c.terms.simplicity_pairs.is_empty(), "simplicity_pairs");
    assert!(c.terms.negation_signals.is_empty(), "negation_signals");
    assert!(c.terms.reframe_signals.is_empty(), "reframe_signals");
    assert!(c.terms.llm_openers.is_empty(), "llm_openers");
    assert!(c.terms.affirmation_closers.is_empty(), "affirmation_closers");
    assert!(c.terms.summative_patterns.is_empty(), "summative_patterns");
    assert!(c.terms.false_question_patterns.is_empty(), "false_question_patterns");
    assert!(c.terms.humble_bragger_phrases.is_empty(), "humble_bragger_phrases");
    assert!(c.terms.jargon_faker_phrases.is_empty(), "jargon_faker_phrases");
    assert!(c.terms.stop_words.is_empty(), "stop_words");
}

#[test]
fn all_absent_thresholds_none() {
    let c = load_fixture_ok("minimal-config.json");
    assert!(c.thresholds.word_count.is_none(), "word_count");
    assert!(c.thresholds.h2_count.is_none(), "h2_count");
    assert!(c.thresholds.h3_min.is_none(), "h3_min");
    assert!(c.thresholds.bold_min.is_none(), "bold_min");
    assert!(c.thresholds.max_paragraph_sentences.is_none(), "max_paragraph_sentences");
    assert!(c.thresholds.max_exclamations_per_paragraph.is_none(), "max_exclamations_per_paragraph");
    assert!(c.thresholds.max_hedges_per_sentence.is_none(), "max_hedges_per_sentence");
    assert!(c.thresholds.flesch_kincaid_min.is_none(), "flesch_kincaid_min");
    assert!(c.thresholds.gunning_fog_max.is_none(), "gunning_fog_max");
    assert!(c.thresholds.avg_sentence_length_max.is_none(), "avg_sentence_length_max");
    assert!(c.thresholds.word_repetition_max.is_none(), "word_repetition_max");
    assert!(c.thresholds.coleman_liau_max.is_none(), "coleman_liau_max");
}

// ═══════════════════════════════════════════════════════════════
// Partial fields
// ═══════════════════════════════════════════════════════════════

#[test]
fn partial_terms_some_present_some_absent() {
    let c = load_fixture_ok("partial-terms.json");
    assert!(c.terms.banned_words.iter().any(|item| item == "present"), "present");
    assert_eq!(c.terms.hedge_words.len(), 1, "present");
    assert!(c.terms.banned_phrases.is_empty(), "absent");
    assert!(c.terms.simplicity_pairs.is_empty(), "absent");
    assert_eq!(c.thresholds.bold_min, Some(5), "present threshold");
    assert!(c.thresholds.word_count.is_none(), "absent threshold");
}

// ═══════════════════════════════════════════════════════════════
// Locale
// ═══════════════════════════════════════════════════════════════

#[test]
fn all_seven_locales() {
    for (code, expected) in [
        ("en", Locale::En), ("ru", Locale::Ru), ("de", Locale::De),
        ("es", Locale::Es), ("pt", Locale::Pt), ("fr", Locale::Fr),
        ("id", Locale::Id),
    ] {
        let json = format!(r#"{{"locale":"{code}","terms":{{}},"thresholds":{{}}}}"#);
        let c = load_json_ok(&json);
        assert_eq!(c.locale, expected, "locale '{code}'");
    }
}

#[test]
fn locale_case_insensitive() {
    for code in ["EN", "En", "eN", "RU", "Ru", "DE", "De"] {
        let json = format!(r#"{{"locale":"{code}","terms":{{}},"thresholds":{{}}}}"#);
        let c = load_json_ok(&json);
        // Just verify it parsed without error — locale value varies
        assert!(!format!("{:?}", c.locale).is_empty(), "locale parsed for '{code}'");
    }
}

#[test]
fn locale_single_char_fails_garde() {
    let err = load_json_err(r#"{"locale":"x","terms":{},"thresholds":{}}"#);
    assert!(matches!(err, ConfigError::ValidationFailed(_)),
        "single-char locale should fail garde — got {err:?}");
}

#[test]
fn locale_too_long_fails_garde() {
    let err = load_json_err(r#"{"locale":"english","terms":{},"thresholds":{}}"#);
    assert!(matches!(err, ConfigError::ValidationFailed(_)),
        "7-char locale should fail garde — got {err:?}");
}

#[test]
fn locale_empty_string_fails() {
    let err = load_json_err(r#"{"locale":"","terms":{},"thresholds":{}}"#);
    assert!(matches!(err, ConfigError::ValidationFailed(_)),
        "empty locale should fail — got {err:?}");
}

#[test]
fn locale_unknown_valid_length_fails() {
    let err = load_json_err(r#"{"locale":"xx","terms":{},"thresholds":{}}"#);
    assert!(matches!(err, ConfigError::ValidationFailed(ref msg) if msg.contains("xx")),
        "unknown locale → ValidationFailed with 'xx' — got {err:?}");
}

#[test]
fn locale_with_whitespace_fails() {
    let err = load_json_err(r#"{"locale":" en ","terms":{},"thresholds":{}}"#);
    // " en " is 4 chars, passes garde length, but parse_locale(" en ") doesn't match
    assert!(matches!(err, ConfigError::ValidationFailed(_)),
        "locale with whitespace should fail — got {err:?}");
}

// ═══════════════════════════════════════════════════════════════
// Presets
// ═══════════════════════════════════════════════════════════════

#[test]
fn all_curated_presets_load() {
    for name in [
        "general-en.json",
        "blog-strict-en.json",
        "technical-article-en.json",
        "docs-en.json",
        "landing-page-en.json",
        "essay-en.json",
    ] {
        let config = load_preset_ok(name);
        assert_eq!(config.locale, Locale::En, "{name}: locale should be en");
    }
}

#[test]
fn docs_preset_is_tighter_on_exclamations_than_general() {
    let docs = load_preset_ok("docs-en.json");
    let general = load_preset_ok("general-en.json");
    assert_eq!(docs.thresholds.max_exclamations_per_paragraph, Some(0), "docs preset");
    assert_eq!(general.thresholds.max_exclamations_per_paragraph, Some(1), "general preset");
}

#[test]
fn landing_page_preset_targets_shorter_copy_than_essay() {
    let landing = load_preset_ok("landing-page-en.json");
    let essay = load_preset_ok("essay-en.json");

    assert_eq!(landing.thresholds.word_count.map(Range::max), Some(700), "landing max words");
    assert_eq!(essay.thresholds.word_count.map(Range::min), Some(900), "essay min words");
}

// ═══════════════════════════════════════════════════════════════
// Garde actually runs (not just parse_locale catching it)
// ═══════════════════════════════════════════════════════════════

#[test]
fn garde_validates_locale_length_before_parse() {
    // "a" is length 1 — fails garde (min=2) BEFORE reaching parse_locale.
    // If garde were removed, parse_locale("a") would return
    // ValidationFailed("unknown locale: a"). We distinguish by checking
    // that the error does NOT contain "unknown locale" (garde produces
    // a different message about length).
    let err = load_json_err(r#"{"locale":"a","terms":{},"thresholds":{}}"#);
    if let ConfigError::ValidationFailed(msg) = &err {
        assert!(!msg.contains("unknown locale"),
            "1-char locale should be caught by garde (length), not parse_locale — got: {msg}");
    } else {
        assert!(matches!(err, ConfigError::ValidationFailed(_)),
            "expected ValidationFailed — got {err:?}");
    }
}

#[test]
fn garde_catches_too_long_before_parse() {
    // "english" is 7 chars — fails garde (max=5). parse_locale would also
    // fail with "unknown locale: english". Distinguish by message content.
    let err = load_json_err(r#"{"locale":"english","terms":{},"thresholds":{}}"#);
    if let ConfigError::ValidationFailed(msg) = &err {
        assert!(!msg.contains("unknown locale"),
            "7-char locale should be caught by garde, not parse_locale — got: {msg}");
    } else {
        assert!(matches!(err, ConfigError::ValidationFailed(_)),
            "expected ValidationFailed — got {err:?}");
    }
}

// ═══════════════════════════════════════════════════════════════
// Range edge cases
// ═══════════════════════════════════════════════════════════════

#[test]
fn range_min_equals_max_valid() {
    let c = load_fixture_ok("range-edge-min-eq-max.json");
    assert!(c.thresholds.word_count.is_some(), "must be Some");
    assert_eq!(c.thresholds.word_count.map(Range::min), Some(500), "min");
    assert_eq!(c.thresholds.word_count.map(Range::max), Some(500), "max");
}

#[test]
fn range_zero_zero_valid() {
    let c = load_fixture_ok("range-edge-zero.json");
    assert!(c.thresholds.word_count.is_some(), "must be Some");
    assert_eq!(c.thresholds.word_count.map(Range::min), Some(0), "min");
    assert_eq!(c.thresholds.word_count.map(Range::max), Some(0), "max");
}

#[test]
fn range_min_gt_max_error() {
    let err = load_json_err(r#"{"locale":"en","terms":{},"thresholds":{"wordCount":{"min":1000,"max":500}}}"#);
    assert!(matches!(err, ConfigError::ValidationFailed(ref msg) if msg.contains("min") && msg.contains("max")),
        "min > max → ValidationFailed — got {err:?}");
}

#[test]
fn range_h2_count_also_validated() {
    let err = load_json_err(r#"{"locale":"en","terms":{},"thresholds":{"h2Count":{"min":10,"max":5}}}"#);
    assert!(matches!(err, ConfigError::ValidationFailed(ref msg) if msg.contains("h2Count")),
        "h2Count min > max → ValidationFailed with field name — got {err:?}");
}

#[test]
fn range_large_values() {
    let c = load_json_ok(r#"{"locale":"en","terms":{},"thresholds":{"wordCount":{"min":0,"max":999999999}}}"#);
    assert_eq!(c.thresholds.word_count.map(Range::max), Some(999_999_999), "large max");
}

// ═══════════════════════════════════════════════════════════════
// SimplePair edge cases
// ═══════════════════════════════════════════════════════════════

#[test]
fn simplicity_pair_empty_strings() {
    let c = load_json_ok(r#"{"locale":"en","terms":{"simplicityPairs":[["",""]]},"thresholds":{}}"#);
    let pair = c.terms.simplicity_pairs.first();
    assert_eq!(pair.map(|p| p.complex.as_str()), Some(""), "empty complex");
    assert_eq!(pair.map(|p| p.simple.as_str()), Some(""), "empty simple");
}

#[test]
fn simplicity_pair_wrong_size_1_fails() {
    let err = load_json_err(r#"{"locale":"en","terms":{"simplicityPairs":[["only_one"]]},"thresholds":{}}"#);
    assert!(matches!(err, ConfigError::InvalidJson(_)), "array of 1 → InvalidJson — got {err:?}");
}

#[test]
fn simplicity_pair_wrong_size_3_fails() {
    let err = load_json_err(r#"{"locale":"en","terms":{"simplicityPairs":[["a","b","c"]]},"thresholds":{}}"#);
    assert!(matches!(err, ConfigError::InvalidJson(_)), "array of 3 → InvalidJson — got {err:?}");
}

#[test]
fn multiple_simplicity_pairs_order_preserved() {
    let c = load_json_ok(r#"{"locale":"en","terms":{"simplicityPairs":[["first","1st"],["second","2nd"],["third","3rd"]]},"thresholds":{}}"#);
    assert_eq!(c.terms.simplicity_pairs.len(), 3, "3 pairs");
    assert_eq!(c.terms.simplicity_pairs.first().map(|p| p.complex.as_str()), Some("first"), "pair 0");
    assert_eq!(c.terms.simplicity_pairs.get(1).map(|p| p.complex.as_str()), Some("second"), "pair 1");
    assert_eq!(c.terms.simplicity_pairs.get(2).map(|p| p.complex.as_str()), Some("third"), "pair 2");
}

// ═══════════════════════════════════════════════════════════════
// Unicode and special characters
// ═══════════════════════════════════════════════════════════════

#[test]
fn unicode_cyrillic_terms() {
    let c = load_fixture_ok("unicode-terms.json");
    assert_eq!(c.terms.banned_words.first().map(String::as_str), Some("кстати"), "Cyrillic");
    let pair = c.terms.simplicity_pairs.first();
    assert_eq!(pair.map(|p| p.complex.as_str()), Some("утилизировать"), "Cyrillic pair");
}

#[test]
fn terms_with_special_chars() {
    let c = load_json_ok(r#"{"locale":"en","terms":{"bannedPhrases":["it's a \"test\"","line\nbreak","tab\there"]},"thresholds":{}}"#);
    assert_eq!(c.terms.banned_phrases.len(), 3, "3 phrases with special chars");
}

#[test]
fn terms_with_emoji() {
    let c = load_json_ok(r#"{"locale":"en","terms":{"bannedWords":["🚀","✨magic✨"]},"thresholds":{}}"#);
    assert_eq!(c.terms.banned_words.first().map(String::as_str), Some("🚀"), "emoji");
}

// ═══════════════════════════════════════════════════════════════
// Threshold type edge cases
// ═══════════════════════════════════════════════════════════════

#[test]
fn negative_float_threshold() {
    let c = load_json_ok(r#"{"locale":"en","terms":{},"thresholds":{"fleschKincaidMin":-10.5}}"#);
    assert_eq!(c.thresholds.flesch_kincaid_min, Some(-10.5), "negative float");
}

#[test]
fn zero_threshold() {
    let c = load_json_ok(r#"{"locale":"en","terms":{},"thresholds":{"boldMin":0}}"#);
    assert_eq!(c.thresholds.bold_min, Some(0), "zero");
}

#[test]
fn integer_coerced_to_float() {
    let c = load_json_ok(r#"{"locale":"en","terms":{},"thresholds":{"fleschKincaidMin":50}}"#);
    assert_eq!(c.thresholds.flesch_kincaid_min, Some(50.0), "int → float");
}

#[test]
fn float_for_usize_fails() {
    let err = load_json_err(r#"{"locale":"en","terms":{},"thresholds":{"boldMin":3.5}}"#);
    assert!(matches!(err, ConfigError::InvalidJson(_)), "float for usize — got {err:?}");
}

#[test]
fn negative_usize_fails() {
    let err = load_json_err(r#"{"locale":"en","terms":{},"thresholds":{"boldMin":-1}}"#);
    assert!(matches!(err, ConfigError::InvalidJson(_)), "negative usize — got {err:?}");
}

#[test]
fn string_for_number_fails() {
    let err = load_json_err(r#"{"locale":"en","terms":{},"thresholds":{"boldMin":"three"}}"#);
    assert!(matches!(err, ConfigError::InvalidJson(_)), "string for number — got {err:?}");
}

#[test]
fn null_threshold_is_none() {
    let c = load_json_ok(r#"{"locale":"en","terms":{},"thresholds":{"boldMin":null}}"#);
    assert!(c.thresholds.bold_min.is_none(), "null → None");
}

// ═══════════════════════════════════════════════════════════════
// JSON structure edge cases
// ═══════════════════════════════════════════════════════════════

#[test]
fn extra_unknown_fields_ignored() {
    let c = load_fixture_ok("extra-fields.json");
    assert_eq!(c.terms.banned_words.len(), 1, "known field works");
}

#[test]
fn missing_locale_fails() {
    let err = load_json_err(r#"{"terms":{},"thresholds":{}}"#);
    assert!(matches!(err, ConfigError::InvalidJson(_)), "missing locale — got {err:?}");
}

#[test]
fn missing_terms_uses_defaults() {
    let c = load_json_ok(r#"{"locale":"en","thresholds":{}}"#);
    assert!(c.terms.banned_words.iter().any(|item| item == "actually"), "defaults applied");
}

#[test]
fn missing_thresholds_uses_quality_defaults() {
    let c = load_json_ok(r#"{"locale":"en","terms":{}}"#);
    assert_eq!(c.thresholds.max_paragraph_sentences, Some(4), "quality defaults synthesized");
}

#[test]
fn target_schema_quality_and_document_policy_normalize() {
    let c = load_json_ok(
        r#"{
          "locale":"en",
          "quality":{
            "lexical":{
              "prohibitedTerms":{"defaults":true,"add":["live coaching calls"],"remove":["actually"]},
              "requiredTerms":["ownership"]
            },
            "heuristics":{
              "wordRepetition":{"max":7,"excludedTerms":{"defaults":true,"add":["ownership"],"remove":["that"]}},
              "paragraphLength":{"maxSentences":5}
            }
          },
          "documentPolicy":{
            "wordCount":{"min":650,"max":1000},
            "headingHierarchy":{"enabled":true},
            "sentenceCaseHeadings":{"enabled":true},
            "codeFences":{"allowed":false}
          }
        }"#,
    );

    assert_eq!(c.quality.lexical.required_terms, vec!["ownership".to_owned()], "required terms");
    assert!(c.terms.banned_words.iter().all(|term| term != "actually"), "removed default term");
    assert!(c.terms.banned_phrases.iter().any(|term| term == "live coaching calls"), "added prohibited phrase");
    assert_eq!(c.thresholds.word_repetition_max, Some(7), "word repetition max");
    assert_eq!(c.thresholds.max_paragraph_sentences, Some(5), "paragraph length max");
    assert_eq!(c.thresholds.word_count.map(Range::min), Some(650), "document policy word count");
    assert!(c.document_policy.heading_hierarchy, "heading hierarchy enabled");
    assert!(c.document_policy.sentence_case_headings, "sentence case enabled");
    assert!(!c.document_policy.allow_code_fences, "code fences disallowed");
}

#[test]
fn wrong_type_string_for_array() {
    let err = load_json_err(r#"{"locale":"en","terms":{"bannedWords":"not_an_array"},"thresholds":{}}"#);
    assert!(matches!(err, ConfigError::InvalidJson(_)), "string for array — got {err:?}");
}

#[test]
fn empty_file_fails() {
    let path = write_temp("empty", "");
    let result = FsConfigLoader.load_config(&path);
    cleanup(&path);
    assert!(matches!(result, Err(ConfigError::InvalidJson(_))), "empty — got {result:?}");
}

#[test]
fn invalid_json_syntax_fails() {
    let err = load_json_err("not json {{{");
    assert!(matches!(err, ConfigError::InvalidJson(_)), "bad syntax — got {err:?}");
}

#[test]
fn json_array_root_fails() {
    let err = load_json_err(r#"[{"locale":"en"}]"#);
    assert!(matches!(err, ConfigError::InvalidJson(_)), "array root — got {err:?}");
}

#[test]
fn json_null_root_fails() {
    let err = load_json_err("null");
    assert!(matches!(err, ConfigError::InvalidJson(_)), "null root — got {err:?}");
}

#[test]
fn json_trailing_comma_fails() {
    let err = load_json_err(r#"{"locale":"en","terms":{},"thresholds":{},}"#);
    assert!(matches!(err, ConfigError::InvalidJson(_)), "trailing comma — got {err:?}");
}

// ═══════════════════════════════════════════════════════════════
// File system
// ═══════════════════════════════════════════════════════════════

#[test]
fn nonexistent_file_not_found() {
    let result = FsConfigLoader.load_config(Path::new("/nonexistent/config.json"));
    assert!(matches!(result, Err(ConfigError::NotFound(_))), "NotFound — got {result:?}");
}

#[test]
fn not_found_error_contains_path() {
    let result = FsConfigLoader.load_config(Path::new("/no/myconfig.json"));
    match result {
        Err(ConfigError::NotFound(msg)) => {
            assert!(msg.contains("myconfig.json"), "should contain filename — got: {msg}");
        }
        other => {
            assert!(matches!(other, Err(ConfigError::NotFound(_))),
                "expected NotFound — got {other:?}");
        }
    }
}

// ═══════════════════════════════════════════════════════════════
// Trait interface
// ═══════════════════════════════════════════════════════════════

#[test]
#[allow(clippy::panic)]
fn trait_object_works() {
    let loader: &dyn ConfigLoader = &FsConfigLoader;
    match loader.load_config(&fixture_path("sample-config.json")) {
        Ok(config) => assert_eq!(config.locale, Locale::En, "correct locale through trait"),
        Err(e) => panic!("trait object should work — got {e:?}"),
    }
}

// ═══════════════════════════════════════════════════════════════
// 4-angle attack findings
// ═══════════════════════════════════════════════════════════════

#[test]
fn locale_3_to_5_chars_passes_garde_but_fails_parse() {
    // Garde allows 2-5 chars, but parse_locale only knows 2-char codes.
    // "pt-br" is 5 chars → passes garde, fails parse_locale.
    // Documents the gap between validation layers.
    let err = load_json_err(r#"{"locale":"pt-br","terms":{},"thresholds":{}}"#);
    assert!(matches!(err, ConfigError::ValidationFailed(ref msg) if msg.contains("unknown locale")),
        "3-5 char locale passes garde but fails parse — got {err:?}");
}

#[test]
fn infinity_threshold_rejected_by_serde() {
    // JSON 1e999 is rejected by serde_json as "number out of range"
    let err = load_json_err(r#"{"locale":"en","terms":{},"thresholds":{"fleschKincaidMin":1e999}}"#);
    assert!(matches!(err, ConfigError::InvalidJson(_)),
        "infinity should be rejected by serde — got {err:?}");
}

#[test]
fn non_utf8_file_fails() {
    // Non-UTF-8 bytes cause read_to_string to fail with InvalidData
    // which maps to ReadError::Io → ConfigError::NotFound (known semantic gap:
    // IO errors are mapped to NotFound because ConfigError has no Io variant)
    let path = write_temp("nonutf8", "");
    // Overwrite with raw bytes
    #[allow(clippy::disallowed_methods)]
    std::fs::write(&path, b"\xff\xfe\x00\x01").unwrap_or_default();
    let result = FsConfigLoader.load_config(&path);
    assert!(matches!(result, Err(ConfigError::NotFound(_))),
        "non-UTF-8 → NotFound (IO mapped to NotFound) — got {result:?}");
    cleanup(&path);
}

// ═══════════════════════════════════════════════════════════════
// 4-angle attack: round 2 fixes
// ═══════════════════════════════════════════════════════════════

#[test]
fn negative_gunning_fog_accepted() {
    let c = load_json_ok(r#"{"locale":"en","terms":{},"thresholds":{"gunningFogMax":-5.0}}"#);
    assert_eq!(c.thresholds.gunning_fog_max, Some(-5.0), "negative gunning fog");
}

#[test]
fn negative_coleman_liau_accepted() {
    let c = load_json_ok(r#"{"locale":"en","terms":{},"thresholds":{"colemanLiauMax":-3.5}}"#);
    assert_eq!(c.thresholds.coleman_liau_max, Some(-3.5), "negative coleman liau");
}

#[test]
fn duplicate_json_keys_rejected() {
    // serde_json rejects duplicate keys
    let err = load_json_err(r#"{"locale":"en","locale":"ru","terms":{},"thresholds":{}}"#);
    assert!(matches!(err, ConfigError::InvalidJson(ref msg) if msg.contains("duplicate")),
        "duplicate keys → InvalidJson — got {err:?}");
}

#[test]
fn nested_unknown_fields_in_terms_ignored() {
    let c = load_json_ok(r#"{"locale":"en","terms":{"bannedWords":["a"],"unknownNested":{"deep":true}},"thresholds":{}}"#);
    assert_eq!(c.terms.banned_words.len(), 1, "known field works despite nested unknown");
}

#[test]
fn utf8_bom_causes_parse_error() {
    // UTF-8 BOM (EF BB BF) before valid JSON — serde_json rejects
    let mut content = vec![0xEF, 0xBB, 0xBF];
    content.extend_from_slice(br#"{"locale":"en","terms":{},"thresholds":{}}"#);
    let path = write_temp("bom", "");
    #[allow(clippy::disallowed_methods)]
    std::fs::write(&path, &content).unwrap_or_default();
    let result = FsConfigLoader.load_config(&path);
    assert!(matches!(result, Err(ConfigError::InvalidJson(_))),
        "BOM should cause parse error — got {result:?}");
    cleanup(&path);
}
