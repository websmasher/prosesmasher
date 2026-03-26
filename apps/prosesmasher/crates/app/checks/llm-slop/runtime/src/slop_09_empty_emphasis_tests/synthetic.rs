use crate::test_helpers::{make_doc, make_doc_code_only};
use prosesmasher_app_checks_llm_slop_assertions::empty_emphasis as assertions;
use prosesmasher_domain_types::{CheckConfig, Locale};

#[test]
fn that_last_part_matters_fails() {
    let doc = make_doc("That last part matters.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_emphasis_failure(
        &doc,
        &config,
        "deictic-part-matters",
        "short deictic filler emphasis should fail",
    );
}

#[test]
fn this_part_matters_fails() {
    let doc = make_doc("This part matters.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_emphasis_failure(
        &doc,
        &config,
        "deictic-part-matters",
        "short deictic part-matters line should fail",
    );
}

#[test]
fn prefixed_part_matters_fails() {
    let doc = make_doc("And that last bit matters.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_emphasis_failure(
        &doc,
        &config,
        "deictic-part-matters",
        "leading conjunctions should not hide empty emphasis",
    );
}

#[test]
fn deictic_change_helped_fails() {
    let doc = make_doc("That one change helped a lot.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_emphasis_failure(
        &doc,
        &config,
        "deictic-change-helped",
        "deictic change-impact line should fail",
    );
}

#[test]
fn deictic_telling_you_something_fails() {
    let doc = make_doc("This is telling you something.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_emphasis_failure(
        &doc,
        &config,
        "deictic-telling-you-something",
        "deictic telling-you-something line should fail",
    );
}

#[test]
fn deictic_real_change_fails() {
    let doc = make_doc("That is still real change.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_emphasis_failure(
        &doc,
        &config,
        "deictic-real-change",
        "deictic real-change magnifier should fail",
    );
}

#[test]
fn deictic_pattern_weakens_fails() {
    let doc = make_doc("That is how the pattern weakens.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_emphasis_failure(
        &doc,
        &config,
        "deictic-pattern-weakens",
        "deictic pattern-weakens line should fail",
    );
}

#[test]
fn longer_explanatory_sentence_passes() {
    let doc = make_doc(
        "That last part matters because the contract changes the failure mode completely.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(
        &doc,
        &config,
        "explanatory sentence with actual content should pass",
    );
}

#[test]
fn concrete_body_telling_you_something_passes() {
    let doc = make_doc("Your body is telling you something important about your stress load.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_passes(
        &doc,
        &config,
        "concrete subject telling-you-something sentence should pass",
    );
}

#[test]
fn concrete_noun_matters_passes() {
    let doc = make_doc("That contract term matters.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_passes(&doc, &config, "concrete noun statements should pass");
}

#[test]
fn explanatory_real_change_passes() {
    let doc = make_doc(
        "That is still real change in how the family handles the morning transition.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(
        &doc,
        &config,
        "longer explanatory real-change sentence should pass",
    );
}

#[test]
fn technical_pattern_explanation_passes() {
    let doc = make_doc(
        "This is how the circuit weakens under repeated over-voltage conditions.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(
        &doc,
        &config,
        "technical weakening explanation should pass",
    );
}

#[test]
fn quoted_phrase_passes() {
    let doc = make_doc(
        "Editors should cut lines like \"That last part matters.\" when they add no content.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(&doc, &config, "quoted discussion should pass");
}

#[test]
fn code_block_phrase_passes() {
    let doc = make_doc_code_only("That last part matters.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_passes(&doc, &config, "code blocks should be ignored");
}

#[test]
fn non_english_is_skipped() {
    let doc = make_doc("That last part matters.", Locale::Fr);
    let config = CheckConfig::default();
    assertions::assert_skips(&doc, &config, "non-English locales should skip");
}

#[test]
fn disabled_check_skips() {
    let doc = make_doc("That last part matters.", Locale::En);
    let mut config = CheckConfig::default();
    config.quality.heuristics.empty_emphasis.enabled = false;
    assertions::assert_skips(&doc, &config, "disabled check should skip");
}

#[test]
fn check_id_and_label() {
    assertions::assert_check_metadata();
}
