//! Empty-emphasis check — flags short deictic filler lines like "That last part matters."

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Document, Locale};
use serde_json::json;

use crate::check::Check;
use crate::support::{
    collect_sentence_evidence, normalize, sentence_evidence, strip_leading_prefixes,
    strip_quoted_segments,
};

#[derive(Debug)]
pub struct EmptyEmphasisCheck;

impl Check for EmptyEmphasisCheck {
    fn id(&self) -> &'static str {
        "empty-emphasis"
    }

    fn label(&self) -> &'static str {
        "Empty Emphasis"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        Some(&[Locale::En])
    }

    fn run(&self, doc: &Document, config: &CheckConfig, suite: &mut ExpectationSuite) {
        if !config.quality.heuristics.empty_emphasis.enabled {
            return;
        }
        if doc.locale != Locale::En {
            return;
        }

        let evidence = collect_empty_emphasis_evidence(doc);
        let _result = suite
            .record_custom_values(
                "empty-emphasis",
                evidence.is_empty(),
                json!({ "min": 0, "max": 0 }),
                json!(evidence.len()),
                &evidence,
            )
            .label("Empty Emphasis")
            .checking("short deictic filler emphasis lines");
    }
}

const LEADING_PREFIXES: &[&str] = &["and ", "but ", "so ", "because "];
const EMPHASIS_REFERENTS: &[&str] = &["part", "bit"];
const EMPHASIS_QUALIFIERS: &[&str] = &["last", "first", "main"];
const WEAKENING_REFERENTS: &[&str] = &["pattern", "cycle", "loop"];

fn collect_empty_emphasis_evidence(doc: &Document) -> Vec<serde_json::Value> {
    collect_sentence_evidence(
        doc,
        |sentence, section_index, paragraph_index, sentence_index| {
            match_empty_emphasis(sentence).map(|matched_text| {
                sentence_evidence(
                    section_index,
                    paragraph_index,
                    sentence_index,
                    &[("matched_text", matched_text), ("sentence", sentence)],
                )
            })
        },
    )
}

fn match_empty_emphasis(sentence: &str) -> Option<&'static str> {
    let normalized = normalize(sentence);
    let stripped = strip_quoted_segments(strip_leading_prefixes(&normalized, LEADING_PREFIXES));
    let tokens: Vec<String> = stripped
        .split_whitespace()
        .map(|token| {
            token
                .trim_matches(|ch: char| !ch.is_alphanumeric() && ch != '\'')
                .to_owned()
        })
        .collect();

    match tokens.as_slice() {
        [deictic, referent, matters]
            if is_deictic(deictic.as_str())
                && EMPHASIS_REFERENTS.contains(&referent.as_str())
                && matters == "matters" =>
        {
            Some("deictic-part-matters")
        }
        [deictic, qualifier, referent, matters]
            if is_deictic(deictic.as_str())
                && EMPHASIS_QUALIFIERS.contains(&qualifier.as_str())
                && EMPHASIS_REFERENTS.contains(&referent.as_str())
                && matters == "matters" =>
        {
            Some("deictic-part-matters")
        }
        [deictic, one, change, helped, a, lot]
            if is_deictic(deictic.as_str())
                && one == "one"
                && change == "change"
                && helped == "helped"
                && a == "a"
                && lot == "lot" =>
        {
            Some("deictic-change-helped")
        }
        [deictic, is, telling, you, something]
            if is_deictic(deictic.as_str())
                && is == "is"
                && telling == "telling"
                && you == "you"
                && something == "something" =>
        {
            Some("deictic-telling-you-something")
        }
        [deictic, is, still, real, change]
            if is_deictic(deictic.as_str())
                && is == "is"
                && still == "still"
                && real == "real"
                && change == "change" =>
        {
            Some("deictic-real-change")
        }
        [deictic, is, how, the, referent, weakens]
            if is_deictic(deictic.as_str())
                && is == "is"
                && how == "how"
                && the == "the"
                && WEAKENING_REFERENTS.contains(&referent.as_str())
                && weakens == "weakens" =>
        {
            Some("deictic-pattern-weakens")
        }
        [what, helps, is, not, brilliant]
            if what == "what" && helps == "helps" && is == "is" && not == "not" && brilliant == "brilliant" =>
        {
            Some("what-helps-not-brilliant")
        }
        _ => None,
    }
}

fn is_deictic(token: &str) -> bool {
    token == "that" || token == "this"
}

#[cfg(test)]
#[path = "slop_09_empty_emphasis_tests/mod.rs"]
mod tests;
