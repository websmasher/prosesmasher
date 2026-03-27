use crate::test_helpers::{make_doc, make_doc_code_only};
use prosesmasher_app_checks_llm_slop_assertions::observer_guidance as assertions;
use prosesmasher_domain_types::{CheckConfig, Locale};

#[test]
fn observer_prompt_fails() {
    let doc = make_doc("You see it everywhere:", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_observer_failure(
        &doc,
        &config,
        "observer-frame",
        "you see it everywhere",
        "short observer prompts should fail",
    );
}

#[test]
fn real_time_watch_prompt_fails() {
    let doc = make_doc("You can watch it happen in real time.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_observer_failure(
        &doc,
        &config,
        "observer-frame",
        "you can watch it happen in real time",
        "standalone real-time watch prompts should fail",
    );
}

#[test]
fn reader_address_fails() {
    let doc = make_doc(
        "If this hits home, start smaller than your pride wants.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_observer_failure(
        &doc,
        &config,
        "reader-address",
        "if this hits home",
        "reader-address prompts should fail",
    );
}

#[test]
fn stuck_frame_fails() {
    let doc = make_doc("This is where people get stuck.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_observer_failure(
        &doc,
        &config,
        "stuck-frame",
        "this is where people get stuck",
        "people-get-stuck guidance should fail",
    );
}

#[test]
fn abstract_where_bridge_fails() {
    let doc = make_doc("That is where the confusion slips in.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_observer_failure(
        &doc,
        &config,
        "where-bridge",
        "that is where the confusion slips in",
        "abstract where-bridge framing should fail",
    );
}

#[test]
fn concrete_dashboard_observation_passes() {
    let doc = make_doc(
        "You can watch it happen in real time on the dashboard.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(&doc, &config, "concrete dashboard observations should pass");
}

#[test]
fn literal_where_location_passes() {
    let doc = make_doc(
        "That is where the parser writes the cache file.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(&doc, &config, "literal location statements should pass");
}

#[test]
fn quoted_observer_prompt_passes() {
    let doc = make_doc(
        "Editors should cut lines like \"you see it everywhere\" when they add no precision.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(&doc, &config, "quoted observer guidance should pass");
}

#[test]
fn code_block_observer_prompt_passes() {
    let doc = make_doc_code_only("You see it everywhere:", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_passes(&doc, &config, "code blocks should be ignored");
}

#[test]
fn non_english_is_skipped() {
    let doc = make_doc("You see it everywhere:", Locale::Fr);
    let config = CheckConfig::default();
    assertions::assert_skips(&doc, &config, "non-English locales should skip");
}

#[test]
fn disabled_check_skips() {
    let doc = make_doc("You see it everywhere:", Locale::En);
    let mut config = CheckConfig::default();
    config.quality.heuristics.observer_guidance.enabled = false;
    assertions::assert_skips(&doc, &config, "disabled observer-guidance should skip");
}

#[test]
fn check_id_and_label() {
    assertions::assert_check_metadata();
}
