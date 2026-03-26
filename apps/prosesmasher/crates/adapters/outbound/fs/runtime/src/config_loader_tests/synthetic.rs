use super::*;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::full_config_contents;
use prosesmasher_adapters_outbound_fs_assertions::{
    assert_config_not_found_contains, assert_config_validation_failed_contains,
    assert_heading_policy, assert_locale, assert_prohibited_terms, assert_readability_thresholds,
    assert_recommended_terms, assert_shared_quality_defaults, assert_simplicity_pair,
    assert_word_count_range, load_json_err, load_json_ok, load_preset_ok,
};
use prosesmasher_domain_types_runtime::{ConfigError, Locale, Range};
use prosesmasher_ports_outbound_traits::ConfigLoader;

fn unique_temp_path(name: &str) -> PathBuf {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!("prosesmasher-fs-config-loader-{name}-{id}"))
}

#[test]
#[allow(clippy::too_many_lines)]
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
              "sentenceCase":{"enabled":true},
              "exclamationDensity":{"maxPerParagraph":1}
            },
            "flow":{
              "wordRepetition":{"max":7,"excludedTerms":{"defaults":false,"add":["ownership"],"remove":[]}},
              "paragraphLength":{"maxSentences":5}
            },
            "readability":{
              "fleschKincaidMin":44.0
            }
          },
          "documentPolicy":{
            "wordCount":{"min":650,"max":1000},
            "headingCounts":{"h2":{"min":2,"max":6},"h3Min":1},
            "headingHierarchy":{"enabled":true},
            "codeFences":{"allowed":false},
            "boldDensity":{"min":3}
          }
        }"#,
    );

    assert_locale(&config, Locale::En, "canonical config");
    assert_prohibited_terms(
        &config,
        &["live coaching calls"],
        &["actually"],
        "canonical config",
    );
    assert_eq!(
        config.quality.lexical.required_terms,
        vec!["ownership".to_owned()]
    );
    assert_recommended_terms(
        &config,
        &["ownership", "borrowing"],
        1,
        true,
        "canonical config",
    );
    assert_simplicity_pair(&config, "utilize", "use", "canonical config");
    assert_eq!(config.quality.flow.word_repetition.max, 7);
    assert_eq!(config.quality.flow.paragraph_length.max_sentences, 5);
    assert_readability_thresholds(
        &config,
        true,
        Some(44.0),
        Some(14.0),
        Some(12.5),
        Some(25),
        "canonical config",
    );
    assert_word_count_range(&config, Some((650, 1000)), "canonical config");
    assert_heading_policy(&config, Some(1), true, "canonical config");
    assert!(config.quality.heuristics.sentence_case.enabled);
    assert!(!config.document_policy.allow_code_fences);
    assert_eq!(config.document_policy.bold_density_min, Some(3));
}

#[test]
fn readability_defaults_false_replaces_builtin_thresholds() {
    let config = load_json_ok(
        r#"{
          "locale":"en",
          "quality":{
            "readability":{
              "defaults":false,
              "fleschKincaidMin":44.0
            }
          },
          "documentPolicy":{}
        }"#,
    );

    assert_readability_thresholds(
        &config,
        false,
        Some(44.0),
        None,
        None,
        None,
        "readability defaults=false replacement",
    );
}

#[test]
fn readability_defaults_false_without_overrides_disables_thresholds() {
    let config = load_json_ok(
        r#"{
          "locale":"en",
          "quality":{"readability":{"defaults":false}},
          "documentPolicy":{}
        }"#,
    );

    assert_readability_thresholds(
        &config,
        false,
        None,
        None,
        None,
        None,
        "readability defaults=false clear",
    );
}

#[test]
fn readability_defaults_true_merges_with_builtin_thresholds() {
    let config = load_json_ok(
        r#"{
          "locale":"en",
          "quality":{
            "readability":{
              "defaults":true,
              "fleschKincaidMin":44.0
            }
          },
          "documentPolicy":{}
        }"#,
    );

    assert_readability_thresholds(
        &config,
        true,
        Some(44.0),
        Some(14.0),
        Some(12.5),
        Some(25),
        "readability defaults=true merge",
    );
}

#[test]
fn full_config_embedded_is_valid() {
    let config = parse_config_json(full_config_contents());
    assert!(config.is_ok(), "full config should parse");
}

#[test]
fn all_curated_presets_load() {
    for name in [
        "general-en",
        "article-en",
        "substack-en",
        "email-en",
        "tweet-en",
    ] {
        let config = load_preset_ok(name);
        assert_locale(&config, Locale::En, &format!("{name}: locale should be en"));
    }
}

#[test]
fn presets_keep_shared_quality_defaults() {
    let article = load_preset_ok("article-en");
    let general = load_preset_ok("general-en");

    assert_shared_quality_defaults(&article, "article preset shared defaults");
    assert_shared_quality_defaults(&general, "general preset shared defaults");
}

#[test]
fn tweet_preset_targets_shorter_copy_than_substack() {
    let tweet = load_preset_ok("tweet-en");
    let substack = load_preset_ok("substack-en");

    assert_eq!(tweet.document_policy.word_count.map(Range::max), Some(60));
    assert_eq!(
        substack.document_policy.word_count.map(Range::min),
        Some(500)
    );
}

#[test]
fn general_preset_keeps_document_policy_off() {
    let general = load_preset_ok("general-en");

    assert!(general.document_policy.word_count.is_none());
    assert!(general.document_policy.heading_counts.h2.is_none());
    assert!(!general.document_policy.heading_hierarchy);
    assert!(general.quality.heuristics.sentence_case.enabled);
}

#[test]
fn legacy_document_policy_sentence_case_still_maps_to_quality() {
    let config = load_json_ok(
        r#"{
          "locale":"en",
          "quality":{},
          "documentPolicy":{"sentenceCaseHeadings":{"enabled":false}}
        }"#,
    );

    assert!(!config.quality.heuristics.sentence_case.enabled);
}

#[test]
fn article_and_substack_use_heading_policy_but_email_and_tweet_do_not() {
    let article = load_preset_ok("article-en");
    let substack = load_preset_ok("substack-en");
    let email = load_preset_ok("email-en");
    let tweet = load_preset_ok("tweet-en");

    assert_eq!(
        article.document_policy.heading_counts.h2.map(Range::min),
        Some(3)
    );
    assert_eq!(
        substack.document_policy.heading_counts.h2.map(Range::min),
        Some(1)
    );
    assert!(article.document_policy.heading_hierarchy);
    assert!(substack.document_policy.heading_hierarchy);

    assert!(email.document_policy.heading_counts.h2.is_none());
    assert!(tweet.document_policy.heading_counts.h2.is_none());
    assert!(!email.document_policy.heading_hierarchy);
    assert!(!tweet.document_policy.heading_hierarchy);
}

#[test]
fn email_is_longer_than_tweet_but_shorter_than_article() {
    let tweet = load_preset_ok("tweet-en");
    let email = load_preset_ok("email-en");
    let article = load_preset_ok("article-en");

    assert_eq!(tweet.document_policy.word_count.map(Range::min), Some(8));
    assert_eq!(email.document_policy.word_count.map(Range::min), Some(80));
    assert_eq!(
        article.document_policy.word_count.map(Range::min),
        Some(1000)
    );
}

#[test]
fn locale_validation_errors() {
    let short_err = load_json_err(r#"{"locale":"x","quality":{},"documentPolicy":{}}"#);
    assert!(matches!(short_err, ConfigError::ValidationFailed(_)));

    let unknown_err = load_json_err(r#"{"locale":"xx","quality":{},"documentPolicy":{}}"#);
    assert_config_validation_failed_contains(&unknown_err, "xx", "unknown locale validation");
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

#[test]
fn missing_config_file_maps_to_not_found() {
    let loader = FsConfigLoader;
    let path = unique_temp_path("missing.json");
    let path_str = path.display().to_string();

    #[allow(clippy::disallowed_methods)]
    let _ = std::fs::remove_file(&path);

    let result = loader.load_config(&path);
    let err = result.expect_err("missing file should map to ConfigError::NotFound(path)");
    assert_config_not_found_contains(&err, &path_str, "missing config file");
}

#[test]
fn directory_config_path_maps_to_cannot_read_not_found() {
    let loader = FsConfigLoader;
    let path = unique_temp_path("directory");
    let path_str = path.display().to_string();

    #[allow(clippy::disallowed_methods)]
    std::fs::create_dir(&path).unwrap_or_else(|e| panic!("failed to create temp dir: {e}"));
    let result = loader.load_config(&path);
    #[allow(clippy::disallowed_methods)]
    std::fs::remove_dir(&path).unwrap_or_else(|e| panic!("failed to remove temp dir: {e}"));

    let err = result.expect_err("directory read should map to ConfigError::NotFound");
    assert_config_not_found_contains(&err, "cannot read config:", "directory config path");
    assert_config_not_found_contains(&err, &path_str, "directory config path");
}

#[cfg(unix)]
#[test]
fn unreadable_config_file_maps_to_cannot_read_not_found() {
    use std::os::unix::fs::{MetadataExt, PermissionsExt};

    let loader = FsConfigLoader;
    let path = unique_temp_path("permission-denied.json");
    let path_str = path.display().to_string();

    #[allow(clippy::disallowed_methods)]
    std::fs::write(&path, "{\"locale\":\"en\"}")
        .unwrap_or_else(|e| panic!("failed to write temp config: {e}"));

    let metadata =
        std::fs::metadata(&path).unwrap_or_else(|e| panic!("failed to stat temp config: {e}"));
    if metadata.uid() == 0 {
        #[allow(clippy::disallowed_methods)]
        std::fs::remove_file(&path).unwrap_or_else(|e| panic!("failed to clean temp config: {e}"));
        return;
    }

    let original_mode = metadata.permissions().mode();
    #[allow(clippy::disallowed_methods)]
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o000))
        .unwrap_or_else(|e| panic!("failed to chmod temp config: {e}"));

    let result = loader.load_config(&path);

    #[allow(clippy::disallowed_methods)]
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(original_mode))
        .unwrap_or_else(|e| panic!("failed to restore temp config permissions: {e}"));
    #[allow(clippy::disallowed_methods)]
    std::fs::remove_file(&path).unwrap_or_else(|e| panic!("failed to clean temp config: {e}"));

    let err = result.expect_err("unreadable config should map to ConfigError::NotFound");
    assert_config_not_found_contains(&err, "cannot read config:", "unreadable config path");
    assert_config_not_found_contains(&err, &path_str, "unreadable config path");
}
