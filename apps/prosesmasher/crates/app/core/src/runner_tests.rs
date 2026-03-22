use crate::check::Check;
use crate::runner::run_checks;
use crate::test_helpers::{make_doc, make_doc_with_word_count};
use prosesmasher_domain_types::{CheckConfig, Locale, Range};

use crate::patterns::EmDashCheck;
use crate::structure::WordCountCheck;
use crate::terms::ProhibitedTermsCheck;

#[test]
fn runner_all_checks_pass() {
    let doc = make_doc_with_word_count(800, Locale::En);
    let config = CheckConfig {
        locale: Locale::En,
        ..CheckConfig::default()
    };
    let mut config = config;
    config.document_policy.word_count = Range::new(650, 1000);
    let checks: Vec<&dyn Check> = vec![&WordCountCheck, &EmDashCheck];
    let result = run_checks(&checks, &doc, &config);
    assert_eq!(result.statistics.unsuccessful_expectations, 0, "all should pass");
    assert_eq!(result.statistics.successful_expectations, 2, "exactly 2 expectations (word-count + em-dashes)");
}

#[test]
fn runner_mixed_pass_fail() {
    // Em-dashes passes, but prohibited terms fails.
    let doc = make_doc("we actually need this word0 word1 word2", Locale::En);
    let config = CheckConfig::default();
    let checks: Vec<&dyn Check> = vec![&EmDashCheck, &ProhibitedTermsCheck];
    let result = run_checks(&checks, &doc, &config);
    assert_eq!(result.statistics.successful_expectations, 1, "em-dash should pass");
    assert_eq!(result.statistics.unsuccessful_expectations, 1, "prohibited terms should fail");
}

#[test]
fn runner_locale_filtering_skips_check() {
    // Create an English-only check
    struct EnglishOnlyCheck;
    impl Check for EnglishOnlyCheck {
        fn id(&self) -> &'static str { "english-only" }
        fn label(&self) -> &'static str { "English Only" }
        fn supported_locales(&self) -> Option<&'static [Locale]> {
            Some(&[Locale::En])
        }
        fn run(&self, _doc: &prosesmasher_domain_types::Document, _config: &CheckConfig, suite: &mut low_expectations::ExpectationSuite) {
            // Always adds an expectation — if skipped, suite is empty
            let _r = suite.expect_value_to_be_between("english-only", 1, 0, 10);
        }
    }

    // Russian document — English-only check should be skipped
    let doc = make_doc("Привет мир", Locale::Ru);
    let config = CheckConfig::default();
    let checks: Vec<&dyn Check> = vec![&EnglishOnlyCheck];
    let result = run_checks(&checks, &doc, &config);
    assert_eq!(result.statistics.evaluated_expectations, 0, "English-only check should be skipped for Ru doc");
}

#[test]
fn runner_locale_filtering_runs_for_matching_locale() {
    struct EnglishOnlyCheck;
    impl Check for EnglishOnlyCheck {
        fn id(&self) -> &'static str { "english-only" }
        fn label(&self) -> &'static str { "English Only" }
        fn supported_locales(&self) -> Option<&'static [Locale]> {
            Some(&[Locale::En])
        }
        fn run(&self, _doc: &prosesmasher_domain_types::Document, _config: &CheckConfig, suite: &mut low_expectations::ExpectationSuite) {
            let _r = suite.expect_value_to_be_between("english-only", 1, 0, 10);
        }
    }

    let doc = make_doc("Hello world", Locale::En);
    let config = CheckConfig::default();
    let checks: Vec<&dyn Check> = vec![&EnglishOnlyCheck];
    let result = run_checks(&checks, &doc, &config);
    assert_eq!(result.statistics.evaluated_expectations, 1, "English-only check should run for En doc");
}

#[test]
fn runner_none_locales_means_all() {
    // WordCountCheck supports all locales (returns None)
    let doc = make_doc_with_word_count(800, Locale::Id);
    let config = CheckConfig {
        locale: Locale::Id,
        ..CheckConfig::default()
    };
    let mut config = config;
    config.document_policy.word_count = Range::new(650, 1000);
    let checks: Vec<&dyn Check> = vec![&WordCountCheck];
    let result = run_checks(&checks, &doc, &config);
    assert_eq!(result.statistics.successful_expectations, 1, "None locales → runs for any locale");
}

#[test]
fn runner_empty_checks_produces_empty_result() {
    let doc = make_doc("Hello world", Locale::En);
    let config = CheckConfig::default();
    let checks: Vec<&dyn Check> = vec![];
    let result = run_checks(&checks, &doc, &config);
    assert_eq!(result.statistics.evaluated_expectations, 0, "no checks → empty result");
}
