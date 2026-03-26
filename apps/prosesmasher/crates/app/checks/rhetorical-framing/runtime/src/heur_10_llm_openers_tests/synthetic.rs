use crate::test_helpers::make_doc;
use prosesmasher_app_checks_rhetorical_framing_assertions::llm_openers as assertions;
use prosesmasher_domain_types::{CheckConfig, Locale};

fn config_with_openers() -> CheckConfig {
    CheckConfig::default()
}

#[test]
fn llm_opener_detected() {
    let doc = make_doc("The interesting part is that nobody noticed.", Locale::En);
    let config = config_with_openers();
    assertions::assert_llm_opener_failure(
        &doc,
        &config,
        "the interesting part is",
        "The interesting part is that nobody noticed.",
        "LLM opener should fail",
    );
}

#[test]
fn normal_opener_passes() {
    let doc = make_doc("Nobody noticed the change at first.", Locale::En);
    let config = config_with_openers();
    assertions::assert_passes(&doc, &config, "normal opener should pass");
}

#[test]
fn default_config_runs() {
    let doc = make_doc("The interesting part is that nobody noticed.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_fails(&doc, &config, "default LLM opener patterns should run");
}

#[test]
fn check_id_and_label() {
    assertions::assert_check_metadata();
}

#[test]
fn llm_opener_in_middle_section_detected() {
    let doc = crate::test_helpers::make_doc_multi_section(
        &[
            "Nothing special here.",
            "The interesting part is that nobody noticed.",
            "Final section with normal text.",
        ],
        Locale::En,
    );
    let config = config_with_openers();
    assertions::assert_fails(&doc, &config, "LLM opener in section 2 of 3 should fail");
}
