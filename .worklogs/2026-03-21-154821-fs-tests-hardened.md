# Harden FS adapter tests — exhaustive field coverage

## Summary

Rewrote config loader tests from 8 to 22. Every single field (16 term lists + 12 thresholds) now verified both populated and absent. Added fixtures for unicode terms, extra fields, range edge cases, missing required fields, wrong types, empty file.

## Key changes

- `all-fields.json` fixture with every field populated to unique values
- Exhaustive assertion: all 16 term list counts + all 12 threshold values
- SimplePair order verification (complex vs simple not swapped)
- Locale case insensitivity (EN, En, eN all → En)
- Range edge cases: min=max, 0=0, min>max
- Error cases: empty file, missing locale, wrong type, bad locale
- Unicode: Cyrillic banned words and simplicity pairs
- Extra unknown fields silently ignored
