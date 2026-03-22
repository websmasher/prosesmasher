use crate::check::Check;
use crate::test_helpers::make_doc;
use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Locale, QualityConfig};

fn config_with_prohibited(terms: &[&str]) -> CheckConfig {
    let mut config = CheckConfig {
        locale: Locale::En,
        ..CheckConfig::default()
    };
    config.quality = QualityConfig {
        lexical: prosesmasher_domain_types::LexicalConfig {
            prohibited_terms: prosesmasher_domain_types::OverrideList {
                defaults: false,
                add: terms.iter().map(|term| (*term).to_owned()).collect(),
                remove: Vec::new(),
            },
            ..config.quality.lexical.clone()
        },
        ..config.quality.clone()
    };
    config
}

#[test]
fn prohibited_word_present_fails() {
    let doc = make_doc("We actually need this.", Locale::En);
    let config = config_with_prohibited(&["actually"]);
    let mut suite = ExpectationSuite::new("test");
    super::ProhibitedTermsCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.unsuccessful_expectations, 1, "word should fail");
}

#[test]
fn prohibited_phrase_present_fails() {
    let doc = make_doc("In terms of speed, this is fine.", Locale::En);
    let config = config_with_prohibited(&["in terms of"]);
    let mut suite = ExpectationSuite::new("test");
    super::ProhibitedTermsCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.unsuccessful_expectations, 1, "phrase should fail");
}

#[test]
fn prohibited_terms_empty_skips() {
    let doc = make_doc("Nothing to see here.", Locale::En);
    let config = config_with_prohibited(&[]);
    let mut suite = ExpectationSuite::new("test");
    super::ProhibitedTermsCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(result.statistics.evaluated_expectations, 0, "empty config skips");
}

#[test]
fn check_id_and_label() {
    let check = super::ProhibitedTermsCheck;
    assert_eq!(check.id(), "prohibited-terms");
    assert_eq!(check.label(), "Prohibited Terms");
}
