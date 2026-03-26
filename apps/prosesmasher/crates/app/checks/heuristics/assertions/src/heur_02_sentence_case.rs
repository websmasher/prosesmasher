use prosesmasher_app_checks_heuristics_runtime::SentenceCaseCheck;

crate::define_rule_assertions!(
    SentenceCaseCheck,
    "sentence-case",
    "Sentence Case",
    Some(&[
        prosesmasher_domain_types::Locale::En,
        prosesmasher_domain_types::Locale::Es,
        prosesmasher_domain_types::Locale::Pt,
        prosesmasher_domain_types::Locale::Fr,
        prosesmasher_domain_types::Locale::Id
    ])
);
