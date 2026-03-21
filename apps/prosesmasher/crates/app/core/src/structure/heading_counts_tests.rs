use crate::check::Check;
use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{
    CheckConfig, Document, DocumentMetadata, HeadingCounts, Locale, Range, Section, Thresholds,
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
    let config = CheckConfig {
        locale: Locale::En,
        thresholds: Thresholds {
            h2_count: Range::new(2, 6),
            ..Thresholds::default()
        },
        ..CheckConfig::default()
    };
    let mut suite = ExpectationSuite::new("test");
    super::HeadingCountsCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.unsuccessful_expectations, 1,
        "h2=1 with min=2 should fail"
    );
}

#[test]
fn h2_within_range_passes() {
    let doc = doc_with_heading_counts(3, 0);
    let config = CheckConfig {
        locale: Locale::En,
        thresholds: Thresholds {
            h2_count: Range::new(2, 6),
            ..Thresholds::default()
        },
        ..CheckConfig::default()
    };
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
    let config = CheckConfig {
        locale: Locale::En,
        thresholds: Thresholds {
            h3_min: Some(3),
            ..Thresholds::default()
        },
        ..CheckConfig::default()
    };
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
    let config = CheckConfig {
        locale: Locale::En,
        thresholds: Thresholds {
            h3_min: Some(3),
            ..Thresholds::default()
        },
        ..CheckConfig::default()
    };
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
