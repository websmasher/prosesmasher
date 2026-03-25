<!--
EXPECTED PARSE RESULTS:
- Section count: 9 (1 implicit from H1 + 8 H2 boundaries)
- H1: 1, H2: 8, H3: 5
- Total paragraphs: 48
- Paragraphs with bold: 28 (P1, P2, P3, P4, P5, P7, P8, P12, P14, P18, P22, P24, P25, P26, P28, P31, P32, P34, P35, P37, P38, P40, P41, P42, P43, P45, P46, P48)
- Paragraphs with italic: 27 (P1, P2, P3, P4, P7, P8, P12, P15, P18, P22, P24, P25, P26, P28, P29, P31, P32, P34, P35, P37, P38, P40, P41, P42, P43, P45, P46)
- Paragraphs with NEITHER bold nor italic: 18 (P6, P9, P10, P11, P13, P16, P17, P19, P20, P21, P23, P27, P30, P33, P36, P39, P44, P47)
- Total links: 36
- Total bold spans: 59 (count every **...** pair with content)
- Total italic spans: 51 (count every *...* pair that is not bold)
- Code blocks: 3
- Block quotes: 2
-->

# The **Tangled** Art of *Formatting*

**Bold at the very start** of this article sets the tone for what follows. Typography has always been a battleground between clarity and chaos, between the *careful precision* of a master typesetter and the **reckless abandon** of someone who just discovered the bold button. This opening paragraph deliberately mixes **strong emphasis**, *subtle emphasis*, and ***the unholy combination of both*** to establish that we are not playing by polite rules here. We will push every formatting construct to its absolute limit, and we will do so while discussing the perfectly reasonable topic of how text formatting evolved from movable type to markdown. Consider this your [introduction to the madness](https://example.com/intro) that follows.

The history of **bold text** traces back to the earliest days of printing, when *punchcutters* carved heavier letterforms into steel to create what we now call boldface. These craftspeople understood something fundamental about visual hierarchy: **weight draws the eye**. A single bold word in a sea of regular text acts like a lighthouse in fog, pulling attention whether the reader wants it or not. The *italic* form, by contrast, was born in Renaissance Italy, modeled after the handwriting of Venetian scribes who tilted their letters at elegant angles to save space and add a sense of motion to the printed page. When a modern writer combines **bold with *italic nested inside* the bold phrase**, they are unconsciously invoking five centuries of typographic tradition, layering emphasis upon emphasis in ways that would have made Aldus Manutius reach for his glasses.

Now consider the reverse construction: *italic with **bold embedded inside** the italic span*. This inversion is less common in practice but equally valid in markdown. The parser must track which delimiter opened first and which closes last, maintaining a stack of emphasis states that mirrors the nested structure of the source text. Getting this wrong means misattributing emphasis to the wrong words, which in a prose quality checker would cascade into incorrect scoring. The challenge compounds when we add ***triple-starred text*** that simultaneously activates both bold and italic, creating a single span that carries maximum visual weight. Professional typographers generally advise against such heavy-handed emphasis, arguing that it overwhelms the reader and undermines the carefully constructed hierarchy of the page, but our job is not to judge — it is to parse correctly. The quality checker may later flag triple emphasis as an anti-pattern, but the parser must first detect it accurately before any downstream analysis can make that determination.

**Bold that begins on one line
and continues across a line break** is a particularly nasty case for parsers that operate line by line. The markdown specification is clear that emphasis can span soft line breaks within a paragraph, but many naive implementations split text at newline boundaries and lose track of open delimiters. A robust parser must treat the entire paragraph as a single text run, only splitting into sentences after emphasis resolution is complete. This paragraph exists to verify that the parser handles *multi-line italic
spanning two lines* with equal grace. If either of these spans fails to register, the paragraph-level `has_bold` and `has_italic` flags will be wrong, and downstream analysis will silently produce garbage.

Here is a paragraph that packs five links into a single dense passage: [the W3C specification](https://www.w3.org/TR/html52/) defines how emphasis maps to HTML, while [CommonMark](https://commonmark.org/) provides the reference implementation that most markdown parsers follow. The [GitHub Flavored Markdown spec](https://github.github.com/gfm/) extends CommonMark with tables and task lists, and [Pandoc's markdown](https://pandoc.org/MANUAL.html#pandocs-markdown) goes even further with footnotes and definition lists. For our purposes, the most relevant reference is **[the pulldown-cmark crate documentation](https://docs.rs/pulldown-cmark/)**, which is the Rust library our parser delegates to. That last link wraps **bold text inside the link**, a construction that tests whether link detection and bold detection interact correctly.

This paragraph contains absolutely no formatting whatsoever. It is plain text from start to finish. There are no bold words, no italic words, no links, no inline code, and no special characters beyond ordinary punctuation. Its purpose is to serve as a control: when the parser processes this paragraph, it should report has_bold as false, has_italic as false, and an empty list of links. A prose quality checker needs to handle unformatted paragraphs just as correctly as heavily formatted ones, because the majority of well-written prose uses emphasis sparingly if at all. In a typical long-form article, perhaps eighty percent of paragraphs contain no emphasis at all, ten percent contain one or two bold or italic phrases, and only a rare few approach the formatting density we see in this fixture file. The parser must handle all three cases with equal precision.

## Nested Emphasis and Its Discontents

The most treacherous formatting patterns involve **nesting one kind of emphasis inside another**. Consider a sentence like this: the *delicate balance of italic containing **a bold kernel** at its center* requires the parser to open an italic span, then open a bold span inside it, close the bold span, and finally close the italic span, all without confusing the delimiter counts. This is the typographic equivalent of nested parentheses in arithmetic, and just as easy to get wrong. A parser that uses a simple state machine with only two states — "in emphasis" and "not in emphasis" — will fail here, because nesting requires a stack-based approach that tracks the depth and type of each open delimiter.

**First bold span** in a paragraph, followed by some plain text, then **second bold span**, more plain text, then **third bold span**, and finally **a fourth bold span** for good measure. Each of these should be counted individually. The paragraph-level `has_bold` flag only needs to be true once, but a detailed analysis of emphasis density needs to know there are exactly four distinct bold runs in this paragraph. Meanwhile, *first italic span* sits next to *second italic span* and *third italic span* to give the italic counter some exercise too.

### The Problem of Empty Delimiters

What happens when someone types four asterisks in a row? Like this: ****. According to the CommonMark specification, this does not constitute a valid bold span because there is no content between the opening and closing delimiters. A correct parser should treat these as literal asterisk characters or as an empty emphasis that produces no output, but it absolutely must not set the `has_bold` flag for the containing paragraph.

Similarly, consider bold wrapped around nothing but whitespace: **   **. Three spaces between bold delimiters. While this technically contains content, the content is semantically empty. A prose quality checker should be opinionated about this: whitespace-only bold is not meaningful emphasis and should not count toward emphasis metrics. This paragraph tests that the parser or the downstream analysis can distinguish between substantive bold and vacuous bold. The same principle applies to italic: *   * wrapping only spaces should not count as meaningful italic emphasis. These degenerate cases arise in practice when writers delete the content of an emphasized phrase but forget to remove the surrounding delimiters.

Another plain paragraph with no formatting at all. Sometimes the best emphasis is no emphasis. Experienced writers know that when everything is bold, nothing is bold. The eye cannot distinguish signal from noise when every other word carries extra weight. This principle applies equally to italic text, to links, to code spans, and to every other form of inline formatting that markdown provides. Restraint is the master typographer's most powerful tool, and the ability to detect restraint — or its absence — is what separates a useful prose quality checker from a mere word counter. This paragraph deliberately contains no formatting to give the parser a clean baseline between the degenerate cases above and the kitchen-sink stress tests below.

## The Kitchen Sink Paragraph

This section exists to test what happens when every possible inline formatting construct appears in a single paragraph, fighting for the parser's attention in a single dense block of text. Here we go: the word **bold** is bold, the word *italic* is italic, the word `code` is inline code, the phrase [linked text](https://example.com/kitchen-sink) is a link, the combination **bold and *italic together*** layers both, and the sequence `**not bold inside code**` must remain unparsed because it sits inside a code span. A parser that handles all of these correctly in a single paragraph has passed a significant integration test, because each construct interacts with the others in subtle ways that only emerge when they coexist in tight proximity.

This paragraph contains no formatting of any kind. It serves as a palate cleanser between the dense formatting above and the dense formatting below. Readers of test fixtures need breathing room too, and the parser needs opportunities to demonstrate that it correctly reports the absence of formatting, not just its presence. A false positive on emphasis detection is just as harmful as a false negative, because both corrupt the downstream quality analysis.

### Entire-Paragraph Emphasis

**This entire paragraph is bold from its first word to its last word. Every single sentence within it carries the bold flag. The purpose of this test case is to verify that the parser correctly identifies paragraphs where bold emphasis wraps the complete text content, not just individual words or phrases. A prose quality checker might flag this as excessive emphasis, since bolding an entire paragraph defeats the purpose of emphasis, which is to draw attention to specific words by contrasting them against their unemphasized surroundings.**

*This entire paragraph is italic from start to finish. Like the bold paragraph above, it wraps every word in emphasis, but this time using the lighter italic form. Italic is traditionally used for titles of works, foreign phrases, and gentle emphasis, not for entire paragraphs. A parser should set has_italic to true for this paragraph and a quality checker might reasonably flag it as an emphasis anti-pattern. The test verifies both that the flag is set and that the paragraph text is captured correctly despite being entirely wrapped in delimiters.*

After those two heavily emphasized paragraphs, we return to plain unformatted text. No bold, no italic, no links, no code. Just words on a page, doing their job without typographic theatrics. This is what most paragraphs in well-edited prose look like, and the parser should handle it as the default case. The transition from a fully-bold paragraph to a fully-italic paragraph to a completely unformatted paragraph exercises the parser's ability to reset its emphasis tracking state at paragraph boundaries, which is essential for correct analysis of multi-paragraph documents.

## Links in the Wild

Links are the connective tissue of the web, and markdown makes them deceptively simple to write. A basic link like [this one](https://example.com/basic) is straightforward, but complexity emerges quickly. Consider a link with a [long descriptive anchor text that spans many words](https://example.com/long-anchor) — the parser must capture the entire anchor as link text while correctly identifying the URL. The anchor text is what the reader sees, the URL is where they go, and the parser must keep both without mangling either. Links with [query parameters and fragments](https://example.com/search?q=markdown&lang=en#results) add another dimension of complexity because the URL itself contains characters like ampersands and hash signs that have meaning in other contexts.

**[Bold link text](https://example.com/bold-link)** is a construction where the bold markers wrap the entire link, including the square brackets and parentheses. The parser must recognize both the link and the bold emphasis. Similarly, *[italic link text](https://example.com/italic-link)* wraps a link in italic. And for maximum chaos, ***[bold italic link text](https://example.com/bold-italic-link)*** wraps a link in both bold and italic simultaneously. These constructions test the interaction between emphasis parsing and link parsing.

Here is a paragraph containing [link one](https://example.com/one), [link two](https://example.com/two), [link three](https://example.com/three), [link four](https://example.com/four), [link five](https://example.com/five), [link six](https://example.com/six), and [link seven](https://example.com/seven). Seven links in a single paragraph is unusual in normal prose but not unheard of in reference material, bibliographies, or resource lists. The parser should capture all seven URLs and their associated anchor texts without dropping any.

A paragraph with no links at all, no bold, no italic. Just pure text discussing the theory of hyperlinks. Ted Nelson coined the term "hypertext" in 1963, envisioning a system where documents would be richly interconnected through bidirectional links. The web as Tim Berners-Lee implemented it gave us half of that vision: unidirectional links that point from one document to another but carry no awareness of incoming connections. Every markdown link is a tiny act of faith. We trust that the destination still exists, that it still says what we think it says, and that the reader will find their way back. The fragility of links is one of the great unsolved problems of the web, and it manifests in every markdown document that contains a URL pointing to a page that no longer exists.

## Code Blocks as Formatting Firewalls

Code blocks are special in markdown because their contents must not be parsed for formatting. Everything between the opening and closing triple backticks is literal text, preserved exactly as written. This is critical for a prose quality checker because code blocks often contain asterisks, brackets, and other characters that would be misinterpreted as formatting if parsed normally. A parser that accidentally treats code block contents as markdown would not only generate spurious emphasis flags but might also corrupt its internal state, producing incorrect results for all subsequent paragraphs. The firewall between code blocks and prose must be absolute.

```python
def emphasize(text: str, bold: bool = False, italic: bool = False) -> str:
    """Apply **bold** or *italic* formatting to text.

    This docstring contains markdown-like syntax that must NOT
    be parsed as actual formatting. The **double asterisks** and
    *single asterisks* here are just characters in a string.

    Links like [this](https://example.com) are also just text.
    """
    result = text
    if bold:
        result = f"**{result}**"
    if italic:
        result = f"*{result}*"
    return result
```

The paragraph after a code block should parse normally. **Bold text** and *italic text* and [a link](https://example.com/after-code) should all be detected correctly, even though the preceding code block contained similar syntax that was intentionally ignored.

```markdown
# This is a heading inside a code block
**This bold** should not be parsed
*This italic* should not be parsed
[This link](https://example.com) should not be parsed
- This list should not be parsed
> This blockquote should not be parsed
```

Another paragraph after another code block. The parser should have a clean state after exiting a code block, with no residual confusion from the markdown-like content inside it. This paragraph is intentionally plain and unformatted, serving as a control to verify that the parser's state machine correctly transitions from code-block mode back to normal paragraph mode. If any bold or italic flags appear on this paragraph, something has gone wrong with the code block boundary detection.

```rust
// A third code block to verify consistent handling
fn parse_markdown(input: &str) -> Document {
    let parser = pulldown_cmark::Parser::new(input);
    // **bold** and *italic* in comments are NOT formatting
    let mut sections = Vec::new();
    for event in parser {
        match event {
            Event::Start(Tag::Heading(level, ..)) => {
                // Handle [heading](link) — not a real link
            }
            _ => {}
        }
    }
    Document { sections }
}
```

## Edge Cases in Character-Level Emphasis

**x** is a single bold character. *y* is a single italic character. These minimal spans test whether the parser handles emphasis around single characters without accidentally merging adjacent delimiters or misinterpreting the asterisks as part of a different construct. In practice, single-character emphasis is rare, but it appears in mathematical notation, variable names, and abbreviations where a writer wants to call attention to a specific letter. The reason this matters for our parser is that the emphasis detection algorithm must handle the smallest possible emphasized text — a single character — with the same correctness it brings to emphasized phrases spanning dozens of words. Edge cases at the minimum end of the size spectrum reveal bugs that larger inputs might accidentally mask.

**Bold at the start** of this paragraph, then plain text in the middle with no special formatting, then *italic at the very end*. This tests boundary detection: the parser must correctly identify that bold emphasis begins at position zero and italic emphasis ends at the final character. Many off-by-one errors in parsers reveal themselves at paragraph boundaries, where the logic for opening and closing delimiters interacts with the logic for paragraph start and paragraph end.

> Block quotes add another layer of complexity. This quoted paragraph contains **bold text** and *italic text* and a [link inside a quote](https://example.com/quoted). The parser must recognize that a block quote is a container block that holds other blocks, including paragraphs with their own inline formatting. The has_bold and has_italic flags should be set for the paragraph inside the quote, not for the quote itself.

> A second block quote paragraph, this one with no formatting at all. Just plain quoted text. The parser should handle block quotes with and without inline emphasis, confirming that the container-block logic does not interfere with inline parsing.

### Consecutive Emphasis Spans

**Alpha** and **Beta** and **Gamma** and **Delta** and **Epsilon** — five consecutive bold spans in a single paragraph, separated by plain text. The parser should count five distinct bold runs. The paragraph-level `has_bold` flag is simply true, but a span-level analysis would enumerate each one. This is a *common* pattern in glossaries and term lists where each defined word is bold and its definition follows in regular weight.

*First* and *second* and *third* and *fourth* and *fifth* — five consecutive italic spans in a single paragraph, mirroring the bold pattern above. Italic is often used for emphasis within running prose, and it is not uncommon for a single paragraph to contain multiple italic phrases when the writer is drawing several parallel comparisons or listing several related concepts. Each *span* here is short, just a single word, but they are distinct and should be counted separately. The spacing between italic spans matters too: if two italic words are adjacent with only whitespace between them, the parser must not merge them into a single span. The closing delimiter of one italic run and the opening delimiter of the next are separate events in the token stream, and the parser must respect that boundary.

## Typography Through the Ages

This section tells a story while continuing to stress the parser with formatting challenges. The history of typography is a fascinating journey from carved wood blocks to digital fonts, and every major technological shift brought new questions about emphasis and decoration. By embedding formatting constructs within a historical narrative, we test whether the parser handles emphasis in the context of natural, flowing prose rather than artificial test sentences designed solely to exercise edge cases.

Johannes **Gutenberg** invented movable type around *1440* in the German city of **Mainz**. His innovation was not the printing press itself — presses existed for wine and linen — but the idea of casting individual letters in metal that could be arranged, printed, and rearranged. The earliest printed books used a single typeface: a heavy blackletter style modeled after the handwriting of German scribes. There was no bold, no italic, no underline. Emphasis was achieved through larger letters, red ink, or hand-painted decorations added after printing. You can learn more about Gutenberg's process at [the Gutenberg Museum's website](https://www.gutenberg-museum.de/en/) and in the detailed [Wikipedia article on movable type](https://en.wikipedia.org/wiki/Movable_type).

The **italic** typeface was developed by *Aldus Manutius* and his punchcutter **Francesco Griffo** in **Venice** around *1500*. Manutius wanted to print affordable pocket editions of classical texts, and the slanted italic letters — modeled after the handwriting of Italian humanist scholars — allowed more words per page than the upright roman style. For the first fifty years of its existence, italic was not used for emphasis at all. It was simply a space-saving alternative to roman type. The idea of mixing roman and italic in the same text, using italic for titles, foreign words, and emphasis, did not emerge until the late sixteenth century. [Manutius's legacy](https://example.com/manutius) lives on in the name of the modern publishing software *Aldus PageMaker*, though that program has long since been supplanted by *Adobe InDesign*.

Plain paragraph about the transition from hand presses to industrial printing in the eighteenth and nineteenth centuries. The nineteenth century saw the invention of steam-powered presses, stereotyping, and eventually linotype machines that could set an entire line of type from a keyboard. None of these technological changes altered the fundamental vocabulary of typographic emphasis. Bold, italic, and roman remained the three pillars of textual hierarchy, joined occasionally by small caps, underlines, and letterspacing. This paragraph contains no formatting because the history it describes was a period of mechanical rather than typographic innovation.

### The Digital Revolution

The arrival of digital typesetting in the *1970s* and personal computers in the *1980s* transformed typography forever. Suddenly anyone with a **Macintosh** and a copy of **MacWrite** could set text in bold or italic with the click of a button. The democratization of formatting was, in many ways, a disaster for document quality. Where professional typesetters used emphasis sparingly and purposefully, amateur users discovered that they could make **everything bold** and *everything italic* and often did both simultaneously in the same document, producing pages that looked like ransom notes assembled from competing typefaces.

The rise of the web in the *1990s* brought HTML, which formalized emphasis through the `<b>`, `<i>`, `<strong>`, and `<em>` tags. The distinction between `<b>` (visual bold) and `<strong>` (semantic strong emphasis) reflected a growing understanding that formatting carries meaning beyond mere appearance. A screen reader encountering `<strong>` text might raise its voice; encountering `<b>` text, it might not. This semantic distinction is [documented in the HTML specification](https://html.spec.whatwg.org/multipage/text-level-semantics.html#the-strong-element) and has implications for accessibility that extend far beyond visual styling. Markdown, which **John Gruber** created in **2004**, deliberately collapsed this distinction: `**text**` becomes `<strong>` and `*text*` becomes `<em>`, always carrying semantic weight. This simplification was intentional and is explained in [Gruber's original markdown syntax description](https://daringfireball.net/projects/markdown/syntax).

## Stress Test: Mixed Formatting Density

This section pushes formatting density to its maximum. Every paragraph contains multiple overlapping formatting constructs designed to test the parser's ability to maintain correct state through rapid transitions between emphasis modes.

**Bold opens this paragraph**, then we shift to *italic for a phrase*, back to plain text, then a [link to somewhere](https://example.com/somewhere), then **more bold with *italic nested inside* the bold again**, then `inline code that **must not** be parsed`, then *italic to finish*. That was seven distinct formatting transitions in a single paragraph. The parser must track each one without losing state or miscounting spans. The result should be: has_bold true, has_italic true, one link, at least two bold spans, at least three italic spans, and one code span that contains literal asterisks rather than emphasis.

Here is another dense paragraph for the density stress test. **Sentence one is bold.** *Sentence two is italic.* Sentence three is [a link to the test results page](https://example.com/results). **Sentence four returns to bold.** ***Sentence five combines bold and italic.*** Sentence six is plain. *Sentence seven is italic again.* **Sentence eight is bold again.** The pattern here is a rapid alternation that forces the parser to open and close emphasis spans in quick succession, verifying that no span leaks into adjacent sentences.

A completely plain paragraph inserted between dense formatting paragraphs. Its job is to confirm that the parser resets its emphasis state cleanly between paragraphs. No bold. No italic. No links. No code. Just words. The parser should treat each paragraph as an independent unit for emphasis detection, carrying no state from the previous paragraph into the next. This isolation property is what makes paragraph-level analysis tractable: we do not need to consider the emphasis history of the entire document when evaluating a single paragraph.

**This paragraph begins bold**, pauses for a plain phrase, then introduces a [link](https://example.com/link-a) and another [link](https://example.com/link-b) in quick succession, followed by *italic that carries through* to the sentence boundary. Then another **bold phrase** appears before the paragraph closes. The interleaving of bold, links, and italic in varying order tests that the parser does not assume a fixed sequence of formatting types within a paragraph.

### Interleaved Emphasis and Links

The trickiest paragraphs for any parser combine **bold emphasis** with [inline links](https://example.com/interleaved) and *italic emphasis* in unpredictable order. When a reader encounters a sentence like this one, their eye naturally follows the visual rhythm created by alternating weight and color: bold pulls forward, italic leans sideways, and the underlined or colored link text creates a third visual register entirely. The parser must assign the correct flags to each word without allowing one formatting type to interfere with another.

Consider a paragraph where **every other phrase is bold**, then *every other phrase is italic*, and then [every](https://example.com/every) [other](https://example.com/other) [phrase](https://example.com/phrase) is a link. The visual density of this construction would be overwhelming in real prose, but it serves an important testing purpose: it proves that the parser can track three independent formatting channels simultaneously without cross-contamination. If the bold flag leaks into a link span, or if an italic flag persists past its closing delimiter, the error will be visible in the parsed output of this paragraph.

There is a subtlety to how emphasis interacts with punctuation that deserves its own test case. Consider **bold followed by a comma**, and then *italic followed by a period*. The closing delimiters appear immediately before the punctuation, and a careless parser might include the punctuation inside the emphasis span or, worse, fail to close the span because it expected whitespace before the closing delimiter. The CommonMark specification is explicit about this: punctuation characters are allowed to flank emphasis delimiters, and their presence should not prevent emphasis from being recognized. This paragraph verifies that the parser handles emphasis-then-punctuation transitions correctly, including **bold before a semicolon**; *italic before a colon*: and **bold before an em dash** — followed by regular text.

A plain paragraph between the interleaving section and the final observations. This paragraph contains no emphasis, no links, no inline code, and no other formatting. It exists solely to verify that the parser returns to its default state after processing the heavily formatted paragraphs above. The transition from formatted to unformatted text is just as important to test as the formatting itself, because state-management bugs often manifest at boundaries rather than in the middle of a formatting run.

## Final Observations

The art of formatting is ultimately the art of contrast. **Bold text** only works because it sits next to text that is not bold. *Italic text* only whispers because it is surrounded by upright roman letterforms. A [hyperlink](https://example.com/contrast) only draws the eye because it is colored differently from its neighbors. When a writer uses these tools well, they guide the reader's attention effortlessly through the text, highlighting key terms, indicating titles and foreign phrases, and providing pathways to related material. The best formatted documents are invisible in their formatting: the reader absorbs the emphasis without consciously noticing it, guided through the argument as if by an unseen hand. The worst formatted documents call attention to their own decoration, distracting the reader from the content they came to read.

When a writer uses these tools poorly — bolding entire paragraphs, italicizing everything, linking every other word — the result is a *visual cacophony* that **exhausts the reader** before they reach the second paragraph. The prose quality checker we are building must detect both conditions: the thoughtful use of emphasis that supports comprehension, and the reckless overuse that undermines it. This fixture file tests the parser's ability to handle both extremes and everything in between, ensuring that the foundation of our analysis is solid no matter what formatting chaos the input contains. The difference between a good writer and a bad writer is not vocabulary or grammar but the ability to modulate emphasis, to know when **a bold word** will illuminate and when it will merely shout. Our tool must measure that modulation accurately, and accurate measurement starts with accurate parsing.

This final paragraph is deliberately plain. No bold, no italic, no links, no code, no block quotes, no headings. Just a quiet ending to a noisy document. The parser should process it without incident and report that all emphasis flags are false. If it does, then the formatting hell has been navigated successfully, and we can proceed with confidence to the next layer of analysis. Every fixture file in this test suite targets a different dimension of parsing complexity, and this one has done its job if the emphasis detection, link extraction, and code block isolation all pass without a single miscount.

**One last bold sentence stands alone as the true final paragraph.** It exists to verify that emphasis detection works correctly at the very end of a document, where some parsers might fail to close open spans because they have reached the end of input rather than encountering an explicit closing delimiter. The bold flag should be true, and the document should be complete. Many parser bugs hide at document boundaries because the end-of-input condition is tested less frequently than mid-document transitions, making this final bold paragraph an essential part of the test suite rather than a mere afterthought.
