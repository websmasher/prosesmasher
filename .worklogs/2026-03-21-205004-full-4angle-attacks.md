# Full 4-angle test attacks: segmenter + markdown

## Summary

Ran proper 4-angle adversarial attacks (completeness, missing scenarios, pattern parity, false positive audit) on segmenter and markdown modules. Both converged.

## Segmenter (31 → 34 tests, converged in 1 round)
- Added: multi-sentence last without period, code-like text, 100-word stress test

## Markdown (61 → 80 tests, converged in 3 rounds)

### Round 1 (61→75): 14 new tests
- Image silent-pass strengthened, inline code in list items, table content leakage
- Strikethrough text preservation, exact paragraph text, bold+italic simultaneous
- Nested emphasis, empty link text, escaped chars (bold/italic/link)
- Code block with heading/link syntax

### Round 2 (75→80): 5 new tests
- Bold doesn't bleed to next paragraph (depth reset across paragraphs)
- Hard break distinct from soft break
- Horizontal rule doesn't corrupt state
- Inline HTML content excluded (not markdown bold)
- Task list marker ignored

### Round 3: CONVERGED
All 4 angles clean. No surviving mutations, no untested branches, no silent-pass patterns, no missing negative cases.
