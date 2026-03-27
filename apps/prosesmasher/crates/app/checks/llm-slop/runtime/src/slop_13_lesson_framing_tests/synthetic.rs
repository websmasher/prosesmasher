use crate::test_helpers::{make_doc, make_doc_code_only};
use prosesmasher_app_checks_llm_slop_assertions::lesson_framing as assertions;
use prosesmasher_domain_types::{CheckConfig, Locale};

#[test]
fn biggest_lesson_summary_fails() {
    let doc = make_doc("The biggest lesson was simple.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_lesson_failure(
        &doc,
        &config,
        "lesson-summary",
        "the biggest lesson was simple",
        "empty biggest-lesson framing should fail",
    );
}

#[test]
fn practical_lesson_summary_fails() {
    let doc = make_doc("The practical lesson for me was simple:", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_lesson_failure(
        &doc,
        &config,
        "lesson-summary",
        "the practical lesson for me was simple",
        "personal lesson-summary wrapper should fail",
    );
}

#[test]
fn boring_fix_wrapper_fails() {
    let doc = make_doc("The fix is boring, which is why it works.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_lesson_failure(
        &doc,
        &config,
        "fix-wrapper",
        "boring",
        "boring fix wrappers should fail",
    );
}

#[test]
fn plain_fix_wrapper_fails() {
    let doc = make_doc(
        "The fix is plain, and it saves a ridiculous amount of time.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_lesson_failure(
        &doc,
        &config,
        "fix-wrapper",
        "plain",
        "plain fix wrappers should fail",
    );
}

#[test]
fn technical_fix_instruction_passes() {
    let doc = make_doc(
        "The fix is to initialize the parser before reading the file.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(&doc, &config, "concrete technical fixes should pass");
}

#[test]
fn concrete_study_lesson_passes() {
    let doc = make_doc(
        "One practical lesson from the study was that shorter breaks improved recall.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(&doc, &config, "concrete study lessons should pass");
}

#[test]
fn quoted_lesson_phrase_passes() {
    let doc = make_doc(
        "Editors should cut lines like \"the biggest lesson was simple\" when they add no value.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(&doc, &config, "quoted discussion should pass");
}

#[test]
fn code_block_lesson_phrase_passes() {
    let doc = make_doc_code_only("The biggest lesson was simple.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_passes(&doc, &config, "code blocks should be ignored");
}

#[test]
fn non_english_is_skipped() {
    let doc = make_doc("The biggest lesson was simple.", Locale::Fr);
    let config = CheckConfig::default();
    assertions::assert_skips(&doc, &config, "non-English locales should skip");
}

#[test]
fn disabled_check_skips() {
    let doc = make_doc("The biggest lesson was simple.", Locale::En);
    let mut config = CheckConfig::default();
    config.quality.heuristics.lesson_framing.enabled = false;
    assertions::assert_skips(&doc, &config, "disabled lesson-framing should skip");
}

#[test]
fn check_id_and_label() {
    assertions::assert_check_metadata();
}
