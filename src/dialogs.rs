//! Dialog windows for the application.

use eframe::egui;

use crate::traits::UiActions;

pub struct DialogState {
    pub show_new_session_dialog: bool,
    pub new_session_name: String,
    pub show_rename_dialog: bool,
    pub rename_session_id: Option<String>,
    pub rename_session_name: String,
    pub system_prompt_text: String,
}

impl DialogState {
    pub fn new() -> Self {
        Self {
            show_new_session_dialog: false,
            new_session_name: String::new(),
            show_rename_dialog: false,
            rename_session_id: None,
            rename_session_name: String::new(),
            system_prompt_text: String::new(),
        }
    }
}

impl Default for DialogState {
    fn default() -> Self {
        Self::new()
    }
}

pub fn render_new_session_dialog(
    ctx: &egui::Context,
    actions: &mut dyn UiActions,
    state: &mut DialogState,
) {
    if !state.show_new_session_dialog {
        return;
    }

    egui::Window::new("New Chat Session")
        .collapsible(false)
        .resizable(false)
        .show(ctx, |ui| {
            ui.label("Session Name:");
            ui.text_edit_singleline(&mut state.new_session_name);

            ui.horizontal(|ui| {
                if ui.button("Create").clicked() {
                    let system_prompt = if state.system_prompt_text.trim().is_empty() {
                        None
                    } else {
                        Some(state.system_prompt_text.clone())
                    };
                    actions.create_session(state.new_session_name.clone(), system_prompt);
                    state.show_new_session_dialog = false;
                }

                if ui.button("Cancel").clicked() {
                    state.show_new_session_dialog = false;
                }
            });
        });
}

pub fn render_rename_session_dialog(
    ctx: &egui::Context,
    actions: &mut dyn UiActions,
    state: &mut DialogState,
) {
    if !state.show_rename_dialog {
        return;
    }

    egui::Window::new("Rename Chat Session")
        .collapsible(false)
        .resizable(false)
        .show(ctx, |ui| {
            ui.label("Session Name:");
            ui.text_edit_singleline(&mut state.rename_session_name);

            ui.horizontal(|ui| {
                if ui.button("Rename").clicked() {
                    if let Some(ref id) = state.rename_session_id {
                        actions.rename_session(id, state.rename_session_name.clone());
                    }
                    state.show_rename_dialog = false;
                }

                if ui.button("Cancel").clicked() {
                    state.show_rename_dialog = false;
                }
            });
        });
}
