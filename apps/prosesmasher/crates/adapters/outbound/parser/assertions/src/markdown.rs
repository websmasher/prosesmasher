use prosesmasher_domain_types::{Block, Document, Heading, Paragraph, Section};

fn count_matching_blocks(blocks: &[Block], predicate: fn(&Block) -> bool) -> usize {
    blocks
        .iter()
        .map(|block| match block {
            Block::BlockQuote(inner) => {
                usize::from(predicate(block)) + count_matching_blocks(inner, predicate)
            }
            Block::Paragraph(_) | Block::List(_) | Block::CodeBlock(_) => {
                usize::from(predicate(block))
            }
        })
        .sum()
}

#[must_use]
pub fn assert_first_paragraph(document: &Document) -> &Paragraph {
    let block = document
        .sections
        .first()
        .and_then(|section| section.blocks.first());
    match block {
        Some(Block::Paragraph(paragraph)) => paragraph,
        other => panic!("expected Block::Paragraph, got {other:?}"),
    }
}

pub fn assert_total_sections(document: &Document, expected: usize, context: &str) {
    assert_eq!(
        document.sections.len(),
        expected,
        "{context}: expected {expected} sections, got {}",
        document.sections.len()
    );
}

pub fn assert_heading(section: &Section, expected_level: u8, expected_text: &str, context: &str) {
    match section.heading.as_ref() {
        Some(Heading { level, text }) => {
            assert_eq!(*level, expected_level, "{context}: heading level");
            assert_eq!(text, expected_text, "{context}: heading text");
        }
        None => panic!("{context}: expected section heading"),
    }
}

pub fn assert_section_headings(
    document: &Document,
    expected: &[(u8, Option<&str>)],
    context: &str,
) {
    assert_total_sections(document, expected.len(), context);

    for (index, ((expected_level, expected_text), section)) in
        expected.iter().zip(&document.sections).enumerate()
    {
        match (expected_text, section.heading.as_ref()) {
            (None, None) => {}
            (Some(text), Some(_)) => assert_heading(section, *expected_level, text, context),
            (None, Some(heading)) => {
                panic!(
                    "{context}: section {index} expected no heading, got {:?}",
                    heading
                );
            }
            (Some(text), None) => {
                panic!(
                    "{context}: section {index} expected heading {:?} at level {}, got none",
                    text, expected_level
                );
            }
        }
    }
}

pub fn assert_heading_counts(
    document: &Document,
    expected_h1: usize,
    expected_h2: usize,
    expected_h3: usize,
    expected_h4_plus: usize,
    context: &str,
) {
    let counts = &document.metadata.heading_counts;
    assert_eq!(counts.h1, expected_h1, "{context}: h1 count");
    assert_eq!(counts.h2, expected_h2, "{context}: h2 count");
    assert_eq!(counts.h3, expected_h3, "{context}: h3 count");
    assert_eq!(counts.h4_plus, expected_h4_plus, "{context}: h4+ count");
}

pub fn assert_paragraph_text(paragraph: &Paragraph, expected: &str, context: &str) {
    let observed: String = paragraph
        .sentences
        .iter()
        .map(|sentence| sentence.text.as_str())
        .collect();
    assert_eq!(
        observed, expected,
        "{context}: expected paragraph text {expected:?}, got {observed:?}"
    );
}

#[must_use]
pub fn assert_first_paragraph_text<'a>(
    document: &'a Document,
    expected: &str,
    context: &str,
) -> &'a Paragraph {
    let paragraph = assert_first_paragraph(document);
    assert_paragraph_text(paragraph, expected, context);
    paragraph
}

pub fn assert_paragraph_formatting(
    paragraph: &Paragraph,
    expected_bold: bool,
    expected_italic: bool,
    context: &str,
) {
    assert_eq!(paragraph.has_bold, expected_bold, "{context}: has_bold");
    assert_eq!(
        paragraph.has_italic, expected_italic,
        "{context}: has_italic"
    );
}

#[must_use]
pub fn assert_first_paragraph_formatting<'a>(
    document: &'a Document,
    expected_bold: bool,
    expected_italic: bool,
    context: &str,
) -> &'a Paragraph {
    let paragraph = assert_first_paragraph(document);
    assert_paragraph_formatting(paragraph, expected_bold, expected_italic, context);
    paragraph
}

pub fn assert_single_link(
    paragraph: &Paragraph,
    expected_text: &str,
    expected_url: &str,
    context: &str,
) {
    assert_eq!(
        paragraph.links.len(),
        1,
        "{context}: expected exactly 1 link, got {}",
        paragraph.links.len()
    );

    let link = &paragraph.links[0];
    assert_eq!(link.text, expected_text, "{context}: link text");
    assert_eq!(link.url, expected_url, "{context}: link url");
}

#[must_use]
pub fn assert_first_paragraph_link<'a>(
    document: &'a Document,
    expected_text: &str,
    expected_url: &str,
    context: &str,
) -> &'a Paragraph {
    let paragraph = assert_first_paragraph(document);
    assert_single_link(paragraph, expected_text, expected_url, context);
    paragraph
}

pub fn assert_recursive_list_count(document: &Document, expected: usize, context: &str) {
    let observed: usize = document
        .sections
        .iter()
        .map(|section| {
            count_matching_blocks(&section.blocks, |block| matches!(block, Block::List(_)))
        })
        .sum();

    assert_eq!(
        observed, expected,
        "{context}: expected {expected} list blocks, got {observed}"
    );
}

pub fn assert_top_level_list_counts(
    document: &Document,
    expected_ordered: usize,
    expected_unordered: usize,
    context: &str,
) {
    let (ordered, unordered) = document
        .sections
        .iter()
        .flat_map(|section| &section.blocks)
        .fold(
            (0usize, 0usize),
            |(ordered, unordered), block| match block {
                Block::List(list) if list.ordered => (ordered.saturating_add(1), unordered),
                Block::List(_) => (ordered, unordered.saturating_add(1)),
                Block::Paragraph(_) | Block::BlockQuote(_) | Block::CodeBlock(_) => {
                    (ordered, unordered)
                }
            },
        );

    assert_eq!(
        ordered, expected_ordered,
        "{context}: expected {expected_ordered} ordered top-level lists, got {ordered}"
    );
    assert_eq!(
        unordered, expected_unordered,
        "{context}: expected {expected_unordered} unordered top-level lists, got {unordered}"
    );
}

pub fn assert_top_level_blockquote_count(document: &Document, expected: usize, context: &str) {
    let observed: usize = document
        .sections
        .iter()
        .flat_map(|section| &section.blocks)
        .filter(|block| matches!(block, Block::BlockQuote(_)))
        .count();

    assert_eq!(
        observed, expected,
        "{context}: expected {expected} top-level blockquotes, got {observed}"
    );
}
