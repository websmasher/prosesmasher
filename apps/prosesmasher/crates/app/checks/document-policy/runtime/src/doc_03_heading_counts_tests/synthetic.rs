use crate::check::Check;
use low_expectations::ExpectationSuite;
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
    let mut suite = ExpectationSuite::new("test");
    super::HeadingCountsCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.unsuccessful_expectations, 1,
        "h2=1 with min=2 should fail"
    );
    let vr = result.results.get("h2-count");
    assert!(vr.is_some(), "h2-count result should exist");
    if let Some(vr) = vr {
        let evidence = vr.result.partial_unexpected_list.as_ref();
        assert!(evidence.is_some(), "evidence should be present");
        assert_eq!(
            evidence
                .and_then(|e| e.first())
                .and_then(|item| item.get("observed"))
                .and_then(serde_json::Value::as_i64),
            Some(1),
            "observed count"
        );
    }
}

#[test]
fn h2_within_range_passes() {
    let doc = doc_with_heading_counts(3, 0);
    let config = config_with_h2_range(2, 6);
    let mut suite = ExpectationSuite::new("test");
    super::HeadingCountsCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.successful_expectations, 1,
        "h2=3 with range 2-6 should pass"
    );
}

#[test]
fn h3_below_min_fails() {
    let doc = doc_with_heading_counts(3, 1);
    let config = config_with_h3_min(3);
    let mut suite = ExpectationSuite::new("test");
    super::HeadingCountsCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.unsuccessful_expectations, 1,
        "h3=1 with min=3 should fail"
    );
}

#[test]
fn h3_above_min_passes() {
    let doc = doc_with_heading_counts(3, 5);
    let config = config_with_h3_min(3);
    let mut suite = ExpectationSuite::new("test");
    super::HeadingCountsCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.successful_expectations, 1,
        "h3=5 with min=3 should pass"
    );
}

#[test]
fn no_thresholds_skips() {
    let doc = doc_with_heading_counts(1, 1);
    let config = CheckConfig::default();
    let mut suite = ExpectationSuite::new("test");
    super::HeadingCountsCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.evaluated_expectations, 0,
        "no thresholds → no expectations"
    );
}

#[test]
fn check_id_and_label() {
    let check = super::HeadingCountsCheck;
    assert_eq!(check.id(), "heading-counts", "id");
    assert_eq!(check.label(), "Heading Counts", "label");
    assert!(check.supported_locales().is_none(), "supports all locales");
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
