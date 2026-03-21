use crate::check::Check;
use crate::test_helpers::make_doc;
use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Locale};

#[test]
fn dramatic_colon_detected() {
    let doc = make_doc("And then it hit me: everything changed.", Locale::En);
    let config = CheckConfig::default();
    let mut suite = ExpectationSuite::new("test");
    super::ColonDramaticCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.unsuccessful_expectations, 1,
        "short clause after colon should fail"
    );
}

#[test]
fn list_after_colon_passes() {
    let doc = make_doc(
        "There are three types: red, blue, and green.",
        Locale::En,
    );
    let config = CheckConfig::default();
    let mut suite = ExpectationSuite::new("test");
    super::ColonDramaticCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.successful_expectations, 1,
        "list after colon should pass"
    );
}

#[test]
fn no_colon_passes() {
    let doc = make_doc("A perfectly normal sentence without colons.", Locale::En);
    let config = CheckConfig::default();
    let mut suite = ExpectationSuite::new("test");
    super::ColonDramaticCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.successful_expectations, 1,
        "no colon should pass"
    );
}

#[test]
fn check_id_and_label() {
    let check = super::ColonDramaticCheck;
    assert_eq!(check.id(), "colon-dramatic");
    assert_eq!(check.label(), "Dramatic Colon");
    assert!(check.supported_locales().is_none());
}
