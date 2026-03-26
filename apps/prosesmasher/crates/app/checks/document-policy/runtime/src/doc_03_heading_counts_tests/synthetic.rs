use prosesmasher_app_checks_document_policy_assertions::heading_counts as assertions;
use prosesmasher_domain_types::{
    CheckConfig, Document, DocumentMetadata, HeadingCounts, Locale, Range, Section,
};

fn doc_with_heading_counts(h2: usize, h3: usize) -> Document {
    Document {
        locale: Locale::En,
        sections: vec![Section {
            heading: None,
            blocks: vec![],
        }],
        metadata: DocumentMetadata {
            heading_counts: HeadingCounts {
                h2,
                h3,
                ..HeadingCounts::default()
            },
            ..DocumentMetadata::default()
        },
    }
}

#[test]
fn h2_below_min_fails() {
    let doc = doc_with_heading_counts(1, 0);
    let config = config_with_h2_range(2, 6);
    assertions::assert_h2_failure_observed(&doc, &config, 1, "h2=1 with min=2 should fail");
}

#[test]
fn h2_within_range_passes() {
    let doc = doc_with_heading_counts(3, 0);
    let config = config_with_h2_range(2, 6);
    assertions::assert_single_success(&doc, &config, "h2=3 with range 2-6 should pass");
}

#[test]
fn h3_below_min_fails() {
    let doc = doc_with_heading_counts(3, 1);
    let config = config_with_h3_min(3);
    assertions::assert_fails(&doc, &config, "h3=1 with min=3 should fail");
}

#[test]
fn h3_above_min_passes() {
    let doc = doc_with_heading_counts(3, 5);
    let config = config_with_h3_min(3);
    assertions::assert_single_success(&doc, &config, "h3=5 with min=3 should pass");
}

#[test]
fn no_thresholds_skips() {
    let doc = doc_with_heading_counts(1, 1);
    let config = CheckConfig::default();
    assertions::assert_skips(&doc, &config, "no thresholds → no expectations");
}

#[test]
fn check_id_and_label() {
    assertions::assert_check_metadata();
}

fn config_with_h2_range(min: usize, max: usize) -> CheckConfig {
    let mut config = CheckConfig {
        locale: Locale::En,
        ..CheckConfig::default()
    };
    config.document_policy.heading_counts.h2 = Range::new(min, max);
    config
}

fn config_with_h3_min(min: usize) -> CheckConfig {
    let mut config = CheckConfig {
        locale: Locale::En,
        ..CheckConfig::default()
    };
    config.document_policy.heading_counts.h3_min = Some(min);
    config
}
