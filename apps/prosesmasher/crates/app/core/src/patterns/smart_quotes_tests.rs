use crate::check::Check;
use crate::test_helpers::make_doc;
use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Locale};

#[test]
fn straight_quotes_pass() {
    let doc = make_doc("He said \"hello\" and it's fine.", Locale::En);
    let config = CheckConfig::default();
    let mut suite = ExpectationSuite::new("test");
    super::SmartQuotesCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.successful_expectations, 1, "straight quotes should pass");
}

#[test]
fn curly_double_quotes_fail() {
    let doc = make_doc("He said \u{201C}hello\u{201D} loudly.", Locale::En);
    let config = CheckConfig::default();
    let mut suite = ExpectationSuite::new("test");
    super::SmartQuotesCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.unsuccessful_expectations, 1, "curly double quotes should fail");
}

#[test]
fn curly_single_quotes_fail() {
    let doc = make_doc("It\u{2019}s a \u{2018}test\u{2019} indeed.", Locale::En);
    let config = CheckConfig::default();
    let mut suite = ExpectationSuite::new("test");
    super::SmartQuotesCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.unsuccessful_expectations, 1, "curly single quotes should fail");
}

#[test]
fn check_id_and_label() {
    let check = super::SmartQuotesCheck;
    assert_eq!(check.id(), "smart-quotes");
    assert_eq!(check.label(), "No Smart Quotes");
    assert!(check.supported_locales().is_none(), "supports all locales");
}
