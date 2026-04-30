use prosesmasher_app_checks_cadence_patterns_assertions::demonstrative_emphasis as assertions;
use prosesmasher_domain_types::{
    Block, CheckConfig, Document, DocumentMetadata, Locale, Paragraph, Section, Sentence, Word,
};

fn make_sentences(texts: &[&str]) -> Vec<Sentence> {
    texts
        .iter()
        .map(|t| {
            let words: Vec<Word> = t
                .split_whitespace()
                .map(|w| Word {
                    text: w.to_owned(),
                    syllable_count: 1,
                })
                .collect();
            Sentence {
                text: (*t).to_owned(),
                words,
            }
        })
        .collect()
}

fn make_doc(texts: &[&str]) -> Document {
    Document {
        locale: Locale::En,
        sections: vec![Section {
            heading: None,
            blocks: vec![Block::Paragraph(Paragraph {
                sentences: make_sentences(texts),
                has_bold: false,
                has_italic: false,
                links: vec![],
            })],
        }],
        metadata: DocumentMetadata::default(),
    }
}

fn config() -> CheckConfig {
    CheckConfig::default()
}

#[test]
fn check_id_and_label() {
    assertions::assert_check_metadata();
}

#[test]
fn cluster_of_demonstrative_sentences_fails() {
    let doc = make_doc(&[
        "That advice is not wrong.",
        "It is just incomplete.",
        "The hard part is the judgment call.",
        "That is where the map earns its keep.",
        "That looks normal.",
        "It is also how cannibalization starts.",
        "That difference matters.",
        "That last part is where most keyword maps get weak.",
        "That is not subtle.",
    ]);
    let config = config();
    assertions::assert_fails(
        &doc,
        &config,
        "cluster of demonstrative emphatic sentences should fail",
    );
}

#[test]
fn single_demonstrative_sentence_passes() {
    let doc = make_doc(&[
        "We rolled out the new feature last week.",
        "That is where we hit the bottleneck.",
        "The metrics recovered after rebuilding the cache layer.",
    ]);
    let config = config();
    assertions::assert_passes(
        &doc,
        &config,
        "single demonstrative sentence below threshold should pass",
    );
}

#[test]
fn two_demonstrative_sentences_pass_at_default_threshold() {
    let doc = make_doc(&[
        "Some narrative content here that is not flagged.",
        "That is where the issue first appeared.",
        "We tracked it down over two days of debugging.",
        "That is one reason we now monitor that metric.",
    ]);
    let config = config();
    assertions::assert_passes(
        &doc,
        &config,
        "two demonstrative sentences at threshold=2 should pass",
    );
}

#[test]
fn long_sentence_with_demonstrative_subject_passes() {
    let doc = make_doc(&[
        "That is where teams struggle most when they migrate from a monolith to microservices because of distributed transactions.",
        "That is where teams struggle most when they migrate from a monolith to microservices because of distributed transactions.",
        "That is where teams struggle most when they migrate from a monolith to microservices because of distributed transactions.",
    ]);
    let config = config();
    assertions::assert_passes(
        &doc,
        &config,
        "long sentences should not be flagged as emphatic",
    );
}

#[test]
fn list_block_content_ignored() {
    let doc = Document {
        locale: Locale::En,
        sections: vec![Section {
            heading: None,
            blocks: vec![Block::List(prosesmasher_domain_types::ListBlock {
                ordered: false,
                items: vec![],
            })],
        }],
        metadata: DocumentMetadata::default(),
    };
    let config = config();
    assertions::assert_passes(&doc, &config, "list content should not contribute");
}

#[test]
fn code_block_ignored() {
    let doc = Document {
        locale: Locale::En,
        sections: vec![Section {
            heading: None,
            blocks: vec![
                Block::CodeBlock(
                    "That is a comment. That is another. That is a third.".to_owned(),
                ),
                Block::CodeBlock(
                    "That is a fourth. That is a fifth. That is a sixth.".to_owned(),
                ),
            ],
        }],
        metadata: DocumentMetadata::default(),
    };
    let config = config();
    assertions::assert_passes(&doc, &config, "code block content should be ignored");
}

#[test]
fn non_demonstrative_short_sentences_pass() {
    let doc = make_doc(&[
        "We shipped on time.",
        "The feature works.",
        "Users like it.",
        "Latency dropped.",
        "Errors are down.",
    ]);
    let config = config();
    assertions::assert_passes(
        &doc,
        &config,
        "non-demonstrative short sentences should pass",
    );
}

#[test]
fn disabled_check_skips() {
    let doc = make_doc(&[
        "That is X.",
        "That is Y.",
        "That is Z.",
        "That is W.",
        "That is V.",
    ]);
    let mut config = config();
    config.quality.heuristics.demonstrative_emphasis.enabled = false;
    assertions::assert_skips(&doc, &config, "disabled check should skip");
}
