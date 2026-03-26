use crate::test_helpers::{make_doc, make_doc_code_only};
use prosesmasher_app_checks_llm_slop_assertions::slogan_punchline as assertions;
use prosesmasher_domain_types::{
    Block, CheckConfig, Document, DocumentMetadata, Locale, Paragraph, Section, Sentence, Word,
};

fn make_sentences(texts: &[&str]) -> Vec<Sentence> {
    texts
        .iter()
        .map(|text| {
            let words: Vec<Word> = text
                .split_whitespace()
                .map(|word| Word {
                    text: word.to_owned(),
                    syllable_count: 1,
                })
                .collect();
            Sentence {
                text: (*text).to_owned(),
                words,
            }
        })
        .collect()
}

fn make_doc_with_sentences(texts: &[&str], locale: Locale) -> Document {
    let sentences = make_sentences(texts);
    let total_words = sentences.iter().map(|sentence| sentence.words.len()).sum();
    Document {
        locale,
        sections: vec![Section {
            heading: None,
            blocks: vec![Block::Paragraph(Paragraph {
                sentences,
                has_bold: false,
                has_italic: false,
                links: vec![],
            })],
        }],
        metadata: DocumentMetadata {
            total_words,
            total_sentences: texts.len(),
            ..Default::default()
        },
    }
}

#[test]
fn part_that_sticks_fails() {
    let doc = make_doc("The rehearsal is the part that sticks.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_punchline_failure(
        &doc,
        &config,
        "part-that-sticks",
        "tight sloganized stickiness line should fail",
    );
}

#[test]
fn part_most_families_miss_fails() {
    let doc = make_doc("That is the part most families miss.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_punchline_failure(
        &doc,
        &config,
        "part-most-x-miss",
        "deictic most-families-miss punchline should fail",
    );
}

#[test]
fn it_changes_everything_fails() {
    let doc = make_doc("It sounds small, and it changes everything.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_punchline_failure(
        &doc,
        &config,
        "it-changes-everything",
        "changes-everything magnification line should fail",
    );
}

#[test]
fn enough_for_this_curriculum_pair_fails() {
    let doc = make_doc_with_sentences(
        &[
            "Daily life is enough for this.",
            "Daily life is the curriculum.",
        ],
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_punchline_failure(
        &doc,
        &config,
        "x-is-enough-x-is-curriculum",
        "paired curriculum slogan should fail",
    );
}

#[test]
fn generic_part_people_skip_passes() {
    let doc = make_doc(
        "That is the part people skip when they talk about burnout as if it were just a resilience issue.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(
        &doc,
        &config,
        "long explanatory part-skip sentence should pass",
    );
}

#[test]
fn with_rent_near_miss_passes() {
    let doc = make_doc("Doing it is the part with rent.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_passes(&doc, &config, "part-with-rent near miss should pass");
}

#[test]
fn concrete_changes_everything_sentence_passes() {
    let doc = make_doc(
        "This patch changes everything in the cache invalidation path.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(
        &doc,
        &config,
        "concrete technical changes-everything usage should pass",
    );
}

#[test]
fn quoted_phrase_passes() {
    let doc = make_doc(
        "Editors should cut lines like \"It sounds small, and it changes everything.\" when they add no substance.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(&doc, &config, "quoted discussion should pass");
}

#[test]
fn code_block_phrase_passes() {
    let doc = make_doc_code_only("The rehearsal is the part that sticks.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_passes(&doc, &config, "code blocks should be ignored");
}

#[test]
fn non_english_is_skipped() {
    let doc = make_doc("The rehearsal is the part that sticks.", Locale::Fr);
    let config = CheckConfig::default();
    assertions::assert_skips(&doc, &config, "non-English locales should skip");
}

#[test]
fn disabled_check_skips() {
    let doc = make_doc("The rehearsal is the part that sticks.", Locale::En);
    let mut config = CheckConfig::default();
    config.quality.heuristics.slogan_punchline.enabled = false;
    assertions::assert_skips(&doc, &config, "disabled check should skip");
}

#[test]
fn check_id_and_label() {
    assertions::assert_check_metadata();
}
