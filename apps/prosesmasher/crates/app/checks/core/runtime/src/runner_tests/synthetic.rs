use crate::check::Check;
use crate::runner::run_checks;
use crate::test_helpers::{make_doc, make_doc_with_word_count};
use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Locale};

struct PassCheck;

impl Check for PassCheck {
    fn id(&self) -> &'static str {
        "pass-check"
    }
    fn label(&self) -> &'static str {
        "Pass Check"
    }
    fn supported_locales(&self) -> Option<&'static [Locale]> {
        None
    }
    fn run(
        &self,
        _doc: &prosesmasher_domain_types::Document,
        _config: &CheckConfig,
        suite: &mut ExpectationSuite,
    ) {
        let _result = suite.expect_value_to_be_between("pass-check", 1, 0, 10);
    }
}

struct AnotherPassCheck;

impl Check for AnotherPassCheck {
    fn id(&self) -> &'static str {
        "another-pass-check"
    }
    fn label(&self) -> &'static str {
        "Another Pass Check"
    }
    fn supported_locales(&self) -> Option<&'static [Locale]> {
        None
    }
    fn run(
        &self,
        _doc: &prosesmasher_domain_types::Document,
        _config: &CheckConfig,
        suite: &mut ExpectationSuite,
    ) {
        let _result = suite.expect_value_to_be_between("another-pass-check", 2, 0, 10);
    }
}

struct FailCheck;

impl Check for FailCheck {
    fn id(&self) -> &'static str {
        "fail-check"
    }
    fn label(&self) -> &'static str {
        "Fail Check"
    }
    fn supported_locales(&self) -> Option<&'static [Locale]> {
        None
    }
    fn run(
        &self,
        _doc: &prosesmasher_domain_types::Document,
        _config: &CheckConfig,
        suite: &mut ExpectationSuite,
    ) {
        let _result = suite.expect_value_to_be_between("fail-check", 20, 0, 10);
    }
}

#[test]
fn runner_all_checks_pass() {
    let doc = make_doc_with_word_count(800, Locale::En);
    let config = CheckConfig {
        locale: Locale::En,
        ..CheckConfig::default()
    };
    let checks: Vec<&dyn Check> = vec![&PassCheck, &AnotherPassCheck];
    let result = run_checks(&checks, &doc, &config);
    assert_eq!(
        result.statistics.unsuccessful_expectations, 0,
        "all should pass"
    );
    assert_eq!(
        result.statistics.successful_expectations, 2,
        "exactly 2 expectations"
    );
}

#[test]
fn runner_mixed_pass_fail() {
    let doc = make_doc("hello world", Locale::En);
    let config = CheckConfig::default();
    let checks: Vec<&dyn Check> = vec![&PassCheck, &FailCheck];
    let result = run_checks(&checks, &doc, &config);
    assert_eq!(
        result.statistics.successful_expectations, 1,
        "pass-check should pass"
    );
    assert_eq!(
        result.statistics.unsuccessful_expectations, 1,
        "fail-check should fail"
    );
}

#[test]
fn runner_locale_filtering_skips_check() {
    // Create an English-only check
    struct EnglishOnlyCheck;
    impl Check for EnglishOnlyCheck {
        fn id(&self) -> &'static str {
            "english-only"
        }
        fn label(&self) -> &'static str {
            "English Only"
        }
        fn supported_locales(&self) -> Option<&'static [Locale]> {
            Some(&[Locale::En])
        }
        fn run(
            &self,
            _doc: &prosesmasher_domain_types::Document,
            _config: &CheckConfig,
            suite: &mut low_expectations::ExpectationSuite,
        ) {
            // Always adds an expectation — if skipped, suite is empty
            let _r = suite.expect_value_to_be_between("english-only", 1, 0, 10);
        }
    }

    // Russian document — English-only check should be skipped
    let doc = make_doc("Привет мир", Locale::Ru);
    let config = CheckConfig::default();
    let checks: Vec<&dyn Check> = vec![&EnglishOnlyCheck];
    let result = run_checks(&checks, &doc, &config);
    assert_eq!(
        result.statistics.evaluated_expectations, 0,
        "English-only check should be skipped for Ru doc"
    );
}

#[test]
fn runner_locale_filtering_runs_for_matching_locale() {
    struct EnglishOnlyCheck;
    impl Check for EnglishOnlyCheck {
        fn id(&self) -> &'static str {
            "english-only"
        }
        fn label(&self) -> &'static str {
            "English Only"
        }
        fn supported_locales(&self) -> Option<&'static [Locale]> {
            Some(&[Locale::En])
        }
        fn run(
            &self,
            _doc: &prosesmasher_domain_types::Document,
            _config: &CheckConfig,
            suite: &mut low_expectations::ExpectationSuite,
        ) {
            let _r = suite.expect_value_to_be_between("english-only", 1, 0, 10);
        }
    }

    let doc = make_doc("Hello world", Locale::En);
    let config = CheckConfig::default();
    let checks: Vec<&dyn Check> = vec![&EnglishOnlyCheck];
    let result = run_checks(&checks, &doc, &config);
    assert_eq!(
        result.statistics.evaluated_expectations, 1,
        "English-only check should run for En doc"
    );
}

#[test]
fn runner_none_locales_means_all() {
    let doc = make_doc_with_word_count(800, Locale::Id);
    let config = CheckConfig {
        locale: Locale::Id,
        ..CheckConfig::default()
    };
    let checks: Vec<&dyn Check> = vec![&PassCheck];
    let result = run_checks(&checks, &doc, &config);
    assert_eq!(
        result.statistics.successful_expectations, 1,
        "None locales → runs for any locale"
    );
}

#[test]
fn runner_empty_checks_produces_empty_result() {
    let doc = make_doc("Hello world", Locale::En);
    let config = CheckConfig::default();
    let checks: Vec<&dyn Check> = vec![];
    let result = run_checks(&checks, &doc, &config);
    assert_eq!(
        result.statistics.evaluated_expectations, 0,
        "no checks → empty result"
    );
}
