use crate::locale::Locale;

/// A simple word → replacement pair for simplicity checks.
#[derive(Debug, Clone)]
pub struct SimplePair {
    pub complex: String,
    pub simple: String,
}

#[derive(Debug, Clone)]
pub struct CheckConfig {
    pub locale: Locale,
    pub quality: QualityConfig,
    pub document_policy: DocumentPolicyConfig,
}

impl Default for CheckConfig {
    fn default() -> Self {
        let locale = Locale::default();
        Self {
            locale,
            quality: default_quality_for_locale(locale),
            document_policy: DocumentPolicyConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct QualityConfig {
    pub lexical: LexicalConfig,
    pub heuristics: HeuristicsConfig,
    pub flow: FlowConfig,
    pub readability: ReadabilityConfig,
}

#[derive(Debug, Clone, Default)]
pub struct LexicalConfig {
    pub prohibited_terms: OverrideList<String>,
    pub prohibited_substrings: OverrideList<String>,
    pub required_terms: Vec<String>,
    pub required_substrings: Vec<String>,
    pub recommended_terms: Option<TermPool>,
    pub simplicity_pairs: OverrideList<SimplePair>,
}

#[derive(Debug, Clone)]
pub struct OverrideList<T> {
    pub defaults: bool,
    pub add: Vec<T>,
    pub remove: Vec<T>,
}

impl<T> Default for OverrideList<T> {
    fn default() -> Self {
        Self {
            defaults: true,
            add: Vec::new(),
            remove: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HeuristicsConfig {
    pub em_dashes: EnabledCheck,
    pub smart_quotes: EnabledCheck,
    pub exclamation_density: ExclamationDensityConfig,
    pub hedge_stacking: HedgeStackingConfig,
    pub negation_reframe: EnabledCheck,
    pub fragment_stacking: EnabledCheck,
    pub triple_repeat: EnabledCheck,
    pub fake_timestamps: EnabledCheck,
    pub colon_dramatic: EnabledCheck,
    pub llm_openers: EnabledCheck,
    pub affirmation_closers: EnabledCheck,
    pub summative_closer: EnabledCheck,
    pub false_question: EnabledCheck,
    pub humble_bragger: EnabledCheck,
    pub jargon_faker: EnabledCheck,
}

impl Default for HeuristicsConfig {
    fn default() -> Self {
        Self {
            em_dashes: EnabledCheck { enabled: true },
            smart_quotes: EnabledCheck { enabled: true },
            exclamation_density: ExclamationDensityConfig {
                enabled: true,
                max_per_paragraph: 1,
            },
            hedge_stacking: HedgeStackingConfig {
                enabled: true,
                max_per_sentence: 2,
            },
            negation_reframe: EnabledCheck { enabled: true },
            fragment_stacking: EnabledCheck { enabled: true },
            triple_repeat: EnabledCheck { enabled: true },
            fake_timestamps: EnabledCheck { enabled: true },
            colon_dramatic: EnabledCheck { enabled: true },
            llm_openers: EnabledCheck { enabled: true },
            affirmation_closers: EnabledCheck { enabled: true },
            summative_closer: EnabledCheck { enabled: true },
            false_question: EnabledCheck { enabled: true },
            humble_bragger: EnabledCheck { enabled: true },
            jargon_faker: EnabledCheck { enabled: true },
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct FlowConfig {
    pub word_repetition: WordRepetitionConfig,
    pub paragraph_length: ParagraphLengthConfig,
}

#[derive(Debug, Clone, Copy)]
pub struct EnabledCheck {
    pub enabled: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct ExclamationDensityConfig {
    pub enabled: bool,
    pub max_per_paragraph: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct HedgeStackingConfig {
    pub enabled: bool,
    pub max_per_sentence: usize,
}

#[derive(Debug, Clone)]
pub struct WordRepetitionConfig {
    pub enabled: bool,
    pub max: usize,
    pub excluded_terms: OverrideList<String>,
}

impl Default for WordRepetitionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max: 5,
            excluded_terms: OverrideList::default(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ParagraphLengthConfig {
    pub enabled: bool,
    pub max_sentences: usize,
}

impl Default for ParagraphLengthConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_sentences: 6,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ReadabilityConfig {
    pub enabled: bool,
    pub defaults: bool,
    pub flesch_kincaid_min: Option<f64>,
    pub gunning_fog_max: Option<f64>,
    pub coleman_liau_max: Option<f64>,
    pub avg_sentence_length_max: Option<usize>,
}

impl Default for ReadabilityConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            defaults: true,
            flesch_kincaid_min: Some(50.0),
            gunning_fog_max: Some(14.0),
            coleman_liau_max: Some(12.5),
            avg_sentence_length_max: Some(25),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DocumentPolicyConfig {
    pub word_count: Option<Range>,
    pub heading_counts: HeadingCountsPolicy,
    pub heading_hierarchy: bool,
    pub sentence_case_headings: bool,
    pub bold_density_min: Option<usize>,
    pub allow_code_fences: bool,
}

impl Default for DocumentPolicyConfig {
    fn default() -> Self {
        Self {
            word_count: None,
            heading_counts: HeadingCountsPolicy::default(),
            heading_hierarchy: false,
            sentence_case_headings: false,
            bold_density_min: None,
            allow_code_fences: true,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct HeadingCountsPolicy {
    pub h2: Option<Range>,
    pub h3_min: Option<usize>,
}

/// A pool of terms where at least `min_count` must appear.
///
/// Used for editorial keyword requirements: "this article should
/// mention at least 3 of these 10 topic keywords."
#[derive(Debug, Clone)]
pub struct TermPool {
    /// The terms to look for.
    pub terms: Vec<String>,
    /// Minimum number that must appear.
    pub min_count: usize,
    /// When true, match word stems (e.g., "screens" matches "screen").
    pub allow_inflections: bool,
}

/// A min/max range. Enforces `min <= max` at construction.
#[derive(Debug, Clone, Copy)]
pub struct Range {
    min: usize,
    max: usize,
}

impl Range {
    /// Create a new range. Returns `None` if `min > max`.
    #[must_use]
    pub const fn new(min: usize, max: usize) -> Option<Self> {
        if min > max {
            None
        } else {
            Some(Self { min, max })
        }
    }

    /// The minimum value.
    #[must_use]
    pub const fn min(self) -> usize {
        self.min
    }

    /// The maximum value.
    #[must_use]
    pub const fn max(self) -> usize {
        self.max
    }
}

#[must_use]
pub fn default_quality_for_locale(locale: Locale) -> QualityConfig {
    let mut quality = QualityConfig {
        lexical: LexicalConfig {
            prohibited_terms: OverrideList::default(),
            prohibited_substrings: OverrideList::default(),
            required_terms: Vec::new(),
            required_substrings: Vec::new(),
            recommended_terms: None,
            simplicity_pairs: OverrideList::default(),
        },
        heuristics: HeuristicsConfig::default(),
        flow: FlowConfig::default(),
        readability: ReadabilityConfig::default(),
    };

    quality.lexical.prohibited_substrings.defaults = false;

    if locale == Locale::En {
        quality.lexical.prohibited_terms.add = vec![
            "actually".to_owned(),
            "leverage".to_owned(),
            "robust".to_owned(),
            "seamless".to_owned(),
            "transformative".to_owned(),
            "unlock".to_owned(),
            "elevate".to_owned(),
            "delve".to_owned(),
            "realm".to_owned(),
            "tapestry".to_owned(),
            "let's dive in".to_owned(),
            "in today's fast-paced world".to_owned(),
            "at the end of the day".to_owned(),
            "it's important to note".to_owned(),
            "the key takeaway is".to_owned(),
            "in conclusion".to_owned(),
        ];
        quality.lexical.simplicity_pairs.add = vec![
            SimplePair { complex: "utilize".to_owned(), simple: "use".to_owned() },
            SimplePair { complex: "implement".to_owned(), simple: "do".to_owned() },
            SimplePair { complex: "facilitate".to_owned(), simple: "help".to_owned() },
            SimplePair { complex: "numerous".to_owned(), simple: "many".to_owned() },
            SimplePair { complex: "commence".to_owned(), simple: "start".to_owned() },
        ];
        quality.flow.word_repetition.excluded_terms.add = vec![
            "the".to_owned(),
            "a".to_owned(),
            "an".to_owned(),
            "is".to_owned(),
            "in".to_owned(),
            "to".to_owned(),
            "and".to_owned(),
            "of".to_owned(),
            "for".to_owned(),
            "that".to_owned(),
        ];
    } else {
        quality.lexical.prohibited_terms.defaults = false;
        quality.lexical.simplicity_pairs.defaults = false;
        quality.flow.word_repetition.excluded_terms.defaults = false;
    }

    quality
}
