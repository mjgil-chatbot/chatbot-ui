//! Standalone egui UI for the chatbot application.
//!
//! All domain knowledge (providers, data models, persistence) lives in the host application.
//! The UI communicates with the host exclusively through trait interfaces defined in [`traits`].
//!
//! ## Usage
//!
//! The host creates concrete implementations of the trait interfaces and passes them
//! to [`run()`], which constructs the eframe app and runs it.

pub mod dialogs;
pub mod main_panel;
pub mod traits;
