//! Serde DTOs for JSON config deserialization.
//!
//! These mirror the domain `CheckConfig` but live in the adapter layer.
//! Domain types stay pure (no serde). Conversion happens via `into_domain`.

use garde::Validate;
use serde::Deserialize;

use prosesmasher_domain_types::{
    CheckConfig, ConfigError, Locale, Range, SimplePair, TermLists, TermPool, Thresholds,
};

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ConfigDto {
    #[garde(length(chars, min = 2, max = 5))]
    pub locale: String,
    #[garde(dive)]
    pub terms: TermListsDto,
    #[garde(dive)]
    pub thresholds: ThresholdsDto,
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TermListsDto {
    #[garde(skip)]
    #[serde(default)]
    pub banned_words: Vec<String>,
    #[garde(skip)]
    #[serde(default)]
    pub banned_phrases: Vec<String>,
    #[garde(skip)]
    #[serde(default)]
    pub gendered_terms: Vec<String>,
    #[garde(skip)]
    #[serde(default)]
    pub forbidden_terms: Vec<String>,
    #[garde(skip)]
    #[serde(default)]
    pub race_terms: Vec<String>,
    #[garde(skip)]
    #[serde(default)]
    pub hedge_words: Vec<String>,
    #[garde(skip)]
    #[serde(default)]
    pub simplicity_pairs: Vec<[String; 2]>,
    #[garde(skip)]
    #[serde(default)]
    pub negation_signals: Vec<String>,
    #[garde(skip)]
    #[serde(default)]
    pub reframe_signals: Vec<String>,
    #[garde(skip)]
    #[serde(default)]
    pub llm_openers: Vec<String>,
    #[garde(skip)]
    #[serde(default)]
    pub affirmation_closers: Vec<String>,
    #[garde(skip)]
    #[serde(default)]
    pub summative_patterns: Vec<String>,
    #[garde(skip)]
    #[serde(default)]
    pub false_question_patterns: Vec<String>,
    #[garde(skip)]
    #[serde(default)]
    pub humble_bragger_phrases: Vec<String>,
    #[garde(skip)]
    #[serde(default)]
    pub jargon_faker_phrases: Vec<String>,
    #[garde(skip)]
    #[serde(default)]
    pub stop_words: Vec<String>,
    #[garde(skip)]
    #[serde(default)]
    pub required_terms: Vec<String>,
    #[garde(skip)]
    #[serde(default)]
    pub recommended_terms: Option<TermPoolDto>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TermPoolDto {
    pub terms: Vec<String>,
    pub min_count: usize,
    #[serde(default)]
    pub allow_inflections: bool,
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ThresholdsDto {
    #[garde(skip)]
    #[serde(default)]
    pub word_count: Option<RangeDto>,
    #[garde(skip)]
    #[serde(default)]
    pub h2_count: Option<RangeDto>,
    #[garde(skip)]
    #[serde(default)]
    pub h3_min: Option<usize>,
    #[garde(skip)]
    #[serde(default)]
    pub bold_min: Option<usize>,
    #[garde(skip)]
    #[serde(default)]
    pub max_paragraph_sentences: Option<usize>,
    #[garde(skip)]
    #[serde(default)]
    pub max_exclamations_per_paragraph: Option<usize>,
    #[garde(skip)]
    #[serde(default)]
    pub max_hedges_per_sentence: Option<usize>,
    #[garde(skip)]
    #[serde(default)]
    pub flesch_kincaid_min: Option<f64>,
    #[garde(skip)]
    #[serde(default)]
    pub gunning_fog_max: Option<f64>,
    #[garde(skip)]
    #[serde(default)]
    pub avg_sentence_length_max: Option<usize>,
    #[garde(skip)]
    #[serde(default)]
    pub word_repetition_max: Option<usize>,
    #[garde(skip)]
    #[serde(default)]
    pub coleman_liau_max: Option<f64>,
}

#[derive(Deserialize)]
pub struct RangeDto {
    pub min: usize,
    pub max: usize,
}

impl ConfigDto {
    /// Convert DTO to domain `CheckConfig`.
    ///
    /// # Errors
    ///
    /// Returns `ConfigError::ValidationFailed` if locale is unknown
    /// or a range has min > max.
    pub fn into_domain(self) -> Result<CheckConfig, ConfigError> {
        let locale = parse_locale(&self.locale)?;
        let terms = convert_terms(self.terms);
        let thresholds = convert_thresholds(self.thresholds)?;

        Ok(CheckConfig {
            locale,
            terms,
            thresholds,
        })
    }
}

fn parse_locale(s: &str) -> Result<Locale, ConfigError> {
    match s.to_lowercase().as_str() {
        "en" => Ok(Locale::En),
        "ru" => Ok(Locale::Ru),
        "de" => Ok(Locale::De),
        "es" => Ok(Locale::Es),
        "pt" => Ok(Locale::Pt),
        "fr" => Ok(Locale::Fr),
        "id" => Ok(Locale::Id),
        _ => Err(ConfigError::ValidationFailed(
            format!("unknown locale: {s}"),
        )),
    }
}

fn convert_terms(dto: TermListsDto) -> TermLists {
    TermLists {
        banned_words: dto.banned_words,
        banned_phrases: dto.banned_phrases,
        gendered_terms: dto.gendered_terms,
        forbidden_terms: dto.forbidden_terms,
        race_terms: dto.race_terms,
        hedge_words: dto.hedge_words,
        simplicity_pairs: dto
            .simplicity_pairs
            .into_iter()
            .map(|pair| {
                let [complex, simple] = pair;
                SimplePair { complex, simple }
            })
            .collect(),
        negation_signals: dto.negation_signals,
        reframe_signals: dto.reframe_signals,
        llm_openers: dto.llm_openers,
        affirmation_closers: dto.affirmation_closers,
        summative_patterns: dto.summative_patterns,
        false_question_patterns: dto.false_question_patterns,
        humble_bragger_phrases: dto.humble_bragger_phrases,
        jargon_faker_phrases: dto.jargon_faker_phrases,
        stop_words: dto.stop_words,
        required_terms: dto.required_terms,
        recommended_terms: dto.recommended_terms.map(|pool| TermPool {
            terms: pool.terms,
            min_count: pool.min_count,
            allow_inflections: pool.allow_inflections,
        }),
    }
}

fn convert_thresholds(dto: ThresholdsDto) -> Result<Thresholds, ConfigError> {
    Ok(Thresholds {
        word_count: convert_range(dto.word_count, "wordCount")?,
        h2_count: convert_range(dto.h2_count, "h2Count")?,
        h3_min: dto.h3_min,
        bold_min: dto.bold_min,
        max_paragraph_sentences: dto.max_paragraph_sentences,
        max_exclamations_per_paragraph: dto.max_exclamations_per_paragraph,
        max_hedges_per_sentence: dto.max_hedges_per_sentence,
        flesch_kincaid_min: dto.flesch_kincaid_min,
        gunning_fog_max: dto.gunning_fog_max,
        avg_sentence_length_max: dto.avg_sentence_length_max,
        word_repetition_max: dto.word_repetition_max,
        coleman_liau_max: dto.coleman_liau_max,
    })
}

type OptionalRangeResult = Result<Option<Range>, ConfigError>;

fn convert_range(dto: Option<RangeDto>, field_name: &str) -> OptionalRangeResult {
    dto.map(|r| {
        Range::new(r.min, r.max).ok_or_else(|| {
            ConfigError::ValidationFailed(format!(
                "{field_name}: min ({}) must be <= max ({})",
                r.min, r.max
            ))
        })
    })
    .transpose()
}
