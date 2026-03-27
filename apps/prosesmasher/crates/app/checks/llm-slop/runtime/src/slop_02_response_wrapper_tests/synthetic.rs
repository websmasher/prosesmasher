use crate::test_helpers::{make_doc, make_doc_code_only, make_doc_in_blockquote};
use prosesmasher_app_checks_llm_slop_assertions::response_wrapper as assertions;
use prosesmasher_domain_types::{CheckConfig, Locale};

#[test]
fn general_information_wrapper_detected() {
    let sentence =
        "I can provide general information about eczema, but not personalized treatment advice.";
    let doc = make_doc(sentence, Locale::En);
    let config = CheckConfig::default();
    assertions::assert_response_wrapper_failure(
        &doc,
        &config,
        "information-wrapper",
        sentence,
        "general-information wrapper should fail",
    );
}

#[test]
fn medical_advice_limitation_detected() {
    let sentence = "However, I do not provide medical advice for individual psoriasis cases.";
    let doc = make_doc(sentence, Locale::En);
    let config = CheckConfig::default();
    assertions::assert_response_wrapper_failure(
        &doc,
        &config,
        "advice-limitation",
        sentence,
        "medical-advice limitation should fail",
    );
}

#[test]
fn diagnosis_limitation_detected() {
    let sentence =
        "I don't have the ability to provide a diagnosis for bipolar disorder from a short prompt.";
    let doc = make_doc(sentence, Locale::En);
    let config = CheckConfig::default();
    assertions::assert_response_wrapper_failure(
        &doc,
        &config,
        "diagnosis-limitation",
        sentence,
        "diagnosis limitation should fail",
    );
}

#[test]
fn medical_expertise_limitation_detected() {
    let sentence = "I do not have medical expertise in pediatric neurology.";
    let doc = make_doc(sentence, Locale::En);
    let config = CheckConfig::default();
    assertions::assert_response_wrapper_failure(
        &doc,
        &config,
        "advice-limitation",
        sentence,
        "medical-expertise limitation should fail",
    );
}

#[test]
fn blockquote_wrapper_detected() {
    let sentence = "I cannot provide specific medical advice about your child's medication.";
    let doc = make_doc_in_blockquote(sentence, Locale::En);
    let config = CheckConfig::default();
    assertions::assert_response_wrapper_failure(
        &doc,
        &config,
        "advice-limitation",
        sentence,
        "blockquote wrapper should still fail",
    );
}

#[test]
fn third_person_explanation_passes() {
    let doc = make_doc(
        "Doctors can provide general information about eczema and tailor advice to the individual case.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(&doc, &config, "third-person educational prose should pass");
}

#[test]
fn first_person_non_wrapper_capability_passes() {
    let doc = make_doc(
        "I can explain how the parser reads headings from the markdown stream.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(
        &doc,
        &config,
        "first-person concrete capability without wrapper object should pass",
    );
}

#[test]
fn plain_consultation_sentence_passes() {
    let doc = make_doc(
        "Consult a qualified healthcare professional for personalized advice.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(&doc, &config, "plain consultation advice alone should pass");
}

#[test]
fn quoted_discussion_of_wrapper_passes() {
    let doc = make_doc(
        "The phrase \"I cannot provide medical advice\" is one of the laziest assistant wrappers around.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(
        &doc,
        &config,
        "quoted discussion of the wrapper should pass",
    );
}

#[test]
fn code_block_wrapper_passes() {
    let doc = make_doc_code_only(
        "I can provide general information about eczema, but not personalized treatment advice.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(&doc, &config, "code blocks should be ignored");
}

#[test]
fn non_english_is_skipped() {
    let doc = make_doc(
        "I can provide general information about eczema.",
        Locale::Fr,
    );
    let config = CheckConfig::default();
    assertions::assert_skips(&doc, &config, "non-English locales should skip");
}

#[test]
fn disabled_check_skips() {
    let doc = make_doc(
        "I can provide general information about eczema.",
        Locale::En,
    );
    let mut config = CheckConfig::default();
    config.quality.heuristics.response_wrapper.enabled = false;
    assertions::assert_skips(&doc, &config, "disabled response-wrapper should skip");
}

#[test]
fn check_id_and_label() {
    assertions::assert_check_metadata();
}
