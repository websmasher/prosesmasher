//! The `Check` trait — the contract every prose check must implement.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Document, Locale};

/// A composable prose quality check.
///
/// Each check inspects a parsed `Document` against a `CheckConfig` and
/// adds expectations to the suite. The runner calls `run` for each
/// registered check, skipping checks whose `supported_locales` don't
/// include the document's locale.
/// Type alias for a boxed check — used by `all_checks()` functions.
pub type BoxedCheck = Box<dyn Check>;

/// A composable prose quality check.
pub trait Check {
    /// Unique check ID (e.g., "prohibited-terms", "em-dashes").
    fn id(&self) -> &'static str;

    /// Human-readable label (e.g., "Prohibited Terms").
    fn label(&self) -> &'static str;

    /// Which locales this check supports. `None` = all locales.
    ///
    /// Checks returning `Some(&[Locale::En])` are skipped for
    /// non-English documents.
    fn supported_locales(&self) -> Option<&'static [Locale]>;

    /// Run the check, adding expectations to the suite.
    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite);
}
