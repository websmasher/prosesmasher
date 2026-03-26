use crate::test_helpers::{make_doc, make_doc_code_only, make_doc_in_blockquote};
use prosesmasher_app_checks_llm_slop_assertions::llm_disclaimer as assertions;
use prosesmasher_domain_types::{CheckConfig, Locale};

#[test]
fn language_model_disclaimer_detected() {
    let sentence = "As a language model, I can provide general information about eczema.";
    let doc = make_doc(sentence, Locale::En);
    let config = CheckConfig::default();
    assertions::assert_disclaimer_failure(
        &doc,
        &config,
        "as a language model",
        sentence,
        "plain language-model disclaimer should fail",
    );
}

#[test]
fn knowledge_cutoff_disclaimer_detected() {
    let sentence = "As of my knowledge cutoff date, I cannot confirm the latest cancer statistics.";
    let doc = make_doc(sentence, Locale::En);
    let config = CheckConfig::default();
    assertions::assert_disclaimer_failure(
        &doc,
        &config,
        "as of my knowledge cutoff",
        sentence,
        "knowledge cutoff disclaimer should fail",
    );
}

#[test]
fn real_time_access_disclaimer_detected_after_prefix() {
    let sentence =
        "However, I do not have access to real-time information about current health status.";
    let doc = make_doc(sentence, Locale::En);
    let config = CheckConfig::default();
    assertions::assert_disclaimer_failure(
        &doc,
        &config,
        "i do not have access to real-time",
        sentence,
        "real-time access disclaimer should fail after a discourse prefix",
    );
}

#[test]
fn disclaimer_inside_blockquote_detected() {
    let sentence = "As I am an AI language model, I do not provide medical advice.";
    let doc = make_doc_in_blockquote(sentence, Locale::En);
    let config = CheckConfig::default();
    assertions::assert_disclaimer_failure(
        &doc,
        &config,
        "as i am an ai language model",
        sentence,
        "blockquote disclaimer should still fail",
    );
}

#[test]
fn quoted_discussion_of_phrase_passes() {
    let doc = make_doc(
        "The phrase \"as a language model\" is a dead giveaway in bad AI copy.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(
        &doc,
        &config,
        "mentioning the disclaimer phrase should not itself fail",
    );
}

#[test]
fn code_block_disclaimer_passes() {
    let doc = make_doc_code_only(
        "As a language model, I cannot provide real-time information.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(&doc, &config, "code blocks should be ignored");
}

#[test]
fn non_english_is_skipped() {
    let doc = make_doc(
        "As a language model, I can provide general information.",
        Locale::Fr,
    );
    let config = CheckConfig::default();
    assertions::assert_skips(&doc, &config, "non-English locales should skip");
}

#[test]
fn disabled_check_skips() {
    let doc = make_doc(
        "As a language model, I can provide general information.",
        Locale::En,
    );
    let mut config = CheckConfig::default();
    config.quality.heuristics.llm_disclaimer.enabled = false;
    assertions::assert_skips(&doc, &config, "disabled llm-disclaimer should skip");
}

#[test]
fn check_id_and_label() {
    assertions::assert_check_metadata();
}
