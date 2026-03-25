use prosesmasher_app_checks_catalog_runtime::{CheckCatalogEntry, check_kind};

pub fn assert_entry_group(entry: &CheckCatalogEntry, expected_group: &str) {
    assert_eq!(entry.group, expected_group, "group");
    assert_eq!(entry.kind, expected_group, "kind");
}

pub fn assert_kind(id: &str, expected_group: &str) {
    assert_eq!(check_kind(id), expected_group, "kind");
}
