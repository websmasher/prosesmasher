use crate::locale::Locale;

/// A simple word → replacement pair for simplicity checks.
#[derive(Debug, Clone)]
pub struct SimplePair {
    pub complex: String,
    pub simple: String,
}

#[derive(Debug, Clone, Default)]
pub struct CheckConfig {
    pub locale: Locale,
    pub terms: TermLists,
    pub thresholds: Thresholds,
}

#[derive(Debug, Clone, Default)]
pub struct TermLists {
    pub banned_words: Vec<String>,
    pub banned_phrases: Vec<String>,
    pub gendered_terms: Vec<String>,
    pub forbidden_terms: Vec<String>,
    pub race_terms: Vec<String>,
    pub hedge_words: Vec<String>,
    pub simplicity_pairs: Vec<SimplePair>,
    pub negation_signals: Vec<String>,
    pub reframe_signals: Vec<String>,
    pub llm_openers: Vec<String>,
    pub affirmation_closers: Vec<String>,
    pub summative_patterns: Vec<String>,
    pub false_question_patterns: Vec<String>,
    pub humble_bragger_phrases: Vec<String>,
    pub jargon_faker_phrases: Vec<String>,
    pub stop_words: Vec<String>,
    /// All of these terms must appear in the document.
    pub required_terms: Vec<String>,
    /// At least `min_count` of these terms must appear.
    pub recommended_terms: Option<TermPool>,
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

#[derive(Debug, Clone, Default)]
pub struct Thresholds {
    pub word_count: Option<Range>,
    pub h2_count: Option<Range>,
    pub h3_min: Option<usize>,
    pub bold_min: Option<usize>,
    pub max_paragraph_sentences: Option<usize>,
    pub max_exclamations_per_paragraph: Option<usize>,
    pub max_hedges_per_sentence: Option<usize>,
    pub flesch_kincaid_min: Option<f64>,
    pub gunning_fog_max: Option<f64>,
    pub avg_sentence_length_max: Option<usize>,
    pub word_repetition_max: Option<usize>,
    pub coleman_liau_max: Option<f64>,
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
