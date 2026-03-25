//! Reusable parser assertions.

pub mod lib_root;
pub mod markdown;
pub mod segmenter;
pub mod syllables;

pub use lib_root::assert_document_has_content;
pub use markdown::assert_first_paragraph;
pub use segmenter::assert_sentence_count;
