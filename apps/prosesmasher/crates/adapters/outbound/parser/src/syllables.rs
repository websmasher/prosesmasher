//! Syllable counting via hyphenation dictionaries.
//!
//! Uses the `hyphenation` crate's Knuth-Liang algorithm to approximate
//! syllable counts. Falls back to vowel-cluster counting for locales
//! without embedded dictionaries (Indonesian) or for words the
//! hyphenator cannot split.

use hyphenation::{Hyphenator, Iter, Language, Load, Standard};
use prosesmasher_domain_types::Locale;

/// Count syllables in a single word for the given locale.
///
/// Strategy:
/// 1. Map locale to a hyphenation `Language`. If no dictionary exists
///    (Indonesian), use vowel-cluster fallback directly.
/// 2. Load the embedded dictionary and hyphenate the word.
/// 3. Syllable count = hyphenation segments count.
/// 4. If hyphenation produces only 1 segment for a word with multiple
///    vowel clusters, fall back to vowel counting (the dictionary may
///    not know the word).
pub fn count_syllables(word: &str, locale: Locale) -> usize {
    if word.is_empty() {
        return 0;
    }

    let Some(language) = locale_to_language(locale) else {
        return count_vowel_clusters(word);
    };

    let Ok(dictionary) = Standard::from_embedded(language) else {
        return count_vowel_clusters(word);
    };

    let hyphenated = dictionary.hyphenate(word);
    let segment_count = hyphenated.iter().count();
    let vowel_count = count_vowel_clusters(word);

    if segment_count <= 1 {
        // Dictionary returned a single segment — either it knows the word
        // and it's genuinely 1 syllable (e.g. "fire"), or it doesn't know
        // the word at all (e.g. "hello"). Use vowel counting as fallback
        // only when it finds strictly more than 1 cluster, suggesting the
        // dictionary missed syllable boundaries.
        if vowel_count > 1 {
            return vowel_count;
        }
    } else {
        // Dictionary split the word but may have under-segmented.
        // Take the higher of dictionary vs vowel count.
        return segment_count.max(vowel_count);
    }

    segment_count.max(1)
}

/// Map a prosesmasher `Locale` to a hyphenation `Language`.
///
/// Returns `None` for locales without an embedded dictionary.
const fn locale_to_language(locale: Locale) -> Option<Language> {
    match locale {
        Locale::En => Some(Language::EnglishUS),
        Locale::Ru => Some(Language::Russian),
        Locale::De => Some(Language::German1996),
        Locale::Es => Some(Language::Spanish),
        Locale::Pt => Some(Language::Portuguese),
        Locale::Fr => Some(Language::French),
        Locale::Id => None, // no Indonesian hyphenation dictionary
    }
}

/// Count vowel clusters as a syllable approximation.
///
/// A vowel cluster is one or more consecutive vowel characters.
/// Handles Latin vowels (a, e, i, o, u, y) and Cyrillic vowels
/// (а, е, ё, и, о, у, ы, э, ю, я).
fn count_vowel_clusters(word: &str) -> usize {
    let mut count: usize = 0;
    let mut in_vowel = false;

    for ch in word.chars() {
        if is_vowel(ch) {
            if !in_vowel {
                count = count.saturating_add(1);
                in_vowel = true;
            }
        } else {
            in_vowel = false;
        }
    }

    count.max(1) // every word has at least 1 syllable
}

/// Check whether a character is a vowel (Latin, accented Latin, or Cyrillic).
const fn is_vowel(ch: char) -> bool {
    matches!(
        ch,
        // Latin
        'a' | 'e' | 'i' | 'o' | 'u' | 'y'
            | 'A' | 'E' | 'I' | 'O' | 'U' | 'Y'
        // Accented Latin — French, German, Spanish, Portuguese
            | 'à' | 'á' | 'â' | 'ã' | 'ä' | 'å'
            | 'è' | 'é' | 'ê' | 'ë'
            | 'ì' | 'í' | 'î' | 'ï'
            | 'ò' | 'ó' | 'ô' | 'õ' | 'ö'
            | 'ù' | 'ú' | 'û' | 'ü'
            | 'ý' | 'ÿ'
            | 'À' | 'Á' | 'Â' | 'Ã' | 'Ä' | 'Å'
            | 'È' | 'É' | 'Ê' | 'Ë'
            | 'Ì' | 'Í' | 'Î' | 'Ï'
            | 'Ò' | 'Ó' | 'Ô' | 'Õ' | 'Ö'
            | 'Ù' | 'Ú' | 'Û' | 'Ü'
            | 'Ý' | 'Ÿ'
        // Cyrillic
            | 'а' | 'е' | 'ё' | 'и' | 'о' | 'у' | 'ы' | 'э' | 'ю' | 'я'
            | 'А' | 'Е' | 'Ё' | 'И' | 'О' | 'У' | 'Ы' | 'Э' | 'Ю' | 'Я'
    )
}

#[cfg(test)]
#[path = "syllables_tests.rs"]
mod tests;
