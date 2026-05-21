//! Vendored subset of the markdown renderer used by `chatbot-ui`.

mod common;
pub mod document;
#[cfg(feature = "egui")]
pub mod egui;
