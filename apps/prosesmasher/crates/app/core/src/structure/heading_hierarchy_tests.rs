use crate::check::Check;
use low_expectations::ExpectationSuite;
use prosesmasher_domain_types::{
    Block, CheckConfig, Document, DocumentMetadata, Heading, Locale, Paragraph, Section, Sentence,
};

type HeadingSpec<'a> = (u8, &'a str);

fn make_headed_doc(headings: &[HeadingSpec<'_>]) -> Document {
    let sections = headings
        .iter()
        .map(|(level, text)| Section {
            heading: Some(Heading {
                level: *level,
                text: (*text).to_owned(),
            }),
            blocks: vec![Block::Paragraph(Paragraph {
                sentences: vec![Sentence {
                    text: "Some content.".to_owned(),
                    words: vec![],
                }],
                has_bold: false,
                has_italic: false,
                links: vec![],
            })],
        })
        .collect();

    Document {
        locale: Locale::En,
        sections,
        metadata: DocumentMetadata::default(),
    }
}

#[test]
fn h1_present_fails() {
    let doc = make_headed_doc(&[(1, "My Title")]);
    let config = CheckConfig::default();
    let mut suite = ExpectationSuite::new("test");
    super::HeadingHierarchyCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.unsuccessful_expectations, 1,
        "H1 in body should fail"
    );
}

#[test]
fn h2_to_h4_skip_fails() {
    let doc = make_headed_doc(&[(2, "Section A"), (4, "Deep section")]);
    let config = CheckConfig::default();
    let mut suite = ExpectationSuite::new("test");
    super::HeadingHierarchyCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert!(
        result.statistics.unsuccessful_expectations >= 1,
        "H2→H4 skip should fail"
    );
}

#[test]
fn h2_h3_h2_passes() {
    let doc = make_headed_doc(&[(2, "First"), (3, "Sub"), (2, "Second")]);
    let config = CheckConfig::default();
    let mut suite = ExpectationSuite::new("test");
    super::HeadingHierarchyCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.unsuccessful_expectations, 0,
        "H2→H3→H2 should pass"
    );
}

#[test]
fn h4_plus_fails() {
    let doc = make_headed_doc(&[(2, "Section"), (3, "Sub"), (5, "Too deep")]);
    let config = CheckConfig::default();
    let mut suite = ExpectationSuite::new("test");
    super::HeadingHierarchyCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert!(
        result.statistics.unsuccessful_expectations >= 1,
        "H5 should be flagged"
    );
}

#[test]
fn no_headings_passes() {
    let doc = Document {
        locale: Locale::En,
        sections: vec![Section {
            heading: None,
            blocks: vec![],
        }],
        metadata: DocumentMetadata::default(),
    };
    let config = CheckConfig::default();
    let mut suite = ExpectationSuite::new("test");
    super::HeadingHierarchyCheck.run(&doc, &config, &mut suite);
    let result = suite.into_suite_result();
    assert_eq!(
        result.statistics.unsuccessful_expectations, 0,
        "no headings should pass"
    );
}

#[test]
fn check_id_and_label() {
    let check = super::HeadingHierarchyCheck;
    assert_eq!(check.id(), "heading-hierarchy", "id");
    assert_eq!(check.label(), "Heading Hierarchy", "label");
    assert!(check.supported_locales().is_none(), "supports all locales");
}
