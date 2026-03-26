use prosesmasher_app_checks_document_policy_assertions::heading_hierarchy as assertions;
use prosesmasher_domain_types::{
    Block, CheckConfig, Document, DocumentMetadata, DocumentPolicyConfig, Heading, Locale,
    Paragraph, Section, Sentence,
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

fn config_enforcing_heading_hierarchy() -> CheckConfig {
    CheckConfig {
        document_policy: DocumentPolicyConfig {
            heading_hierarchy: true,
            ..DocumentPolicyConfig::default()
        },
        ..CheckConfig::default()
    }
}

#[test]
fn h1_present_fails() {
    let doc = make_headed_doc(&[(1, "My Title")]);
    let config = config_enforcing_heading_hierarchy();
    assertions::assert_h1_failure(&doc, &config, "My Title", "H1 in body should fail");
}

#[test]
fn h2_to_h4_skip_fails() {
    let doc = make_headed_doc(&[(2, "Section A"), (4, "Deep section")]);
    let config = config_enforcing_heading_hierarchy();
    assertions::assert_failure_total(
        &doc,
        &config,
        2,
        "H2→H4 skip should produce 2 failures (H4+ and skip)",
    );
}

#[test]
fn h2_h3_h2_passes() {
    let doc = make_headed_doc(&[(2, "First"), (3, "Sub"), (2, "Second")]);
    let config = config_enforcing_heading_hierarchy();
    assertions::assert_clean(&doc, &config, "H2→H3→H2 should pass");
}

#[test]
fn h4_plus_fails() {
    let doc = make_headed_doc(&[(2, "Section"), (3, "Sub"), (5, "Too deep")]);
    let config = config_enforcing_heading_hierarchy();
    assertions::assert_failure_total(&doc, &config, 2, "H5 should be flagged");
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
    let config = config_enforcing_heading_hierarchy();
    assertions::assert_clean(&doc, &config, "no headings should pass");
}

#[test]
fn check_id_and_label() {
    assertions::assert_check_metadata();
}
