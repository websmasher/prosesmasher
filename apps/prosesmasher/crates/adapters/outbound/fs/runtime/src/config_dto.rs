//! Serde DTOs for canonical JSON config deserialization.

use garde::Validate;
use serde::Deserialize;

use prosesmasher_domain_types_runtime::{
    CheckConfig, ConfigError, DocumentPolicyConfig, Locale, OverrideList, Range, SimplePair,
    TermPool, default_quality_for_locale,
};

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ConfigDto {
    #[garde(length(chars, min = 2, max = 5))]
    pub locale: String,
    #[garde(dive)]
    #[serde(default)]
    pub quality: QualityDto,
    #[garde(dive)]
    #[serde(default)]
    pub document_policy: DocumentPolicyDto,
}

#[derive(Deserialize, Validate, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct QualityDto {
    #[garde(dive)]
    #[serde(default)]
    pub lexical: LexicalDto,
    #[garde(dive)]
    #[serde(default)]
    pub heuristics: HeuristicsDto,
    #[garde(dive)]
    #[serde(default)]
    pub flow: FlowDto,
    #[garde(dive)]
    #[serde(default)]
    pub readability: ReadabilityDto,
}

#[derive(Deserialize, Validate, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
pub struct HeuristicsDto {
    #[garde(skip)]
    #[serde(default)]
    pub em_dashes: Option<EnabledDto>,
    #[garde(skip)]
    #[serde(default)]
    pub sentence_case: Option<EnabledDto>,
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
    pub fragment_stacking: Option<EnabledDto>,
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
    pub llm_disclaimer: Option<EnabledDto>,
    #[garde(skip)]
    #[serde(default)]
    pub response_wrapper: Option<EnabledDto>,
    #[garde(skip)]
    #[serde(default)]
    pub generic_signposting: Option<AccumulativeDto>,
    #[garde(skip)]
    #[serde(default)]
    pub boilerplate_framing: Option<AccumulativeDto>,
    #[garde(skip)]
    #[serde(default)]
    pub llm_vocabulary: Option<AccumulativeDto>,
    #[garde(skip)]
    #[serde(default)]
    pub softening_language: Option<AccumulativeDto>,
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
}

#[derive(Deserialize, Validate, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct FlowDto {
    #[garde(skip)]
    #[serde(default)]
    pub word_repetition: Option<WordRepetitionDto>,
    #[garde(skip)]
    #[serde(default)]
    pub paragraph_length: Option<ParagraphLengthDto>,
}

#[derive(Deserialize, Validate, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
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

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct TermPoolDto {
    pub terms: Vec<String>,
    pub min_count: usize,
    #[serde(default)]
    pub allow_inflections: bool,
}

#[derive(Deserialize, Default, Clone, Copy)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct EnabledDto {
    #[serde(default = "default_true")]
    pub enabled: bool,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ExclamationDensityDto {
    #[serde(default = "default_true")]
    pub enabled: bool,
    pub max_per_paragraph: Option<usize>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct HedgeStackingDto {
    #[serde(default = "default_true")]
    pub enabled: bool,
    pub max_per_sentence: Option<usize>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct AccumulativeDto {
    #[serde(default = "default_true")]
    pub enabled: bool,
    pub max_per_document: Option<usize>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct WordRepetitionDto {
    #[serde(default = "default_true")]
    pub enabled: bool,
    pub max: Option<usize>,
    #[serde(default)]
    pub excluded_terms: Option<StringListOverrideDto>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ParagraphLengthDto {
    #[serde(default = "default_true")]
    pub enabled: bool,
    pub max_sentences: Option<usize>,
}

#[derive(Deserialize, Validate, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ReadabilityDto {
    #[garde(skip)]
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[garde(skip)]
    #[serde(default = "default_true")]
    pub defaults: bool,
    #[garde(skip)]
    pub flesch_kincaid_min: Option<f64>,
    #[garde(skip)]
    pub gunning_fog_max: Option<f64>,
    #[garde(skip)]
    pub coleman_liau_max: Option<f64>,
    #[garde(skip)]
    pub avg_sentence_length_max: Option<usize>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct HeadingCountsDto {
    #[serde(default)]
    pub h2: Option<RangeDto>,
    #[serde(default)]
    pub h3_min: Option<usize>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct BoldDensityDto {
    pub min: Option<usize>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct CodeFencesDto {
    pub allowed: Option<bool>,
}

#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct RangeDto {
    pub min: usize,
    pub max: usize,
}

type RangeResult = Result<Option<Range>, ConfigError>;

impl ConfigDto {
    /// Convert DTO to domain `CheckConfig`.
    ///
    /// # Errors
    ///
    /// Returns `ConfigError::ValidationFailed` if locale is unknown
    /// or a range has min > max.
    pub fn into_domain(self) -> Result<CheckConfig, ConfigError> {
        let locale = parse_locale(&self.locale)?;
        let legacy_sentence_case = self.document_policy.sentence_case_headings;
        let mut config = CheckConfig {
            locale,
            quality: default_quality_for_locale(locale),
            document_policy: DocumentPolicyConfig {
                allow_code_fences: true,
                ..DocumentPolicyConfig::default()
            },
        };

        apply_quality_dto(&mut config, self.quality);
        apply_document_policy_dto(&mut config.document_policy, self.document_policy)?;
        if let Some(enabled) = legacy_sentence_case {
            config.quality.heuristics.sentence_case.enabled = enabled.enabled;
        }

        Ok(config)
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
        _ => Err(ConfigError::ValidationFailed(format!(
            "unknown locale: {s}"
        ))),
    }
}

fn apply_quality_dto(config: &mut CheckConfig, dto: QualityDto) {
    apply_lexical_dto(config, dto.lexical);
    apply_heuristics_dto(config, &dto.heuristics);
    apply_flow_dto(config, dto.flow);
    apply_readability_dto(config, &dto.readability);
}

fn apply_lexical_dto(config: &mut CheckConfig, dto: LexicalDto) {
    if let Some(override_dto) = dto.prohibited_terms {
        config.quality.lexical.prohibited_terms = convert_string_override(override_dto);
    }
    if let Some(override_dto) = dto.prohibited_substrings {
        config.quality.lexical.prohibited_substrings = convert_string_override(override_dto);
    }
    if !dto.required_terms.is_empty() {
        config.quality.lexical.required_terms = dto.required_terms;
    }
    if !dto.required_substrings.is_empty() {
        config.quality.lexical.required_substrings = dto.required_substrings;
    }
    if let Some(pool) = dto.recommended_terms {
        config.quality.lexical.recommended_terms = Some(convert_term_pool(pool));
    }
    if let Some(pairs) = dto.simplicity_pairs {
        config.quality.lexical.simplicity_pairs = convert_simple_pair_override(pairs);
    }
}

const fn apply_heuristics_dto(config: &mut CheckConfig, dto: &HeuristicsDto) {
    let heuristics = &mut config.quality.heuristics;

    apply_toggle_heuristics(heuristics, dto);
}

const fn apply_toggle_heuristics(
    heuristics: &mut prosesmasher_domain_types_runtime::HeuristicsConfig,
    dto: &HeuristicsDto,
) {
    if let Some(enabled) = dto.em_dashes {
        heuristics.em_dashes.enabled = enabled.enabled;
    }
    if let Some(enabled) = dto.sentence_case {
        heuristics.sentence_case.enabled = enabled.enabled;
    }
    if let Some(enabled) = dto.smart_quotes {
        heuristics.smart_quotes.enabled = enabled.enabled;
    }
    if let Some(exclamation) = dto.exclamation_density.as_ref() {
        heuristics.exclamation_density.enabled = exclamation.enabled;
        if let Some(max) = exclamation.max_per_paragraph {
            heuristics.exclamation_density.max_per_paragraph = max;
        }
    }
    if let Some(hedge) = dto.hedge_stacking.as_ref() {
        heuristics.hedge_stacking.enabled = hedge.enabled;
        if let Some(max) = hedge.max_per_sentence {
            heuristics.hedge_stacking.max_per_sentence = max;
        }
    }
    if let Some(enabled) = dto.negation_reframe {
        heuristics.negation_reframe.enabled = enabled.enabled;
    }
    if let Some(enabled) = dto.fragment_stacking {
        heuristics.fragment_stacking.enabled = enabled.enabled;
    }
    if let Some(enabled) = dto.triple_repeat {
        heuristics.triple_repeat.enabled = enabled.enabled;
    }
    if let Some(enabled) = dto.fake_timestamps {
        heuristics.fake_timestamps.enabled = enabled.enabled;
    }
    if let Some(enabled) = dto.colon_dramatic {
        heuristics.colon_dramatic.enabled = enabled.enabled;
    }
    if let Some(enabled) = dto.llm_openers {
        heuristics.llm_openers.enabled = enabled.enabled;
    }
    if let Some(enabled) = dto.llm_disclaimer {
        heuristics.llm_disclaimer.enabled = enabled.enabled;
    }
    if let Some(enabled) = dto.response_wrapper {
        heuristics.response_wrapper.enabled = enabled.enabled;
    }
    if let Some(accumulative) = dto.generic_signposting.as_ref() {
        heuristics.generic_signposting.enabled = accumulative.enabled;
        if let Some(max) = accumulative.max_per_document {
            heuristics.generic_signposting.max_per_document = max;
        }
    }
    if let Some(accumulative) = dto.boilerplate_framing.as_ref() {
        heuristics.boilerplate_framing.enabled = accumulative.enabled;
        if let Some(max) = accumulative.max_per_document {
            heuristics.boilerplate_framing.max_per_document = max;
        }
    }
    if let Some(accumulative) = dto.llm_vocabulary.as_ref() {
        heuristics.llm_vocabulary.enabled = accumulative.enabled;
        if let Some(max) = accumulative.max_per_document {
            heuristics.llm_vocabulary.max_per_document = max;
        }
    }
    if let Some(accumulative) = dto.softening_language.as_ref() {
        heuristics.softening_language.enabled = accumulative.enabled;
        if let Some(max) = accumulative.max_per_document {
            heuristics.softening_language.max_per_document = max;
        }
    }
    if let Some(enabled) = dto.affirmation_closers {
        heuristics.affirmation_closers.enabled = enabled.enabled;
    }
    if let Some(enabled) = dto.summative_closer {
        heuristics.summative_closer.enabled = enabled.enabled;
    }
    if let Some(enabled) = dto.false_question {
        heuristics.false_question.enabled = enabled.enabled;
    }
    if let Some(enabled) = dto.humble_bragger {
        heuristics.humble_bragger.enabled = enabled.enabled;
    }
    if let Some(enabled) = dto.jargon_faker {
        heuristics.jargon_faker.enabled = enabled.enabled;
    }
}

fn apply_flow_dto(config: &mut CheckConfig, dto: FlowDto) {
    let flow = &mut config.quality.flow;

    if let Some(word_repetition) = dto.word_repetition {
        flow.word_repetition.enabled = word_repetition.enabled;
        if let Some(max) = word_repetition.max {
            flow.word_repetition.max = max;
        }
        if let Some(excluded_terms) = word_repetition.excluded_terms {
            flow.word_repetition.excluded_terms = convert_string_override(excluded_terms);
        }
    }
    if let Some(paragraph_length) = dto.paragraph_length {
        flow.paragraph_length.enabled = paragraph_length.enabled;
        if let Some(max) = paragraph_length.max_sentences {
            flow.paragraph_length.max_sentences = max;
        }
    }
}

const fn apply_readability_dto(config: &mut CheckConfig, dto: &ReadabilityDto) {
    let readability = &mut config.quality.readability;

    readability.enabled = dto.enabled;
    readability.defaults = dto.defaults;
    if !dto.defaults {
        readability.flesch_kincaid_min = None;
        readability.gunning_fog_max = None;
        readability.coleman_liau_max = None;
        readability.avg_sentence_length_max = None;
    }
    if dto.flesch_kincaid_min.is_some() {
        readability.flesch_kincaid_min = dto.flesch_kincaid_min;
    }
    if dto.gunning_fog_max.is_some() {
        readability.gunning_fog_max = dto.gunning_fog_max;
    }
    if dto.coleman_liau_max.is_some() {
        readability.coleman_liau_max = dto.coleman_liau_max;
    }
    if dto.avg_sentence_length_max.is_some() {
        readability.avg_sentence_length_max = dto.avg_sentence_length_max;
    }
}

fn apply_document_policy_dto(
    document_policy: &mut DocumentPolicyConfig,
    dto: DocumentPolicyDto,
) -> Result<(), ConfigError> {
    document_policy.word_count = convert_range(dto.word_count, "wordCount")?;
    if let Some(heading_counts) = dto.heading_counts {
        document_policy.heading_counts.h2 = convert_range(heading_counts.h2, "headingCounts.h2")?;
        document_policy.heading_counts.h3_min = heading_counts.h3_min;
    }
    if let Some(enabled) = dto.heading_hierarchy {
        document_policy.heading_hierarchy = enabled.enabled;
    }
    if let Some(bold_density) = dto.bold_density {
        document_policy.bold_density_min = bold_density.min;
    }
    if let Some(code_fences) = dto.code_fences
        && let Some(allowed) = code_fences.allowed
    {
        document_policy.allow_code_fences = allowed;
    }

    Ok(())
}

fn convert_string_override(dto: StringListOverrideDto) -> OverrideList<String> {
    OverrideList {
        defaults: dto.defaults,
        add: dto.add,
        remove: dto.remove,
    }
}

fn convert_simple_pair_override(dto: SimplePairOverrideDto) -> OverrideList<SimplePair> {
    OverrideList {
        defaults: dto.defaults,
        add: dto.add.into_iter().map(convert_simple_pair).collect(),
        remove: dto.remove.into_iter().map(convert_simple_pair).collect(),
    }
}

fn convert_simple_pair(pair: [String; 2]) -> SimplePair {
    let [complex, simple] = pair;
    SimplePair { complex, simple }
}

fn convert_term_pool(dto: TermPoolDto) -> TermPool {
    TermPool {
        terms: dto.terms,
        min_count: dto.min_count,
        allow_inflections: dto.allow_inflections,
    }
}

fn convert_range(dto: Option<RangeDto>, field: &str) -> RangeResult {
    let Some(dto) = dto else {
        return Ok(None);
    };

    Range::new(dto.min, dto.max)
        .map(Some)
        .ok_or_else(|| ConfigError::ValidationFailed(format!("{field}: min must be <= max")))
}
