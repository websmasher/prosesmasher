use prosesmasher_app_checks_heuristics_runtime::FakeTimestampCheck;

crate::define_rule_assertions!(
    FakeTimestampCheck,
    "fake-timestamps",
    "Fake Timestamps",
    Some(&[prosesmasher_domain_types::Locale::En])
);
