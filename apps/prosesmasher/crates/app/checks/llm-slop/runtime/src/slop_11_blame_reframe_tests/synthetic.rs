use crate::test_helpers::{make_doc, make_doc_code_only};
use prosesmasher_app_checks_llm_slop_assertions::blame_reframe as assertions;
use prosesmasher_domain_types::{CheckConfig, Locale};

#[test]
fn development_not_malice_fails() {
    let doc = make_doc(
        "But it usually comes from development, not malice.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_blame_reframe_failure(
        &doc,
        &config,
        "source-not-blame",
        "short development-not-malice coaching line should fail",
    );
}

#[test]
fn skill_building_instead_of_shame_fails() {
    let doc = make_doc("Do it as skill-building instead of shame.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_blame_reframe_failure(
        &doc,
        &config,
        "growth-instead-of-blame",
        "skill-building instead of shame line should fail",
    );
}

#[test]
fn technical_not_malice_passes() {
    let doc = make_doc("The error comes from caching, not malice.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_passes(&doc, &config, "technical not-malice sentence should pass");
}

#[test]
fn troubleshooting_instead_of_guesswork_passes() {
    let doc = make_doc(
        "Treat it as troubleshooting instead of guesswork.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(&doc, &config, "non-moral instead-of contrast should pass");
}

#[test]
fn quoted_phrase_passes() {
    let doc = make_doc(
        "Editors should cut lines like \"Do it as skill-building instead of shame.\" when they flatten the advice.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(&doc, &config, "quoted discussion should pass");
}

#[test]
fn code_block_phrase_passes() {
    let doc = make_doc_code_only("Do it as skill-building instead of shame.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_passes(&doc, &config, "code blocks should be ignored");
}

#[test]
fn non_english_is_skipped() {
    let doc = make_doc("Do it as skill-building instead of shame.", Locale::Fr);
    let config = CheckConfig::default();
    assertions::assert_skips(&doc, &config, "non-English locales should skip");
}

#[test]
fn disabled_check_skips() {
    let doc = make_doc("Do it as skill-building instead of shame.", Locale::En);
    let mut config = CheckConfig::default();
    config.quality.heuristics.blame_reframe.enabled = false;
    assertions::assert_skips(&doc, &config, "disabled check should skip");
}

#[test]
fn check_id_and_label() {
    assertions::assert_check_metadata();
}
