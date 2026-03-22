use super::*;
use prosesmasher_domain_types::Locale;
use std::path::Path;

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
    #[allow(clippy::disallowed_methods, clippy::panic)]
    {
        std::fs::write(&path, content)
            .unwrap_or_else(|e| panic!("failed to write temp {}: {e}", path.display()));
    }
    path
}

fn cleanup(path: &Path) {
    #[allow(clippy::disallowed_methods)]
    let _ = std::fs::remove_file(path);
}

#[allow(clippy::panic)]
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

#[allow(clippy::panic)]
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

#[allow(clippy::panic)]
fn load_preset_ok(name: &str) -> prosesmasher_domain_types::CheckConfig {
    match FsConfigLoader.load_config(&preset_path(name)) {
        Ok(config) => config,
        Err(e) => panic!("failed to load preset {name}: {e}"),
    }
}

#[test]
fn canonical_config_normalizes() {
    let config = load_json_ok(
        r#"{
          "locale":"en",
          "quality":{
            "lexical":{
              "prohibitedTerms":{"defaults":true,"add":["live coaching calls"],"remove":["actually"]},
              "requiredTerms":["ownership"],
              "recommendedTerms":{"terms":["ownership","borrowing"],"minCount":1,"allowInflections":true},
              "simplicityPairs":{"defaults":false,"add":[["utilize","use"]],"remove":[]}
            },
            "heuristics":{
              "wordRepetition":{"max":7,"excludedTerms":{"defaults":false,"add":["ownership"],"remove":[]}},
              "paragraphLength":{"maxSentences":5},
              "readability":{"fleschKincaidMin":44.0}
            }
          },
          "documentPolicy":{
            "wordCount":{"min":650,"max":1000},
            "headingCounts":{"h2":{"min":2,"max":6},"h3Min":1},
            "headingHierarchy":{"enabled":true},
            "sentenceCaseHeadings":{"enabled":true},
            "codeFences":{"allowed":false},
            "boldDensity":{"min":3}
          }
        }"#,
    );

    assert_eq!(config.locale, Locale::En);
    assert!(
        config
            .quality
            .lexical
            .prohibited_terms
            .add
            .iter()
            .any(|term| term == "live coaching calls")
    );
    assert!(
        config
            .quality
            .lexical
            .prohibited_terms
            .remove
            .iter()
            .any(|term| term == "actually")
    );
    assert_eq!(config.quality.lexical.required_terms, vec!["ownership".to_owned()]);
    assert_eq!(
        config.quality.lexical.recommended_terms.as_ref().map(|pool| pool.min_count),
        Some(1)
    );
    assert_eq!(
        config.quality.heuristics.word_repetition.max,
        7
    );
    assert_eq!(
        config.quality.heuristics.paragraph_length.max_sentences,
        5
    );
    assert_eq!(
        config.quality.heuristics.readability.flesch_kincaid_min,
        Some(44.0)
    );
    assert_eq!(
        config.document_policy.word_count.map(prosesmasher_domain_types::Range::min),
        Some(650)
    );
    assert_eq!(
        config.document_policy.heading_counts.h3_min,
        Some(1)
    );
    assert!(config.document_policy.heading_hierarchy);
    assert!(config.document_policy.sentence_case_headings);
    assert!(!config.document_policy.allow_code_fences);
    assert_eq!(config.document_policy.bold_density_min, Some(3));
}

#[test]
fn all_curated_presets_load() {
    for name in [
        "general-en.json",
        "article-short-en.json",
        "article-medium-en.json",
        "article-long-en.json",
        "docs-en.json",
        "landing-page-en.json",
    ] {
        let config = load_preset_ok(name);
        assert_eq!(config.locale, Locale::En, "{name}: locale should be en");
    }
}

#[test]
fn docs_preset_is_tighter_on_exclamations_than_general() {
    let docs = load_preset_ok("docs-en.json");
    let general = load_preset_ok("general-en.json");

    assert_eq!(docs.quality.heuristics.exclamation_density.max_per_paragraph, 0);
    assert_eq!(general.quality.heuristics.exclamation_density.max_per_paragraph, 1);
}

#[test]
fn landing_page_preset_targets_shorter_copy_than_article_long() {
    let landing = load_preset_ok("landing-page-en.json");
    let article_long = load_preset_ok("article-long-en.json");

    assert_eq!(landing.document_policy.word_count.map(prosesmasher_domain_types::Range::max), Some(700));
    assert_eq!(article_long.document_policy.word_count.map(prosesmasher_domain_types::Range::min), Some(1600));
}

#[test]
fn general_preset_keeps_document_policy_off() {
    let general = load_preset_ok("general-en.json");

    assert!(general.document_policy.word_count.is_none());
    assert!(general.document_policy.heading_counts.h2.is_none());
    assert!(!general.document_policy.heading_hierarchy);
}

#[test]
fn article_tiers_increase_word_count_and_heading_density() {
    let short = load_preset_ok("article-short-en.json");
    let medium = load_preset_ok("article-medium-en.json");
    let long = load_preset_ok("article-long-en.json");

    assert_eq!(short.document_policy.word_count.map(prosesmasher_domain_types::Range::min), Some(500));
    assert_eq!(medium.document_policy.word_count.map(prosesmasher_domain_types::Range::min), Some(1000));
    assert_eq!(long.document_policy.word_count.map(prosesmasher_domain_types::Range::min), Some(1600));

    assert_eq!(short.document_policy.heading_counts.h3_min, Some(0));
    assert_eq!(medium.document_policy.heading_counts.h3_min, Some(1));
    assert_eq!(long.document_policy.heading_counts.h3_min, Some(2));

    assert!(short.document_policy.heading_hierarchy);
    assert!(medium.document_policy.heading_hierarchy);
    assert!(long.document_policy.heading_hierarchy);
}

#[test]
fn locale_validation_errors() {
    let short_err = load_json_err(r#"{"locale":"x","quality":{},"documentPolicy":{}}"#);
    assert!(matches!(short_err, ConfigError::ValidationFailed(_)));

    let unknown_err = load_json_err(r#"{"locale":"xx","quality":{},"documentPolicy":{}}"#);
    assert!(matches!(unknown_err, ConfigError::ValidationFailed(ref msg) if msg.contains("xx")));
}

#[test]
fn invalid_range_errors() {
    let err = load_json_err(
        r#"{
          "locale":"en",
          "quality":{},
          "documentPolicy":{"wordCount":{"min":1000,"max":500}}
        }"#,
    );
    assert!(matches!(err, ConfigError::ValidationFailed(ref msg) if msg.contains("wordCount")));
}

#[test]
fn legacy_terms_thresholds_are_rejected() {
    let err = load_json_err(
        r#"{
          "locale":"en",
          "terms":{"bannedWords":["actually"]},
          "thresholds":{"wordCount":{"min":650,"max":1000}}
        }"#,
    );
    assert!(matches!(err, ConfigError::InvalidJson(ref msg) if msg.contains("unknown field")));
}

#[test]
fn unknown_nested_fields_are_rejected() {
    let err = load_json_err(
        r#"{
          "locale":"en",
          "quality":{"heuristics":{"headingHierarchy":{"enabled":true}}},
          "documentPolicy":{}
        }"#,
    );
    assert!(matches!(err, ConfigError::InvalidJson(ref msg) if msg.contains("headingHierarchy")));
}

#[test]
fn simplicity_pairs_validate_shape() {
    let config = load_json_ok(
        r#"{
          "locale":"en",
          "quality":{"lexical":{"simplicityPairs":{"defaults":false,"add":[["utilize","use"]],"remove":[]}}},
          "documentPolicy":{}
        }"#,
    );
    let pair = config.quality.lexical.simplicity_pairs.add.first();
    assert_eq!(pair.map(|p| p.complex.as_str()), Some("utilize"));
    assert_eq!(pair.map(|p| p.simple.as_str()), Some("use"));

    let err = load_json_err(
        r#"{
          "locale":"en",
          "quality":{"lexical":{"simplicityPairs":{"defaults":false,"add":[["only_one"]],"remove":[]}}},
          "documentPolicy":{}
        }"#,
    );
    assert!(matches!(err, ConfigError::InvalidJson(_)));
}

#[test]
fn missing_locale_fails() {
    let err = load_json_err(r#"{"quality":{},"documentPolicy":{}}"#);
    assert!(matches!(err, ConfigError::InvalidJson(_)));
}
