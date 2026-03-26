use crate::test_helpers::make_doc;
use prosesmasher_app_checks_persona_signals_assertions::humble_bragger as assertions;
use prosesmasher_domain_types::{CheckConfig, Locale};

fn config_with_phrases() -> CheckConfig {
    CheckConfig::default()
}

#[test]
fn humble_brag_detected() {
    let doc = make_doc(
        "In my experience working with startups, this is common.",
        Locale::En,
    );
    let config = config_with_phrases();
    assertions::assert_humble_brag_failure(
        &doc,
        &config,
        "in my experience",
        "In my experience working with startups, this is common.",
        "humble brag should fail",
    );
}

#[test]
fn normal_sentence_passes() {
    let doc = make_doc("Startups often struggle with funding.", Locale::En);
    let config = config_with_phrases();
    assertions::assert_passes(&doc, &config, "normal sentence should pass");
}

#[test]
fn default_config_runs() {
    let doc = make_doc("In my experience this is common.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_fails(&doc, &config, "default humble-bragger patterns should run");
}

#[test]
fn check_id_and_label() {
    assertions::assert_check_metadata();
}
