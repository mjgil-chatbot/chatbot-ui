use crate::common::Alignment;
use crate::inline::InlineNode;
use std::collections::{HashMap, VecDeque};
mod cache;
mod parse_support;
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MarkdownDocument {
    pub blocks: Vec<Block>,
    pub footnotes: Vec<FootnoteDefinition>,
    pub references: Vec<LinkReferenceDefinition>,
}
#[derive(Debug, Default, Clone)]
pub struct DocumentCache {
    full_documents: HashMap<String, MarkdownDocument>,
    fragment_documents: HashMap<String, MarkdownDocument>,
    full_order: VecDeque<String>,
    fragment_order: VecDeque<String>,
    full_document_bytes: usize,
    fragment_document_bytes: usize,
}
impl DocumentCache {
    pub fn clear(&mut self) {
        self.full_documents.clear();
        self.fragment_documents.clear();
        self.full_order.clear();
        self.fragment_order.clear();
        self.full_document_bytes = 0;
        self.fragment_document_bytes = 0;
    }}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Block {
    Paragraph(ParagraphBlock),
    Heading(HeadingBlock),
    CodeBlock(CodeBlock),
    Table(TableBlock),
    BlockQuote(Vec<Block>),
    DefinitionList(Vec<DefinitionItem>),
    TaskList(Vec<TaskItem>),
    UnorderedList(UnorderedListBlock),
    OrderedList(OrderedListBlock),
    HorizontalRule,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnorderedListBlock {
    pub indent: usize,
    pub items: Vec<ListItem>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderedListBlock {
    pub start: usize,
    pub indent: usize,
    pub items: Vec<ListItem>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParagraphBlock {
    pub lines: Vec<ParagraphLine>,
    pub content: InlineContent,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParagraphLine {
    pub content: InlineContent,
    pub hard_break: bool,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeadingBlock {
    pub level: usize,
    pub content: InlineContent,
    pub id: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeBlock {
    pub language: Option<String>,
    pub code: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableBlock {
    pub headers: Vec<InlineContent>,
    pub alignments: Vec<Option<Alignment>>,
    pub rows: Vec<Vec<InlineContent>>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefinitionItem {
    pub term: InlineContent,
    pub definitions: Vec<InlineContent>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskItem {
    pub checked: bool,
    pub content: InlineContent,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListItem {
    pub content: InlineContent,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InlineContent {
    pub source: String,
    pub nodes: Vec<InlineNode>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FootnoteDefinition {
    pub id: String,
    pub content: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkReferenceDefinition {
    pub label: String,
    pub url: String,
    pub title: Option<String>,
}
