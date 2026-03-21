use crate::check::Check;
use crate::test_helpers::make_doc;
use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Locale};

#[test]
fn no_em_dashes_passes() {
    let doc = make_doc("Hello, world. No special dashes here.", Locale::En);
    let config = CheckConfig::default();
    let mut suite = ExpectationSuite::new("test");
    super::EmDashCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.successful_expectations, 1, "no em-dashes → pass");
}

#[test]
fn one_em_dash_fails() {
    let doc = make_doc("Hello \u{2014} world.", Locale::En);
    let config = CheckConfig::default();
    let mut suite = ExpectationSuite::new("test");
    super::EmDashCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.unsuccessful_expectations, 1, "1 em-dash → fail");
}

#[test]
fn multiple_em_dashes_fail() {
    let doc = make_doc("First \u{2014} second \u{2014} third \u{2014} end.", Locale::En);
    let config = CheckConfig::default();
    let mut suite = ExpectationSuite::new("test");
    super::EmDashCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.unsuccessful_expectations, 1, "multiple em-dashes → fail");
}

#[test]
fn en_dash_does_not_trigger() {
    // U+2013 en-dash is different from U+2014 em-dash
    let doc = make_doc("Pages 10\u{2013}20 in the book.", Locale::En);
    let config = CheckConfig::default();
    let mut suite = ExpectationSuite::new("test");
    super::EmDashCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.successful_expectations, 1, "en-dash should not trigger em-dash check");
}

#[test]
fn regular_hyphen_does_not_trigger() {
    let doc = make_doc("A well-known fact.", Locale::En);
    let config = CheckConfig::default();
    let mut suite = ExpectationSuite::new("test");
    super::EmDashCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.successful_expectations, 1, "regular hyphen should not trigger");
}

#[test]
fn check_id_and_label() {
    let check = super::EmDashCheck;
    assert_eq!(check.id(), "em-dashes", "id");
    assert_eq!(check.label(), "No Em-Dashes", "label");
    assert!(check.supported_locales().is_none(), "supports all locales");
}
