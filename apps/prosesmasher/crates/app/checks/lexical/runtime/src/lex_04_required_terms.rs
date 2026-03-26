//! Required terms check — ALL configured terms must appear in the document.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Document, Locale};

use crate::check::Check;

/// Checks that every term in `config.quality.lexical.required_terms` appears
/// in the document. Each missing term is a failure.
#[derive(Debug)]
pub struct RequiredTermsCheck;

impl Check for RequiredTermsCheck {
    fn id(&self) -> &'static str {
        "required-terms"
    }

    fn label(&self) -> &'static str {
        "Required Terms"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        None
    }

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        if config.quality.lexical.required_terms.is_empty() {
            return;
        }

        let all_words: Vec<&str> = doc
            .sections
            .iter()
            .flat_map(|s| &s.blocks)
            .flat_map(|b| super::collect_paragraph_words(b))
            .collect();

        let lower_words: Vec<String> = all_words.iter().map(|w| w.to_lowercase()).collect();

        for term in &config.quality.lexical.required_terms {
            let lower_term = term.to_lowercase();
            let found = lower_words.contains(&lower_term);
            let observed = i64::from(found);
            let _result = suite
                .expect_value_to_be_at_least(&format!("required-term-{term}"), observed, 1)
                .label("Required Terms")
                .checking(&format!("term \"{term}\" must appear"));
        }
    }
}

#[cfg(test)]
#[path = "lex_04_required_terms_tests/mod.rs"]
mod tests;
