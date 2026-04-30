use prosesmasher_app_checks_cadence_patterns_runtime::DemonstrativeEmphasisCheck;
use prosesmasher_domain_types::Locale;

crate::define_rule_assertions!(
    DemonstrativeEmphasisCheck,
    "demonstrative-emphasis",
    "Demonstrative Emphasis",
    Some(&[Locale::En])
);
