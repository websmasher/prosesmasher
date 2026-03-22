//! Serde DTOs for JSON config deserialization.
//!
//! The loader accepts both the legacy `terms` / `thresholds` schema and the
//! new `quality` / `documentPolicy` schema. Both are normalized into a single
//! domain `CheckConfig`.

use std::collections::BTreeSet;

use garde::Validate;
use serde::Deserialize;

use prosesmasher_domain_types::{
    CheckConfig, ConfigError, DocumentPolicyConfig, HeuristicsConfig, LexicalConfig, Locale,
    OverrideList, QualityConfig, Range, SimplePair, TermLists, TermPool, Thresholds,
    default_quality_for_locale,
};

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ConfigDto {
    #[garde(length(chars, min = 2, max = 5))]
    pub locale: String,
    #[garde(dive)]
    #[serde(default)]
    pub terms: Option<LegacyTermListsDto>,
    #[garde(dive)]
    #[serde(default)]
    pub thresholds: Option<LegacyThresholdsDto>,
    #[garde(dive)]
    #[serde(default)]
    pub quality: Option<QualityDto>,
    #[garde(dive)]
    #[serde(default)]
    pub document_policy: Option<DocumentPolicyDto>,
}

#[derive(Deserialize, Validate, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LegacyTermListsDto {
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

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TermPoolDto {
    pub terms: Vec<String>,
    pub min_count: usize,
    #[serde(default)]
    pub allow_inflections: bool,
}

#[derive(Deserialize, Validate, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LegacyThresholdsDto {
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

#[derive(Deserialize, Validate, Default)]
#[serde(rename_all = "camelCase")]
pub struct QualityDto {
    #[garde(dive)]
    #[serde(default)]
    pub lexical: Option<LexicalDto>,
    #[garde(dive)]
    #[serde(default)]
    pub heuristics: Option<HeuristicsDto>,
}

#[derive(Deserialize, Validate, Default)]
#[serde(rename_all = "camelCase")]
pub struct LexicalDto {
    #[garde(dive)]
    #[serde(default)]
    pub prohibited_terms: Option<StringListOverrideDto>,
    #[garde(dive)]
    #[serde(default)]
    pub prohibited_substrings: Option<StringListOverrideDto>,
    #[garde(skip)]
    #[serde(default)]
    pub required_terms: Vec<String>,
    #[garde(skip)]
    #[serde(default)]
    pub required_substrings: Vec<String>,
    #[garde(skip)]
    #[serde(default)]
    pub recommended_terms: Option<TermPoolDto>,
    #[garde(dive)]
    #[serde(default)]
    pub simplicity_pairs: Option<SimplePairOverrideDto>,
}

#[derive(Deserialize, Validate, Default)]
#[serde(rename_all = "camelCase")]
pub struct HeuristicsDto {
    #[garde(skip)]
    #[serde(default)]
    pub em_dashes: Option<EnabledDto>,
    #[garde(skip)]
    #[serde(default)]
    pub smart_quotes: Option<EnabledDto>,
    #[garde(skip)]
    #[serde(default)]
    pub exclamation_density: Option<ExclamationDensityDto>,
    #[garde(skip)]
    #[serde(default)]
    pub hedge_stacking: Option<HedgeStackingDto>,
    #[garde(skip)]
    #[serde(default)]
    pub negation_reframe: Option<EnabledDto>,
    #[garde(skip)]
    #[serde(default)]
    pub triple_repeat: Option<EnabledDto>,
    #[garde(skip)]
    #[serde(default)]
    pub fake_timestamps: Option<EnabledDto>,
    #[garde(skip)]
    #[serde(default)]
    pub colon_dramatic: Option<EnabledDto>,
    #[garde(skip)]
    #[serde(default)]
    pub llm_openers: Option<EnabledDto>,
    #[garde(skip)]
    #[serde(default)]
    pub affirmation_closers: Option<EnabledDto>,
    #[garde(skip)]
    #[serde(default)]
    pub summative_closer: Option<EnabledDto>,
    #[garde(skip)]
    #[serde(default)]
    pub false_question: Option<EnabledDto>,
    #[garde(skip)]
    #[serde(default)]
    pub humble_bragger: Option<EnabledDto>,
    #[garde(skip)]
    #[serde(default)]
    pub jargon_faker: Option<EnabledDto>,
    #[garde(skip)]
    #[serde(default)]
    pub word_repetition: Option<WordRepetitionDto>,
    #[garde(skip)]
    #[serde(default)]
    pub paragraph_length: Option<ParagraphLengthDto>,
    #[garde(skip)]
    #[serde(default)]
    pub readability: Option<ReadabilityDto>,
}

#[derive(Deserialize, Validate, Default)]
#[serde(rename_all = "camelCase")]
pub struct DocumentPolicyDto {
    #[garde(skip)]
    #[serde(default)]
    pub word_count: Option<RangeDto>,
    #[garde(skip)]
    #[serde(default)]
    pub heading_counts: Option<HeadingCountsDto>,
    #[garde(skip)]
    #[serde(default)]
    pub heading_hierarchy: Option<EnabledDto>,
    #[garde(skip)]
    #[serde(default)]
    pub sentence_case_headings: Option<EnabledDto>,
    #[garde(skip)]
    #[serde(default)]
    pub bold_density: Option<BoldDensityDto>,
    #[garde(skip)]
    #[serde(default)]
    pub code_fences: Option<CodeFencesDto>,
}

#[derive(Deserialize, Validate, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StringListOverrideDto {
    #[garde(skip)]
    #[serde(default = "default_true")]
    pub defaults: bool,
    #[garde(skip)]
    #[serde(default)]
    pub add: Vec<String>,
    #[garde(skip)]
    #[serde(default)]
    pub remove: Vec<String>,
}

#[derive(Deserialize, Validate, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SimplePairOverrideDto {
    #[garde(skip)]
    #[serde(default = "default_true")]
    pub defaults: bool,
    #[garde(skip)]
    #[serde(default)]
    pub add: Vec<[String; 2]>,
    #[garde(skip)]
    #[serde(default)]
    pub remove: Vec<[String; 2]>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EnabledDto {
    #[serde(default = "default_true")]
    pub enabled: bool,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExclamationDensityDto {
    #[serde(default = "default_true")]
    pub enabled: bool,
    pub max_per_paragraph: Option<usize>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct HedgeStackingDto {
    #[serde(default = "default_true")]
    pub enabled: bool,
    pub max_per_sentence: Option<usize>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WordRepetitionDto {
    #[serde(default = "default_true")]
    pub enabled: bool,
    pub max: Option<usize>,
    #[serde(default)]
    pub excluded_terms: Option<StringListOverrideDto>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ParagraphLengthDto {
    #[serde(default = "default_true")]
    pub enabled: bool,
    pub max_sentences: Option<usize>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReadabilityDto {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_true")]
    pub defaults: bool,
    pub flesch_kincaid_min: Option<f64>,
    pub gunning_fog_max: Option<f64>,
    pub coleman_liau_max: Option<f64>,
    pub avg_sentence_length_max: Option<usize>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct HeadingCountsDto {
    #[serde(default)]
    pub h2: Option<RangeDto>,
    #[serde(default)]
    pub h3_min: Option<usize>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BoldDensityDto {
    pub min: Option<usize>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CodeFencesDto {
    pub allowed: Option<bool>,
}

#[derive(Deserialize, Clone)]
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
        let Self {
            locale,
            terms: legacy_terms_dto,
            thresholds: legacy_thresholds_dto,
            quality: quality_dto,
            document_policy: document_policy_dto,
        } = self;

        let locale = parse_locale(&locale)?;
        let mut quality = default_quality_for_locale(locale);
        let mut document_policy = DocumentPolicyConfig {
            allow_code_fences: true,
            ..DocumentPolicyConfig::default()
        };
        let using_target_schema = quality_dto.is_some() || document_policy_dto.is_some();
        let compat_terms = if using_target_schema {
            None
        } else {
            legacy_terms_dto.as_ref().map(convert_legacy_terms_direct)
        };
        let compat_thresholds = if using_target_schema {
            None
        } else {
            legacy_thresholds_dto
                .as_ref()
                .map(convert_legacy_thresholds_direct)
                .transpose()?
        };

        if let Some(legacy_terms_input) = legacy_terms_dto {
            apply_legacy_terms(&mut quality, legacy_terms_input);
        }

        if let Some(legacy_thresholds_input) = legacy_thresholds_dto {
            apply_legacy_thresholds(&mut quality, &mut document_policy, legacy_thresholds_input)?;
        }

        if let Some(quality_dto) = quality_dto {
            apply_quality_dto(&mut quality, quality_dto);
        }

        if let Some(policy_dto) = document_policy_dto {
            apply_document_policy_dto(&mut document_policy, policy_dto)?;
        }

        let legacy_terms_view = compat_terms.unwrap_or_else(|| synthesize_legacy_terms(locale, &quality));
        let legacy_thresholds_view =
            compat_thresholds.unwrap_or_else(|| synthesize_legacy_thresholds(&quality, &document_policy));

        Ok(CheckConfig {
            locale,
            quality,
            document_policy,
            terms: legacy_terms_view,
            thresholds: legacy_thresholds_view,
        })
    }
}

const fn default_true() -> bool {
    true
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
        _ => Err(ConfigError::ValidationFailed(format!("unknown locale: {s}"))),
    }
}

fn apply_legacy_terms(quality: &mut QualityConfig, dto: LegacyTermListsDto) {
    let prohibited_add = dto
        .banned_words
        .into_iter()
        .chain(dto.banned_phrases)
        .chain(dto.gendered_terms)
        .chain(dto.forbidden_terms)
        .chain(dto.race_terms)
        .collect();
    quality.lexical.prohibited_terms.add = prohibited_add;
    quality.lexical.required_terms = dto.required_terms;
    quality.lexical.recommended_terms = dto.recommended_terms.map(convert_term_pool);
    quality.lexical.simplicity_pairs.add = dto
        .simplicity_pairs
        .into_iter()
        .map(convert_simple_pair)
        .collect();

    if !dto.stop_words.is_empty() {
        quality.heuristics.word_repetition.excluded_terms.add = dto.stop_words;
    }

    if !dto.hedge_words.is_empty() {
        quality.heuristics.hedge_stacking.enabled = true;
    }

    if !dto.negation_signals.is_empty() || !dto.reframe_signals.is_empty() {
        quality.heuristics.negation_reframe.enabled = true;
    }

    if !dto.llm_openers.is_empty() {
        quality.heuristics.llm_openers.enabled = true;
    }
    if !dto.affirmation_closers.is_empty() {
        quality.heuristics.affirmation_closers.enabled = true;
    }
    if !dto.summative_patterns.is_empty() {
        quality.heuristics.summative_closer.enabled = true;
    }
    if !dto.false_question_patterns.is_empty() {
        quality.heuristics.false_question.enabled = true;
    }
    if !dto.humble_bragger_phrases.is_empty() {
        quality.heuristics.humble_bragger.enabled = true;
    }
    if !dto.jargon_faker_phrases.is_empty() {
        quality.heuristics.jargon_faker.enabled = true;
    }
}

fn apply_legacy_thresholds(
    quality: &mut QualityConfig,
    document_policy: &mut DocumentPolicyConfig,
    dto: LegacyThresholdsDto,
) -> Result<(), ConfigError> {
    document_policy.word_count = convert_range(dto.word_count, "wordCount")?;
    document_policy.heading_counts.h2 = convert_range(dto.h2_count, "h2Count")?;
    document_policy.heading_counts.h3_min = dto.h3_min;
    document_policy.bold_density_min = dto.bold_min;

    if let Some(max) = dto.max_paragraph_sentences {
        quality.heuristics.paragraph_length.max_sentences = max;
    }
    if let Some(max) = dto.max_exclamations_per_paragraph {
        quality.heuristics.exclamation_density.max_per_paragraph = max;
    }
    if let Some(max) = dto.max_hedges_per_sentence {
        quality.heuristics.hedge_stacking.max_per_sentence = max;
    }
    if dto.flesch_kincaid_min.is_some() {
        quality.heuristics.readability.flesch_kincaid_min = dto.flesch_kincaid_min;
    }
    if dto.gunning_fog_max.is_some() {
        quality.heuristics.readability.gunning_fog_max = dto.gunning_fog_max;
    }
    if dto.avg_sentence_length_max.is_some() {
        quality.heuristics.readability.avg_sentence_length_max = dto.avg_sentence_length_max;
    }
    if dto.coleman_liau_max.is_some() {
        quality.heuristics.readability.coleman_liau_max = dto.coleman_liau_max;
    }
    if let Some(max) = dto.word_repetition_max {
        quality.heuristics.word_repetition.max = max;
    }

    Ok(())
}

fn apply_quality_dto(quality: &mut QualityConfig, dto: QualityDto) {
    if let Some(lexical) = dto.lexical {
        apply_lexical_dto(&mut quality.lexical, lexical);
    }
    if let Some(heuristics) = dto.heuristics {
        apply_heuristics_dto(&mut quality.heuristics, &heuristics);
    }
}

fn apply_lexical_dto(lexical: &mut LexicalConfig, dto: LexicalDto) {
    if let Some(list) = dto.prohibited_terms {
        merge_string_override(&mut lexical.prohibited_terms, list);
    }
    if let Some(list) = dto.prohibited_substrings {
        merge_string_override(&mut lexical.prohibited_substrings, list);
    }
    if !dto.required_terms.is_empty() {
        lexical.required_terms = dto.required_terms;
    }
    if !dto.required_substrings.is_empty() {
        lexical.required_substrings = dto.required_substrings;
    }
    if let Some(pool) = dto.recommended_terms {
        lexical.recommended_terms = Some(convert_term_pool(pool));
    }
    if let Some(pairs) = dto.simplicity_pairs {
        merge_pair_override(&mut lexical.simplicity_pairs, pairs);
    }
}

fn apply_heuristics_dto(heuristics: &mut HeuristicsConfig, dto: &HeuristicsDto) {
    if let Some(config) = &dto.em_dashes {
        heuristics.em_dashes.enabled = config.enabled;
    }
    if let Some(config) = &dto.smart_quotes {
        heuristics.smart_quotes.enabled = config.enabled;
    }
    if let Some(config) = &dto.exclamation_density {
        heuristics.exclamation_density.enabled = config.enabled;
        if let Some(max) = config.max_per_paragraph {
            heuristics.exclamation_density.max_per_paragraph = max;
        }
    }
    if let Some(config) = &dto.hedge_stacking {
        heuristics.hedge_stacking.enabled = config.enabled;
        if let Some(max) = config.max_per_sentence {
            heuristics.hedge_stacking.max_per_sentence = max;
        }
    }
    if let Some(config) = &dto.negation_reframe {
        heuristics.negation_reframe.enabled = config.enabled;
    }
    if let Some(config) = &dto.triple_repeat {
        heuristics.triple_repeat.enabled = config.enabled;
    }
    if let Some(config) = &dto.fake_timestamps {
        heuristics.fake_timestamps.enabled = config.enabled;
    }
    if let Some(config) = &dto.colon_dramatic {
        heuristics.colon_dramatic.enabled = config.enabled;
    }
    if let Some(config) = &dto.llm_openers {
        heuristics.llm_openers.enabled = config.enabled;
    }
    if let Some(config) = &dto.affirmation_closers {
        heuristics.affirmation_closers.enabled = config.enabled;
    }
    if let Some(config) = &dto.summative_closer {
        heuristics.summative_closer.enabled = config.enabled;
    }
    if let Some(config) = &dto.false_question {
        heuristics.false_question.enabled = config.enabled;
    }
    if let Some(config) = &dto.humble_bragger {
        heuristics.humble_bragger.enabled = config.enabled;
    }
    if let Some(config) = &dto.jargon_faker {
        heuristics.jargon_faker.enabled = config.enabled;
    }
    apply_wordy_heuristics_dto(heuristics, dto);
}

fn apply_wordy_heuristics_dto(heuristics: &mut HeuristicsConfig, dto: &HeuristicsDto) {
    if let Some(config) = &dto.word_repetition {
        heuristics.word_repetition.enabled = config.enabled;
        if let Some(max) = config.max {
            heuristics.word_repetition.max = max;
        }
        if let Some(excluded) = config.excluded_terms.clone() {
            merge_string_override(&mut heuristics.word_repetition.excluded_terms, excluded);
        }
    }
    if let Some(config) = &dto.paragraph_length {
        heuristics.paragraph_length.enabled = config.enabled;
        if let Some(max) = config.max_sentences {
            heuristics.paragraph_length.max_sentences = max;
        }
    }
    if let Some(config) = &dto.readability {
        heuristics.readability.enabled = config.enabled;
        heuristics.readability.defaults = config.defaults;
        if config.flesch_kincaid_min.is_some() {
            heuristics.readability.flesch_kincaid_min = config.flesch_kincaid_min;
        }
        if config.gunning_fog_max.is_some() {
            heuristics.readability.gunning_fog_max = config.gunning_fog_max;
        }
        if config.coleman_liau_max.is_some() {
            heuristics.readability.coleman_liau_max = config.coleman_liau_max;
        }
        if config.avg_sentence_length_max.is_some() {
            heuristics.readability.avg_sentence_length_max = config.avg_sentence_length_max;
        }
    }
}

fn apply_document_policy_dto(
    policy: &mut DocumentPolicyConfig,
    dto: DocumentPolicyDto,
) -> Result<(), ConfigError> {
    if let Some(range) = dto.word_count {
        policy.word_count = convert_range(Some(range), "documentPolicy.wordCount")?;
    }
    if let Some(counts) = dto.heading_counts {
        if let Some(h2) = counts.h2 {
            policy.heading_counts.h2 = convert_range(Some(h2), "documentPolicy.headingCounts.h2")?;
        }
        if counts.h3_min.is_some() {
            policy.heading_counts.h3_min = counts.h3_min;
        }
    }
    if let Some(config) = dto.heading_hierarchy {
        policy.heading_hierarchy = config.enabled;
    }
    if let Some(config) = dto.sentence_case_headings {
        policy.sentence_case_headings = config.enabled;
    }
    if let Some(config) = dto.bold_density
        && config.min.is_some()
    {
        policy.bold_density_min = config.min;
    }
    if let Some(config) = dto.code_fences
        && config.allowed.is_some()
    {
        policy.allow_code_fences = config.allowed.unwrap_or(true);
    }

    Ok(())
}

fn synthesize_legacy_terms(locale: Locale, quality: &QualityConfig) -> TermLists {
    let prohibited_terms = resolve_string_override(&quality.lexical.prohibited_terms);
    let (banned_words, banned_phrases) = split_terms_and_phrases(&prohibited_terms);
    let simplicity_pairs = resolve_pair_override(&quality.lexical.simplicity_pairs);
    let repetition_excluded = resolve_string_override(&quality.heuristics.word_repetition.excluded_terms);

    TermLists {
        banned_words,
        banned_phrases,
        gendered_terms: Vec::new(),
        forbidden_terms: Vec::new(),
        race_terms: Vec::new(),
        hedge_words: default_hedge_words(locale),
        simplicity_pairs,
        negation_signals: if quality.heuristics.negation_reframe.enabled {
            default_negation_signals(locale)
        } else {
            Vec::new()
        },
        reframe_signals: if quality.heuristics.negation_reframe.enabled {
            default_reframe_signals(locale)
        } else {
            Vec::new()
        },
        llm_openers: if quality.heuristics.llm_openers.enabled {
            default_llm_openers(locale)
        } else {
            Vec::new()
        },
        affirmation_closers: if quality.heuristics.affirmation_closers.enabled {
            default_affirmation_closers(locale)
        } else {
            Vec::new()
        },
        summative_patterns: if quality.heuristics.summative_closer.enabled {
            default_summative_patterns(locale)
        } else {
            Vec::new()
        },
        false_question_patterns: if quality.heuristics.false_question.enabled {
            default_false_question_patterns(locale)
        } else {
            Vec::new()
        },
        humble_bragger_phrases: if quality.heuristics.humble_bragger.enabled {
            default_humble_bragger_phrases(locale)
        } else {
            Vec::new()
        },
        jargon_faker_phrases: if quality.heuristics.jargon_faker.enabled {
            default_jargon_faker_phrases(locale)
        } else {
            Vec::new()
        },
        stop_words: repetition_excluded,
        required_terms: quality.lexical.required_terms.clone(),
        recommended_terms: quality.lexical.recommended_terms.clone(),
    }
}

fn synthesize_legacy_thresholds(
    quality: &QualityConfig,
    document_policy: &DocumentPolicyConfig,
) -> Thresholds {
    let readability = quality.heuristics.readability;

    Thresholds {
        word_count: document_policy.word_count,
        h2_count: document_policy.heading_counts.h2,
        h3_min: document_policy.heading_counts.h3_min,
        bold_min: document_policy.bold_density_min,
        max_paragraph_sentences: quality
            .heuristics
            .paragraph_length
            .enabled
            .then_some(quality.heuristics.paragraph_length.max_sentences),
        max_exclamations_per_paragraph: quality
            .heuristics
            .exclamation_density
            .enabled
            .then_some(quality.heuristics.exclamation_density.max_per_paragraph),
        max_hedges_per_sentence: quality
            .heuristics
            .hedge_stacking
            .enabled
            .then_some(quality.heuristics.hedge_stacking.max_per_sentence),
        flesch_kincaid_min: readability.enabled.then_some(readability.flesch_kincaid_min).flatten(),
        gunning_fog_max: readability.enabled.then_some(readability.gunning_fog_max).flatten(),
        avg_sentence_length_max: readability
            .enabled
            .then_some(readability.avg_sentence_length_max)
            .flatten(),
        word_repetition_max: quality
            .heuristics
            .word_repetition
            .enabled
            .then_some(quality.heuristics.word_repetition.max),
        coleman_liau_max: readability.enabled.then_some(readability.coleman_liau_max).flatten(),
    }
}

fn convert_term_pool(dto: TermPoolDto) -> TermPool {
    TermPool {
        terms: dto.terms,
        min_count: dto.min_count,
        allow_inflections: dto.allow_inflections,
    }
}

fn convert_legacy_terms_direct(dto: &LegacyTermListsDto) -> TermLists {
    TermLists {
        banned_words: dto.banned_words.clone(),
        banned_phrases: dto.banned_phrases.clone(),
        gendered_terms: dto.gendered_terms.clone(),
        forbidden_terms: dto.forbidden_terms.clone(),
        race_terms: dto.race_terms.clone(),
        hedge_words: dto.hedge_words.clone(),
        simplicity_pairs: dto.simplicity_pairs.clone().into_iter().map(convert_simple_pair).collect(),
        negation_signals: dto.negation_signals.clone(),
        reframe_signals: dto.reframe_signals.clone(),
        llm_openers: dto.llm_openers.clone(),
        affirmation_closers: dto.affirmation_closers.clone(),
        summative_patterns: dto.summative_patterns.clone(),
        false_question_patterns: dto.false_question_patterns.clone(),
        humble_bragger_phrases: dto.humble_bragger_phrases.clone(),
        jargon_faker_phrases: dto.jargon_faker_phrases.clone(),
        stop_words: dto.stop_words.clone(),
        required_terms: dto.required_terms.clone(),
        recommended_terms: dto.recommended_terms.as_ref().map(|pool| convert_term_pool(TermPoolDto {
            terms: pool.terms.clone(),
            min_count: pool.min_count,
            allow_inflections: pool.allow_inflections,
        })),
    }
}

fn convert_legacy_thresholds_direct(dto: &LegacyThresholdsDto) -> Result<Thresholds, ConfigError> {
    Ok(Thresholds {
        word_count: convert_range(dto.word_count.clone(), "wordCount")?,
        h2_count: convert_range(dto.h2_count.clone(), "h2Count")?,
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

fn convert_simple_pair(pair: [String; 2]) -> SimplePair {
    let [complex, simple] = pair;
    SimplePair { complex, simple }
}

fn resolve_string_override(override_list: &OverrideList<String>) -> Vec<String> {
    let mut values = override_list.add.clone();

    if !override_list.remove.is_empty() {
        let remove: BTreeSet<String> = override_list.remove.iter().map(|item| item.to_lowercase()).collect();
        values.retain(|item| !remove.contains(&item.to_lowercase()));
    }

    dedupe_strings(values)
}

fn resolve_pair_override(override_list: &OverrideList<SimplePair>) -> Vec<SimplePair> {
    let mut values = override_list.add.clone();
    if !override_list.remove.is_empty() {
        values.retain(|pair| {
            !override_list.remove.iter().any(|removed| {
                removed.complex.eq_ignore_ascii_case(&pair.complex)
                    && removed.simple.eq_ignore_ascii_case(&pair.simple)
            })
        });
    }
    values
}

fn merge_string_override(current: &mut OverrideList<String>, dto: StringListOverrideDto) {
    if dto.defaults {
        current.defaults = true;
        current.add.extend(dto.add);
        current.remove = dto.remove;
    } else {
        *current = OverrideList {
            defaults: false,
            add: dto.add,
            remove: dto.remove,
        };
    }
}

fn merge_pair_override(current: &mut OverrideList<SimplePair>, dto: SimplePairOverrideDto) {
    if dto.defaults {
        current.defaults = true;
        current.add.extend(dto.add.into_iter().map(convert_simple_pair));
        current.remove = dto.remove.into_iter().map(convert_simple_pair).collect();
    } else {
        *current = OverrideList {
            defaults: false,
            add: dto.add.into_iter().map(convert_simple_pair).collect(),
            remove: dto.remove.into_iter().map(convert_simple_pair).collect(),
        };
    }
}

fn dedupe_strings(values: Vec<String>) -> Vec<String> {
    let mut seen = BTreeSet::new();
    let mut deduped = Vec::new();
    for value in values {
        let lowered = value.to_lowercase();
        if seen.insert(lowered) {
            deduped.push(value);
        }
    }
    deduped
}

type TermPhraseSplit = (Vec<String>, Vec<String>);

fn split_terms_and_phrases(values: &[String]) -> TermPhraseSplit {
    let mut words = Vec::new();
    let mut phrases = Vec::new();
    for value in values {
        if value.split_whitespace().count() > 1 {
            phrases.push(value.clone());
        } else {
            words.push(value.clone());
        }
    }
    (words, phrases)
}

fn default_hedge_words(locale: Locale) -> Vec<String> {
    if locale == Locale::En {
        vec![
            "might".to_owned(),
            "maybe".to_owned(),
            "perhaps".to_owned(),
            "somewhat".to_owned(),
            "arguably".to_owned(),
        ]
    } else {
        Vec::new()
    }
}

fn default_negation_signals(locale: Locale) -> Vec<String> {
    if locale == Locale::En {
        vec!["not".to_owned(), "isn't".to_owned(), "aren't".to_owned(), "doesn't".to_owned()]
    } else {
        Vec::new()
    }
}

fn default_reframe_signals(locale: Locale) -> Vec<String> {
    if locale == Locale::En {
        vec!["it's".to_owned(), "that's".to_owned(), "instead".to_owned(), "rather".to_owned()]
    } else {
        Vec::new()
    }
}

fn default_llm_openers(locale: Locale) -> Vec<String> {
    if locale == Locale::En {
        vec!["the interesting part is".to_owned(), "in the world of".to_owned()]
    } else {
        Vec::new()
    }
}

fn default_affirmation_closers(locale: Locale) -> Vec<String> {
    if locale == Locale::En {
        vec!["and that's the key.".to_owned(), "that's what matters.".to_owned()]
    } else {
        Vec::new()
    }
}

fn default_summative_patterns(locale: Locale) -> Vec<String> {
    if locale == Locale::En {
        vec!["and that's what makes".to_owned(), "that's why this".to_owned()]
    } else {
        Vec::new()
    }
}

fn default_false_question_patterns(locale: Locale) -> Vec<String> {
    if locale == Locale::En {
        vec!["isn't that what we all".to_owned(), "isn't that the point".to_owned()]
    } else {
        Vec::new()
    }
}

fn default_humble_bragger_phrases(locale: Locale) -> Vec<String> {
    if locale == Locale::En {
        vec!["in my experience".to_owned(), "as someone who has".to_owned(), "having worked with".to_owned()]
    } else {
        Vec::new()
    }
}

fn default_jargon_faker_phrases(locale: Locale) -> Vec<String> {
    if locale == Locale::En {
        vec![
            "debug your".to_owned(),
            "debug our".to_owned(),
            "optimizing for".to_owned(),
            "iterating on your".to_owned(),
        ]
    } else {
        Vec::new()
    }
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
