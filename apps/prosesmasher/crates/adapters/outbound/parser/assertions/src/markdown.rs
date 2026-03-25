use prosesmasher_domain_types::{Block, Document, Paragraph};

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
