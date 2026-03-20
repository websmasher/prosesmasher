#[derive(Debug, Clone, Default)]
pub struct DocumentMetadata {
    pub total_words: usize,
    pub total_sentences: usize,
    pub total_syllables: usize,
    pub heading_counts: HeadingCounts,
    pub bold_count: usize,
    pub italic_count: usize,
    pub paragraph_count: usize,
    pub link_count: usize,
}

#[derive(Debug, Clone, Default)]
pub struct HeadingCounts {
    pub h1: usize,
    pub h2: usize,
    pub h3: usize,
    pub h4_plus: usize,
}
