//! Vendored subset of the markdown renderer used by `chatbot-ui`.

mod common;
pub mod document;
pub mod inline;
mod parse_util;
pub use document::{
    parse_document, parse_document_cached, parse_document_fragment, parse_document_fragment_cached,
    Block, CodeBlock, DefinitionItem, DocumentCache, FootnoteDefinition, HeadingBlock,
    InlineContent, LinkReferenceDefinition, ListItem, MarkdownDocument, OrderedListBlock,
    ParagraphBlock, ParagraphLine, TableBlock, TaskItem, UnorderedListBlock,
};
#[cfg(feature = "egui")]
pub mod egui;
