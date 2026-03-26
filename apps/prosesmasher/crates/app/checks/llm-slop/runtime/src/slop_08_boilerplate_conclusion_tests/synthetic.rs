use crate::test_helpers::{make_doc, make_doc_multi_section};
use prosesmasher_app_checks_llm_slop_assertions::boilerplate_conclusion as assertions;
use prosesmasher_domain_types::{CheckConfig, Locale};

#[test]
fn key_insight_close_at_end_fails() {
    let doc = make_doc(
        "The most important insight from decades of research may be the simplest: imperfect action is almost always better than no action.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_conclusion_failure(
        &doc,
        &config,
        "insight-close",
        "most important",
        "importance + insight enders should fail",
    );
}

#[test]
fn research_is_clear_close_fails() {
    let doc = make_doc("The research is clear: it almost always is.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_conclusion_failure(
        &doc,
        &config,
        "authority-close",
        "the research is clear",
        "authority certainty closers should fail",
    );
}

#[test]
fn acceptance_as_normal_close_fails() {
    let doc = make_doc(
        "Waking up tired after a full night of sleep is not something you have to accept as normal.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_conclusion_failure(
        &doc,
        &config,
        "acceptance-close",
        "not something you have to accept as normal",
        "acceptance-normality closers should fail",
    );
}

#[test]
fn practical_response_plain_close_fails() {
    let doc = make_doc(
        "The practical response is plain: make connection easier to start, easier to repeat, and easier to keep.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_conclusion_failure(
        &doc,
        &config,
        "response-close",
        "the practical response is plain",
        "practical-response summary closers should fail",
    );
}

#[test]
fn basic_rule_simple_close_fails() {
    let doc = make_doc("The basic rule is simple.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_conclusion_failure(
        &doc,
        &config,
        "response-close",
        "the basic rule is simple",
        "basic-rule summary closers should fail",
    );
}

#[test]
fn whole_trick_close_fails() {
    let doc = make_doc("That is the whole trick.", Locale::En);
    let config = CheckConfig::default();
    assertions::assert_conclusion_failure(
        &doc,
        &config,
        "compression-close",
        "the whole trick",
        "whole-trick compression closers should fail",
    );
}

#[test]
fn whole_trick_mid_paragraph_fails() {
    let doc = make_doc(
        "Choose one behavior, one cue, and one tiny starting point. That is the whole trick. It is not glamorous. It works anyway.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_conclusion_failure(
        &doc,
        &config,
        "compression-close",
        "the whole trick",
        "strong compression lines should fail even away from the document ending",
    );
}

#[test]
fn same_phrase_mid_article_passes() {
    let doc = make_doc_multi_section(
        &[
            "The research is clear: hydration matters.",
            "The randomized trial reduced symptoms by 12 points after six weeks.",
            "Hydration improved adherence in the treatment arm.",
            "Sleep quality also improved after six weeks.",
        ],
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(&doc, &config, "non-closing occurrences should not fail");
}

#[test]
fn practical_response_mid_article_fails() {
    let doc = make_doc_multi_section(
        &[
            "Loneliness has real health consequences when it becomes chronic.",
            "The practical response is plain: make connection easier to start, easier to repeat, and easier to keep.",
            "That shift matters more than one big gesture.",
            "Small repeated contact works better than heroic bursts.",
        ],
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_conclusion_failure(
        &doc,
        &config,
        "response-close",
        "the practical response is plain",
        "strong practical-response framing should fail even when it appears before the final section close",
    );
}

#[test]
fn concrete_rule_statement_passes() {
    let doc = make_doc(
        "The basic rule is simple: parse the file first, then normalize the visible text.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(
        &doc,
        &config,
        "concrete explanatory rule statements should pass",
    );
}

#[test]
fn concrete_factual_conclusion_passes() {
    let doc = make_doc(
        "A 2024 randomized trial found a 12-point reduction in symptoms after six weeks.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(&doc, &config, "concrete sourced conclusions should pass");
}

#[test]
fn quoted_phrase_passes() {
    let doc = make_doc(
        "Editors should be suspicious of the phrase \"the research is clear\" when the draft never names any study.",
        Locale::En,
    );
    let config = CheckConfig::default();
    assertions::assert_passes(&doc, &config, "quoted meta discussion should pass");
}

#[test]
fn non_english_is_skipped() {
    let doc = make_doc("The research is clear: it almost always is.", Locale::Fr);
    let config = CheckConfig::default();
    assertions::assert_skips(&doc, &config, "non-English locales should skip");
}

#[test]
fn disabled_check_skips() {
    let doc = make_doc("The research is clear: it almost always is.", Locale::En);
    let mut config = CheckConfig::default();
    config.quality.heuristics.boilerplate_conclusion.enabled = false;
    assertions::assert_skips(&doc, &config, "disabled check should skip");
}

#[test]
fn check_id_and_label() {
    assertions::assert_check_metadata();
}
