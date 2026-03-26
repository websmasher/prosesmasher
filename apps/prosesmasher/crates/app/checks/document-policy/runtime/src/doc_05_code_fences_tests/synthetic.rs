use crate::test_helpers::{make_doc, make_doc_code_only};
use prosesmasher_app_checks_document_policy_assertions::code_fences as assertions;
use prosesmasher_domain_types::{CheckConfig, DocumentPolicyConfig, Locale};

fn config_disallowing_code_fences() -> CheckConfig {
    CheckConfig {
        document_policy: DocumentPolicyConfig {
            allow_code_fences: false,
            ..DocumentPolicyConfig::default()
        },
        ..CheckConfig::default()
    }
}

#[test]
fn doc_with_code_block_fails() {
    let doc = make_doc_code_only("fn main() {}", Locale::En);
    let config = config_disallowing_code_fences();
    assertions::assert_code_block_failure(
        &doc,
        &config,
        "fn main() {}",
        "document with code block should fail",
    );
}

#[test]
fn doc_without_code_block_passes() {
    let doc = make_doc("This is a normal paragraph without code.", Locale::En);
    let config = config_disallowing_code_fences();
    assertions::assert_passes(&doc, &config, "document without code block should pass");
}

#[test]
fn check_id_and_label() {
    assertions::assert_check_metadata();
}
