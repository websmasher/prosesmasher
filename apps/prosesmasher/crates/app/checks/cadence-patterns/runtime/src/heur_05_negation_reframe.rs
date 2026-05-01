//! Negation-reframe check — flags corrective "X, not Y" / "not Y. It's X." rhetoric.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{Block, CheckConfig, Document, Locale, Paragraph, Sentence};
use serde_json::{Value, json};

use crate::check::Check;

const ACTION_NEGATION_PHRASES: &[&str] = &[
    "could not",
    "did not",
    "do not",
    "does not",
    "cannot",
    "can't",
    "won't",
    "would not",
    "should not",
    "have not",
    "has not",
    "had not",
];

const COPULAR_NEGATION_STARTS: &[&str] = &[
    "this isn't ",
    "that isn't ",
    "it isn't ",
    "they aren't ",
    "you aren't ",
    "we aren't ",
    "this is not ",
    "that is not ",
    "it is not ",
    "they are not ",
    "you are not ",
    "we are not ",
    "this wasn't ",
    "that wasn't ",
    "it wasn't ",
    "they weren't ",
    "you weren't ",
    "we weren't ",
];

const AFFIRMATIVE_REFRAME_STARTS: &[&str] = &[
    "it's ",
    "it is ",
    "this is ",
    "that is ",
    "they're ",
    "they are ",
    "you're ",
    "you are ",
    "we're ",
    "we are ",
];
const INFINITIVE_NEGATION_STARTS: &[&str] = &["not to "];
const INFINITIVE_REFRAME_STARTS: &[&str] = &["to "];
type FramingVerb = (&'static str, &'static str);
const CORRECTIVE_PRONOUN_REFRAME_STARTS: &[&str] = &["they ", "you ", "we ", "he ", "she ", "it "];
const INTERNAL_STATE_TERMS: &[&str] = &[
    "feeling", "feelings", "emotion", "emotions", "distress", "fear", "anger", "sadness", "grief",
    "pain",
];
const EXPRESSION_REFRAME_PHRASES: &[&str] = &[
    "stop showing",
    "hide it",
    "hide them",
    "start hiding",
    "keep it in",
    "keep them in",
    "bottle it",
    "bottle them",
    "suppress it",
    "suppress them",
];
const LIFECYCLE_NEGATION_CUES: &[&str] = &[
    "doesn't begin ",
    "does not begin ",
    "doesn't start ",
    "does not start ",
];
const LIFECYCLE_REFRAME_STARTS: &[&str] = &["it ends ", "that ends ", "this ends "];
const FRAME_BLOCKING_PREPOSITIONS: &[&str] = &[
    "on ", "at ", "by ", "after ", "before ", "when ", "during ", "with ",
];
const LESS_LIKE_STARTS: &[&str] = &["less like "];
const MORE_LIKE_STARTS: &[&str] = &["more like "];
const ABSTRACT_FRAME_NEGATIONS: &[(&str, &str)] = &[
    ("goal", "the goal is not "),
    ("goal", "the goal isn't "),
    ("point", "the point is not "),
    ("point", "the point isn't "),
    ("aim", "the aim is not "),
    ("aim", "the aim isn't "),
    ("biggest sign", "the biggest sign is not "),
    ("biggest sign", "the biggest sign isn't "),
    ("best result", "the best result is not "),
    ("best result", "the best result isn't "),
    ("answer", "the answer is not "),
    ("answer", "the answer isn't "),
    ("replacement", "the replacement is not "),
    ("replacement", "the replacement isn't "),
    ("your job", "your job is not "),
    ("your job", "your job isn't "),
    ("useful alternative", "the useful alternative is not "),
    ("useful alternative", "the useful alternative isn't "),
    ("useful alternatives", "the useful alternatives are not "),
    ("useful alternatives", "the useful alternatives aren't "),
];
const ABSTRACT_FRAME_AFFIRMATIVES: &[(&str, &str)] = &[
    ("goal", "the goal is "),
    ("point", "the point is "),
    ("aim", "the aim is "),
    ("biggest sign", "the biggest sign is "),
    ("best result", "the best result is "),
    ("answer", "the answer is "),
    ("replacement", "the replacement is "),
    ("your job", "your job is "),
    ("useful alternative", "the useful alternative is "),
    ("useful alternatives", "the useful alternatives are "),
];
const NEED_NEGATION_STARTS: &[(&str, &str)] = &[
    ("i", "i do not need to "),
    ("i", "i don't need to "),
    ("you", "you do not need to "),
    ("you", "you don't need to "),
    ("we", "we do not need to "),
    ("we", "we don't need to "),
    ("they", "they do not need to "),
    ("they", "they don't need to "),
];
const NEED_AFFIRMATIVE_STARTS: &[(&str, &str)] = &[
    ("i", "i need to "),
    ("i", "i just need to "),
    ("you", "you need to "),
    ("you", "you just need to "),
    ("we", "we need to "),
    ("we", "we just need to "),
    ("they", "they need to "),
    ("they", "they just need to "),
];
const NEED_NOUN_NEGATION_PHRASES: &[&str] = &[
    " does not need ",
    " doesn't need ",
    " do not need ",
    " don't need ",
];
const NEED_NOUN_AFFIRMATIVE_PHRASES: &[&str] = &[" needs ", " just needs "];
const NEED_PRONOUN_AFFIRMATIVE_PHRASES: &[&str] = &["they need ", "they just need "];
const HUMAN_PLURAL_NOUNS: &[&str] = &[
    "adults", "children", "families", "kids", "moms", "parents", "people", "students", "teachers",
    "women", "men",
];
const HUMAN_SINGULAR_NOUNS: &[&str] = &[
    "adult", "baby", "child", "dad", "kid", "man", "mom", "parent", "person", "student", "teacher",
    "woman",
];
const HUMAN_CORRECTIVE_PRONOUN_FOLLOWUPS: &[&str] = &[
    "they keep ",
    "they shorten ",
    "they tell ",
    "they are telling ",
    "they're telling ",
];
const CORRECTIVE_PLURAL_SUBJECTS: &[&str] = &["they", "we", "you"];
const PRESENT_COPULAR_NEGATION_FORMS: &[(&str, &str)] = &[("are not", "are"), ("aren't", "are")];
const WANT_NEGATION_STARTS: &[&str] = &["you do not want to ", "you don't want to "];
const WANT_TRANSFORM_AFFIRMATIVE_STARTS: &[&str] = &["you want to turn "];

const FRAMING_VERBS: &[FramingVerb] = &[
    ("mean", "means"),
    ("reflect", "reflects"),
    ("indicate", "indicates"),
    ("signal", "signals"),
    ("suggest", "suggests"),
];

/// Detects corrective contrast rather than generic negation:
/// - inline "X, not Y"
/// - adjacent relabeling like "This isn't defiance. It's developmental."
#[derive(Debug)]
pub struct NegationReframeCheck;

impl Check for NegationReframeCheck {
    fn id(&self) -> &'static str {
        "negation-reframe"
    }

    fn label(&self) -> &'static str {
        "Negation-Reframe Pattern"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        None
    }

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        if !config.quality.heuristics.negation_reframe.enabled {
            return;
        }
        if config.locale != Locale::En {
            return;
        }
        let evidence = collect_negation_reframe_evidence(doc);
        let _result = suite
            .record_custom_values(
                "negation-reframe",
                evidence.is_empty(),
                json!({ "min": 0, "max": 0 }),
                json!(evidence.len()),
                &evidence,
            )
            .label("Negation-Reframe Pattern")
            .checking("consecutive negation + reframe sentence pairs");
    }
}

fn collect_negation_reframe_evidence(doc: &Document) -> Vec<Value> {
    let mut evidence = Vec::new();

    for (section_index, section) in doc.sections.iter().enumerate() {
        let mut paragraph_index: usize = 0;
        for block in &section.blocks {
            collect_negation_reframe_evidence_from_block(
                block,
                section_index,
                &mut paragraph_index,
                &mut evidence,
            );
        }
    }

    evidence
}

fn collect_negation_reframe_evidence_from_block(
    block: &Block,
    section_index: usize,
    paragraph_index: &mut usize,
    evidence: &mut Vec<Value>,
) {
    match block {
        Block::Paragraph(paragraph) => {
            collect_negation_reframe_evidence_from_paragraph(
                paragraph,
                section_index,
                *paragraph_index,
                evidence,
            );
            *paragraph_index = paragraph_index.saturating_add(1);
        }
        Block::BlockQuote(inner) => {
            for inner_block in inner {
                collect_negation_reframe_evidence_from_block(
                    inner_block,
                    section_index,
                    paragraph_index,
                    evidence,
                );
            }
        }
        Block::List(_) | Block::CodeBlock(_) => {}
    }
}

fn collect_negation_reframe_evidence_from_paragraph(
    para: &Paragraph,
    section_index: usize,
    paragraph_index: usize,
    evidence: &mut Vec<Value>,
) {
    for (sentence_index, sentence) in para.sentences.iter().enumerate() {
        if let Some(item) =
            inline_corrective_evidence(sentence, section_index, paragraph_index, sentence_index)
        {
            evidence.push(item);
        }
    }

    for (sentence_index, pair) in para.sentences.windows(2).enumerate() {
        let Some(a) = pair.first() else {
            continue;
        };
        let Some(b) = pair.get(1) else {
            continue;
        };
        if let Some(item) =
            adjacent_corrective_evidence(a, b, section_index, paragraph_index, sentence_index)
        {
            evidence.push(item);
        }
    }

    for (sentence_index, triplet) in para.sentences.windows(3).enumerate() {
        let Some(a) = triplet.first() else {
            continue;
        };
        let Some(b) = triplet.get(1) else {
            continue;
        };
        let Some(c) = triplet.get(2) else {
            continue;
        };
        if let Some(item) =
            interrupted_corrective_evidence(a, b, c, section_index, paragraph_index, sentence_index)
        {
            evidence.push(item);
        }
    }
}

fn inline_corrective_evidence(
    sentence: &Sentence,
    _section_index: usize,
    _paragraph_index: usize,
    _sentence_index: usize,
) -> Option<Value> {
    let text = normalize_text(&sentence.text);
    let matched_text = inline_corrective_match(&text, sentence.word_count())?;

    Some(json!({
        "matched_text": matched_text,
        "sentence": sentence.text,
    }))
}

fn adjacent_corrective_evidence(
    a: &Sentence,
    b: &Sentence,
    _section_index: usize,
    _paragraph_index: usize,
    _sentence_index: usize,
) -> Option<Value> {
    let a_text = normalize_text(&a.text);
    let b_text = normalize_text(&b.text);

    if let Some(matched_text) =
        repeated_pronoun_looking_corrective(&a_text, &b_text, a.word_count(), b.word_count())
    {
        return Some(json!({
            "matched_text": matched_text,
            "sentence": a.text,
            "next_sentence": b.text,
        }));
    }

    if let Some(matched_text) =
        repeated_need_corrective(&a_text, &b_text, a.word_count(), b.word_count())
    {
        return Some(json!({
            "matched_text": matched_text,
            "sentence": a.text,
            "next_sentence": b.text,
        }));
    }

    if let Some(matched_text) =
        repeated_human_subject_corrective(&a_text, &b_text, a.word_count(), b.word_count())
    {
        return Some(json!({
            "matched_text": matched_text,
            "sentence": a.text,
            "next_sentence": b.text,
        }));
    }

    if !looks_like_negated_label_sentence(&a_text, a.word_count()) {
        return non_copular_corrective_evidence(a, b, &a_text, &b_text);
    }
    if !looks_like_affirmative_relabel_sentence(&b_text, b.word_count()) {
        return None;
    }

    Some(json!({
        "matched_text": "not y -> x",
        "sentence": a.text,
        "next_sentence": b.text,
    }))
}

fn non_copular_corrective_evidence(
    a: &Sentence,
    b: &Sentence,
    a_text: &str,
    b_text: &str,
) -> Option<Value> {
    if looks_like_infinitive_negation_sentence(a_text, a.word_count())
        && looks_like_infinitive_reframe_sentence(b_text, b.word_count())
    {
        return Some(json!({
            "matched_text": "not to x -> to y",
            "sentence": a.text,
            "next_sentence": b.text,
        }));
    }

    let negated_framing_verb = looks_like_framing_negation_sentence(a_text, a.word_count());
    if negated_framing_verb.is_some()
        && framing_reframe_verb(b_text, b.word_count()) == negated_framing_verb
    {
        return Some(json!({
            "matched_text": "does not x -> it xs",
            "sentence": a.text,
            "next_sentence": b.text,
        }));
    }

    if looks_like_internal_state_negation_sentence(a_text, a.word_count())
        && looks_like_expression_reframe_sentence(b_text, b.word_count())
    {
        return Some(json!({
            "matched_text": "don't x -> they y",
            "sentence": a.text,
            "next_sentence": b.text,
        }));
    }

    if looks_like_lifecycle_frame_reversal(a_text, b_text, a.word_count(), b.word_count()) {
        return Some(json!({
            "matched_text": "doesn't begin x -> it ends y",
            "sentence": a.text,
            "next_sentence": b.text,
        }));
    }

    if shared_progressive_corrective_verb(a_text, b_text, a.word_count(), b.word_count()).is_some()
    {
        return Some(json!({
            "matched_text": "i was not x -> i was x",
            "sentence": a.text,
            "next_sentence": b.text,
        }));
    }

    if looks_like_explicit_make_contrast_sentence(a_text, b_text, a.word_count(), b.word_count()) {
        return Some(json!({
            "matched_text": "doesn't make x -> but it makes y",
            "sentence": a.text,
            "next_sentence": b.text,
        }));
    }

    if looks_like_make_okay_explain_contrast_sentence(
        a_text,
        b_text,
        a.word_count(),
        b.word_count(),
    ) {
        return Some(json!({
            "matched_text": "does not make x okay -> it does explain y",
            "sentence": a.text,
            "next_sentence": b.text,
        }));
    }

    if looks_like_teaches_not_teach_regulation(a_text, b_text, a.word_count(), b.word_count()) {
        return Some(json!({
            "matched_text": "x teaches y -> it does not teach z",
            "sentence": a.text,
            "next_sentence": b.text,
        }));
    }

    if looks_like_less_more_like_pair(a_text, b_text, a.word_count(), b.word_count()) {
        return Some(json!({
            "matched_text": "less like x -> more like y",
            "sentence": a.text,
            "next_sentence": b.text,
        }));
    }

    if let Some(matched_text) =
        repeated_abstract_frame_corrective(a_text, b_text, a.word_count(), b.word_count())
    {
        return Some(json!({
            "matched_text": matched_text,
            "sentence": a.text,
            "next_sentence": b.text,
        }));
    }

    if let Some(matched_text) =
        repeated_need_corrective(a_text, b_text, a.word_count(), b.word_count())
    {
        return Some(json!({
            "matched_text": matched_text,
            "sentence": a.text,
            "next_sentence": b.text,
        }));
    }

    if let Some(matched_text) =
        repeated_want_transform_corrective(a_text, b_text, a.word_count(), b.word_count())
    {
        return Some(json!({
            "matched_text": matched_text,
            "sentence": a.text,
            "next_sentence": b.text,
        }));
    }

    if let Some(matched_text) =
        problem_reframe_corrective(a_text, b_text, a.word_count(), b.word_count())
    {
        return Some(json!({
            "matched_text": matched_text,
            "sentence": a.text,
            "next_sentence": b.text,
        }));
    }

    if let Some(matched_text) =
        repeated_subject_copular_corrective(a_text, b_text, a.word_count(), b.word_count())
    {
        return Some(json!({
            "matched_text": matched_text,
            "sentence": a.text,
            "next_sentence": b.text,
        }));
    }

    if let Some(matched_text) =
        np_modal_negation_to_pronoun_reframe(a_text, b_text, a.word_count(), b.word_count())
    {
        return Some(json!({
            "matched_text": matched_text,
            "sentence": a.text,
            "next_sentence": b.text,
        }));
    }

    if let Some(matched_text) =
        agentive_action_verb_corrective(a_text, b_text, a.word_count(), b.word_count())
    {
        return Some(json!({
            "matched_text": matched_text,
            "sentence": a.text,
            "next_sentence": b.text,
        }));
    }

    if let Some(matched_text) =
        pronoun_verb_mirror_corrective(a_text, b_text, a.word_count(), b.word_count())
    {
        return Some(json!({
            "matched_text": matched_text,
            "sentence": a.text,
            "next_sentence": b.text,
        }));
    }

    if let Some(matched_text) = agentive_np_copular_negation_to_pronoun_reframe(
        a_text,
        b_text,
        a.word_count(),
        b.word_count(),
    ) {
        return Some(json!({
            "matched_text": matched_text,
            "sentence": a.text,
            "next_sentence": b.text,
        }));
    }

    if let Some(matched_text) = np_action_verb_to_it_corrective(
        a_text,
        b_text,
        a.word_count(),
        b.word_count(),
    ) {
        return Some(json!({
            "matched_text": matched_text,
            "sentence": a.text,
            "next_sentence": b.text,
        }));
    }

    None
}

fn interrupted_corrective_evidence(
    a: &Sentence,
    b: &Sentence,
    c: &Sentence,
    _section_index: usize,
    _paragraph_index: usize,
    _sentence_index: usize,
) -> Option<Value> {
    if !looks_like_short_interrupt_sentence(&normalize_text(&b.text), b.word_count()) {
        return None;
    }

    let a_text = normalize_text(&a.text);
    let c_text = normalize_text(&c.text);

    if let Some(matched_text) =
        repeated_pronoun_looking_corrective(&a_text, &c_text, a.word_count(), c.word_count())
    {
        return Some(json!({
            "matched_text": matched_text,
            "sentence": a.text,
            "interrupting_sentence": b.text,
            "next_sentence": c.text,
        }));
    }

    if let Some(matched_text) =
        repeated_need_corrective(&a_text, &c_text, a.word_count(), c.word_count())
    {
        return Some(json!({
            "matched_text": matched_text,
            "sentence": a.text,
            "interrupting_sentence": b.text,
            "next_sentence": c.text,
        }));
    }

    if let Some(matched_text) =
        repeated_human_subject_corrective(&a_text, &c_text, a.word_count(), c.word_count())
    {
        return Some(json!({
            "matched_text": matched_text,
            "sentence": a.text,
            "interrupting_sentence": b.text,
            "next_sentence": c.text,
        }));
    }

    if let Some(matched_text) =
        repeated_abstract_frame_corrective(&a_text, &c_text, a.word_count(), c.word_count())
    {
        return Some(json!({
            "matched_text": matched_text,
            "sentence": a.text,
            "interrupting_sentence": b.text,
            "next_sentence": c.text,
        }));
    }

    if let Some(matched_text) =
        repeated_need_corrective(&a_text, &c_text, a.word_count(), c.word_count())
    {
        return Some(json!({
            "matched_text": matched_text,
            "sentence": a.text,
            "interrupting_sentence": b.text,
            "next_sentence": c.text,
        }));
    }

    repeated_want_transform_corrective(&a_text, &c_text, a.word_count(), c.word_count())
        .map(|matched_text| {
            json!({
                "matched_text": matched_text,
                "sentence": a.text,
                "interrupting_sentence": b.text,
                "next_sentence": c.text,
            })
        })
        .or_else(|| {
            problem_reframe_corrective(&a_text, &c_text, a.word_count(), c.word_count()).map(
                |matched_text| {
                    json!({
                        "matched_text": matched_text,
                        "sentence": a.text,
                        "interrupting_sentence": b.text,
                        "next_sentence": c.text,
                    })
                },
            )
        })
}

fn inline_corrective_match(text: &str, word_count: usize) -> Option<&'static str> {
    if word_count > 24 || !text.contains(" not ") {
        return None;
    }
    if contains_action_negation(text) {
        return None;
    }
    if text.contains(", not ") || text.contains(" but not ") {
        if has_copular_frame_before_not(text) {
            return Some("x, not y");
        }
    }

    None
}

fn looks_like_negated_label_sentence(text: &str, word_count: usize) -> bool {
    if word_count > 12 || contains_action_negation(text) {
        return false;
    }
    if text.starts_with("not to ") {
        return false;
    }

    text.starts_with("not ")
        || COPULAR_NEGATION_STARTS
            .iter()
            .any(|prefix| text.starts_with(prefix))
}

fn looks_like_affirmative_relabel_sentence(text: &str, word_count: usize) -> bool {
    if word_count > 8 {
        return false;
    }

    AFFIRMATIVE_REFRAME_STARTS
        .iter()
        .any(|prefix| text.starts_with(prefix))
        || is_short_nominal_label(text, word_count)
}

fn looks_like_infinitive_negation_sentence(text: &str, word_count: usize) -> bool {
    word_count <= 8
        && INFINITIVE_NEGATION_STARTS
            .iter()
            .any(|prefix| text.starts_with(prefix))
}

fn looks_like_infinitive_reframe_sentence(text: &str, word_count: usize) -> bool {
    word_count <= 16
        && INFINITIVE_REFRAME_STARTS
            .iter()
            .any(|prefix| text.starts_with(prefix))
}

fn looks_like_framing_negation_sentence(text: &str, word_count: usize) -> Option<&'static str> {
    if word_count > 20 {
        return None;
    }

    FRAMING_VERBS.iter().find_map(|(base, _third_person)| {
        [
            format!("does not {base} "),
            format!("doesn't {base} "),
            format!("did not {base} "),
        ]
        .iter()
        .any(|pattern| text.contains(pattern))
        .then_some(*base)
    })
}

fn framing_reframe_verb(text: &str, word_count: usize) -> Option<&'static str> {
    if word_count > 18 {
        return None;
    }

    FRAMING_VERBS.iter().find_map(|(base, third_person)| {
        ["it ", "this ", "that "]
            .iter()
            .map(|subject| format!("{subject}{third_person} "))
            .any(|pattern| text.starts_with(&pattern))
            .then_some(*base)
    })
}

fn looks_like_internal_state_negation_sentence(text: &str, word_count: usize) -> bool {
    if word_count > 16 {
        return false;
    }

    (text.contains("don't stop ") || text.contains("do not stop "))
        && INTERNAL_STATE_TERMS.iter().any(|term| text.contains(term))
}

fn looks_like_expression_reframe_sentence(text: &str, word_count: usize) -> bool {
    if word_count > 8
        || !CORRECTIVE_PRONOUN_REFRAME_STARTS
            .iter()
            .any(|prefix| text.starts_with(prefix))
    {
        return false;
    }

    EXPRESSION_REFRAME_PHRASES
        .iter()
        .any(|phrase| text.contains(phrase))
}

fn looks_like_lifecycle_frame_reversal(
    a_text: &str,
    b_text: &str,
    a_word_count: usize,
    b_word_count: usize,
) -> bool {
    if a_word_count > 24 || b_word_count > 8 {
        return false;
    }

    let Some(suffix) = LIFECYCLE_NEGATION_CUES
        .iter()
        .find_map(|cue| a_text.split_once(cue).map(|(_, rest)| rest))
    else {
        return false;
    };

    if FRAME_BLOCKING_PREPOSITIONS
        .iter()
        .any(|prefix| suffix.starts_with(prefix))
    {
        return false;
    }

    LIFECYCLE_REFRAME_STARTS
        .iter()
        .any(|prefix| b_text.starts_with(prefix))
}

fn shared_progressive_corrective_verb(
    a_text: &str,
    b_text: &str,
    a_word_count: usize,
    b_word_count: usize,
) -> Option<String> {
    if a_word_count > 40 || b_word_count > 32 {
        return None;
    }

    let a_prefixes = ["i was not ", "we were not ", "they were not "];
    let b_prefixes = ["i was ", "we were ", "they were "];

    for (a_prefix, b_prefix) in a_prefixes.iter().zip(b_prefixes.iter()) {
        let Some((_, a_rest)) = a_text.split_once(a_prefix) else {
            continue;
        };
        let Some((_, b_rest)) = b_text.split_once(b_prefix) else {
            continue;
        };
        let Some(a_verb) = a_rest.split_whitespace().next() else {
            continue;
        };
        let Some(b_verb) = b_rest.split_whitespace().next() else {
            continue;
        };
        if a_verb == b_verb && a_verb.ends_with("ing") {
            return Some(a_verb.to_owned());
        }
    }

    None
}

fn looks_like_explicit_make_contrast_sentence(
    a_text: &str,
    b_text: &str,
    a_word_count: usize,
    b_word_count: usize,
) -> bool {
    if a_word_count > 10 || b_word_count > 10 {
        return false;
    }

    [
        "that doesn't make ",
        "that does not make ",
        "this doesn't make ",
        "this does not make ",
        "it doesn't make ",
        "it does not make ",
    ]
    .iter()
    .any(|prefix| a_text.starts_with(prefix))
        && ["but it makes ", "but this makes ", "but that makes "]
            .iter()
            .any(|prefix| b_text.starts_with(prefix))
}

fn looks_like_make_okay_explain_contrast_sentence(
    a_text: &str,
    b_text: &str,
    a_word_count: usize,
    b_word_count: usize,
) -> bool {
    if a_word_count > 12 || b_word_count > 10 {
        return false;
    }

    [
        "that doesn't make ",
        "that does not make ",
        "this doesn't make ",
        "this does not make ",
        "it doesn't make ",
        "it does not make ",
    ]
    .iter()
    .any(|prefix| a_text.starts_with(prefix))
        && a_text.ends_with(" okay")
        && [
            "it does explain ",
            "this does explain ",
            "that does explain ",
        ]
        .iter()
        .any(|prefix| b_text.starts_with(prefix))
}

fn looks_like_teaches_not_teach_regulation(
    a_text: &str,
    b_text: &str,
    a_word_count: usize,
    b_word_count: usize,
) -> bool {
    if a_word_count > 14 || b_word_count > 8 {
        return false;
    }

    a_text.contains(" teaches ")
        && ["it does not teach ", "it doesn't teach "]
            .iter()
            .any(|prefix| b_text.starts_with(prefix))
        && ["regulation", "self-control", "restraint", "repair"]
            .iter()
            .any(|term| b_text.contains(term))
}

fn looks_like_less_more_like_pair(
    a_text: &str,
    b_text: &str,
    a_word_count: usize,
    b_word_count: usize,
) -> bool {
    a_word_count <= 6
        && b_word_count <= 18
        && LESS_LIKE_STARTS
            .iter()
            .any(|prefix| a_text.starts_with(prefix))
        && MORE_LIKE_STARTS
            .iter()
            .any(|prefix| b_text.starts_with(prefix))
}

fn repeated_abstract_frame_corrective(
    a_text: &str,
    b_text: &str,
    a_word_count: usize,
    b_word_count: usize,
) -> Option<&'static str> {
    if a_word_count > 24 || b_word_count > 20 {
        return None;
    }

    let a_text = a_text.strip_prefix("so ").unwrap_or(a_text);
    let subject = ABSTRACT_FRAME_NEGATIONS
        .iter()
        .find_map(|(subject, prefix)| {
            let usually_prefix = prefix.replace(" is not ", " is usually not ");
            let usually_contracted_prefix = prefix.replace(" isn't ", " usually isn't ");
            (a_text.starts_with(prefix)
                || a_text.starts_with(&usually_prefix)
                || a_text.starts_with(&usually_contracted_prefix))
            .then_some(*subject)
        })?;

    if ABSTRACT_FRAME_AFFIRMATIVES
        .iter()
        .any(|(candidate, prefix)| *candidate == subject && b_text.starts_with(prefix))
    {
        return Some("goal is not x -> goal is y");
    }

    (b_text.starts_with("it is ")
        || b_text.starts_with("it's ")
        || b_text.starts_with("they are ")
        || b_text.starts_with("they're "))
    .then_some("goal is not x -> it is y")
}

fn repeated_need_corrective(
    a_text: &str,
    b_text: &str,
    a_word_count: usize,
    b_word_count: usize,
) -> Option<&'static str> {
    if a_word_count > 18 || b_word_count > 18 {
        return None;
    }

    if let Some(subject) = NEED_NEGATION_STARTS
        .iter()
        .find_map(|(subject, prefix)| a_text.starts_with(prefix).then_some(*subject))
    {
        if NEED_AFFIRMATIVE_STARTS
            .iter()
            .any(|(candidate, prefix)| *candidate == subject && b_text.starts_with(prefix))
        {
            return Some("do not need x -> need y");
        }
    }

    repeated_noun_need_corrective(a_text, b_text)
}

fn repeated_noun_need_corrective(a_text: &str, b_text: &str) -> Option<&'static str> {
    for negation_phrase in NEED_NOUN_NEGATION_PHRASES {
        let Some((subject, _)) = a_text.split_once(negation_phrase) else {
            continue;
        };
        let subject_word_count = subject.split_whitespace().count();
        if !(2..=4).contains(&subject_word_count) {
            continue;
        }
        if subject == "there" {
            continue;
        }

        for affirmative_phrase in NEED_NOUN_AFFIRMATIVE_PHRASES {
            let expected = format!("{subject}{affirmative_phrase}");
            if b_text.starts_with(&expected) {
                return Some("x does not need y -> x needs z");
            }
        }

        if looks_like_human_plural_subject(subject)
            && NEED_PRONOUN_AFFIRMATIVE_PHRASES
                .iter()
                .any(|phrase| b_text.starts_with(phrase))
        {
            return Some("x does not need y -> they need z");
        }
    }

    None
}

fn repeated_want_transform_corrective(
    a_text: &str,
    b_text: &str,
    a_word_count: usize,
    b_word_count: usize,
) -> Option<&'static str> {
    if a_word_count > 18 || b_word_count > 18 {
        return None;
    }
    if !WANT_NEGATION_STARTS
        .iter()
        .any(|prefix| a_text.starts_with(prefix))
    {
        return None;
    }
    if !WANT_TRANSFORM_AFFIRMATIVE_STARTS
        .iter()
        .any(|prefix| b_text.starts_with(prefix))
    {
        return None;
    }
    b_text
        .contains(" into ")
        .then_some("do not want x -> want to turn y into z")
}

fn problem_reframe_corrective(
    a_text: &str,
    b_text: &str,
    a_word_count: usize,
    b_word_count: usize,
) -> Option<&'static str> {
    if a_word_count > 18 || b_word_count > 18 {
        return None;
    }
    let problem_negations = [" is not the problem", " isn't the problem"];
    if problem_negations
        .iter()
        .any(|suffix| a_text.ends_with(suffix))
        && (b_text.starts_with("the ") || b_text.starts_with("it "))
        && (b_text.ends_with(" is") || b_text.starts_with("it is "))
    {
        return Some("x is not the problem -> y is");
    }
    None
}

fn repeated_pronoun_looking_corrective(
    a_text: &str,
    b_text: &str,
    a_word_count: usize,
    b_word_count: usize,
) -> Option<&'static str> {
    if a_word_count > 22 || b_word_count > 22 {
        return None;
    }

    same_subject_copular_corrective(
        a_text,
        b_text,
        CORRECTIVE_PLURAL_SUBJECTS,
        PRESENT_COPULAR_NEGATION_FORMS,
        "looking for ",
        "looking for ",
    )
    .then_some("they are not looking for x -> they are looking for y")
}

fn repeated_human_subject_corrective(
    a_text: &str,
    b_text: &str,
    a_word_count: usize,
    b_word_count: usize,
) -> Option<&'static str> {
    if a_word_count > 20 || b_word_count > 18 {
        return None;
    }

    for negation_phrase in [" do not ", " don't "] {
        let Some((subject, _)) = a_text.split_once(negation_phrase) else {
            continue;
        };
        if !looks_like_human_subject(subject) {
            continue;
        }

        if repeated_human_subject_verb_corrective(a_text, b_text, negation_phrase)
            || b_text.contains(" because ") && b_text.starts_with("they ")
            || HUMAN_CORRECTIVE_PRONOUN_FOLLOWUPS
                .iter()
                .any(|prefix| b_text.starts_with(prefix))
        {
            return Some("x do not y -> they z");
        }
    }

    for negation_phrase in [" is not ", " isn't ", " are not ", " aren't "] {
        let Some((subject, _)) = a_text.split_once(negation_phrase) else {
            continue;
        };
        if !looks_like_human_subject(subject) {
            continue;
        }

        if b_text.starts_with("they are ") || b_text.starts_with("they're ") {
            return Some("x is not y -> they are z");
        }
    }

    None
}

fn repeated_human_subject_verb_corrective(
    a_text: &str,
    b_text: &str,
    negation_phrase: &str,
) -> bool {
    let Some((_, remainder)) = a_text.split_once(negation_phrase) else {
        return false;
    };
    let Some(verb) = remainder.split_whitespace().next() else {
        return false;
    };
    let expected = format!("they {verb} ");
    b_text.starts_with(&expected)
}

fn same_subject_copular_corrective(
    a_text: &str,
    b_text: &str,
    subjects: &[&str],
    copular_forms: &[(&str, &str)],
    negated_tail: &str,
    affirmative_tail: &str,
) -> bool {
    subjects.iter().any(|subject| {
        copular_forms.iter().any(|(negative_aux, affirmative_aux)| {
            let negated_prefix = format!("{subject} {negative_aux} {negated_tail}");
            let affirmative_prefix = format!("{subject} {affirmative_aux} {affirmative_tail}");
            a_text.starts_with(&negated_prefix) && b_text.starts_with(&affirmative_prefix)
        })
    })
}

fn looks_like_quantified_human_plural_subject(subject: &str) -> bool {
    let tokens: Vec<&str> = subject.split_whitespace().collect();
    let Some(first) = tokens.first() else {
        return false;
    };
    if !matches!(*first, "most" | "many" | "some") {
        return false;
    }
    let Some(last) = tokens.last() else {
        return false;
    };
    HUMAN_PLURAL_NOUNS.contains(last)
}

fn looks_like_human_plural_subject(subject: &str) -> bool {
    let tokens: Vec<&str> = subject.split_whitespace().collect();
    if !(1..=4).contains(&tokens.len()) {
        return false;
    }
    let Some(last) = tokens.last() else {
        return false;
    };
    HUMAN_PLURAL_NOUNS.contains(last)
}

fn looks_like_human_subject(subject: &str) -> bool {
    let tokens: Vec<&str> = subject.split_whitespace().collect();
    if !(1..=8).contains(&tokens.len()) {
        return false;
    }

    let Some(last) = tokens.last() else {
        return false;
    };
    HUMAN_PLURAL_NOUNS.contains(last)
        || HUMAN_SINGULAR_NOUNS.contains(last)
        || looks_like_quantified_human_plural_subject(subject)
        || starts_with_human_relative_subject(subject)
}

fn starts_with_human_relative_subject(subject: &str) -> bool {
    [
        "a child ",
        "the child ",
        "this child ",
        "that child ",
        "a parent ",
        "the parent ",
        "this parent ",
        "that parent ",
        "a kid ",
        "the kid ",
    ]
    .iter()
    .any(|prefix| subject.starts_with(prefix))
}

fn looks_like_short_interrupt_sentence(text: &str, word_count: usize) -> bool {
    word_count <= 4
        && (text == "you will"
            || text == "you won't"
            || text == "you can"
            || text == "you can't"
            || text == "it will"
            || text == "it won't"
            || text == "they will"
            || text == "they won't")
}

fn contains_action_negation(text: &str) -> bool {
    ACTION_NEGATION_PHRASES
        .iter()
        .any(|phrase| text.contains(phrase))
}

fn has_copular_frame_before_not(text: &str) -> bool {
    let Some((before_not, _)) = text.split_once(" not ") else {
        return false;
    };

    [
        " is ", " are ", " was ", " were ", " be ", " being ", " been ",
    ]
    .iter()
    .any(|cue| before_not.contains(cue))
        || before_not.ends_with("'s")
        || before_not.ends_with("'re")
}

fn is_short_nominal_label(text: &str, word_count: usize) -> bool {
    if word_count == 0 || word_count > 3 {
        return false;
    }

    !text.contains(' ')
        || ![
            " is ", " are ", " was ", " were ", " have ", " has ", " had ", " do ", " does ",
        ]
        .iter()
        .any(|cue| text.contains(cue))
}

const SUBJECT_DETERMINERS: &[&str] = &[
    "the ", "a ", "an ", "this ", "that ", "these ", "those ", "your ", "our ", "my ",
];

const COPULAR_AUXILIARIES: &[&str] = &["is", "are", "was", "were"];

const SUBJECT_LEADING_PREFIXES: &[&str] = &["so ", "but ", "and ", "however, ", "however "];

/// Pattern A: literal NP subject mirror with copular preserved.
/// "The decision is not X. The decision is Y."
/// "The problem is not X. The problem is Y."
fn repeated_subject_copular_corrective(
    a_text: &str,
    b_text: &str,
    a_word_count: usize,
    b_word_count: usize,
) -> Option<&'static str> {
    if a_word_count > 28 || b_word_count > 28 {
        return None;
    }

    let a_text = strip_subject_leading_prefix(a_text);
    let b_text = strip_subject_leading_prefix(b_text);

    for copula in COPULAR_AUXILIARIES {
        let neg_pattern = format!(" {copula} not ");
        let aff_pattern = format!(" {copula} ");

        let Some((a_subject, _)) = a_text.split_once(&neg_pattern) else {
            continue;
        };

        if !subject_starts_with_determiner(a_subject) {
            continue;
        }
        let subject_word_count = a_subject.split_whitespace().count();
        if !(2..=8).contains(&subject_word_count) {
            continue;
        }

        let expected_b_prefix = format!("{a_subject}{aff_pattern}");
        if b_text.starts_with(&expected_b_prefix) && !b_text.contains(&neg_pattern) {
            return Some("the x is not y -> the x is z");
        }
    }

    None
}

/// Pattern B: NP + modal copular negation followed by pronoun reframe with same modal.
/// "The page should not be X. It should help Y."
/// "The system must not be Y. It should produce Z."
fn np_modal_negation_to_pronoun_reframe(
    a_text: &str,
    b_text: &str,
    a_word_count: usize,
    b_word_count: usize,
) -> Option<&'static str> {
    if a_word_count > 32 || b_word_count > 32 {
        return None;
    }

    let a_text = strip_subject_leading_prefix(a_text);
    let b_text = strip_subject_leading_prefix(b_text);

    for modal in ["should", "could", "would", "must"] {
        let neg_be = format!(" {modal} not be ");
        let neg_bare = format!(" {modal} not ");

        let Some((a_subject, _)) = a_text
            .split_once(&neg_be)
            .or_else(|| a_text.split_once(&neg_bare))
        else {
            continue;
        };

        if !subject_starts_with_determiner(a_subject) {
            continue;
        }
        let subject_word_count = a_subject.split_whitespace().count();
        if !(2..=8).contains(&subject_word_count) {
            continue;
        }

        for pronoun in ["it", "they", "this", "that"] {
            let prefix = format!("{pronoun} {modal} ");
            if b_text.starts_with(&prefix) && !b_text.contains(&neg_bare) {
                return Some("np modal not be x -> pronoun modal y");
            }
        }
    }

    None
}

/// Pattern C: agentive NP + action-verb negation, pronoun reframe with same verb.
/// "A searcher does not want X. They want Y."
/// "Users do not need X. They need Y."
const AGENTIVE_NOUNS: &[&str] = &[
    "user",
    "users",
    "customer",
    "customers",
    "visitor",
    "visitors",
    "buyer",
    "buyers",
    "reader",
    "readers",
    "viewer",
    "viewers",
    "consumer",
    "consumers",
    "client",
    "clients",
    "searcher",
    "searchers",
    "shopper",
    "shoppers",
    "subscriber",
    "subscribers",
    "founder",
    "founders",
    "leader",
    "leaders",
    "manager",
    "managers",
    "audience",
    "prospect",
    "prospects",
    "candidate",
    "candidates",
];

const AGENTIVE_INTENT_VERBS: &[&str] = &[
    "want", "wants", "need", "needs", "expect", "expects", "prefer", "prefers", "deserve",
    "deserves", "demand", "demands", "seek", "seeks",
];

fn agentive_action_verb_corrective(
    a_text: &str,
    b_text: &str,
    a_word_count: usize,
    b_word_count: usize,
) -> Option<&'static str> {
    if a_word_count > 24 || b_word_count > 20 {
        return None;
    }

    let a_text = strip_subject_leading_prefix(a_text);
    let b_text = strip_subject_leading_prefix(b_text);

    for negation in [" does not ", " doesn't ", " do not ", " don't "] {
        let Some((subject, remainder)) = a_text.split_once(negation) else {
            continue;
        };
        if !subject_starts_with_determiner(subject)
            && !subject.split_whitespace().count().eq(&1)
        {
            continue;
        }
        if !looks_like_agentive_subject(subject) {
            continue;
        }
        let Some(verb) = remainder.split_whitespace().next() else {
            continue;
        };
        if !AGENTIVE_INTENT_VERBS.contains(&verb) {
            continue;
        }

        let mirrored_verb = mirror_intent_verb_for_pronoun(verb);
        let expected_prefix = format!("they {mirrored_verb} ");
        if b_text.starts_with(&expected_prefix) {
            return Some("agentive np does not v x -> they v y");
        }
    }

    None
}

/// Pattern: pronoun + verb-mirror corrective.
/// "We do not start with X. We start with Y."
/// "You do not need to X. You need to Y."  (already handled but we accept it too)
/// "They do not see X. They see Y."
const PRONOUN_VERB_MIRROR_SUBJECTS: &[&str] = &["we", "you", "they", "i"];

fn pronoun_verb_mirror_corrective(
    a_text: &str,
    b_text: &str,
    a_word_count: usize,
    b_word_count: usize,
) -> Option<&'static str> {
    if a_word_count > 24 || b_word_count > 36 {
        return None;
    }

    let a_text = strip_subject_leading_prefix(a_text);
    let b_text = strip_subject_leading_prefix(b_text);

    for subject in PRONOUN_VERB_MIRROR_SUBJECTS {
        for negation in [" do not ", " don't ", " does not ", " doesn't "] {
            let prefix = format!("{subject}{negation}");
            let Some(rest) = a_text.strip_prefix(&prefix) else {
                continue;
            };
            let mut tokens = rest.split_whitespace();
            let Some(verb) = tokens.next() else {
                continue;
            };
            if !is_plausible_action_verb(verb) {
                continue;
            }
            // Skip "[V] to [INFINITIVE]" — that's instructional, not slop drumbeat.
            if tokens.next() == Some("to") {
                continue;
            }
            let mirrored_b_prefix = format!("{subject} {verb} ");
            if b_text.starts_with(&mirrored_b_prefix) {
                return Some("pronoun do not v x -> pronoun v y");
            }
        }
    }

    None
}

/// Pattern: NP + "does not [V] X. It [V]s Y." with subject-verb agreement conjugation.
/// "Google does not rank the companies you compete against. It ranks whoever owns the questions."
/// "The map does not show every detail. It shows the relevant ones."
fn np_action_verb_to_it_corrective(
    a_text: &str,
    b_text: &str,
    a_word_count: usize,
    b_word_count: usize,
) -> Option<&'static str> {
    if a_word_count > 28 || b_word_count > 36 {
        return None;
    }

    let a_text = strip_subject_leading_prefix(a_text);
    let b_text = strip_subject_leading_prefix(b_text);

    let Some((subject, rest)) = a_text.split_once(" does not ") else {
        return None;
    };
    let subject_word_count = subject.split_whitespace().count();
    if !(1..=2).contains(&subject_word_count) {
        return None;
    }
    if PRONOUN_VERB_MIRROR_SUBJECTS.iter().any(|p| subject == *p) {
        return None;
    }
    // Skip determiner-led subjects ("the parser", "a request") — those collide
    // with legitimate technical descriptions of behavior. Bare proper-noun-style
    // subjects ("Google", "Slack") are the slop signal.
    if subject_starts_with_determiner(subject) {
        return None;
    }
    if subject.split_whitespace().any(|tok| tok == "the" || tok == "a" || tok == "an") {
        return None;
    }

    let rest = strip_optional_adverb(rest);
    let mut tokens = rest.split_whitespace();
    let verb = tokens.next()?;
    if !is_plausible_action_verb(verb) {
        return None;
    }
    if tokens.next() == Some("to") {
        return None;
    }

    let s2_after_it = b_text.strip_prefix("it ")?;
    if s2_after_it.starts_with("does not ") || s2_after_it.starts_with("doesn't ") {
        return None;
    }
    let s2_verb = s2_after_it.split_whitespace().next()?;
    if !is_third_person_singular_of(s2_verb, verb) {
        return None;
    }

    Some("np does not v x -> it vs y")
}

fn strip_optional_adverb(text: &str) -> &str {
    for adv in ["always ", "necessarily ", "usually ", "really ", "actually ", "just "] {
        if let Some(rest) = text.strip_prefix(adv) {
            return rest;
        }
    }
    text
}

fn is_third_person_singular_of(candidate: &str, base: &str) -> bool {
    if candidate == base {
        return true;
    }
    if candidate == &format!("{base}s") {
        return true;
    }
    if candidate == &format!("{base}es") {
        return true;
    }
    if base.ends_with('y') && candidate == &format!("{}ies", &base[..base.len() - 1]) {
        return true;
    }
    false
}

fn is_plausible_action_verb(token: &str) -> bool {
    if token.is_empty() {
        return false;
    }
    let first = token.chars().next().unwrap_or(' ');
    if !first.is_ascii_lowercase() {
        return false;
    }
    if matches!(
        token,
        "to" | "the" | "a" | "an" | "of" | "in" | "on" | "at" | "by" | "for" | "with" | "and"
            | "or" | "but" | "is" | "are" | "was" | "were" | "be" | "being" | "been"
    ) {
        return false;
    }
    if token.len() < 2 {
        return false;
    }
    true
}

/// Pattern: agentive NP + copular negation followed by pronoun reframe.
/// "The buyers searching these terms are not at the comparison stage. They are figuring out X."
/// "The user is not the customer. They are the one paying."
fn agentive_np_copular_negation_to_pronoun_reframe(
    a_text: &str,
    b_text: &str,
    a_word_count: usize,
    b_word_count: usize,
) -> Option<&'static str> {
    if a_word_count > 28 || b_word_count > 28 {
        return None;
    }

    let a_text = strip_subject_leading_prefix(a_text);
    let b_text = strip_subject_leading_prefix(b_text);

    for (copula, pronoun_form) in [
        ("is", "it is "),
        ("is", "it's "),
        ("is", "they are "),
        ("is", "they're "),
        ("are", "they are "),
        ("are", "they're "),
        ("was", "it was "),
        ("was", "they were "),
        ("were", "they were "),
    ] {
        let neg_pattern = format!(" {copula} not ");
        let Some((subject, _)) = a_text.split_once(&neg_pattern) else {
            continue;
        };
        if !subject_starts_with_determiner(subject) {
            continue;
        }
        if !looks_like_agentive_subject(subject) {
            continue;
        }
        if b_text.starts_with(pronoun_form) && !b_text.contains(&neg_pattern) {
            return Some("agentive np is not x -> pronoun is y");
        }
    }

    None
}

fn mirror_intent_verb_for_pronoun(verb: &str) -> &str {
    match verb {
        "wants" => "want",
        "needs" => "need",
        "expects" => "expect",
        "prefers" => "prefer",
        "deserves" => "deserve",
        "demands" => "demand",
        "seeks" => "seek",
        other => other,
    }
}

fn looks_like_agentive_subject(subject: &str) -> bool {
    let tokens: Vec<&str> = subject.split_whitespace().collect();
    if tokens.is_empty() || tokens.len() > 8 {
        return false;
    }
    let head_window = tokens.len().min(4);
    tokens[..head_window].iter().any(|token| {
        AGENTIVE_NOUNS.contains(token)
            || HUMAN_PLURAL_NOUNS.contains(token)
            || HUMAN_SINGULAR_NOUNS.contains(token)
    })
}

fn subject_starts_with_determiner(subject: &str) -> bool {
    SUBJECT_DETERMINERS
        .iter()
        .any(|prefix| subject.starts_with(prefix))
}

fn strip_subject_leading_prefix(text: &str) -> &str {
    for prefix in SUBJECT_LEADING_PREFIXES {
        if let Some(rest) = text.strip_prefix(prefix) {
            return rest;
        }
    }
    text
}

fn normalize_text(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|ch| {
            if ch.is_alphanumeric() || ch.is_whitespace() || ch == '\'' || ch == ',' {
                ch
            } else {
                ' '
            }
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
#[path = "heur_05_negation_reframe_tests/mod.rs"]
mod tests;
