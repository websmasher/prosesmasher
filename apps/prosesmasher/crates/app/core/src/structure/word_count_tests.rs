use crate::check::Check;
use crate::test_helpers::make_doc_with_word_count;
use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Locale, Range, Thresholds};

fn config_with_word_range(min: usize, max: usize) -> CheckConfig {
    CheckConfig {
        locale: Locale::En,
        thresholds: Thresholds {
            word_count: Range::new(min, max),
            ..Thresholds::default()
        },
        ..CheckConfig::default()
    }
}

#[test]
fn word_count_within_range_passes() {
    let doc = make_doc_with_word_count(800, Locale::En);
    let config = config_with_word_range(650, 1000);
    let mut suite = ExpectationSuite::new("test");
    super::WordCountCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.successful_expectations, 1, "should pass");
    assert_eq!(result.statistics.unsuccessful_expectations, 0, "no failures");
}

#[test]
fn word_count_below_min_fails() {
    let doc = make_doc_with_word_count(500, Locale::En);
    let config = config_with_word_range(650, 1000);
    let mut suite = ExpectationSuite::new("test");
    super::WordCountCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.unsuccessful_expectations, 1, "should fail — below min");
}

#[test]
fn word_count_above_max_fails() {
    let doc = make_doc_with_word_count(1500, Locale::En);
    let config = config_with_word_range(650, 1000);
    let mut suite = ExpectationSuite::new("test");
    super::WordCountCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.unsuccessful_expectations, 1, "should fail — above max");
}

#[test]
fn word_count_at_exact_min_passes() {
    let doc = make_doc_with_word_count(650, Locale::En);
    let config = config_with_word_range(650, 1000);
    let mut suite = ExpectationSuite::new("test");
    super::WordCountCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.successful_expectations, 1, "exact min should pass");
}

#[test]
fn word_count_at_exact_max_passes() {
    let doc = make_doc_with_word_count(1000, Locale::En);
    let config = config_with_word_range(650, 1000);
    let mut suite = ExpectationSuite::new("test");
    super::WordCountCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.successful_expectations, 1, "exact max should pass");
}

#[test]
fn word_count_no_threshold_skips() {
    let doc = make_doc_with_word_count(50, Locale::En);
    let config = CheckConfig::default(); // no word_count threshold
    let mut suite = ExpectationSuite::new("test");
    super::WordCountCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.evaluated_expectations, 0, "no threshold → no expectation added");
}

#[test]
fn check_id_and_label() {
    let check = super::WordCountCheck;
    assert_eq!(check.id(), "word-count", "id");
    assert_eq!(check.label(), "Word Count", "label");
    assert!(check.supported_locales().is_none(), "supports all locales");
}
