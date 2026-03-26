//! Word count check — validates total word count is within configured range.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Document, Locale};

use crate::check::Check;

/// Checks that the document's total word count falls within the configured range.
#[derive(Debug)]
pub struct WordCountCheck;

impl Check for WordCountCheck {
    fn id(&self) -> &'static str {
        "word-count"
    }

    fn label(&self) -> &'static str {
        "Word Count"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        None
    }

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        if let Some(range) = config.document_policy.word_count {
            let observed = i64::try_from(doc.metadata.total_words).unwrap_or(i64::MAX);
            let min = i64::try_from(range.min()).unwrap_or(0);
            let max = i64::try_from(range.max()).unwrap_or(i64::MAX);
            let _result = suite
                .expect_value_to_be_between("word-count", observed, min, max)
                .label("Word Count")
                .checking("total prose words");
        }
    }
}

#[cfg(test)]
#[path = "doc_01_word_count_tests/mod.rs"]
mod tests;
