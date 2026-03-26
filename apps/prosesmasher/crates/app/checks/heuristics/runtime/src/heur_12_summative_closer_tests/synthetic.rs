use crate::test_helpers::make_doc;
use prosesmasher_app_checks_heuristics_assertions::summative_closer as assertions;
use prosesmasher_domain_types::{CheckConfig, Locale};

fn config_with_patterns() -> CheckConfig {
    CheckConfig::default()
}

#[test]
fn summative_closer_detected() {
    let doc = make_doc(
        "And that's what makes this approach so powerful.",
        Locale::En,
    );
    let config = config_with_patterns();
    assertions::assert_summative_closer_failure(
        &doc,
        &config,
        "and that's what makes",
        "And that's what makes this approach so powerful.",
        "summative closer should fail",
    );
}

#[test]
fn normal_closer_passes() {
    let doc = make_doc("The data backs this up.", Locale::En);
    let config = config_with_patterns();
    assertions::assert_passes(&doc, &config, "normal closer should pass");
}

#[test]
fn default_config_runs() {
    let doc = make_doc(
        "And that's what makes this approach so powerful.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_fails(&doc, &config, "default summative patterns should run");
}

#[test]
fn check_id_and_label() {
    assertions::assert_check_metadata();
}

#[test]
fn summative_closer_in_middle_section_detected() {
    let doc = crate::test_helpers::make_doc_multi_section(
        &[
            "First section with normal text.",
            "And that's what makes this approach so powerful.",
            "Final section with normal text.",
        ],
        Locale::En,
    );
    let config = config_with_patterns();
    assertions::assert_fails(
        &doc,
        &config,
        "summative closer in section 2 of 3 should fail",
    );
}
