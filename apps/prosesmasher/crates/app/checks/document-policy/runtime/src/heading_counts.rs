//! Heading counts check — validates H2/H3 counts against configured thresholds.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Document, Locale};
use serde_json::json;

use crate::check::Check;

/// Checks that heading counts (H2, H3) fall within configured ranges.
#[derive(Debug)]
pub struct HeadingCountsCheck;

impl Check for HeadingCountsCheck {
    fn id(&self) -> &'static str {
        "heading-counts"
    }

    fn label(&self) -> &'static str {
        "Heading Counts"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        None
    }

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        if let Some(range) = config.document_policy.heading_counts.h2 {
            let observed = i64::try_from(doc.metadata.heading_counts.h2).unwrap_or(i64::MAX);
            let min = i64::try_from(range.min()).unwrap_or(0);
            let max = i64::try_from(range.max()).unwrap_or(i64::MAX);
            let _result = suite
                .record_custom_values(
                    "h2-count",
                    observed >= min && observed <= max,
                    json!({ "min": min, "max": max, "count_type": "h2" }),
                    json!({ "observed": observed, "count_type": "h2" }),
                    &[json!({
                        "count_type": "h2",
                        "observed": observed,
                        "min": min,
                        "max": max,
                    })],
                )
                .label("Heading Counts")
                .checking("H2 heading count");
        }

        if let Some(h3_min) = config.document_policy.heading_counts.h3_min {
            let observed = i64::try_from(doc.metadata.heading_counts.h3).unwrap_or(i64::MAX);
            let min = i64::try_from(h3_min).unwrap_or(0);
            let _result = suite
                .record_custom_values(
                    "h3-count",
                    observed >= min,
                    json!({ "min": min, "count_type": "h3" }),
                    json!({ "observed": observed, "count_type": "h3" }),
                    &[json!({
                        "count_type": "h3",
                        "observed": observed,
                        "min": min,
                    })],
                )
                .label("Heading Counts")
                .checking("H3 heading count");
        }
    }
}

#[cfg(test)]
#[path = "heading_counts_tests/mod.rs"]
mod tests;
