use crate::test_helpers::make_doc_with_word_count;
use prosesmasher_app_checks_document_policy_assertions::word_count as assertions;
use prosesmasher_domain_types::{CheckConfig, Locale, Range};

fn config_with_word_range(min: usize, max: usize) -> CheckConfig {
    let mut config = CheckConfig {
        locale: Locale::En,
        ..CheckConfig::default()
    };
    config.document_policy.word_count = Range::new(min, max);
    config
}

#[test]
fn word_count_within_range_passes() {
    let doc = make_doc_with_word_count(800, Locale::En);
    let config = config_with_word_range(650, 1000);
    assertions::assert_observed_word_count(&doc, &config, 800, "should pass");
}

#[test]
fn word_count_below_min_fails() {
    let doc = make_doc_with_word_count(500, Locale::En);
    let config = config_with_word_range(650, 1000);
    assertions::assert_range_failure(&doc, &config, 500, 650, 1000, "should fail — below min");
}

#[test]
fn word_count_above_max_fails() {
    let doc = make_doc_with_word_count(1500, Locale::En);
    let config = config_with_word_range(650, 1000);
    assertions::assert_range_failure(&doc, &config, 1500, 650, 1000, "should fail — above max");
}

#[test]
fn word_count_at_exact_min_passes() {
    let doc = make_doc_with_word_count(650, Locale::En);
    let config = config_with_word_range(650, 1000);
    assertions::assert_observed_word_count(&doc, &config, 650, "exact min should pass");
}

#[test]
fn word_count_at_exact_max_passes() {
    let doc = make_doc_with_word_count(1000, Locale::En);
    let config = config_with_word_range(650, 1000);
    assertions::assert_observed_word_count(&doc, &config, 1000, "exact max should pass");
}

#[test]
fn word_count_no_threshold_skips() {
    let doc = make_doc_with_word_count(50, Locale::En);
    let config = CheckConfig::default(); // no word_count threshold
    assertions::assert_skips(&doc, &config, "no threshold → no expectation added");
}

#[test]
fn check_id_and_label() {
    assertions::assert_check_metadata();
}
