use crate::test_helpers::make_doc;
use prosesmasher_app_checks_heuristics_assertions::exclamation_density as assertions;
use prosesmasher_domain_types::{CheckConfig, Locale};

fn config_with_max(max: usize) -> CheckConfig {
    let mut config = CheckConfig::default();
    config
        .quality
        .heuristics
        .exclamation_density
        .max_per_paragraph = max;
    config
}

#[test]
fn within_threshold_passes() {
    let doc = make_doc("Great! This is fine.", Locale::En);
    let config = config_with_max(2);
    assertions::assert_passes(&doc, &config, "1 exclamation within max 2 should pass");
}

#[test]
fn exceeds_threshold_fails() {
    let doc = make_doc("Wow! Amazing! Incredible!", Locale::En);
    let config = config_with_max(1);
    assertions::assert_exclamation_failure(
        &doc,
        &config,
        "Wow! Amazing! Incredible!",
        3,
        "3 exclamations with max 1 should fail",
    );
}

#[test]
fn default_config_runs() {
    let doc = make_doc("Wow! Amazing! Incredible!", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_fails(&doc, &config, "default threshold should run");
}

#[test]
fn check_id_and_label() {
    assertions::assert_check_metadata();
}
