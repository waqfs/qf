use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DocumentType {
    Page,
    Project,
    Article,
}

#[derive(Debug, Clone)]
pub struct Metadata {
    pub doc_type: DocumentType,
    pub title: String,
    pub summary: Option<String>,
    pub date: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Document {
    pub source_path: PathBuf,
    pub metadata: Metadata,
    pub route: String,
    pub blocks: Vec<Block>,
}

#[derive(Debug, Clone)]
pub enum Block {
    Paragraph(Vec<Inline>),
    Heading {
        level: u8,
        content: Vec<Inline>,
    },
    UnorderedList(Vec<Vec<Inline>>),
    OrderedList(Vec<Vec<Inline>>),
    BlockQuote(Vec<Inline>),
    CodeBlock {
        language: Option<String>,
        code: String,
    },
    Image(Image),
}

#[derive(Debug, Clone)]
pub struct Image {
    pub source: String,
    pub alt: ImageAlt,
    pub caption: Option<String>,
}

#[derive(Debug, Clone)]
pub enum ImageAlt {
    Description(String),
    Decorative,
}

#[derive(Debug, Clone)]
pub enum Inline {
    Text(String),
    Emphasis(Vec<Inline>),
    Strong(Vec<Inline>),
    Code(String),
    Link { label: Vec<Inline>, href: String },
}
