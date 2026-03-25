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
    assert_eq!(
        result.statistics.successful_expectations, 1,
        "straight quotes should pass"
    );
}

#[test]
fn curly_double_quotes_fail() {
    let doc = make_doc("He said \u{201C}hello\u{201D} loudly.", Locale::En);
    let config = CheckConfig::default();
    let mut suite = ExpectationSuite::new("test");
    super::SmartQuotesCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.unsuccessful_expectations, 1,
        "curly double quotes should fail"
    );
    let vr = result.results.get("smart-quotes");
    assert!(vr.is_some(), "smart-quotes result should exist");
    if let Some(vr) = vr {
        let evidence = vr.result.partial_unexpected_list.as_ref();
        assert!(evidence.is_some(), "evidence should be present");
        assert_eq!(
            evidence
                .and_then(|e| e.first())
                .and_then(|item| item.get("matched_text"))
                .and_then(serde_json::Value::as_str),
            Some("\u{201C}\u{201D}"),
            "matched quotes"
        );
        assert_eq!(
            evidence
                .and_then(|e| e.first())
                .and_then(|item| item.get("sentence"))
                .and_then(serde_json::Value::as_str),
            Some("He said \u{201C}hello\u{201D} loudly."),
            "sentence evidence"
        );
    }
}

#[test]
fn curly_single_quotes_fail() {
    let doc = make_doc("It\u{2019}s a \u{2018}test\u{2019} indeed.", Locale::En);
    let config = CheckConfig::default();
    let mut suite = ExpectationSuite::new("test");
    super::SmartQuotesCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.unsuccessful_expectations, 1,
        "curly single quotes should fail"
    );
}

#[test]
fn check_id_and_label() {
    let check = super::SmartQuotesCheck;
    assert_eq!(check.id(), "smart-quotes");
    assert_eq!(check.label(), "No Smart Quotes");
    assert!(check.supported_locales().is_none(), "supports all locales");
}

#[test]
fn em_dash_and_en_dash_not_flagged() {
    let doc = make_doc(
        "The result \u{2014} as expected \u{2013} was positive.",
        Locale::En,
    );
    let config = CheckConfig::default();
    let mut suite = ExpectationSuite::new("test");
    super::SmartQuotesCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.successful_expectations, 1,
        "em-dash U+2014 and en-dash U+2013 should not trigger smart quotes check"
    );
}
