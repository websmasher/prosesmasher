<!--
EXPECTED PARSE RESULTS:
- Section count: 32 (one per heading)
- Heading counts: H1=1, H2=22, H3=5, H4=2, H5=1, H6=1 (total=32)
- Total paragraphs: 112 (101 outside block quotes + 11 inside block quotes)
- Code blocks: 5 (fenced=3, indented=2)
- Block quotes: 6 (max nesting depth: 5)
- Unordered lists: 3, Ordered lists: 1 (top-level; plus 1 nested unordered + 1 nested ordered)
- Total links: 39 (36 inline + 1 autolink + 2 reference-style)
- Images: 5 (should NOT count as links or paragraph text)
- Bold paragraphs: 2, Italic paragraphs: 1
- Tables: 2
- Thematic breaks: 6

HEADING DETAILS:
- H1: 1 setext (underlined with ===)
- H2: 21 ATX + 1 setext (underlined with ---) = 22 total
  - Includes: 1 empty heading (##), 1 with trailing hashes (## ... ##),
    1 with bold+italic+code+link, 1 over 150 chars, 1 with bold in heading text
- H3: 5 ATX (3 consecutive with no body between them)
- H4: 2 ATX
- H5: 1 ATX
- H6: 1 ATX
- Fake headings (NOT parsed as headings): 3 (#NotAHeading, ##AlsoNotAHeading, ###StillNotAHeading)
- 11 consecutive headings with zero body text between them (H3,H3,H3,H4,H3,H2,H3,H4,H5,H6,H2)

INVARIANTS:
- Parser must not panic on any input
- Escaped formatting must NOT set bold/italic flags
- Code block content must be raw, not parsed
- Image alt text must not appear as paragraph text
- HTML tags should be handled gracefully (ignored or extracted)
- Tight list items should NOT generate Paragraph events
- Non-breaking space (U+00A0) and zero-width space (U+200B) must not cause panic
- Setext headings must be recognized (=== for H1, --- for H2)
- Reference link definitions must not appear as paragraph text
-->

## Stress-Testing Your Assumptions: A Deliberately Hostile Markdown Document

Every parser author believes their code is correct. They write a few tests with clean inputs, see green checkmarks, and ship it. Then the real world arrives — a user pastes something from a Word document, a CMS auto-generates markdown with trailing whitespace everywhere, someone writes a blog post in Arabic and Chinese in the same paragraph. The parser breaks. Not with a clean error, but with silent data corruption that nobody notices for months.

This document exists to break your parser before your users do. It is structured as a tech blog post, but every paragraph, every heading, every code block has been designed to probe a specific edge case. If your parser survives this file without panicking, dropping content, or misclassifying elements, it might actually be ready for production. Probably not, but at least you will know where it fails.

The philosophy here is simple: trust nothing. The CommonMark spec is enormous and riddled with surprising corner cases. GitHub Flavored Markdown adds more. And real-world documents do not read the spec at all — they just throw characters at the page and expect something reasonable to come out the other end.

### The Case Against H1

### Why This Document Has No H1 at the Top

### A Third H3 to Compound the Problem

#### Going Deeper Without Any Body Text

### Back to H3 Again

## Interrupting With an H2

### Then H3

#### Then H4

##### Then H5

###### Then H6

## Back to H2 After the Heading Avalanche

If your parser made it through that sequence without crashing, congratulations. You just encountered eleven consecutive headings with zero body paragraphs between them. Many parsers assume that a heading is always followed by at least one paragraph before the next heading. That assumption is wrong. Section trees that rely on "the content between heading A and heading B" must gracefully handle sections that contain nothing at all — no paragraphs, no lists, no code, nothing.

The heading sequence above also tests level transitions in every direction. We went from H2 to H3 (down), H3 to H3 (same), H3 to H4 (down), H4 to H3 (up), H3 to H2 (up again), then cascaded all the way down from H2 through H3, H4, H5, to H6. Your section tree builder must handle every one of these transitions without assuming monotonic descent.

Now let us talk about what is NOT a heading. The following lines look like they might be headings if you are doing naive regex matching, but CommonMark is very clear: there must be a space after the hash characters for it to be an ATX heading.

#NotAHeading because there is no space after the hash.

##AlsoNotAHeading even with two hashes.

###StillNotAHeading with three hashes smashed against text.

Those three lines should parse as ordinary paragraph text. If your parser treated any of them as headings, you have a bug. The CommonMark spec requires at least one space between the hash marks and the heading content. Many regex-based parsers get this wrong because they match on `^#{1,6}` without checking for the space.

## Heading With Trailing Hashes ##

This heading has trailing hash characters in the source. According to CommonMark, the trailing hashes are optional closing sequences and should be stripped from the heading text. The heading text should be "Heading With Trailing Hashes" — no trailing hashes, no trailing spaces.

Here is another paragraph after the trailing-hash heading, just to make sure the parser correctly transitions from heading to paragraph content.

Setext Level One Heading
========================

This paragraph follows a setext-style H1. Setext headings are created by underlining text with equals signs (for H1) or dashes (for H2). They are weird, rarely used in modern markdown, and a common source of parser bugs because the underline line looks like it could be a thematic break or just random text.

Setext Level Two Heading
------------------------

This paragraph follows a setext-style H2. The dashes underneath "Setext Level Two Heading" create an H2, not a horizontal rule. The parser must distinguish between a line of dashes that follows text (setext heading) and a line of dashes that follows a blank line (thematic break). This distinction is critical and many parsers get it wrong.

## The **Bold**, *Italic*, `Code`, and [Linked](https://example.com) Heading

This heading contains bold, italic, inline code, and a hyperlink — all in the same heading text. A parser that only stores plain text for headings will lose the formatting. A parser that tries to parse heading content for formatting must handle all four formatting types simultaneously. Either approach is valid, but the parser must not crash or produce malformed output.

##

That was an empty heading — just two hash marks and nothing else. CommonMark allows it. The heading text is an empty string. Your parser must handle this without panicking, returning null where a string is expected, or creating a heading node with undefined content.

## A Very Long Heading That Deliberately Exceeds One Hundred Characters Because Some Parsers Have Fixed-Size Buffers And We Want To Know If Yours Does Too

That heading is over 150 characters. If your parser allocates a fixed buffer for heading text, it just overflowed. If it uses a growable string (as it should), this is a non-issue. But it is worth testing because some parsers are written in languages with fixed-size arrays, and buffer overflows in heading parsing are a real class of bugs.

## Thematic Breaks — Lines That Look Like They Mean Something

Above this paragraph is a heading. Below it will be a thematic break. Thematic breaks (horizontal rules) should not create new sections, should not consume adjacent paragraphs, and should not corrupt the document tree. They are purely presentational elements.

---

That was a thematic break created with three dashes. The parser should record it as a thematic break element, not as a section divider. Let us try the other variants.

***

That was a thematic break with asterisks. Same semantics, different syntax.

___

That was a thematic break with underscores. Still the same thing. All three variants — dashes, asterisks, underscores — should produce identical parse results.

---

---

---

Three thematic breaks in a row with blank lines between them. This is unusual but valid. The parser should not merge them, skip them, or treat them as some kind of section boundary. They are three separate thematic break elements.

And here is a paragraph after the triple thematic break, just to verify that normal content parsing resumes correctly after an unusual sequence of break elements.

## HTML — The Unwanted Guest at the Markdown Party

Markdown and HTML have always had an uncomfortable relationship. The original Markdown spec allowed arbitrary HTML, and most parsers still do. But for a prose quality checker, HTML is noise — we care about the text content, not the markup.

This paragraph has <strong>HTML bold</strong> and <em>HTML italic</em> and <a href="https://example.com/html-link">an HTML link</a> mixed right in with normal markdown text. The parser must either extract the text content from these tags or ignore the tags entirely. What it must not do is include the raw HTML tags as part of the paragraph text, because then sentence boundaries, word counts, and formatting analysis would all be wrong.

Here is an inline <code>HTML code tag</code> that should probably be treated like markdown backtick code. And here is a <span style="color:red">styled span</span> that should be either stripped or ignored.

<!-- This is an HTML comment. It should be completely invisible to the parser. If any of this text appears in the parse output, something is very wrong. Comments are not content. They are metadata for humans reading the source. -->

<div class="callout warning">
  <p>This is a block-level HTML element. It contains a paragraph inside a div.</p>
  <p>And a second paragraph inside the same div. Block HTML is particularly tricky because pulldown-cmark may emit it as a single Html event containing the entire block.</p>
</div>

After that HTML block, we should be back to normal markdown. This paragraph should parse normally, with no contamination from the div above.

Self-closing tags are another edge case: <br/> should create a line break, and <hr/> should create a thematic break. But in the context of a prose checker, we probably want to ignore both. Either way, the parser must not crash on them. Some parsers choke on self-closing tags because they expect a matching close tag that never comes.

## Escaping — When Asterisks Are Just Asterisks

Markdown's escape mechanism is simple: prefix a special character with a backslash, and it loses its special meaning. But getting this right in a parser is surprisingly hard, because the backslash itself can be escaped, and different characters have different escaping rules.

\*This is not italic.\* The asterisks are escaped, so they are literal asterisk characters. Your bold/italic detection must not flag this paragraph.

\*\*This is not bold.\*\* Same thing with double asterisks. If your parser sets a bold flag on this paragraph, your escape handling is broken.

\[This is not a link\](and-this-is-not-a-url). The square brackets and parentheses are all escaped. No link should be created from this text.

\# This is not a heading. It is a paragraph that starts with a literal hash character. Heading detection must respect escapes.

Here is a backslash at end of line: test\
This line should be a continuation after a hard line break. The backslash-before-newline creates a hard break in CommonMark. This is different from the two-trailing-spaces hard break, and parsers must support both.

## Images — Not Paragraphs, Not Links

Images in markdown look almost identical to links, but with an exclamation point prefix. They are fundamentally different from a prose analysis perspective: the alt text of an image is metadata, not prose content. A paragraph that contains only an image should probably be treated differently from a paragraph that contains text.

Here is a standalone image: ![A beautiful sunset over the ocean with vibrant orange and purple hues](https://example.com/sunset.png)

That image has descriptive alt text. The alt text should NOT appear as paragraph content in the parse output. If your parser includes "A beautiful sunset over the ocean with vibrant orange and purple hues" as a sentence in a paragraph, your image handling is broken.

Now here is an image embedded in a paragraph with text before and after it. The paragraph starts with some text, then ![inline image](https://example.com/inline.jpg) appears in the middle, and then the paragraph continues with more text after the image.

Here is an image with no alt text at all: ![](https://example.com/no-alt.png). This is common on the web. The parser must handle the empty alt text without crashing or producing a null string where a string is expected.

And here is the notorious image-inside-a-link pattern: [![Click this image to visit the site](https://example.com/clickable.jpg)](https://example.com/destination). This creates a clickable image — the image is the link text, and the outer brackets form the link. The parser must recognize both the image and the link without getting confused by the nested bracket syntax.

Here is one more image for good measure, this time with special characters in the alt text: ![Photo of café résumé naïve über](https://example.com/unicode-alt.png). The alt text contains accented characters that might trip up parsers with ASCII-only assumptions.

## Code Blocks — The Parser's Safe Space

Code blocks are supposed to be opaque. Everything inside a code block should be treated as raw text, never parsed for markdown formatting. This is critical for a prose quality checker because code blocks contain... code, not prose. Any word counting, sentence detection, or readability analysis that processes code block content will produce garbage results.

Here is a fenced code block with a language tag:

```rust
fn main() {
    // This **bold** text should NOT be parsed as bold
    // This *italic* text should NOT be parsed as italic
    // This [link](https://example.com) should NOT be parsed as a link
    let x = "# Not a heading";
    let y = "> Not a block quote";
    println!("Hello, world!");

    // What about HTML? <strong>Not bold</strong>
    // Or images? ![not an image](not-a-url)

    for i in 0..20 {
        println!("Line {}: still inside the code block", i);
    }

    // More lines to ensure we exceed the 20-line threshold
    let a = 1;
    let b = 2;
    let c = 3;
    let d = 4;
    let e = 5;
    let f = a + b + c + d + e;
    println!("Sum: {}", f);
}
```

That code block is over 20 lines. Every line inside it contains markdown-like syntax that must NOT be parsed. If your parser found bold text, italic text, links, headings, block quotes, or images inside that code block, your code block handling is fundamentally broken.

Here is a fenced code block without a language tag:

```
This is plain fenced code with no language specified.
It should still be treated as a code block.
The parser should not try to guess the language.
**This is not bold.** *This is not italic.* [Not a link](nowhere).
```

And here is an indented code block — four spaces of indentation:

    This is an indented code block.
    It uses four spaces instead of fence markers.
    Indented code blocks are the original markdown code syntax.
    They are less common now but still valid.
    **Not bold** *Not italic* [Not a link](nowhere)

Some parsers only handle fenced code blocks and forget about indented ones. Both must work.

Now for the tricky one — backticks inside a fenced code block. You need more backticks on the fence than appear inside:

````
This code block contains triple backticks:
```
These inner backticks do not end the code block.
```
Because the outer fence uses four backticks.
````

And inline code containing markdown-like characters: `**not bold**` and `*not italic*` and `[not a link](url)` should all render as literal text inside code spans. The backticks create an opaque boundary just like fenced code blocks do.

Here is a second indented code block to test a different scenario:

    fn adversarial_function() {
        let heading = "## Not a heading";
        let bold = "**Not bold**";
        let link = "[Not a link](url)";
    }

That completes the code block section.

## Tables — Structured Data in a Prose Format

GitHub Flavored Markdown supports tables, and pulldown-cmark can parse them with the right feature flags. Tables are tricky because they introduce a grid structure into what is otherwise a linear document.

| Feature | Status | Notes |
|---------|--------|-------|
| Bold | Supported | Uses `**text**` syntax |
| Italic | Supported | Uses `*text*` syntax |
| Links | Supported | Uses `[text](url)` syntax |

That was a simple 3x3 table (plus header row). The parser should recognize it as a table element and either extract the cell contents or skip it entirely. What it must not do is treat the pipe characters as paragraph text.

Now here is a table with formatting inside the cells:

| Name | Description | Link |
|------|-------------|------|
| **Rust** | A *systems* language | [rust-lang.org](https://www.rust-lang.org) |
| **Go** | A *compiled* language | [go.dev](https://go.dev) |
| **Python** | An *interpreted* language | [python.org](https://www.python.org) |

This paragraph comes immediately after the table with no blank line separation in the rendered output. The parser must correctly identify where the table ends and this paragraph begins.

## Block Quotes — Nesting All the Way Down

Block quotes are one of the most complex elements in markdown because they can contain anything — paragraphs, headings, lists, code blocks, even other block quotes. And they nest recursively, so a block quote can contain a block quote that contains a block quote, with no defined maximum depth.

> This is a simple single-level block quote. It contains one paragraph of text. Nothing fancy, just establishing a baseline. The parser should wrap this in a BlockQuote node containing a Paragraph node.

That was the easy case. Now let us go deeper.

> Level one of the nested quote. This is the outermost layer.
>
> > Level two. We are now inside a block quote inside a block quote.
> >
> > > Level three. Three levels deep. Some parsers start to struggle here.
> > >
> > > > Level four. Four levels deep. The indentation is getting ridiculous.
> > > >
> > > > > Level five. Five levels deep. If your parser uses a fixed-size nesting stack, it might overflow here. If it uses recursion without a depth limit, it might blow the call stack on pathological inputs with hundreds of nesting levels. Five is enough for our purposes.

After that five-level nesting, here is a normal paragraph. The parser must correctly close all five levels of block quote and return to the top-level document context.

> Here is a block quote.

Then a normal paragraph between quotes. The parser must close the first quote, record this paragraph, then open a new quote.

> Here is another block quote after a paragraph.

And another normal paragraph. This alternating pattern — quote, paragraph, quote, paragraph — tests the parser's ability to correctly open and close block quote contexts.

> This block quote contains **bold text**, *italic text*, and a [link inside the quote](https://example.com/quoted-link). It also contains `inline code`. The parser must handle formatting inside block quotes exactly the same way it handles formatting in regular paragraphs.

> This is a multi-paragraph block quote. The first paragraph says one thing.
>
> The second paragraph says another. Both paragraphs should be inside the same block quote element. Some parsers incorrectly split this into two separate block quotes.

## Links — The Gauntlet

Links are deceptively complex in markdown. There are inline links, reference links, autolinks, and links that contain special characters in the URL. A prose quality checker needs to count links accurately, extract the link text for analysis, and handle every variant without crashing.

Here is a [simple link](https://example.com) to start. And [another link](https://example.com/page-two) right after it. This paragraph has [a third link](https://example.com/page-three) as well. Three links in one paragraph, all inline style.

Now a link with an empty text: [](https://example.com/empty-text). This is valid markdown — the link exists but has no visible text. Some parsers crash on empty link text because they assume link text is always non-empty.

Here is a link with special URL characters: [search results](https://example.com/search?q=hello+world&lang=en&page=1#results). The URL contains a query string with multiple parameters and a fragment identifier. URL parsing must handle ampersands, equals signs, hash marks, and plus signs without confusing them for markdown syntax.

And the classic Wikipedia problem — a link with parentheses in the URL: [Rust programming language](https://en.wikipedia.org/wiki/Rust_(programming_language)). The closing parenthesis in "Rust_(programming_language)" must not be confused with the closing parenthesis of the link syntax. CommonMark handles this by matching parentheses, but many simple parsers get it wrong.

Here is an autolink: <https://example.com/autolink>. Autolinks use angle brackets instead of the `[text](url)` syntax. They are both the link text and the URL. The parser must count autolinks in the total link count.

Reference-style links use a different syntax. Here is [a reference link][ref1] and [another reference link][ref2]. The actual URLs are defined at the bottom of the document. Reference links are convenient for documents with many repeated URLs, but they require a two-pass parse or a lookup table.

Let us add more links to hit our count. [Link seven](https://example.com/7), [link eight](https://example.com/8), [link nine](https://example.com/9), [link ten](https://example.com/10). That is ten inline links so far.

Here is a paragraph with links scattered throughout. We have [eleven](https://11.example.com) here and [twelve](https://12.example.com) here. Then [thirteen](https://13.example.com) and [fourteen](https://14.example.com). The parser should count every single one.

More links: [fifteen](https://15.example.com), [sixteen](https://16.example.com), [seventeen](https://17.example.com), [eighteen](https://18.example.com), [nineteen](https://19.example.com), [twenty](https://20.example.com).

Still going. [Link twenty-one](https://21.example.com), [twenty-two](https://22.example.com), and [twenty-three](https://23.example.com). We also have the links inside tables — [rust-lang.org](https://www.rust-lang.org), [go.dev](https://go.dev), and [python.org](https://www.python.org) — which bring the total even higher. Plus the [link in the formatted heading](https://example.com) and the links scattered through block quotes, list items, and nested formatting sections. Counting them all is left as an exercise for the parser.

[ref1]: https://example.com/reference-one "Reference Link One"
[ref2]: https://example.com/reference-two "Reference Link Two"

## Lists — Ordered and Unordered

Lists interact with every other element. A list item can contain paragraphs, block quotes, code blocks, and nested lists. For a prose quality checker, list items are prose content that must be analyzed for readability.

- First item in an unordered list, contains a straightforward sentence
- Second item with **bold text** for emphasis
- Third item with a [link to somewhere](https://example.com/list-link) embedded in it
- Fourth item that is deliberately much longer than the others because we want to test whether the parser handles long list items that wrap across multiple lines in the rendered output correctly
- Fifth item with `inline code` and *italic text* mixed together

1. First ordered item explains the initial step in a process
2. Second ordered item follows up with the next logical action
3. Third ordered item concludes the sequence with a final result

Here is a paragraph between lists to test the parser's ability to close a list context and return to normal paragraphs before opening a new list context.

- Standalone single-item unordered list

And another paragraph after the single-item list, testing that the parser does not accidentally merge this paragraph with the list item above.

- Item one of a nested list demonstration
  - Nested item one-A under the first item
  - Nested item one-B under the first item
- Item two of the outer list, no nesting here
- Item three with a nested ordered list inside
  1. Nested ordered item three-A
  2. Nested ordered item three-B

## Whitespace — The Invisible Adversary

Whitespace is the most dangerous class of input for a text parser because it is invisible. You cannot see it in most editors, it varies between operating systems, and Unicode defines dozens of whitespace characters beyond the basic space and tab.

This  paragraph  has  multiple  spaces  between  every  word. Some parsers collapse multiple spaces into one. Others preserve them. For a prose quality checker, the behavior matters because word boundary detection depends on it.

This paragraph has trailing whitespace on some lines.  
And this line follows after trailing spaces that create a hard line break.  
Another line with trailing whitespace for good measure.

This paragraph mentions a tab character. Tabs are not the same as spaces, even though they look similar in many editors. A	tab	embedded	in	the	middle	of	words tests whether the parser handles tab characters inside paragraph text without corrupting word boundaries or triggering code block detection.

Here is a paragraph with a non-breaking space (U+00A0): word word. That invisible character between "word" and "word" is not a regular space — it is Unicode character U+00A0, NO-BREAK SPACE. Some tokenizers treat it as whitespace; others treat it as a regular character. Either behavior is defensible, but the parser must not crash on it.

And here is a paragraph with a zero-width space (U+200B): word​word. There is a zero-width space between "word" and "word" in the source. This character has zero width, so it is truly invisible, but it is a valid Unicode character that might appear in copy-pasted text. The parser must handle it without panicking.

This paragraph is preceded by three blank lines in the source. Extra blank lines should be collapsed into a single paragraph break. The parser should not create empty paragraph nodes for the blank lines themselves.

## Unicode — Beyond ASCII

The assumption that text is ASCII is the original sin of text processing. Real-world prose uses the full breadth of Unicode, and a parser that chokes on non-ASCII characters is useless for any language besides English — and even English uses non-ASCII characters more than most programmers realize.

Em-dashes — like this one — are used in English prose all the time. They are Unicode character U+2014, and they often appear where a parser might expect two hyphens. Similarly, en-dashes – like this one – are used for ranges and are U+2013.

"Smart quotes" and 'smart apostrophes' are another common source of parser bugs. These are distinct Unicode characters: U+201C (left double), U+201D (right double), U+2018 (left single), U+2019 (right single). Many parsers only handle the ASCII straight quotes " and ' and break when they encounter their typographic cousins.

The ellipsis character … (U+2026) is a single character, not three periods. Sentence boundary detection must treat it as a single punctuation mark, not as three separate periods that might each terminate a sentence.

Let us add some emoji for good measure: 🎉 This paragraph celebrates with party poppers. 🚀 Rockets indicate progress. ✨ Sparkles mean something magical. Emoji are multi-byte UTF-8 sequences, and some of them are even multi-codepoint (emoji with skin tone modifiers, for example). Your parser should handle them as regular characters.

Right-to-left text is the final boss of Unicode handling. Here is some Arabic: مرحبا بكم في اختبار المحلل. The text direction changes mid-paragraph, which can confuse parsers that assume left-to-right ordering. Bidirectional text requires careful handling of the Unicode Bidirectional Algorithm, though most markdown parsers can safely treat it as opaque character data.

CJK characters require no spaces between words: 你好世界这是一个测试。Japanese text works similarly: こんにちは世界。Korean text too: 안녕하세요 세계. Word boundary detection in CJK text cannot rely on spaces — it requires language-specific segmentation rules, which is why ICU4X exists.

Combining characters are a subtle trap. The word "café" can be represented two ways in Unicode: as five characters (c-a-f-é, where é is U+00E9, a precomposed character), or as six characters (c-a-f-e-◌́, where the é is an e followed by U+0301, a combining acute accent). Both representations look identical when rendered, but they have different byte lengths and different character counts. Your parser must handle both without double-counting or miscounting characters.

Here is a paragraph that mixes everything: She said "hello" — greeting him with a wave 🎉 — before switching to Arabic (مرحبا) and then Chinese (你好). The café served résumés… or was it naïve über-documents? Either way, the parser must survive this paragraph intact.

## **An Entirely Bold Paragraph Test**

**This entire paragraph is bold. Every single word, from the first to the last, is wrapped in bold formatting. The parser should set the bold flag for this paragraph. This is important for prose quality analysis because an entirely bold paragraph might indicate a warning, an important note, or just poor formatting taste. Regardless of the reason, the bold flag must be set correctly.**

**And this is a second entirely bold paragraph, because one is not enough to be sure. If the parser correctly identifies both of these paragraphs as bold, the bold detection logic is working. If it only catches one, there is a state management bug.**

*This entire paragraph is italic. Every single word is wrapped in italic formatting. The parser should set the italic flag for this paragraph. Italic paragraphs are common in block quotes, attributions, and introductory text. The italic flag must be set independently of the bold flag — they are not mutually exclusive.*

## Nested Formatting — The Combinatorial Explosion

Formatting can nest in arbitrary combinations: **bold with *italic inside* the bold** and *italic with **bold inside** the italic*. These nested combinations must not confuse the formatting state tracker.

What about ***bold italic*** as a single unit? Three asterisks open both bold and italic simultaneously, and three asterisks close both. The parser must handle this without getting the nesting order wrong.

Here is **bold with `code inside` the bold** and *italic with `code inside` the italic*. Inline code inside bold or italic is common in technical writing.

And **bold with [a link](https://example.com/bold-link) inside** and *italic with [a link](https://example.com/italic-link) inside*. Links inside formatting, formatting inside links — all valid, all must work.

A deeply nested example: **bold *italic `code` italic* bold**. That is code inside italic inside bold. Three levels of formatting nesting. The parser must track all three levels correctly and close them in the right order.

## The Final Paragraph

If you have reached this point and your parser has not panicked, produced wrong structure, or silently dropped content, you are in good shape. This document has tested headings at every level (including setext and empty headings), thematic breaks in every variant, HTML inline and block elements, escaped characters, images with and without alt text, code blocks both fenced and indented, tables with formatting, block quotes nested five levels deep, dozens of links in various styles, whitespace attacks including non-breaking and zero-width spaces, Unicode from every major script family, and combinatorial formatting nesting.

The adversarial mindset is the only reliable way to build a correct parser. Assume your code is wrong. Write inputs that would break it. Fix the breaks. Repeat until you run out of ideas, then ask someone else to try to break it. That is the process. There are no shortcuts.

This document is not exhaustive. There are edge cases we have not covered — footnotes, definition lists, task lists, strikethrough, subscript, superscript, and whatever new extensions the markdown ecosystem invents next. But the cases here represent the core attack surface. Master these, and the rest will follow.
