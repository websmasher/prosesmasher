use crate::test_helpers::make_doc;
use prosesmasher_app_checks_style_signals_assertions::smart_quotes as assertions;
use prosesmasher_domain_types::{CheckConfig, Locale};

#[test]
fn straight_quotes_pass() {
    let doc = make_doc("He said \"hello\" and it's fine.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_passes(&doc, &config, "straight quotes should pass");
}

#[test]
fn curly_double_quotes_fail() {
    let doc = make_doc("He said \u{201C}hello\u{201D} loudly.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_quote_failure(
        &doc,
        &config,
        "\u{201C}\u{201D}",
        "He said \u{201C}hello\u{201D} loudly.",
        "curly double quotes should fail",
    );
}

#[test]
fn curly_single_quotes_fail() {
    let doc = make_doc("It\u{2019}s a \u{2018}test\u{2019} indeed.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_fails(&doc, &config, "curly single quotes should fail");
}

#[test]
fn check_id_and_label() {
    assertions::assert_check_metadata();
}

#[test]
fn em_dash_and_en_dash_not_flagged() {
    let doc = make_doc(
        "The result \u{2014} as expected \u{2013} was positive.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(
        &doc,
        &config,
        "em-dash U+2014 and en-dash U+2013 should not trigger smart quotes check",
    );
}
