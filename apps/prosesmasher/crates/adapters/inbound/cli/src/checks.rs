//! Check collection — gathers all checks, optionally filtered by group or ID.

use prosesmasher_app_core::check::BoxedCheck;

type CheckResult = Result<Vec<BoxedCheck>, String>;

/// Collect checks, optionally filtered by group name.
///
/// # Errors
///
/// Returns `Err` if an unknown group name is provided.
pub fn collect_checks(group: Option<&str>) -> CheckResult {
    match group {
        Some("quality") => Ok(prosesmasher_app_core::quality::all_checks()),
        Some("document-policy") => Ok(prosesmasher_app_core::document_policy::all_checks()),
        Some("lexical") => Ok(prosesmasher_app_core::quality::lexical::all_checks()),
        Some("heuristics") => Ok(prosesmasher_app_core::quality::heuristics::all_checks()),
        Some("flow") => Ok(prosesmasher_app_core::quality::flow::all_checks()),
        Some("readability") => Ok(prosesmasher_app_core::quality::readability::all_checks()),
        None => {
            let mut all = Vec::new();
            all.extend(prosesmasher_app_core::quality::all_checks());
            all.extend(prosesmasher_app_core::document_policy::all_checks());
            Ok(all)
        }
        Some(unknown) => Err(format!(
            "Unknown check group: {unknown}. Valid groups: quality, document-policy, lexical, heuristics, flow, readability"
        )),
    }
}

/// Filter checks by comma-separated check IDs.
///
/// # Errors
///
/// Returns `Err` if any requested check ID is not found.
pub fn filter_checks_by_id(
    checks: Vec<BoxedCheck>,
    ids_csv: &str,
) -> CheckResult {
    let requested: Vec<&str> = ids_csv.split(',').map(str::trim).filter(|s| !s.is_empty()).collect();
    if requested.is_empty() {
        return Ok(checks);
    }

    // Validate all requested IDs exist
    let available_ids: Vec<&str> = checks.iter().map(|c| c.id()).collect();
    let mut unknown = Vec::new();
    for id in &requested {
        if !available_ids.contains(id) {
            unknown.push(*id);
        }
    }
    if !unknown.is_empty() {
        let available = available_ids.join(", ");
        return Err(format!(
            "Unknown check IDs: {}. Available: {available}",
            unknown.join(", ")
        ));
    }

    // Filter to only requested checks, preserving order
    let filtered = checks
        .into_iter()
        .filter(|c| requested.contains(&c.id()))
        .collect();
    Ok(filtered)
}

#[cfg(test)]
#[path = "checks_tests.rs"]
mod tests;
