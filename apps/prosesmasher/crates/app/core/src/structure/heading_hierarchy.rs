//! Heading hierarchy check — validates heading levels follow a correct structure.

use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{CheckConfig, Document, Locale};

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

        for section in &doc.sections {
            let Some(heading) = &section.heading else {
                continue;
            };

            let level = heading.level;

            // Flag H1 headings — body shouldn't have H1
            if level == 1 {
                let col = format!("heading-h1-{}", heading.text);
                let _result = suite
                    .record_custom(
                        &col,
                        false,
                        "no H1 headings in body",
                        &format!("found H1: \"{}\"", heading.text),
                        &[format!("H1 heading found: \"{}\"", heading.text)],
                    )
                    .label("Heading Hierarchy")
                    .checking("H1 in body");
            }

            // Flag H4+ headings — only H2/H3 allowed
            if level >= 4 {
                let col = format!("heading-h{level}-{}", heading.text);
                let _result = suite
                    .record_custom(
                        &col,
                        false,
                        "only H2/H3 headings allowed",
                        &format!("found H{level}: \"{}\"", heading.text),
                        &[format!("H{level} heading found: \"{}\"", heading.text)],
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
                    let col = format!("heading-skip-h{prev}-h{level}-{}", heading.text);
                    let _result = suite
                        .record_custom(
                            &col,
                            false,
                            &format!("expected H{expected_next} after H{prev}"),
                            &format!("found H{level}: \"{}\"", heading.text),
                            &[format!(
                                "skipped from H{prev} to H{level}: \"{}\"",
                                heading.text
                            )],
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
