use prosesmasher_app_checks_document_policy_runtime::CodeFencesCheck;
use prosesmasher_app_checks_test_support::result_helpers::assert_first_evidence_str;

crate::define_rule_assertions!(CodeFencesCheck, "code-fences", "Code Fences");

pub fn assert_code_block_failure(
    doc: &Document,
    config: &CheckConfig,
    expected_code_block: &str,
    message: &str,
) {
    let result = run(doc, config);
    assert_fail(&result, message);
    assert_first_evidence_str(
        &result,
        "code-fences",
        "code_block_text",
        expected_code_block,
    );
}
