use crate::check::Check;
use low_expectations::ExpectationSuite;
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
    let mut suite = ExpectationSuite::new("test");
    super::BoldDensityCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.unsuccessful_expectations, 1,
        "bold=1 with min=3 should fail"
    );
}

#[test]
fn bold_above_min_passes() {
    let doc = doc_with_bold_count(5);
    let config = config_with_bold_min(3);
    let mut suite = ExpectationSuite::new("test");
    super::BoldDensityCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.successful_expectations, 1,
        "bold=5 with min=3 should pass"
    );
}

#[test]
fn bold_at_exact_min_passes() {
    let doc = doc_with_bold_count(3);
    let config = config_with_bold_min(3);
    let mut suite = ExpectationSuite::new("test");
    super::BoldDensityCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.successful_expectations, 1,
        "bold=3 with min=3 should pass"
    );
}

#[test]
fn no_threshold_skips() {
    let doc = doc_with_bold_count(0);
    let config = CheckConfig::default();
    let mut suite = ExpectationSuite::new("test");
    super::BoldDensityCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.evaluated_expectations, 0,
        "no threshold → no expectation"
    );
}

#[test]
fn check_id_and_label() {
    let check = super::BoldDensityCheck;
    assert_eq!(check.id(), "bold-density", "id");
    assert_eq!(check.label(), "Bold Density", "label");
    assert!(check.supported_locales().is_none(), "supports all locales");
}

fn config_with_bold_min(min: usize) -> CheckConfig {
    let mut config = CheckConfig {
        locale: Locale::En,
        ..CheckConfig::default()
    };
    config.document_policy.bold_density_min = Some(min);
    config
}
