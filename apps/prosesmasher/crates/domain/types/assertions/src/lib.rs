//! Reusable assertions for domain types tests.

use prosesmasher_domain_types_runtime as _;

pub mod lib_root;

pub use lib_root::{
    assert_clone_preserves_display,
    assert_display_contains,
    assert_send_sync,
};
