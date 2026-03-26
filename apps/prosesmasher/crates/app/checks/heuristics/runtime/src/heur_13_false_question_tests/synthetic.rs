use crate::test_helpers::make_doc;
use prosesmasher_app_checks_heuristics_assertions::false_question as assertions;
use prosesmasher_domain_types::{CheckConfig, Locale};

fn config_with_patterns() -> CheckConfig {
    CheckConfig::default()
}

#[test]
fn false_question_detected() {
    let doc = make_doc("And isn't that what we all want?", Locale::En);
    let config = config_with_patterns();
    assertions::assert_false_question_failure(
        &doc,
        &config,
        "isn't that what we all",
        "And isn't that what we all want?",
        "false question should fail",
    );
}

#[test]
fn genuine_question_passes() {
    let doc = make_doc("So who's going to build the alternative?", Locale::En);
    let config = config_with_patterns();
    assertions::assert_passes(&doc, &config, "genuine question should pass");
}

#[test]
fn non_question_passes() {
    let doc = make_doc("That's what we all want.", Locale::En);
    let config = config_with_patterns();
    assertions::assert_passes(
        &doc,
        &config,
        "non-question should pass even with matching text",
    );
}

#[test]
fn default_config_runs() {
    let doc = make_doc("And isn't that what we all want?", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_fails(&doc, &config, "default false-question patterns should run");
}

#[test]
fn check_id_and_label() {
    assertions::assert_check_metadata();
}

#[test]
fn false_question_in_middle_section_detected() {
    let doc = crate::test_helpers::make_doc_multi_section(
        &[
            "First section with normal text.",
            "And isn't that what we all want?",
            "Final section with normal text.",
        ],
        Locale::En,
    );
    let config = config_with_patterns();
    assertions::assert_fails(
        &doc,
        &config,
        "false question in section 2 of 3 should fail",
    );
}
