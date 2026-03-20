use crate::locale::Locale;
use crate::metadata::DocumentMetadata;

#[derive(Debug, Clone)]
pub struct Document {
    pub locale: Locale,
    pub sections: Vec<Section>,
    pub metadata: DocumentMetadata,
}

#[derive(Debug, Clone)]
pub struct Section {
    pub heading: Option<Heading>,
    pub blocks: Vec<Block>,
}

#[derive(Debug, Clone)]
pub struct Heading {
    pub level: u8,
    pub text: String,
}

#[derive(Debug, Clone)]
pub enum Block {
    Paragraph(Paragraph),
    List(ListBlock),
    BlockQuote(Vec<Self>),
    CodeBlock(String),
}

#[derive(Debug, Clone)]
pub struct Paragraph {
    pub sentences: Vec<Sentence>,
    pub has_bold: bool,
    pub has_italic: bool,
    pub links: Vec<Link>,
}

#[derive(Debug, Clone)]
pub struct Sentence {
    pub text: String,
    pub words: Vec<Word>,
}

impl Sentence {
    /// Word count derived from the words vec — single source of truth.
    #[must_use]
    pub const fn word_count(&self) -> usize {
        self.words.len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Word {
    pub text: String,
    pub syllable_count: usize,
}

#[derive(Debug, Clone)]
pub struct Link {
    pub text: String,
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct ListBlock {
    pub ordered: bool,
    pub items: Vec<String>,
}
