//! Bold density check — validates that bold usage meets a minimum threshold.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Document, Locale};

use crate::check::Check;

/// Checks that the document has at least the configured minimum number of bold elements.
#[derive(Debug)]
pub struct BoldDensityCheck;

impl Check for BoldDensityCheck {
    fn id(&self) -> &'static str {
        "bold-density"
    }

    fn label(&self) -> &'static str {
        "Bold Density"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        None
    }

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        if let Some(bold_min) = config.document_policy.bold_density_min {
            let observed = i64::try_from(doc.metadata.bold_count).unwrap_or(i64::MAX);
            let min = i64::try_from(bold_min).unwrap_or(0);
            let _result = suite
                .expect_value_to_be_between("bold-density", observed, min, i64::MAX)
                .label("Bold Density")
                .checking("bold element count");
        }
    }
}

#[cfg(test)]
#[path = "bold_density_tests.rs"]
mod tests;
