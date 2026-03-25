//! Thin facade over the app-layer check catalog.

pub use prosesmasher_app_checks_catalog_runtime::{
    CheckCatalogEntry, check_kind, collect_checks, filter_checks_by_id, list_checks,
};
