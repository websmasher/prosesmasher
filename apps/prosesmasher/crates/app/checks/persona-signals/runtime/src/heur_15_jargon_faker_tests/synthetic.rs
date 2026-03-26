use crate::test_helpers::make_doc;
use prosesmasher_app_checks_persona_signals_assertions::jargon_faker as assertions;
use prosesmasher_domain_types::{CheckConfig, Locale};

fn config_with_phrases() -> CheckConfig {
    CheckConfig::default()
}

#[test]
fn jargon_detected() {
    let doc = make_doc("We need to debug our morning routine.", Locale::En);
    let config = config_with_phrases();
    assertions::assert_jargon_failure(
        &doc,
        &config,
        "debug our",
        "We need to debug our morning routine.",
        "fake jargon should fail",
    );
}

#[test]
fn normal_sentence_passes() {
    let doc = make_doc("We need to fix our morning routine.", Locale::En);
    let config = config_with_phrases();
    assertions::assert_passes(&doc, &config, "normal sentence should pass");
}

#[test]
fn default_config_runs() {
    let doc = make_doc("We need to debug our morning routine.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_fails(&doc, &config, "default jargon-faker patterns should run");
}

#[test]
fn check_id_and_label() {
    assertions::assert_check_metadata();
}
