use crate::test_helpers::make_doc;
use prosesmasher_app_checks_lexical_assertions::prohibited_terms as assertions;
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
    assertions::assert_prohibited_matches(&doc, &config, &["actually"], 1, "word should fail");
}

#[test]
fn prohibited_phrase_present_fails() {
    let doc = make_doc("In terms of speed, this is fine.", Locale::En);
    let config = config_with_prohibited(&["in terms of"]);
    assertions::assert_prohibited_matches(&doc, &config, &["in terms of"], 1, "phrase should fail");
}

#[test]
fn prohibited_terms_empty_skips() {
    let doc = make_doc("Nothing to see here.", Locale::En);
    let config = config_with_prohibited(&[]);
    assertions::assert_skips(&doc, &config, "empty config skips");
}

#[test]
fn check_id_and_label() {
    assertions::assert_check_metadata();
}
