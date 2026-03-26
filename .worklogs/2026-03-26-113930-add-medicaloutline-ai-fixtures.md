# Add MedicalOutline AI Fixtures

**Date:** 2026-03-26 11:39
**Scope:** `fixtures/medicaloutline/*`

## Summary
Added ten markdown fixtures extracted from the actual search results at Medical Outline for `as an ai language model`. The fixtures are the article bodies converted to markdown with their titles preserved as top-level headings, giving the repo a compact set of real-world, obviously AI-written medical FAQ examples.

## Context & Problem
The project needed a batch of clearly AI-written real-world fixtures, not hand-authored slop examples. The user pointed to a specific Medical Outline search results page and asked for every article visible on that page to be turned into markdown fixtures under `fixtures/medicaloutline/`.

## Decisions Made

### Extract only the ten actual search-result articles
- **Chose:** Parse the search results page and keep only the ten `<article>` result links under `article h2.entry-title a`.
- **Why:** The page includes a large amount of unrelated navigation, sidebar links, and widget content. Restricting extraction to the actual search-result cards prevents accidental fixture pollution.
- **Alternatives considered:**
  - Use all page links under the domain — rejected because it would mix in sidebar and menu content.
  - Hand-pick a subset of the results — rejected because the request was to capture every article on that page.

### Use article-body extraction rather than raw HTML dumping
- **Chose:** Fetch each article page and extract the main content into markdown, then prepend the page title as `# Title`.
- **Why:** The fixtures should be usable as prose inputs for `prosesmasher`, not as HTML snapshots. Markdown article bodies are the right test artifact.
- **Alternatives considered:**
  - Save raw HTML — rejected because it adds a lot of irrelevant template noise and is less useful for prose-rule testing.
  - Copy-paste manually — rejected because it is slower and more error-prone than deterministic extraction.

### Use local Python extraction tooling instead of Crawl4AI
- **Chose:** Use `requests` + `BeautifulSoup` to collect result URLs and `trafilatura` to extract markdown from the article pages.
- **Why:** `crawl4ai` was not actually installed in the environment. `trafilatura` produced clean markdown from these WordPress pages and preserved the obvious AI boilerplate that makes these fixtures valuable.
- **Alternatives considered:**
  - Stop and ask about the missing tool — rejected because the task was directly fulfillable with local tooling.
  - Force-install Crawl4AI — rejected because it was unnecessary for this page shape and would have added heavier tooling for no benefit.

## Architectural Notes
These fixtures are intentionally “dirty real world” inputs. They are good candidates for future:
- `llm-disclaimer`
- `response-wrapper`
- `knowledge-cutoff` style rules
- general AI-slop regression tests

The files are stored as plain markdown under `fixtures/medicaloutline/` with one article per file and slug-based names derived from the source URL path.

## Information Sources
- `https://www.medicaloutline.com/page/14/?s=as+an+ai+language+model` — source search-results page
- The ten extracted Medical Outline article URLs from that page
- Local extraction using `requests`, `BeautifulSoup`, and `trafilatura`

## Open Questions / Future Considerations
- If these fixtures become part of automated regression tests, we may want a small manifest with source URLs and expected rule hits rather than inferring provenance from commit history.
- Some of these articles are quite similar in structure. Later we may want to tag or cluster them by slop pattern so tests can target specific future `llm-slop` rules.

## Key Files for Context
- `fixtures/medicaloutline/how-can-i-help-my-child-with-adhd-without-medication.md` — example with explicit “As a language model AI” opener
- `fixtures/medicaloutline/what-are-the-10-worst-cancers.md` — example with knowledge-cutoff disclaimer and ranked list structure
- `fixtures/medicaloutline/is-yolanda-hadid-still-ill.md` — compact disclaimer-heavy example
- `.worklogs/2026-03-26-112504-harden-triple-repeat-synthetic-tests.md` — most recent adjacent work in the fixture/test-hardening area

## Next Steps / Continuation Plan
1. Run the future `llm-slop` family against `fixtures/medicaloutline/` once those rules land, and record which fixtures hit which rules.
2. Add a lightweight fixture manifest if we start asserting expected failures per file in automated tests.
3. Use these files as seed material for synthetic parity tests so the ruleset is not only trained on toy examples.
