//! Reusable parser assertions.

pub mod lib_root;
pub mod markdown;
pub mod segmenter;
pub mod syllables;

pub use lib_root::assert_document_has_content;
pub use markdown::{
    assert_first_paragraph, assert_first_paragraph_formatting, assert_first_paragraph_link,
    assert_first_paragraph_text, assert_heading, assert_heading_counts,
    assert_paragraph_formatting, assert_paragraph_text, assert_recursive_list_count,
    assert_section_headings, assert_single_link, assert_top_level_blockquote_count,
    assert_top_level_list_counts, assert_total_sections,
};
pub use segmenter::{
    assert_non_zero_syllables, assert_sentence_count, assert_sentence_texts, assert_word_syllable,
    assert_word_texts,
};
