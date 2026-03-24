//! Check collection — gathers all checks, optionally filtered by group or ID.

use prosesmasher_app_core::check::{BoxedCheck, Check};
use prosesmasher_domain_types::Locale;
use serde::Serialize;

type CheckResult = Result<Vec<BoxedCheck>, String>;
type CheckCatalogResult = Result<Vec<CheckCatalogEntry>, String>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CheckCatalogEntry {
    pub id: String,
    pub label: String,
    pub group: String,
    pub kind: String,
    pub default_enabled: bool,
    pub supported_locales: Vec<String>,
}

const VALID_GROUPS: &str = "quality, document-policy, lexical, heuristics, flow, readability";

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
            "Unknown check group: {unknown}. Valid groups: {VALID_GROUPS}"
        )),
    }
}

#[must_use]
pub fn check_kind(id: &str) -> &'static str {
    match id {
        "prohibited-terms" | "hedge-stacking" | "simplicity" | "required-terms"
        | "recommended-terms" => "lexical",
        "paragraph-length" | "word-repetition" => "flow",
        "flesch-kincaid" | "gunning-fog" | "coleman-liau" | "avg-sentence-length" => {
            "readability"
        }
        "word-count" | "heading-hierarchy" | "h2-count" | "h3-count" | "bold-density"
        | "code-fences" => "document-policy",
        _ => "heuristics",
    }
}

/// List check metadata, optionally filtered by group.
///
/// # Errors
///
/// Returns `Err` if an unknown group name is provided.
pub fn list_checks(group: Option<&str>) -> CheckCatalogResult {
    let checks = collect_checks(group)?;
    Ok(checks
        .iter()
        .map(|check| CheckCatalogEntry {
            id: check.id().to_owned(),
            label: check.label().to_owned(),
            group: check_kind(check.id()).to_owned(),
            kind: check_kind(check.id()).to_owned(),
            default_enabled: check_kind(check.id()) != "document-policy",
            supported_locales: supported_locale_codes(check.as_ref()),
        })
        .collect())
}

fn supported_locale_codes(check: &dyn Check) -> Vec<String> {
    check.supported_locales().map_or_else(
        || vec!["all".to_owned()],
        |locales| locales.iter().map(|locale| locale_code(*locale).to_owned()).collect(),
    )
}

const fn locale_code(locale: Locale) -> &'static str {
    match locale {
        Locale::En => "en",
        Locale::Ru => "ru",
        Locale::De => "de",
        Locale::Es => "es",
        Locale::Pt => "pt",
        Locale::Fr => "fr",
        Locale::Id => "id",
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
