use prosesmasher_app_checks_document_policy_assertions::bold_density as assertions;
use prosesmasher_domain_types::{CheckConfig, Document, DocumentMetadata, Locale, Section};

fn doc_with_bold_count(bold_count: usize) -> Document {
    Document {
        locale: Locale::En,
        sections: vec![Section {
            heading: None,
            blocks: vec![],
        }],
        metadata: DocumentMetadata {
            bold_count,
            ..DocumentMetadata::default()
        },
    }
}

#[test]
fn bold_below_min_fails() {
    let doc = doc_with_bold_count(1);
    let config = config_with_bold_min(3);
    assertions::assert_bold_count(&doc, &config, 1, 0, 1, "bold=1 with min=3 should fail");
}

#[test]
fn bold_above_min_passes() {
    let doc = doc_with_bold_count(5);
    let config = config_with_bold_min(3);
    assertions::assert_bold_count(&doc, &config, 5, 1, 0, "bold=5 with min=3 should pass");
}

#[test]
fn bold_at_exact_min_passes() {
    let doc = doc_with_bold_count(3);
    let config = config_with_bold_min(3);
    assertions::assert_bold_count(&doc, &config, 3, 1, 0, "bold=3 with min=3 should pass");
}

#[test]
fn no_threshold_skips() {
    let doc = doc_with_bold_count(0);
    let config = CheckConfig::default();
    assertions::assert_skips(&doc, &config, "no threshold → no expectation");
}

#[test]
fn check_id_and_label() {
    assertions::assert_check_metadata();
}

fn config_with_bold_min(min: usize) -> CheckConfig {
    let mut config = CheckConfig {
        locale: Locale::En,
        ..CheckConfig::default()
    };
    config.document_policy.bold_density_min = Some(min);
    config
}
