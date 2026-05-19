//! Side panel UI for session management
//!
//! Renders the session list and provides controls for creating,
//! selecting, and deleting chat sessions.

use eframe::egui;
use std::sync::Arc;

use crate::traits::{SessionListItem, UiActions};

pub struct SidePanelState {
    pub search_filter: String,
}

impl SidePanelState {
    pub fn new() -> Self {
        Self {
            search_filter: String::new(),
        }
    }
}

impl Default for SidePanelState {
    fn default() -> Self {
        Self::new()
    }
}

/// Render the side panel with session list
pub fn render(
    ui: &mut egui::Ui,
    sessions: &[Arc<dyn SessionListItem>],
    highlight_id: Option<&str>,
    actions: &mut dyn UiActions,
    state: &mut SidePanelState,
) {
    ui.set_width(ui.available_width());
    let search_id = ui.make_persistent_id("session_search");

    ui.horizontal(|ui| {
        ui.heading("Chat Sessions");
    });
    ui.separator();

    ui.horizontal(|ui| {
        if ui.button("+ New Chat").clicked() {
            actions.show_new_session_dialog();
        }

        let search = egui::TextEdit::singleline(&mut state.search_filter)
            .id_source(search_id)
            .hint_text("Search…")
            .desired_width(ui.available_width());
        ui.add(search);
    });

    ui.separator();

    let mut clicked_session: Option<String> = None;

    egui::ScrollArea::vertical()
        .id_salt("sessions")
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Truncate);

            for session_info in sessions {
                let is_selected = highlight_id.map_or(false, |id| id == session_info.id());

                ui.push_id(session_info.id(), |ui| {
                    ui.horizontal(|ui| {
                        let delete_button_width = ui.spacing().interact_size.x;
                        let row_height = ui.spacing().interact_size.y;
                        let label_max_width = (ui.available_width()
                            - delete_button_width
                            - ui.spacing().item_spacing.x)
                            .max(0.0);

                        let label_text =
                            format!("{} ({})", session_info.name(), session_info.message_count());

                        let session_response = ui
                            .scope(|ui| {
                                ui.set_max_width(label_max_width);
                                ui.add(egui::SelectableLabel::new(
                                    is_selected,
                                    egui::RichText::new(&label_text),
                                ))
                            })
                            .inner;

                        if session_response.double_clicked() {
                            actions.show_rename_dialog(session_info.id().to_string());
                        } else if session_response.clicked() {
                            clicked_session = Some(session_info.id().to_string());
                        }

                        if ui
                            .add_sized([delete_button_width, row_height], egui::Button::new("🗑"))
                            .clicked()
                        {
                            actions.delete_session(session_info.id());
                        }
                    });
                });

                ui.add_space(2.0);
            }
        });

    if let Some(session_id) = clicked_session {
        actions.load_session(&session_id);
    }
}

/// Get the side panel configuration
pub fn panel_config() -> egui::SidePanel {
    egui::SidePanel::left("session_panel")
        .resizable(true)
        .default_width(280.0)
        .min_width(180.0)
        .max_width(560.0)
}
