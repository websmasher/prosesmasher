use crate::test_helpers::{make_doc, make_doc_code_only};
use prosesmasher_app_checks_llm_slop_assertions::generic_signposting as assertions;
use prosesmasher_domain_types::{
    Block, CheckConfig, Document, DocumentMetadata, Locale, Paragraph, Section, Sentence, Word,
};

fn make_sentence(text: &str) -> Sentence {
    Sentence {
        text: text.to_owned(),
        words: text
            .split_whitespace()
            .map(|word| Word {
                text: word.to_owned(),
                syllable_count: 1,
            })
            .collect(),
    }
}

fn make_multi_sentence_doc(sentences: &[&str], locale: Locale) -> Document {
    let sentence_values: Vec<Sentence> = sentences.iter().map(|text| make_sentence(text)).collect();
    let total_words: usize = sentence_values
        .iter()
        .map(|sentence| sentence.words.len())
        .sum();
    Document {
        locale,
        sections: vec![Section {
            heading: None,
            blocks: vec![Block::Paragraph(Paragraph {
                sentences: sentence_values,
                has_bold: false,
                has_italic: false,
                links: vec![],
            })],
        }],
        metadata: DocumentMetadata {
            total_words,
            total_sentences: sentences.len(),
            ..Default::default()
        },
    }
}

fn make_multi_sentence_blockquote_doc(sentences: &[&str], locale: Locale) -> Document {
    let sentence_values: Vec<Sentence> = sentences.iter().map(|text| make_sentence(text)).collect();
    let total_words: usize = sentence_values
        .iter()
        .map(|sentence| sentence.words.len())
        .sum();
    Document {
        locale,
        sections: vec![Section {
            heading: None,
            blocks: vec![Block::BlockQuote(vec![Block::Paragraph(Paragraph {
                sentences: sentence_values,
                has_bold: false,
                has_italic: false,
                links: vec![],
            })])],
        }],
        metadata: DocumentMetadata {
            total_words,
            total_sentences: sentences.len(),
            ..Default::default()
        },
    }
}

#[test]
fn repeated_important_to_signposts_fail() {
    let doc = make_multi_sentence_doc(
        &[
            "It's important to note that eczema triggers vary by person.",
            "It's important to remember that treatment plans vary too.",
        ],
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_signposting_failure(
        &doc,
        &config,
        "important-to",
        "it's important to note",
        "repeated important-to signposts should fail",
    );
}

#[test]
fn answer_and_question_frames_fail() {
    let doc = make_multi_sentence_doc(
        &[
            "The answer is simple.",
            "The useful question is whether they are building any skill from it or just rehearsing chaos.",
        ],
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_signposting_failure(
        &doc,
        &config,
        "answer-frame",
        "the answer is simple",
        "empty answer and question frames should fail once repeated",
    );
}

#[test]
fn answer_and_sequence_frames_fail() {
    let doc = make_multi_sentence_doc(
        &["The answer is simple.", "A simple sequence works well:"],
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_signposting_failure(
        &doc,
        &config,
        "answer-frame",
        "the answer is simple",
        "answer and sequence frames should fail once repeated",
    );
}

#[test]
fn mixed_signpost_families_fail() {
    let doc = make_multi_sentence_doc(
        &[
            "That being said, outcomes vary by case.",
            "It's always best to consult a qualified healthcare professional.",
        ],
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_signposting_failure(
        &doc,
        &config,
        "transition",
        "that being said",
        "mixed generic signposts should fail once they repeat in the document",
    );
}

#[test]
fn repeated_signposting_inside_blockquote_fails() {
    let doc = make_multi_sentence_blockquote_doc(
        &[
            "Please note that this is general information.",
            "As such, it should not replace individual medical guidance.",
        ],
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_signposting_failure(
        &doc,
        &config,
        "note-signpost",
        "please note that",
        "blockquote signposting should still count",
    );
}

#[test]
fn one_signpost_passes() {
    let doc = make_doc(
        "It's important to note that eczema triggers vary by person.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(
        &doc,
        &config,
        "a single signpost should stay under the default threshold",
    );
}

#[test]
fn single_question_frame_fails() {
    let doc = make_doc(
        "The useful question is whether the setup is helping or hurting.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_signposting_failure(
        &doc,
        &config,
        "question-frame",
        "the useful question is",
        "single empty question frame should now fail",
    );
}

#[test]
fn useful_move_frame_fails() {
    let doc = make_doc(
        "The useful move is to respect the intensity mismatch.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_signposting_failure(
        &doc,
        &config,
        "abstract-evaluation-frame",
        "the-useful-move-is",
        "single useful-move frame should now fail",
    );
}

#[test]
fn bigger_win_frame_fails() {
    let doc = make_doc(
        "The bigger win is a child who comes back faster after the drop-off.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_signposting_failure(
        &doc,
        &config,
        "abstract-evaluation-frame",
        "the-bigger-win-is",
        "single bigger-win frame should now fail",
    );
}

#[test]
fn result_worth_caring_about_fails() {
    let doc = make_doc(
        "The result worth caring about is a morning that stops feeling like an ambush.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_signposting_failure(
        &doc,
        &config,
        "abstract-evaluation-frame",
        "the-result-worth-caring-about",
        "result-worth-caring-about frame should fail",
    );
}

#[test]
fn point_is_to_abstract_tail_fails() {
    let doc = make_doc("The point is to make starting less dramatic.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_signposting_failure(
        &doc,
        &config,
        "abstract-evaluation-frame",
        "the-point-is-to",
        "abstract point-is-to frame should now fail",
    );
}

#[test]
fn what_matters_frame_fails() {
    let doc = make_doc("What matters is never missing twice.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_signposting_failure(
        &doc,
        &config,
        "abstract-evaluation-frame",
        "what-matters-is",
        "what-matters frame should now fail",
    );
}

#[test]
fn what_helps_frame_fails() {
    let doc = make_doc("What helps is boring, repeatable structure.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_signposting_failure(
        &doc,
        &config,
        "abstract-evaluation-frame",
        "what-helps-is",
        "what-helps frame should now fail",
    );
}

#[test]
fn practical_move_frame_fails() {
    let doc = make_doc(
        "The practical move is simple: fix the sleep basics first.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_signposting_failure(
        &doc,
        &config,
        "question-frame",
        "the practical move is",
        "single practical-move frame should now fail",
    );
}

#[test]
fn procedural_point_is_to_passes() {
    let doc = make_doc(
        "The point is to parse the file before validating it.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(&doc, &config, "procedural point-is-to sentence should pass");
}

#[test]
fn concrete_what_helps_passes() {
    let doc = make_doc(
        "What helps is reducing caffeine after noon and keeping the room darker.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(&doc, &config, "concrete what-helps sentence should pass");
}

#[test]
fn short_answer_frame_fails() {
    let doc = make_doc(
        "The short answer is: it can, but not in the cartoon version of the story.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_signposting_failure(
        &doc,
        &config,
        "answer-frame",
        "the short answer is",
        "single short-answer frame should now fail",
    );
}

#[test]
fn practical_answer_frame_fails() {
    let doc = make_doc("The practical answer is just as consistent.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_signposting_failure(
        &doc,
        &config,
        "answer-frame",
        "the practical answer is",
        "single practical-answer frame should now fail",
    );
}

#[test]
fn short_version_frame_fails() {
    let doc = make_doc(
        "The short version: your first few weeks are not proof that you are bad at habits.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_signposting_failure(
        &doc,
        &config,
        "frame-signpost",
        "the short version",
        "single short-version frame should now fail",
    );
}

#[test]
fn practical_version_frame_fails() {
    let doc = make_doc(
        "The practical version is simpler than the literature sounds.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_signposting_failure(
        &doc,
        &config,
        "frame-signpost",
        "the practical version is",
        "single practical-version frame should now fail",
    );
}

#[test]
fn useful_frame_fails() {
    let doc = make_doc("That is the useful frame.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_signposting_failure(
        &doc,
        &config,
        "frame-signpost",
        "the useful frame",
        "single useful-frame line should now fail",
    );
}

#[test]
fn useful_conclusion_simple_fails() {
    let doc = make_doc(
        "The useful conclusion is simple. Stress can absolutely cause physical symptoms.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_signposting_failure(
        &doc,
        &config,
        "answer-frame",
        "the useful conclusion is simple",
        "single useful-conclusion frame should now fail",
    );
}

#[test]
fn point_plain_enough_fails() {
    let doc = make_doc(
        "The point is plain enough: people do not multitask because they are cursed.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_signposting_failure(
        &doc,
        &config,
        "frame-signpost",
        "the point is plain enough",
        "plain-enough framing should now fail",
    );
}

#[test]
fn concrete_practical_move_phrase_passes() {
    let doc = make_doc(
        "Reducing temptation is another practical move when attention is thin.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(
        &doc,
        &config,
        "ordinary concrete use of practical move should pass",
    );
}

#[test]
fn single_note_signpost_still_passes() {
    let doc = make_doc("Please note that this is general information.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_passes(
        &doc,
        &config,
        "plain note signposts should remain accumulative rather than immediate",
    );
}

#[test]
fn concrete_point_statement_passes() {
    let doc = make_doc(
        "The point is that the parser preserves visible text and drops hidden HTML.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(
        &doc,
        &config,
        "ordinary explanatory point statements should still pass",
    );
}

#[test]
fn ordinary_consultation_without_signpost_passes() {
    let doc = make_doc(
        "Consult a qualified healthcare professional for personalized advice.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(
        &doc,
        &config,
        "plain consultation advice should not count as generic signposting",
    );
}

#[test]
fn quoted_signposts_pass() {
    let doc = make_doc(
        "The phrase \"it's important to note\" is one of the most recycled transitions in bad AI copy.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(
        &doc,
        &config,
        "quoted discussion of the signpost should pass",
    );
}

#[test]
fn code_block_signposts_pass() {
    let doc = make_doc_code_only(
        "That being said, it's always best to consult a qualified healthcare professional.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(&doc, &config, "code blocks should be ignored");
}

#[test]
fn non_english_is_skipped() {
    let doc = make_doc(
        "It is important to note that eczema triggers vary by person.",
        Locale::Fr,
    );
    let config = CheckConfig::default();
    assertions::assert_skips(&doc, &config, "non-English locales should skip");
}

#[test]
fn disabled_check_skips() {
    let doc = make_doc(
        "It is important to note that eczema triggers vary by person. It is important to remember that treatment plans vary too.",
        Locale::En,
    );
    let mut config = CheckConfig::default();
    config.quality.heuristics.generic_signposting.enabled = false;
    assertions::assert_skips(&doc, &config, "disabled generic-signposting should skip");
}

#[test]
fn raised_threshold_passes_two_hits() {
    let doc = make_multi_sentence_doc(
        &[
            "It is important to note that eczema triggers vary by person.",
            "It is important to remember that treatment plans vary too.",
        ],
        Locale::En,
    );
    let mut config = CheckConfig::default();
    config
        .quality
        .heuristics
        .generic_signposting
        .max_per_document = 2;
    assertions::assert_passes(
        &doc,
        &config,
        "two signposts should pass once the threshold is raised",
    );
}

#[test]
fn check_id_and_label() {
    assertions::assert_check_metadata();
}
