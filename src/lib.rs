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
pub mod state;
pub mod traits;

pub use dialogs::{DialogState, render_new_session_dialog, render_rename_session_dialog};
pub use main_panel::MainPanelState;
pub use message::{CopyFeedback, render_message, render_processing_indicator};
pub use message_list::{MessageListState, render_empty_message_area, render_message_area};
pub use state::UiState;
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

/// Run the chatbot UI with the given trait implementations.
pub fn run<A: UiActions + 'static>(args: ChatbotUiArgs<A>) -> Result<(), eframe::Error> {
    let ChatbotUiArgs {
        session,
        provider,
        model_catalog,
        processing,
        workspaces,
        actions,
    } = args;

    let app = ChatbotUiApp {
        session,
        provider,
        model_catalog,
        processing,
        workspaces,
        actions,
        message_state: MessageListState::new(),
        dialog_state: DialogState::new(),
        panel_state: MainPanelState::new(),
        markdown_cache: tmc::egui::MarkdownCache::default(),
        copy_feedback: CopyFeedback::new(),
    };

    let options = eframe::NativeOptions::default();
    eframe::run_native("Chatbot", options, Box::new(|_cc| Ok(Box::new(app))))
}

/// The eframe App wrapper that delegates to trait implementations.
struct ChatbotUiApp<A: UiActions> {
    session: Option<Arc<dyn SessionView>>,
    provider: Arc<dyn ProviderInfo>,
    model_catalog: Option<Arc<dyn ModelCatalog>>,
    processing: Arc<dyn ProcessingState>,
    workspaces: Option<Arc<dyn WorkspaceList>>,
    actions: A,
    message_state: MessageListState,
    dialog_state: DialogState,
    panel_state: MainPanelState,
    markdown_cache: tmc::egui::MarkdownCache,
    copy_feedback: CopyFeedback,
}

impl<A: UiActions> eframe::App for ChatbotUiApp<A> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Side panel
        side_panel::panel_config().show(ctx, |ui| {
            side_panel::render(
                ui,
                &self.get_sessions(),
                self.session.as_ref().map(|s| s.id()),
                &mut self.actions,
                &mut self.panel_state.side_panel,
            );
        });

        // Main panel
        egui::CentralPanel::default().show(ctx, |ui| {
            main_panel::render(
                ui,
                self.session.as_ref(),
                &*self.provider,
                self.model_catalog.as_ref().map(|m| &**m),
                &*self.processing,
                self.workspaces.as_ref().map(|w| &**w),
                &mut self.actions,
                &mut self.message_state,
                &mut self.dialog_state,
                &mut self.panel_state,
                &mut self.markdown_cache,
                &mut self.copy_feedback,
            );
        });

        // Dialogs
        dialogs::render_new_session_dialog(ctx, &mut self.actions, &mut self.dialog_state);
        dialogs::render_rename_session_dialog(ctx, &mut self.actions, &mut self.dialog_state);
    }
}

impl<A: UiActions> ChatbotUiApp<A> {
    fn get_sessions(&self) -> Vec<Arc<dyn SessionListItem>> {
        self.actions.get_sessions()
    }
}

use traits::SessionListItem;
