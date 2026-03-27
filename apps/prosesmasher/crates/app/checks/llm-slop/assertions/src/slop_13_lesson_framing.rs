use prosesmasher_app_checks_llm_slop_runtime::LessonFramingCheck;
use prosesmasher_app_checks_test_support::result_helpers::assert_first_evidence_str;
use prosesmasher_domain_types::Locale;

crate::define_rule_assertions!(
    LessonFramingCheck,
    "lesson-framing",
    "Lesson Framing",
    Some(&[Locale::En])
);

pub fn assert_lesson_failure(
    doc: &Document,
    config: &CheckConfig,
    expected_pattern_kind: &str,
    expected_match: &str,
    message: &str,
) {
    let result = run(doc, config);
    assert_fail(&result, message);
    assert_first_evidence_str(
        &result,
        "lesson-framing",
        "pattern_kind",
        expected_pattern_kind,
    );
    assert_first_evidence_str(&result, "lesson-framing", "matched_text", expected_match);
}
