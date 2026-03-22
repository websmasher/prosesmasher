//! Heading hierarchy check — validates heading levels follow a correct structure.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Document, Locale};
use serde_json::json;

use crate::check::Check;

/// Checks that headings follow a valid hierarchy: no H1 in body, no H4+, no level skips.
#[derive(Debug)]
pub struct HeadingHierarchyCheck;

impl Check for HeadingHierarchyCheck {
    fn id(&self) -> &'static str {
        "heading-hierarchy"
    }

    fn label(&self) -> &'static str {
        "Heading Hierarchy"
    }

    fn supported_locales(&self) -> Option<&'static [Locale]> {
        None
    }

    fn run(&self, doc: &Document, _config: &CheckConfig, suite: &mut ExpectationSuite) {
        let mut last_level: Option<u8> = None;

        for (section_index, section) in doc.sections.iter().enumerate() {
            let Some(heading) = &section.heading else {
                continue;
            };

            let level = heading.level;

            // Flag H1 headings — body shouldn't have H1
            if level == 1 {
                let col = format!("heading-h1-{}-{section_index}", heading.text);
                let _result = suite
                    .record_custom_values(
                        &col,
                        false,
                        json!({ "forbidden_level": 1, "rule": "no H1 headings in body" }),
                        json!({ "heading_level": 1, "heading_text": heading.text }),
                        &[json!({
                            "section_index": section_index,
                            "heading_level": 1,
                            "heading_text": heading.text,
                            "issue": "H1 in body",
                        })],
                    )
                    .label("Heading Hierarchy")
                    .checking("H1 in body");
            }

            // Flag H4+ headings — only H2/H3 allowed
            if level >= 4 {
                let col = format!("heading-h{level}-{}-{section_index}", heading.text);
                let _result = suite
                    .record_custom_values(
                        &col,
                        false,
                        json!({ "allowed_levels": [2, 3], "rule": "only H2/H3 headings allowed" }),
                        json!({ "heading_level": level, "heading_text": heading.text }),
                        &[json!({
                            "section_index": section_index,
                            "heading_level": level,
                            "heading_text": heading.text,
                            "issue": "heading too deep",
                        })],
                    )
                    .label("Heading Hierarchy")
                    .checking(&format!("H{level} in body"));
            }

            // Flag level skips (e.g., H2 → H4 skips H3). Going back up is OK.
            if let Some(prev) = last_level
                && level > prev
            {
                let expected_next = prev.saturating_add(1);
                if level > expected_next {
                    let col = format!("heading-skip-h{prev}-h{level}-{}-{section_index}", heading.text);
                    let _result = suite
                        .record_custom_values(
                            &col,
                            false,
                            json!({
                                "expected_next_level": expected_next,
                                "previous_level": prev,
                            }),
                            json!({
                                "heading_level": level,
                                "heading_text": heading.text,
                            }),
                            &[json!({
                                "section_index": section_index,
                                "previous_level": prev,
                                "expected_next_level": expected_next,
                                "heading_level": level,
                                "heading_text": heading.text,
                                "issue": "heading level skip",
                            })],
                        )
                        .label("Heading Hierarchy")
                        .checking("heading level skip");
                }
            }

            last_level = Some(level);
        }
    }
}

#[cfg(test)]
#[path = "heading_hierarchy_tests.rs"]
mod tests;
