//! Check runner — executes all checks against a document.

use low_expectations::types::SuiteValidationResult;
use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Document};

use crate::check::Check;

/// Run all checks against a document and return the suite result.
///
/// Checks are skipped if their `supported_locales` do not include
/// the document's locale.
///
/// # Errors
///
/// This function is infallible — individual check failures are
/// recorded as failed expectations in the suite result, not as errors.
#[must_use]
pub fn run_checks(
    checks: &[&dyn Check],
    doc: &Document,
    config: &CheckConfig,
) -> SuiteValidationResult {
    let mut suite = ExpectationSuite::new("prosesmasher");

    for check in checks {
        // Skip checks that don't support this document's locale
        if let Some(locales) = check.supported_locales()
            && !locales.contains(&doc.locale)
        {
            continue;
        }
        check.run(doc, config, &mut suite);
    }

    suite.into_suite_result()
}

#[cfg(test)]
#[path = "runner_tests.rs"]
mod tests;
