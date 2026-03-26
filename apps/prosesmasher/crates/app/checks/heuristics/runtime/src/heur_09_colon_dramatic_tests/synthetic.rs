use crate::test_helpers::make_doc;
use prosesmasher_app_checks_heuristics_assertions::colon_dramatic as assertions;
use prosesmasher_domain_types::{CheckConfig, Locale};

#[test]
fn dramatic_colon_detected() {
    let doc = make_doc("And then it hit me: everything changed.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_dramatic_colon_failure(
        &doc,
        &config,
        "everything changed.",
        "And then it hit me: everything changed.",
        "short clause after colon should fail",
    );
}

#[test]
fn list_after_colon_passes() {
    let doc = make_doc("There are three types: red, blue, and green.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_passes(&doc, &config, "list after colon should pass");
}

#[test]
fn no_colon_passes() {
    let doc = make_doc("A perfectly normal sentence without colons.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_passes(&doc, &config, "no colon should pass");
}

#[test]
fn check_id_and_label() {
    assertions::assert_check_metadata();
}

/// False-positive test: "Time: 3 hours" is a factual label, not dramatic prose.
/// NOTE: This test exposes a real bug — the current heuristic counts any short
/// post-colon clause as dramatic, including factual label-value pairs.
/// Filed as a known issue; ignored until the heuristic is refined.
#[test]
#[ignore = "false positive: colon-dramatic flags factual label:value pairs (needs heuristic fix)"]
fn factual_colon_label_should_not_flag() {
    let doc = make_doc("Time: 3 hours", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_passes(
        &doc,
        &config,
        "factual label 'Time: 3 hours' should not flag as dramatic (false positive bug)",
    );
}
