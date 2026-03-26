use crate::test_helpers::make_doc;
use prosesmasher_app_checks_style_signals_assertions::fake_timestamps as assertions;
use prosesmasher_domain_types::{CheckConfig, Locale};

#[test]
fn timestamp_detected() {
    let doc = make_doc("At 5:47 PM I realized everything changed.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_timestamp_failure(
        &doc,
        &config,
        "5:47 PM",
        "At 5:47 PM I realized everything changed.",
        "fake timestamp should fail",
    );
}

#[test]
fn no_timestamp_passes() {
    let doc = make_doc("Later that evening I realized.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_passes(&doc, &config, "no timestamp should pass");
}

#[test]
fn am_timestamp_detected() {
    let doc = make_doc("It was 2:30 AM when the alarm rang.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_fails(&doc, &config, "AM timestamp should fail");
}

#[test]
fn check_id_and_label() {
    assertions::assert_check_metadata();
}

#[test]
fn twenty_four_hour_format_not_flagged() {
    let doc = make_doc("The meeting starts at 14:30 in the main room.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_passes(&doc, &config, "24-hour time without AM/PM should pass");
}
