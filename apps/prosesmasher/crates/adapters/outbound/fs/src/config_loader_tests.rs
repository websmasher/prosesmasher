use super::*;
use prosesmasher_domain_types::Locale;
use std::path::Path;

fn fixture_path(name: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

// ═══════════════════════════════════════════════════════════════
// Happy path
// ═══════════════════════════════════════════════════════════════

#[test]
fn load_sample_config_all_fields() {
    let loader = FsConfigLoader;
    let result = loader.load_config(&fixture_path("sample-config.json"));
    assert!(result.is_ok(), "should load valid config — got {result:?}");
    let config = result.unwrap_or_default();

    // Locale
    assert_eq!(config.locale, Locale::En, "locale");

    // Terms
    assert_eq!(config.terms.banned_words.len(), 2, "banned_words count");
    assert_eq!(config.terms.banned_words.first().map(String::as_str), Some("actually"), "first banned word");
    assert_eq!(config.terms.banned_phrases.len(), 1, "banned_phrases count");
    assert_eq!(config.terms.simplicity_pairs.len(), 2, "simplicity_pairs count");
    if let Some(pair) = config.terms.simplicity_pairs.first() {
        assert_eq!(pair.complex, "utilize", "first pair complex");
        assert_eq!(pair.simple, "use", "first pair simple");
    }
    assert_eq!(config.terms.hedge_words.len(), 2, "hedge_words count");
    assert_eq!(config.terms.stop_words.len(), 6, "stop_words count");

    // Thresholds
    assert!(config.thresholds.word_count.is_some(), "word_count present");
    if let Some(wc) = config.thresholds.word_count {
        assert_eq!(wc.min(), 650, "word_count min");
        assert_eq!(wc.max(), 1000, "word_count max");
    }
    assert!(config.thresholds.h2_count.is_some(), "h2_count present");
    assert_eq!(config.thresholds.h3_min, Some(0), "h3_min");
    assert_eq!(config.thresholds.bold_min, Some(3), "bold_min");
    assert_eq!(config.thresholds.max_paragraph_sentences, Some(4), "max_paragraph_sentences");
    assert_eq!(config.thresholds.flesch_kincaid_min, Some(50.0), "flesch_kincaid_min");
    assert_eq!(config.thresholds.gunning_fog_max, Some(14.0), "gunning_fog_max");
    assert_eq!(config.thresholds.avg_sentence_length_max, Some(25), "avg_sentence_length_max");
    assert_eq!(config.thresholds.word_repetition_max, Some(5), "word_repetition_max");
}

#[test]
fn load_minimal_config_defaults() {
    let loader = FsConfigLoader;
    let result = loader.load_config(&fixture_path("minimal-config.json"));
    assert!(result.is_ok(), "should load minimal config — got {result:?}");
    let config = result.unwrap_or_default();

    assert_eq!(config.locale, Locale::Ru, "locale should be Russian");
    assert!(config.terms.banned_words.is_empty(), "banned_words empty");
    assert!(config.terms.simplicity_pairs.is_empty(), "simplicity_pairs empty");
    assert!(config.thresholds.word_count.is_none(), "word_count absent → None");
    assert!(config.thresholds.flesch_kincaid_min.is_none(), "flesch absent → None");
}

#[test]
fn load_all_locales() {
    // Verify all 7 locale strings parse correctly
    for (code, expected) in [
        ("en", Locale::En),
        ("ru", Locale::Ru),
        ("de", Locale::De),
        ("es", Locale::Es),
        ("pt", Locale::Pt),
        ("fr", Locale::Fr),
        ("id", Locale::Id),
    ] {
        let json = format!(r#"{{"locale":"{code}","terms":{{}},"thresholds":{{}}}}"#);
        let path = std::env::temp_dir().join(format!("prosesmasher-test-locale-{code}.json"));
        #[allow(clippy::disallowed_methods)] // test setup
        std::fs::write(&path, &json).unwrap_or_default();
        let result = FsConfigLoader.load_config(&path);
        assert!(result.is_ok(), "locale '{code}' should parse — got {result:?}");
        assert_eq!(result.unwrap_or_default().locale, expected, "locale '{code}'");
        #[allow(clippy::disallowed_methods)] // test cleanup
        let _ = std::fs::remove_file(&path);
    }
}

// ═══════════════════════════════════════════════════════════════
// Error cases
// ═══════════════════════════════════════════════════════════════

#[test]
fn load_nonexistent_file() {
    let loader = FsConfigLoader;
    let result = loader.load_config(Path::new("/nonexistent/config.json"));
    assert!(result.is_err(), "should fail");
    assert!(matches!(result, Err(ConfigError::NotFound(_))), "should be NotFound — got {result:?}");
}

#[test]
fn load_invalid_json() {
    let path = std::env::temp_dir().join("prosesmasher-test-bad-json.json");
    #[allow(clippy::disallowed_methods)] // test setup — writing temp fixture files
    std::fs::write(&path, "not valid json {{{").unwrap_or_default();
    let result = FsConfigLoader.load_config(&path);
    assert!(matches!(result, Err(ConfigError::InvalidJson(_))), "should be InvalidJson — got {result:?}");
    #[allow(clippy::disallowed_methods)] // test cleanup
        let _ = std::fs::remove_file(&path);
}

#[test]
fn load_invalid_range_min_gt_max() {
    let loader = FsConfigLoader;
    let result = loader.load_config(&fixture_path("invalid-range.json"));
    assert!(matches!(result, Err(ConfigError::ValidationFailed(_))),
        "min > max should be ValidationFailed — got {result:?}");
    if let Err(ConfigError::ValidationFailed(msg)) = &result {
        assert!(msg.contains("min"), "error should mention min — got: {msg}");
    }
}

#[test]
fn load_bad_locale() {
    let loader = FsConfigLoader;
    let result = loader.load_config(&fixture_path("bad-locale.json"));
    assert!(matches!(result, Err(ConfigError::ValidationFailed(_))),
        "unknown locale should be ValidationFailed — got {result:?}");
    if let Err(ConfigError::ValidationFailed(msg)) = &result {
        assert!(msg.contains("xx"), "error should mention the bad locale — got: {msg}");
    }
}

#[test]
fn trait_object_works() {
    let loader: &dyn ConfigLoader = &FsConfigLoader;
    let result = loader.load_config(&fixture_path("sample-config.json"));
    assert!(result.is_ok(), "should work through trait object");
}
