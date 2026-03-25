//! Shared builders and result helpers for check tests.

pub mod builders;
pub mod result_helpers;

pub use builders::{
    make_doc, make_doc_code_only, make_doc_in_blockquote, make_doc_multi_section,
    make_doc_with_word_count,
};
