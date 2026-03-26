use prosesmasher_adapters_outbound_fs_runtime::{
    FsConfigLoader, config_loader::parse_config_json, preset_contents,
};
use prosesmasher_domain_types_runtime::{CheckConfig, ConfigError, Locale};
use prosesmasher_ports_outbound_traits::ConfigLoader;

use crate::support::{cleanup, write_temp};

#[allow(clippy::panic)]
#[must_use]
pub fn load_json_ok(json: &str) -> CheckConfig {
    let path = write_temp("json", json);
    let result = FsConfigLoader.load_config(&path);
    cleanup(&path);
    match result {
        Ok(config) => config,
        Err(err) => panic!("expected Ok, got Err: {err:?}"),
    }
}

#[allow(clippy::panic)]
#[must_use]
pub fn load_json_err(json: &str) -> ConfigError {
    let path = write_temp("json", json);
    let result = FsConfigLoader.load_config(&path);
    cleanup(&path);
    match result {
        Ok(_) => panic!("expected Err, got Ok"),
        Err(err) => err,
    }
}

#[allow(clippy::panic)]
#[must_use]
pub fn load_preset_ok(name: &str) -> CheckConfig {
    let json = preset_contents(name).unwrap_or_else(|| panic!("missing preset {name}"));
    parse_config_json(json).unwrap_or_else(|err| panic!("failed to load preset {name}: {err}"))
}

pub fn assert_locale(config: &CheckConfig, expected: Locale, context: &str) {
    assert_eq!(config.locale, expected, "{context}: locale");
}

pub fn assert_readability_thresholds(
    config: &CheckConfig,
    expected_defaults: bool,
    flesch_kincaid_min: Option<f64>,
    gunning_fog_max: Option<f64>,
    coleman_liau_max: Option<f64>,
    avg_sentence_length_max: Option<usize>,
    context: &str,
) {
    assert_eq!(
        config.quality.readability.defaults, expected_defaults,
        "{context}: readability defaults"
    );
    assert_eq!(
        config.quality.readability.flesch_kincaid_min, flesch_kincaid_min,
        "{context}: flesch_kincaid_min"
    );
    assert_eq!(
        config.quality.readability.gunning_fog_max, gunning_fog_max,
        "{context}: gunning_fog_max"
    );
    assert_eq!(
        config.quality.readability.coleman_liau_max, coleman_liau_max,
        "{context}: coleman_liau_max"
    );
    assert_eq!(
        config.quality.readability.avg_sentence_length_max, avg_sentence_length_max,
        "{context}: avg_sentence_length_max"
    );
}

pub fn assert_word_count_range(
    config: &CheckConfig,
    expected: Option<(usize, usize)>,
    context: &str,
) {
    let actual = config
        .document_policy
        .word_count
        .map(|range| (range.min(), range.max()));
    assert_eq!(actual, expected, "{context}: word_count");
}

pub fn assert_heading_policy(
    config: &CheckConfig,
    expected_h3_min: Option<usize>,
    expected_heading_hierarchy: bool,
    context: &str,
) {
    assert_eq!(
        config.document_policy.heading_counts.h3_min, expected_h3_min,
        "{context}: heading_counts.h3_min"
    );
    assert_eq!(
        config.document_policy.heading_hierarchy, expected_heading_hierarchy,
        "{context}: heading_hierarchy"
    );
}

pub fn assert_prohibited_terms(
    config: &CheckConfig,
    expected_add_contains: &[&str],
    expected_remove_contains: &[&str],
    context: &str,
) {
    for expected in expected_add_contains {
        assert!(
            config
                .quality
                .lexical
                .prohibited_terms
                .add
                .iter()
                .any(|term| term == expected),
            "{context}: missing prohibited term add `{expected}`"
        );
    }
    for expected in expected_remove_contains {
        assert!(
            config
                .quality
                .lexical
                .prohibited_terms
                .remove
                .iter()
                .any(|term| term == expected),
            "{context}: missing prohibited term remove `{expected}`"
        );
    }
}

pub fn assert_recommended_terms(
    config: &CheckConfig,
    expected_terms: &[&str],
    expected_min_count: usize,
    expected_allow_inflections: bool,
    context: &str,
) {
    let pool = config
        .quality
        .lexical
        .recommended_terms
        .as_ref()
        .unwrap_or_else(|| panic!("{context}: expected recommended terms pool"));
    assert_eq!(pool.min_count, expected_min_count, "{context}: min_count");
    assert_eq!(
        pool.allow_inflections, expected_allow_inflections,
        "{context}: allow_inflections"
    );
    for expected in expected_terms {
        assert!(
            pool.terms.iter().any(|term| term == expected),
            "{context}: missing recommended term `{expected}`"
        );
    }
}

pub fn assert_simplicity_pair(
    config: &CheckConfig,
    expected_complex: &str,
    expected_simple: &str,
    context: &str,
) {
    assert!(
        config
            .quality
            .lexical
            .simplicity_pairs
            .add
            .iter()
            .any(|pair| pair.complex == expected_complex && pair.simple == expected_simple),
        "{context}: missing simplicity pair {expected_complex}->{expected_simple}"
    );
}

pub fn assert_shared_quality_defaults(config: &CheckConfig, context: &str) {
    assert!(
        config.quality.heuristics.llm_disclaimer.enabled,
        "{context}: llm disclaimer enabled by default"
    );
    assert!(
        config.quality.heuristics.response_wrapper.enabled,
        "{context}: response wrapper enabled by default"
    );
    assert_eq!(
        config
            .quality
            .heuristics
            .exclamation_density
            .max_per_paragraph,
        1,
        "{context}: exclamation density default"
    );
    assert_eq!(
        config.quality.flow.paragraph_length.max_sentences, 6,
        "{context}: paragraph length default"
    );
}

pub fn assert_config_not_found_contains(err: &ConfigError, expected: &str, context: &str) {
    match err {
        ConfigError::NotFound(message) => {
            assert!(
                message.contains(expected),
                "{context}: expected `{expected}` in `{message}`"
            );
        }
        other => panic!("{context}: expected ConfigError::NotFound, got {other:?}"),
    }
}

pub fn assert_config_validation_failed_contains(err: &ConfigError, expected: &str, context: &str) {
    match err {
        ConfigError::ValidationFailed(message) => {
            assert!(
                message.contains(expected),
                "{context}: expected `{expected}` in `{message}`"
            );
        }
        other => panic!("{context}: expected ConfigError::ValidationFailed, got {other:?}"),
    }
}
