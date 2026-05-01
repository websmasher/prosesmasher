use prosesmasher_app_checks_cadence_patterns_assertions::negation_reframe as assertions;
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

fn make_doc_with_sentences(texts: &[&str], locale: Locale) -> Document {
    let sentences = make_sentences(texts);
    let word_count: usize = sentences.iter().map(|s| s.words.len()).sum();
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
            total_words: word_count,
            total_sentences: texts.len(),
            ..Default::default()
        },
    }
}

fn config_with_signals() -> CheckConfig {
    CheckConfig::default()
}

#[test]
fn negation_reframe_detected() {
    let doc = make_doc_with_sentences(&["This isn't defiance.", "It's developmental."], Locale::En);
    let config = config_with_signals();
    assertions::assert_negation_reframe_pair(
        &doc,
        &config,
        "not y -> x",
        "This isn't defiance.",
        "It's developmental.",
        "negation + reframe pair should fail",
    );
}

#[test]
fn inline_corrective_detected() {
    let doc = make_doc_with_sentences(
        &["The goal is corrective contrast, not generic negation."],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_negation_reframe_failure(
        &doc,
        &config,
        "x, not y",
        "inline x-not-y contrast should fail",
    );
}

#[test]
fn action_negation_narration_does_not_trigger() {
    let doc = make_doc_with_sentences(
        &[
            "I could not fix the banana.",
            "My second instinct was to explain that bananas sometimes break and this is fine and we can eat both pieces.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_passes(&doc, &config, "action negation plus narration should pass");
}

#[test]
fn infinitive_contrast_detected() {
    let doc = make_doc_with_sentences(
        &[
            "Not to act on the anger.",
            "To notice you're assembling a bonfire and stop adding to it.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_negation_reframe_failure(
        &doc,
        &config,
        "not to x -> to y",
        "infinitive contrast detected",
    );
}

#[test]
fn meaning_contrast_detected() {
    let doc = make_doc_with_sentences(
        &[
            "That does not mean you're failing.",
            "It means your alarm system is miscalibrated.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_negation_reframe_failure(
        &doc,
        &config,
        "does not x -> it xs",
        "meaning contrast detected",
    );
}

#[test]
fn same_root_framing_contrast_detected() {
    let doc = make_doc_with_sentences(
        &["That does not reflect defiance.", "It reflects overload."],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_fails(&doc, &config, "same-root framing contrast detected");
}

#[test]
fn same_root_non_framing_pair_does_not_trigger() {
    let doc = make_doc_with_sentences(
        &[
            "That does not make the tantrums fun.",
            "It does make them easier to read.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_passes(&doc, &config, "same-root non-framing pair does not trigger");
}

#[test]
fn technical_explanation_without_same_root_does_not_trigger() {
    let doc = make_doc_with_sentences(
        &[
            "That does not mean the server is healthy.",
            "It needs a deeper health check.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_passes(
        &doc,
        &config,
        "technical explanation without same root does not trigger",
    );
}

#[test]
fn internal_state_expression_contrast_detected() {
    let doc = make_doc_with_sentences(
        &[
            "Kids who learn that crying gets no response don't stop having feelings.",
            "They stop showing them.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_negation_reframe_failure(
        &doc,
        &config,
        "don't x -> they y",
        "internal state expression contrast detected",
    );
}

#[test]
fn normal_behavioral_followup_does_not_trigger() {
    let doc = make_doc_with_sentences(
        &[
            "Children don't stop at the corner.",
            "They turn left instead.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_passes(&doc, &config, "normal behavioral followup does not trigger");
}

#[test]
fn narrative_frame_contrast_detected() {
    let doc = make_doc_with_sentences(
        &["It doesn't begin the story.", "It ends the buildup."],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_negation_reframe_failure(
        &doc,
        &config,
        "doesn't begin x -> it ends y",
        "narrative frame contrast detected",
    );
}

#[test]
fn shared_progressive_corrective_detected() {
    let doc = make_doc_with_sentences(
        &[
            "I was not living with a tiny chaos agent who woke up each day searching for weak points in my character.",
            "I was living with a child who keeps hitting the edge of her capacity and does not know that is what is happening.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_fails(&doc, &config, "shared progressive corrective detected");
}

#[test]
fn explicit_make_contrast_detected() {
    let doc = make_doc_with_sentences(
        &[
            "That doesn't make the meltdowns fun.",
            "But it makes them something I can read.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_negation_reframe_failure(
        &doc,
        &config,
        "doesn't make x -> but it makes y",
        "explicit make contrast detected",
    );
}

#[test]
fn repeated_goal_frame_detected() {
    let doc = make_doc_with_sentences(
        &[
            "The goal is not perfect confidence.",
            "The goal is to stop treating every conversation like a referendum on your worth.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_negation_reframe_failure(
        &doc,
        &config,
        "goal is not x -> goal is y",
        "repeated goal frame corrective should fail",
    );
}

#[test]
fn repeated_job_frame_detected() {
    let doc = make_doc_with_sentences(
        &[
            "Your job is not to outsmart the chemistry.",
            "Your job is to make the ten or fifteen minutes less terrifying.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_negation_reframe_failure(
        &doc,
        &config,
        "goal is not x -> goal is y",
        "repeated job frame corrective should fail",
    );
}

#[test]
fn repeated_best_result_frame_detected() {
    let doc = make_doc_with_sentences(
        &[
            "The best result is usually not instant obedience.",
            "It is a child who melts down less violently over time.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_negation_reframe_failure(
        &doc,
        &config,
        "goal is not x -> it is y",
        "best result frame corrective should fail",
    );
}

#[test]
fn repeated_answer_frame_detected() {
    let doc = make_doc_with_sentences(
        &[
            "So the answer is not tougher energy.",
            "It is steadier energy.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_negation_reframe_failure(
        &doc,
        &config,
        "goal is not x -> it is y",
        "answer frame corrective should fail",
    );
}

#[test]
fn useful_alternatives_frame_detected() {
    let doc = make_doc_with_sentences(
        &[
            "The useful alternatives are not softer punishments.",
            "They are different moves.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_negation_reframe_failure(
        &doc,
        &config,
        "goal is not x -> it is y",
        "useful alternatives corrective should fail",
    );
}

#[test]
fn make_okay_explain_contrast_detected() {
    let doc = make_doc_with_sentences(
        &[
            "That does not make the hitting okay.",
            "It does explain the pattern.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_negation_reframe_failure(
        &doc,
        &config,
        "does not make x okay -> it does explain y",
        "make-okay then explain contrast should fail",
    );
}

#[test]
fn teaches_regulation_contrast_detected() {
    let doc = make_doc_with_sentences(
        &[
            "Hitting back teaches fear and confusion.",
            "It does not teach regulation.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_negation_reframe_failure(
        &doc,
        &config,
        "x teaches y -> it does not teach z",
        "teaches versus does-not-teach-regulation contrast should fail",
    );
}

#[test]
fn pronoun_looking_for_pair_detected() {
    let doc = make_doc_with_sentences(
        &[
            "They are not looking for a TED talk in that moment.",
            "They are looking for proof that an adult is calm enough to stay.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_negation_reframe_failure(
        &doc,
        &config,
        "they are not looking for x -> they are looking for y",
        "pronoun looking-for corrective should fail",
    );
}

#[test]
fn human_plural_need_then_they_need_detected() {
    let doc = make_doc_with_sentences(
        &[
            "Volatile kids do not need speeches about self-control.",
            "They need repetition, modeling, and a family language for what to do when the heat rises.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_negation_reframe_failure(
        &doc,
        &config,
        "x does not need y -> they need z",
        "human plural subject need corrective should fail",
    );
}

#[test]
fn human_plural_contrastive_followup_detected() {
    let doc = make_doc_with_sentences(
        &[
            "Adults do not procrastinate because they love pain.",
            "They procrastinate because the task feels worse right now than the consequences do later.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_negation_reframe_failure(
        &doc,
        &config,
        "x do not y -> they z",
        "human plural corrective followup should fail",
    );
}

#[test]
fn singular_human_copular_followup_detected() {
    let doc = make_doc_with_sentences(
        &[
            "A child who keeps misbehaving after punishment is not giving you a review of your authority.",
            "They are telling you the tool missed the problem.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_negation_reframe_failure(
        &doc,
        &config,
        "x is not y -> they are z",
        "singular human corrective followup should fail",
    );
}

#[test]
fn technical_entity_not_problem_does_not_trigger_human_followup() {
    let doc = make_doc_with_sentences(
        &[
            "The parser does not need a network connection.",
            "It needs a file path and a locale.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_passes(
        &doc,
        &config,
        "non-human technical entity need corrective should pass",
    );
}

#[test]
fn legitimate_human_analysis_without_they_followup_does_not_trigger() {
    let doc = make_doc_with_sentences(
        &[
            "Adults do not respond well to noise at midnight.",
            "Sleep fragmentation raises stress the next day.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_passes(
        &doc,
        &config,
        "human subject without corrective they followup should pass",
    );
}

#[test]
fn technical_make_explain_pair_passes() {
    let doc = make_doc_with_sentences(
        &[
            "That does not make the result deterministic.",
            "It does explain the latency spike under load.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_passes(
        &doc,
        &config,
        "technical make-then-explain pair should pass",
    );
}

#[test]
fn teachs_non_regulation_object_passes() {
    let doc = make_doc_with_sentences(
        &[
            "The training video teaches the setup order.",
            "It does not teach deployment ergonomics.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_passes(
        &doc,
        &config,
        "non-regulation teaching contrast should pass",
    );
}

#[test]
fn methodological_caveat_does_not_trigger_looking_for_rule() {
    let doc = make_doc_with_sentences(
        &[
            "They are not looking for a miracle here.",
            "They are tracking remission rates across twelve months of follow-up.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_passes(&doc, &config, "non looking-for followup should pass");
}

#[test]
fn repeated_point_frame_detected() {
    let doc = make_doc_with_sentences(
        &[
            "The point is not the distance.",
            "The point is that the identity survives even on a terrible day.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_negation_reframe_failure(
        &doc,
        &config,
        "goal is not x -> goal is y",
        "repeated point frame corrective should fail",
    );
}

#[test]
fn goal_frame_with_short_interrupt_detected() {
    let doc = make_doc_with_sentences(
        &[
            "The goal is not to never lose motivation.",
            "You will.",
            "The goal is to make your next action so small that motivation becomes optional.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_negation_reframe_failure(
        &doc,
        &config,
        "goal is not x -> goal is y",
        "short interrupt should not hide a repeated goal corrective",
    );
}

#[test]
fn repeated_need_to_corrective_detected() {
    let doc = make_doc_with_sentences(
        &[
            "You do not need to sell school as magical.",
            "You need to make it familiar and survivable.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_negation_reframe_failure(
        &doc,
        &config,
        "do not need x -> need y",
        "you do not need x / you need y corrective should fail",
    );
}

#[test]
fn repeated_noun_need_corrective_detected() {
    let doc = make_doc_with_sentences(
        &[
            "The child does not need a lesson in dignity while melting down in aisle seven.",
            "The child needs a steady adult and less sensory noise.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_negation_reframe_failure(
        &doc,
        &config,
        "x does not need y -> x needs z",
        "same-subject noun need corrective should fail",
    );
}

#[test]
fn goal_without_reframe_passes() {
    let doc = make_doc_with_sentences(
        &[
            "The goal is not necessarily to eliminate screens entirely.",
            "Children still need sleep and face-to-face play.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_passes(
        &doc,
        &config,
        "goal negation without a paired corrective should pass",
    );
}

#[test]
fn noun_need_without_repeated_subject_passes() {
    let doc = make_doc_with_sentences(
        &[
            "The child does not need a lecture right now.",
            "A steadier environment usually helps more.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_passes(
        &doc,
        &config,
        "noun need sentence without repeated subject corrective should pass",
    );
}

#[test]
fn quantified_human_need_with_they_detected() {
    let doc = make_doc_with_sentences(
        &[
            "Most new moms do not need more products.",
            "They need less load.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_negation_reframe_failure(
        &doc,
        &config,
        "x does not need y -> they need z",
        "quantified human subject with they corrective should fail",
    );
}

#[test]
fn quantified_nonhuman_need_with_they_passes() {
    let doc = make_doc_with_sentences(
        &[
            "Most servers do not need more memory.",
            "They need better tuning.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_passes(
        &doc,
        &config,
        "quantified nonhuman subject with they corrective should pass",
    );
}

#[test]
fn repeated_want_transform_corrective_detected() {
    let doc = make_doc_with_sentences(
        &[
            "You do not want to crush leadership.",
            "You want to turn orders into invitations.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_negation_reframe_failure(
        &doc,
        &config,
        "do not want x -> want to turn y into z",
        "transformative repeated want corrective should fail",
    );
}

#[test]
fn repeated_want_without_transform_passes() {
    let doc = make_doc_with_sentences(
        &[
            "You do not want to click twice.",
            "You want to save the draft.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_passes(
        &doc,
        &config,
        "ordinary repeated want instruction should pass",
    );
}

#[test]
fn biggest_sign_reframe_detected() {
    let doc = make_doc_with_sentences(
        &[
            "The biggest sign is not age.",
            "It is whether your life gets easier or uglier when you start.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_negation_reframe_failure(
        &doc,
        &config,
        "goal is not x -> it is y",
        "biggest-sign corrective reframe should fail",
    );
}

#[test]
fn problem_reframe_detected() {
    let doc = make_doc_with_sentences(
        &[
            "The sensitivity is not the problem.",
            "The mismatch between the child and the environment usually is.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_negation_reframe_failure(
        &doc,
        &config,
        "x is not the problem -> y is",
        "problem corrective reframe should fail",
    );
}

#[test]
fn technical_problem_reframe_passes() {
    let doc = make_doc_with_sentences(
        &[
            "Latency is not the problem.",
            "The parser already is fast enough.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_passes(&doc, &config, "technical problem sentence should pass");
}

#[test]
fn less_more_like_pair_detected() {
    let doc = make_doc_with_sentences(
        &[
            "Less like a judge.",
            "More like someone who got there late and is trying to understand what happened before she arrived.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_fails(&doc, &config, "less more like pair detected");
}

#[test]
fn ordinary_begin_end_pair_does_not_trigger() {
    let doc = make_doc_with_sentences(
        &["The meeting doesn't begin on time.", "It ends at five."],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_passes(&doc, &config, "ordinary begin end pair does not trigger");
}

#[test]
fn lifecycle_frame_reversal_detected() {
    let doc = make_doc_with_sentences(
        &[
            "And when I miss the signs I try to remember that the screaming doesn't begin the story.",
            "It ends the buildup.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_negation_reframe_failure(
        &doc,
        &config,
        "doesn't begin x -> it ends y",
        "lifecycle frame reversal detected",
    );
}

#[test]
fn no_pattern_passes() {
    let doc = make_doc_with_sentences(
        &[
            "It works more like a philosophy than a tool.",
            "The approach is unconventional.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_passes(&doc, &config, "no negation-reframe pair should pass");
}

#[test]
fn default_config_runs() {
    let doc = make_doc_with_sentences(&["This isn't defiance.", "It's developmental."], Locale::En);
    let config = CheckConfig::default();
    assertions::assert_fails(
        &doc,
        &config,
        "default negation/reframe patterns should run",
    );
}

#[test]
fn check_id_and_label() {
    assertions::assert_check_metadata();
}

#[test]
fn pronoun_verb_mirror_we_detected() {
    let doc = make_doc_with_sentences(
        &[
            "We do not start with a logo grid.",
            "We start with the keywords the client wants to rank for.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_negation_reframe_failure(
        &doc,
        &config,
        "pronoun do not v x -> pronoun v y",
        "pronoun verb-mirror corrective detected",
    );
}

#[test]
fn pronoun_verb_mirror_they_detected() {
    let doc = make_doc_with_sentences(
        &[
            "They do not fail because people are careless.",
            "They fail because the language gets fuzzy.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_negation_reframe_failure(
        &doc,
        &config,
        "pronoun do not v x -> pronoun v y",
        "pronoun they verb-mirror corrective detected",
    );
}

#[test]
fn pronoun_verb_mirror_excludes_to_infinitive() {
    let doc = make_doc_with_sentences(
        &[
            "You do not want to click twice.",
            "You want to save the draft.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_passes(
        &doc,
        &config,
        "pronoun + want to + infinitive should not fire on verb-mirror",
    );
}

#[test]
fn agentive_np_copular_to_pronoun_reframe_detected() {
    let doc = make_doc_with_sentences(
        &[
            "The buyers searching these terms are not at the comparison stage yet.",
            "They are figuring out what BANT means.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_negation_reframe_failure(
        &doc,
        &config,
        "agentive np is not x -> pronoun is y",
        "agentive NP copular -> pronoun reframe detected",
    );
}

#[test]
fn non_agentive_np_copular_to_pronoun_does_not_trigger() {
    let doc = make_doc_with_sentences(
        &[
            "The function is not deprecated.",
            "It is part of the public API.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_passes(
        &doc,
        &config,
        "non-agentive subject (function) with pronoun reframe should pass",
    );
}

#[test]
fn subject_mirror_copular_corrective_detected() {
    let doc = make_doc_with_sentences(
        &[
            "The decision is not which keyword goes in which row.",
            "The decision is which URL gets the job.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_negation_reframe_failure(
        &doc,
        &config,
        "the x is not y -> the x is z",
        "subject-mirror copular corrective detected",
    );
}

#[test]
fn subject_mirror_problem_corrective_detected() {
    let doc = make_doc_with_sentences(
        &[
            "The problem is not that the site has too many pages.",
            "The problem is that no page has clean ownership.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_negation_reframe_failure(
        &doc,
        &config,
        "the x is not y -> the x is z",
        "subject-mirror problem corrective detected",
    );
}

#[test]
fn subject_mirror_with_pronoun_in_b_does_not_trigger() {
    let doc = make_doc_with_sentences(
        &[
            "The error is not in the parser.",
            "It is in the lexer fallback path.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_passes(
        &doc,
        &config,
        "pronoun reframe without literal subject mirror should pass",
    );
}

#[test]
fn np_modal_negation_pronoun_reframe_detected() {
    let doc = make_doc_with_sentences(
        &[
            "The page should not be a sales pitch with a demo button.",
            "It should help a startup evaluate when a CRM is needed.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_negation_reframe_failure(
        &doc,
        &config,
        "np modal not be x -> pronoun modal y",
        "np modal copular reframe detected",
    );
}

#[test]
fn np_modal_legitimate_should_not_trigger() {
    let doc = make_doc_with_sentences(
        &[
            "The deploy script should not be run on Fridays.",
            "Run it after the weekly backup completes.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_passes(
        &doc,
        &config,
        "np modal negation followed by imperative should pass",
    );
}

#[test]
fn agentive_action_verb_corrective_detected() {
    let doc = make_doc_with_sentences(
        &[
            "A searcher typing pipedrive vs hubspot does not want a general CRM page.",
            "They want a decision.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_negation_reframe_failure(
        &doc,
        &config,
        "agentive np does not v x -> they v y",
        "agentive action-verb corrective detected",
    );
}

#[test]
fn agentive_users_need_corrective_detected() {
    let doc = make_doc_with_sentences(
        &[
            "Users do not need another dashboard widget.",
            "They need fewer steps to the answer.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_negation_reframe_failure(
        &doc,
        &config,
        "agentive np does not v x -> they v y",
        "agentive users-need corrective detected",
    );
}

#[test]
fn non_agentive_technical_entity_does_not_trigger() {
    let doc = make_doc_with_sentences(
        &[
            "The parser does not need a network connection.",
            "It needs a file path and a locale.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_passes(
        &doc,
        &config,
        "non-agentive subject with it-pronoun reframe should pass",
    );
}

#[test]
fn agentive_without_intent_verb_does_not_trigger() {
    let doc = make_doc_with_sentences(
        &[
            "Users do not arrive on the homepage.",
            "They click links from search results.",
        ],
        Locale::En,
    );
    let config = config_with_signals();
    assertions::assert_passes(
        &doc,
        &config,
        "agentive subject with non-intent verb should pass",
    );
}

#[test]
fn negation_reframe_inside_blockquote_detected() {
    let sentences = make_sentences(&["This isn't defiance.", "It's developmental."]);
    let doc = Document {
        locale: Locale::En,
        sections: vec![Section {
            heading: None,
            blocks: vec![Block::BlockQuote(vec![Block::Paragraph(Paragraph {
                sentences,
                has_bold: false,
                has_italic: false,
                links: vec![],
            })])],
        }],
        metadata: DocumentMetadata::default(),
    };
    let config = config_with_signals();
    assertions::assert_fails(
        &doc,
        &config,
        "negation-reframe inside blockquote must be detected",
    );
}

#[test]
fn code_block_ignored() {
    let doc = Document {
        locale: Locale::En,
        sections: vec![Section {
            heading: None,
            blocks: vec![Block::CodeBlock("This isn't code. It's fine.".to_owned())],
        }],
        metadata: DocumentMetadata::default(),
    };
    let config = config_with_signals();
    assertions::assert_passes(
        &doc,
        &config,
        "code block content ignored → 0 matches → pass",
    );
}
