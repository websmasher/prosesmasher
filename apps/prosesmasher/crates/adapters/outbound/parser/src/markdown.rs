//! Markdown → `Document` conversion via pulldown-cmark.
//!
//! Walks the pulldown-cmark event stream and builds the domain
//! `Document` tree: sections split at H1/H2 boundaries, paragraphs
//! with sentence/word segmentation, formatting flags, links,
//! lists, block quotes, and code blocks.

use prosesmasher_domain_types::{
    Block, Document, DocumentMetadata, Heading, HeadingCounts, Link, ListBlock, Locale, Paragraph,
    Section,
};
use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd, TextMergeStream};

use crate::html_text::{extract_html_block_paragraphs, extract_inline_html_text};
use crate::segmenter::segment_paragraph;

/// Parse a markdown string into a `Document`.
///
/// Sections are split at H1 and H2 boundaries. Content before the
/// first H1/H2 goes into a headingless first section. Each H1/H2
/// starts a new section. H3+ headings are recorded in metadata but
/// do not create new sections.
///
/// # Errors
///
/// Currently infallible — pulldown-cmark does not fail on any input.
/// Returns `Document` directly. Error handling reserved for future
/// validation steps.
#[must_use]
pub fn parse_markdown(markdown: &str, locale: Locale) -> Document {
    let options = Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH;
    let parser = TextMergeStream::new(Parser::new_ext(markdown, options));

    let mut builder = DocumentBuilder::new(locale);

    for event in parser {
        builder.process_event(event);
    }

    builder.finish()
}

// ─── Builder state machine ───────────────────────────────────────

/// State for a list being built. Stacked to support nesting.
struct ListState {
    ordered: bool,
    items: Vec<String>,
}

#[allow(clippy::struct_excessive_bools)] // state machine needs multiple bool flags
struct DocumentBuilder {
    locale: Locale,
    sections: Vec<Section>,

    // Current section being built
    current_heading: Option<Heading>,
    current_blocks: Vec<Block>,

    // Paragraph accumulation
    para_text: String,
    para_has_bold: bool,
    para_has_italic: bool,
    para_links: Vec<Link>,
    in_paragraph: bool,

    // Emphasis tracking
    bold_depth: usize,
    italic_depth: usize,

    // Link tracking
    in_link: bool,
    link_url: String,
    link_text: String,

    // List tracking — stacked for nesting (BUG-03 fix)
    list_stack: Vec<ListState>,
    list_item_text: String,
    in_list_item: bool,

    // Code block tracking
    in_code_block: bool,
    code_block_text: String,

    // Block quote tracking
    blockquote_depth: usize,
    blockquote_blocks: Vec<Vec<Block>>,

    // Heading text accumulation
    in_heading: bool,
    heading_level: u8,
    heading_text: String,

    // Image tracking — skip image alt text
    in_image: bool,

    // H3+ headings tracked for metadata (not section headings)
    sub_headings: Vec<Heading>,

    // Raw HTML block tracking — preserve visible text from block HTML
    in_html_block: bool,
    html_block_content: String,
}

impl DocumentBuilder {
    #[allow(clippy::missing_const_for_fn)] // Vec::new() is not const-stable
    fn new(locale: Locale) -> Self {
        Self {
            locale,
            sections: Vec::new(),
            current_heading: None,
            current_blocks: Vec::new(),
            para_text: String::new(),
            para_has_bold: false,
            para_has_italic: false,
            para_links: Vec::new(),
            in_paragraph: false,
            bold_depth: 0,
            italic_depth: 0,
            in_link: false,
            link_url: String::new(),
            link_text: String::new(),
            list_stack: Vec::new(),
            list_item_text: String::new(),
            in_list_item: false,
            in_code_block: false,
            code_block_text: String::new(),
            blockquote_depth: 0,
            blockquote_blocks: Vec::new(),
            in_heading: false,
            heading_level: 0,
            heading_text: String::new(),
            in_image: false,
            sub_headings: Vec::new(),
            in_html_block: false,
            html_block_content: String::new(),
        }
    }

    fn process_event(&mut self, event: Event<'_>) {
        match event {
            Event::Start(tag) => self.handle_start(tag),
            Event::End(tag_end) => self.handle_end(tag_end),
            Event::Text(text) => self.handle_text(&text),
            Event::Code(code) => self.handle_code(&code),
            Event::Html(html) => self.handle_html(&html),
            Event::InlineHtml(html) => self.handle_inline_html(&html),
            Event::SoftBreak | Event::HardBreak => self.handle_break(),
            Event::Rule
            | Event::FootnoteReference(_)
            | Event::InlineMath(_)
            | Event::DisplayMath(_)
            | Event::TaskListMarker(_) => {
                // Ignored for prose analysis
            }
        }
    }

    #[allow(clippy::needless_pass_by_value)] // pulldown-cmark yields owned Tag
    fn handle_start(&mut self, tag: Tag<'_>) {
        match tag {
            Tag::Heading { level, .. } => {
                self.in_heading = true;
                self.heading_level = heading_level_to_u8(level);
                self.heading_text.clear();
            }
            Tag::Paragraph => {
                if !self.in_list_item {
                    self.begin_paragraph();
                }
            }
            Tag::Strong => {
                self.bold_depth = self.bold_depth.saturating_add(1);
                if self.in_paragraph {
                    self.para_has_bold = true;
                }
            }
            Tag::Emphasis => {
                self.italic_depth = self.italic_depth.saturating_add(1);
                if self.in_paragraph {
                    self.para_has_italic = true;
                }
            }
            Tag::Link { dest_url, .. } => {
                self.in_link = true;
                self.link_url = dest_url.into_string();
                self.link_text.clear();
            }
            Tag::Image { .. } => {
                self.in_image = true;
            }
            Tag::List(start_number) => {
                // Push new list state onto stack (supports nesting)
                self.list_stack.push(ListState {
                    ordered: start_number.is_some(),
                    items: Vec::new(),
                });
            }
            Tag::Item => {
                self.in_list_item = true;
                self.list_item_text.clear();
            }
            Tag::CodeBlock(_kind) => {
                self.in_code_block = true;
                self.code_block_text.clear();
            }
            Tag::BlockQuote(_) => {
                self.blockquote_depth = self.blockquote_depth.saturating_add(1);
                self.blockquote_blocks.push(Vec::new());
            }
            Tag::FootnoteDefinition(_)
            | Tag::Table(_)
            | Tag::TableHead
            | Tag::TableRow
            | Tag::TableCell
            | Tag::Strikethrough
            | Tag::MetadataBlock(_)
            | Tag::DefinitionList
            | Tag::DefinitionListTitle
            | Tag::DefinitionListDefinition => {}
            Tag::HtmlBlock => {
                self.in_html_block = true;
                self.html_block_content.clear();
            }
        }
    }

    #[allow(clippy::too_many_lines)] // match arms for all TagEnd variants
    fn handle_end(&mut self, tag_end: TagEnd) {
        match tag_end {
            TagEnd::Heading(_level) => {
                self.in_heading = false;
                let level = self.heading_level;
                let text = self.heading_text.trim().to_owned();

                // H1 and H2 start new sections (BUG-01/02 fix)
                if level <= 2 {
                    self.flush_section();
                    self.current_heading = Some(Heading { level, text });
                } else {
                    // H3+ headings recorded for metadata but don't create sections
                    self.sub_headings.push(Heading { level, text });
                }
            }
            TagEnd::Paragraph => {
                if !self.in_list_item {
                    self.end_paragraph();
                }
            }
            TagEnd::Strong => {
                self.bold_depth = self.bold_depth.saturating_sub(1);
            }
            TagEnd::Emphasis => {
                self.italic_depth = self.italic_depth.saturating_sub(1);
            }
            TagEnd::Link => {
                self.in_link = false;
                let link = Link {
                    text: self.link_text.trim().to_owned(),
                    url: self.link_url.clone(),
                };
                if self.in_paragraph {
                    self.para_links.push(link);
                }
                self.link_url.clear();
                self.link_text.clear();
            }
            TagEnd::Image => {
                self.in_image = false;
            }
            TagEnd::List(_) => {
                // Pop the list state from the stack (BUG-03 fix)
                if let Some(list_state) = self.list_stack.pop() {
                    let block = Block::List(ListBlock {
                        ordered: list_state.ordered,
                        items: list_state.items,
                    });
                    self.push_block(block);
                }
            }
            TagEnd::Item => {
                self.in_list_item = false;
                let item = self.list_item_text.trim().to_owned();
                if !item.is_empty() {
                    // Push to the current (innermost) list on the stack
                    if let Some(list_state) = self.list_stack.last_mut() {
                        list_state.items.push(item);
                    }
                }
                self.list_item_text.clear();
            }
            TagEnd::CodeBlock => {
                self.in_code_block = false;
                let code = self.code_block_text.clone();
                self.push_block(Block::CodeBlock(code));
                self.code_block_text.clear();
            }
            TagEnd::BlockQuote(_) => {
                let inner_blocks = self.blockquote_blocks.pop().unwrap_or_default();
                let quote = Block::BlockQuote(inner_blocks);
                self.blockquote_depth = self.blockquote_depth.saturating_sub(1);
                self.push_block(quote);
            }
            TagEnd::FootnoteDefinition
            | TagEnd::Table
            | TagEnd::TableHead
            | TagEnd::TableRow
            | TagEnd::TableCell
            | TagEnd::Strikethrough
            | TagEnd::MetadataBlock(_)
            | TagEnd::DefinitionList
            | TagEnd::DefinitionListTitle
            | TagEnd::DefinitionListDefinition => {}
            TagEnd::HtmlBlock => {
                self.in_html_block = false;
                self.flush_html_block();
            }
        }
    }

    fn handle_text(&mut self, text: &str) {
        if self.in_html_block {
            self.html_block_content.push_str(text);
            return;
        }

        if self.in_code_block {
            self.code_block_text.push_str(text);
            return;
        }

        if self.in_image {
            return;
        }

        // Always accumulate link text if inside a link (BUG-04/05 fix)
        if self.in_link {
            self.link_text.push_str(text);
        }

        if self.in_heading {
            self.heading_text.push_str(text);
            return;
        }

        if self.in_list_item {
            self.list_item_text.push_str(text);
            return;
        }

        if self.in_paragraph {
            self.para_text.push_str(text);
        }
    }

    fn handle_code(&mut self, code: &str) {
        // Inline code: add text but don't parse as formatting
        if self.in_list_item {
            self.list_item_text.push_str(code);
            return;
        }
        if self.in_paragraph {
            self.para_text.push_str(code);
        }
        if self.in_heading {
            self.heading_text.push_str(code);
        }
    }

    fn handle_html(&mut self, html: &str) {
        if self.in_html_block {
            self.html_block_content.push_str(html);
            return;
        }

        self.handle_inline_html(html);
    }

    fn handle_inline_html(&mut self, html: &str) {
        let extracted = extract_inline_html_text(html);
        if extracted.is_empty() {
            return;
        }

        self.handle_text(&extracted);
    }

    fn handle_break(&mut self) {
        if self.in_paragraph {
            self.para_text.push(' ');
        }
        if self.in_list_item {
            self.list_item_text.push(' ');
        }
    }

    fn begin_paragraph(&mut self) {
        self.in_paragraph = true;
        self.para_text.clear();
        self.para_has_bold = false;
        self.para_has_italic = false;
        self.para_links.clear();
    }

    fn end_paragraph(&mut self) {
        self.in_paragraph = false;
        let text = self.para_text.trim().to_owned();
        if text.is_empty() {
            return;
        }

        let sentences = segment_paragraph(&text, self.locale);
        if sentences.is_empty() {
            return;
        }

        let paragraph = Paragraph {
            sentences,
            has_bold: self.para_has_bold,
            has_italic: self.para_has_italic,
            links: self.para_links.clone(),
        };
        self.push_block(Block::Paragraph(paragraph));

        self.para_text.clear();
        self.para_links.clear();
    }

    fn flush_html_block(&mut self) {
        let paragraphs = extract_html_block_paragraphs(&self.html_block_content);
        for paragraph_text in paragraphs {
            self.push_paragraph_from_text(&paragraph_text, false, false, Vec::new());
        }
        self.html_block_content.clear();
    }

    fn push_paragraph_from_text(
        &mut self,
        text: &str,
        has_bold: bool,
        has_italic: bool,
        links: Vec<Link>,
    ) {
        let trimmed = text.trim();
        if trimmed.is_empty() {
            return;
        }

        let sentences = segment_paragraph(trimmed, self.locale);
        if sentences.is_empty() {
            return;
        }

        let paragraph = Paragraph {
            sentences,
            has_bold,
            has_italic,
            links,
        };
        self.push_block(Block::Paragraph(paragraph));
    }

    fn push_block(&mut self, block: Block) {
        if self.blockquote_depth > 0 {
            if let Some(inner) = self.blockquote_blocks.last_mut() {
                inner.push(block);
            }
        } else {
            self.current_blocks.push(block);
        }
    }

    fn flush_section(&mut self) {
        if self.current_heading.is_some() || !self.current_blocks.is_empty() {
            self.sections.push(Section {
                heading: self.current_heading.take(),
                blocks: std::mem::take(&mut self.current_blocks),
            });
        }
    }

    fn finish(mut self) -> Document {
        if self.in_paragraph {
            self.end_paragraph();
        }

        if self.in_html_block {
            self.flush_html_block();
        }

        self.flush_section();

        if self.sections.is_empty() && !self.current_blocks.is_empty() {
            self.sections.push(Section {
                heading: None,
                blocks: std::mem::take(&mut self.current_blocks),
            });
        }

        let metadata = compute_metadata(&self.sections, &self.sub_headings);

        Document {
            locale: self.locale,
            sections: self.sections,
            metadata,
        }
    }
}

// ─── Metadata computation ────────────────────────────────────────

fn compute_metadata(sections: &[Section], sub_headings: &[Heading]) -> DocumentMetadata {
    let mut meta = DocumentMetadata::default();

    for section in sections {
        if let Some(ref heading) = section.heading {
            count_heading(&mut meta.heading_counts, heading.level);
        }
        count_blocks(&section.blocks, &mut meta);
    }

    for heading in sub_headings {
        count_heading(&mut meta.heading_counts, heading.level);
    }

    meta
}

const fn count_heading(counts: &mut HeadingCounts, level: u8) {
    match level {
        1 => counts.h1 = counts.h1.saturating_add(1),
        2 => counts.h2 = counts.h2.saturating_add(1),
        3 => counts.h3 = counts.h3.saturating_add(1),
        _ => counts.h4_plus = counts.h4_plus.saturating_add(1),
    }
}

fn count_blocks(blocks: &[Block], meta: &mut DocumentMetadata) {
    for block in blocks {
        match block {
            Block::Paragraph(p) => {
                meta.paragraph_count = meta.paragraph_count.saturating_add(1);
                if p.has_bold {
                    meta.bold_count = meta.bold_count.saturating_add(1);
                }
                if p.has_italic {
                    meta.italic_count = meta.italic_count.saturating_add(1);
                }
                meta.link_count = meta.link_count.saturating_add(p.links.len());
                for sentence in &p.sentences {
                    meta.total_sentences = meta.total_sentences.saturating_add(1);
                    meta.total_words = meta.total_words.saturating_add(sentence.words.len());
                    for word in &sentence.words {
                        meta.total_syllables =
                            meta.total_syllables.saturating_add(word.syllable_count);
                    }
                }
            }
            Block::BlockQuote(inner) => count_blocks(inner, meta),
            Block::List(_) | Block::CodeBlock(_) => {}
        }
    }
}

const fn heading_level_to_u8(level: pulldown_cmark::HeadingLevel) -> u8 {
    match level {
        pulldown_cmark::HeadingLevel::H1 => 1,
        pulldown_cmark::HeadingLevel::H2 => 2,
        pulldown_cmark::HeadingLevel::H3 => 3,
        pulldown_cmark::HeadingLevel::H4 => 4,
        pulldown_cmark::HeadingLevel::H5 => 5,
        pulldown_cmark::HeadingLevel::H6 => 6,
    }
}

#[cfg(test)]
#[path = "markdown_tests.rs"]
mod tests;
