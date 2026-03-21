//! Check collection — gathers all checks, optionally filtered by group.

use prosesmasher_app_core::check::BoxedCheck;

type CheckResult = Result<Vec<BoxedCheck>, String>;

/// Collect checks, optionally filtered by group name.
///
/// # Errors
///
/// Returns `Err` if an unknown group name is provided.
pub fn collect_checks(group: Option<&str>) -> CheckResult {
    match group {
        Some("terms") => Ok(prosesmasher_app_core::terms::all_checks()),
        Some("patterns") => Ok(prosesmasher_app_core::patterns::all_checks()),
        Some("structure") => Ok(prosesmasher_app_core::structure::all_checks()),
        Some("readability") => Ok(prosesmasher_app_core::readability::all_checks()),
        None => {
            let mut all = Vec::new();
            all.extend(prosesmasher_app_core::terms::all_checks());
            all.extend(prosesmasher_app_core::patterns::all_checks());
            all.extend(prosesmasher_app_core::structure::all_checks());
            all.extend(prosesmasher_app_core::readability::all_checks());
            Ok(all)
        }
        Some(unknown) => Err(format!("Unknown check group: {unknown}. Valid groups: terms, patterns, structure, readability")),
    }
}
