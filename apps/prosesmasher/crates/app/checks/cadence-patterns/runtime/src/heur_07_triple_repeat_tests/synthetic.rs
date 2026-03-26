use prosesmasher_app_checks_cadence_patterns_assertions::triple_repeat as assertions;
use prosesmasher_domain_types::{
    Block, CheckConfig, Document, DocumentMetadata, ListBlock, Locale, Paragraph, Section,
    Sentence, Word,
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

fn make_doc_with_sentences(texts: &[&str], locale: Locale) -> Document {
    make_doc_with_blocks(vec![make_paragraph_block(texts)], locale)
}

fn make_paragraph_block(texts: &[&str]) -> Block {
    Block::Paragraph(Paragraph {
        sentences: make_sentences(texts),
        has_bold: false,
        has_italic: false,
        links: vec![],
    })
}

fn make_doc_with_blocks(blocks: Vec<Block>, locale: Locale) -> Document {
    let word_count: usize = blocks
        .iter()
        .map(count_words_in_block)
        .sum();
    let sentence_count: usize = blocks
        .iter()
        .map(count_sentences_in_block)
        .sum();

    Document {
        locale,
        sections: vec![Section {
            heading: None,
            blocks,
        }],
        metadata: DocumentMetadata {
            total_words: word_count,
            total_sentences: sentence_count,
            ..Default::default()
        },
    }
}

fn count_words_in_block(block: &Block) -> usize {
    match block {
        Block::Paragraph(paragraph) => paragraph
            .sentences
            .iter()
            .map(|sentence| sentence.words.len())
            .sum(),
        Block::BlockQuote(inner) => inner.iter().map(count_words_in_block).sum(),
        Block::List(ListBlock { items, .. }) => items
            .iter()
            .map(|item| item.split_whitespace().count())
            .sum(),
        Block::CodeBlock(code) => code.split_whitespace().count(),
    }
}

fn count_sentences_in_block(block: &Block) -> usize {
    match block {
        Block::Paragraph(paragraph) => paragraph.sentences.len(),
        Block::BlockQuote(inner) => inner.iter().map(count_sentences_in_block).sum(),
        Block::List(ListBlock { .. }) | Block::CodeBlock(_) => 0,
    }
}

#[test]
fn triple_same_opener_fails() {
    let doc = make_doc_with_sentences(
        &["It's fast.", "It's reliable.", "It's revolutionary."],
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_triple_repeat_failure(
        &doc,
        &config,
        "it's",
        "It's fast.",
        "It's reliable.",
        "It's revolutionary.",
        "triple repeat should fail",
    );
}

#[test]
fn different_openers_pass() {
    let doc = make_doc_with_sentences(
        &["It's fast.", "The engine purrs.", "Nothing breaks."],
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(&doc, &config, "different openers should pass");
}

#[test]
fn fewer_than_three_sentences_passes() {
    let doc = make_doc_with_sentences(&["It's fast.", "It's reliable."], Locale::En);
    let config = CheckConfig::default();
    assertions::assert_passes(&doc, &config, "only two sentences should pass");
}

#[test]
fn age_ladder_fixture_shape_fails() {
    let sentence_1 =
        "Your four-year-old can't get the Lego piece to snap on, and suddenly she's hurling it.";
    let sentence_2 =
        "Your six-year-old loses a board game and punches a hole in the couch cushion.";
    let sentence_3 = "Your three-year-old bites another child at pickup before you see it build.";
    let doc = make_doc_with_sentences(&[sentence_1, sentence_2, sentence_3], Locale::En);
    let config = CheckConfig::default();
    assertions::assert_triple_repeat_failure(
        &doc,
        &config,
        "your",
        sentence_1,
        sentence_2,
        sentence_3,
        "fixture-parity age ladder should fail",
    );
}

#[test]
fn mixed_capitalization_same_opener_fails() {
    let sentence_1 = "Your first draft limps.";
    let sentence_2 = "your second draft sprawls.";
    let sentence_3 = "YOUR third draft finally lands.";
    let doc = make_doc_with_sentences(&[sentence_1, sentence_2, sentence_3], Locale::En);
    let config = CheckConfig::default();
    assertions::assert_triple_repeat_failure(
        &doc,
        &config,
        "your",
        sentence_1,
        sentence_2,
        sentence_3,
        "case-insensitive triple repeat should fail",
    );
}

#[test]
fn sliding_window_triple_repeat_fails() {
    let sentence_1 = "But first, the draft needs air.";
    let sentence_2 = "Your first opener drags.";
    let sentence_3 = "Your second opener stalls.";
    let sentence_4 = "Your third opener finally confesses the pattern.";
    let doc = make_doc_with_sentences(&[sentence_1, sentence_2, sentence_3, sentence_4], Locale::En);
    let config = CheckConfig::default();
    assertions::assert_triple_repeat_failure(
        &doc,
        &config,
        "your",
        sentence_2,
        sentence_3,
        sentence_4,
        "later sliding window should still fail",
    );
}

#[test]
fn triple_repeat_inside_blockquote_fails() {
    let sentence_1 = "Your inbox explodes before breakfast.";
    let sentence_2 = "Your phone blinks through lunch.";
    let sentence_3 = "Your calendar mutinies by dinner.";
    let doc = make_doc_with_blocks(
        vec![Block::BlockQuote(vec![make_paragraph_block(&[
            sentence_1,
            sentence_2,
            sentence_3,
        ])])],
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_triple_repeat_failure(
        &doc,
        &config,
        "your",
        sentence_1,
        sentence_2,
        sentence_3,
        "blockquote paragraphs should still be scanned",
    );
}

#[test]
fn repeated_opener_across_paragraphs_passes() {
    let doc = make_doc_with_blocks(
        vec![
            make_paragraph_block(&["Your first paragraph starts the pattern."]),
            make_paragraph_block(&[
                "Your second paragraph continues it.",
                "Your third sentence would complete it if boundaries were ignored.",
            ]),
        ],
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(
        &doc,
        &config,
        "repeats split across paragraph boundaries should pass",
    );
}

#[test]
fn interrupted_repeat_window_passes() {
    let doc = make_doc_with_sentences(
        &[
            "Your first sentence starts the run.",
            "Your second sentence keeps it going.",
            "This interruption should break the window.",
            "Your fourth sentence should not revive the earlier pair.",
        ],
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(
        &doc,
        &config,
        "non-consecutive repeats should pass",
    );
}

#[test]
fn code_block_with_repeated_lines_passes() {
    let doc = make_doc_with_blocks(
        vec![Block::CodeBlock(
            "Your fake sentence.\nYour fake sentence.\nYour fake sentence.".to_owned(),
        )],
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(
        &doc,
        &config,
        "code block text should not trigger triple repeat",
    );
}

#[test]
fn check_id_and_label() {
    assertions::assert_check_metadata();
}
