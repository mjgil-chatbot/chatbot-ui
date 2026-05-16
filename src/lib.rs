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
pub mod message;
pub mod message_list;
pub mod side_panel;
pub mod traits;
pub use dialogs::{DialogState, render_new_session_dialog, render_rename_session_dialog};
pub use main_panel::MainPanelState;
pub use traits::*;
use eframe::egui;
use std::sync::Arc;
use traits::{ModelCatalog, ProcessingState, ProviderInfo, SessionView, UiActions, WorkspaceList};
/// Arguments needed to run the chatbot UI.
pub struct ChatbotUiArgs<A: UiActions> {
    pub session: Option<Arc<dyn SessionView>>,
    pub provider: Arc<dyn ProviderInfo>,
    pub model_catalog: Option<Arc<dyn ModelCatalog>>,
    pub processing: Arc<dyn ProcessingState>,
    pub workspaces: Option<Arc<dyn WorkspaceList>>,
    pub actions: A,
}
