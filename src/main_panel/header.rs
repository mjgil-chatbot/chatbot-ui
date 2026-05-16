//! Header rendering for the main chat panel.

use eframe::egui;
use std::sync::Arc;

use crate::traits::{ModelCatalog, ProviderInfo, SessionView, UiActions, WorkspaceList};

pub fn render_header(
    ui: &mut egui::Ui,
    session: Option<&Arc<dyn SessionView>>,
    provider: &dyn ProviderInfo,
    model_catalog: Option<&dyn ModelCatalog>,
    workspaces: Option<&dyn WorkspaceList>,
    actions: &mut dyn UiActions,
    state: &mut super::MainPanelState,
) -> egui::Rect {
    let header_width = ui.available_width();

    let response = ui.allocate_ui(egui::vec2(header_width, 0.0), |ui| {
        clamp_content_width(ui, header_width);
        ui.vertical(|ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let system_prompt_text = if state.show_system_prompt {
                        "Hide System Prompt"
                    } else {
                        "Show System Prompt"
                    };
                    if ui.button(system_prompt_text).clicked() {
                        state.show_system_prompt = !state.show_system_prompt;
                    }

                    let json_summary_text = if state.show_json_summary {
                        "Hide JSON Summary"
                    } else {
                        "Show JSON Summary"
                    };
                    if ui.button(json_summary_text).clicked() {
                        state.show_json_summary = !state.show_json_summary;
                    }
                });

                let title_width = ui.available_width().max(0.0);
                ui.allocate_ui_with_layout(
                    egui::vec2(title_width, 0.0),
                    egui::Layout::top_down(egui::Align::LEFT),
                    |ui| {
                        ui.set_width(title_width);
                        ui.spacing_mut().item_spacing.y = 2.0;

                        show_header_row(ui, |ui| {
                            if let Some(session) = session {
                                let id_hash = &session.id()[..8.min(session.id().len())];
                                let title = format!("{} ({})", session.name(), id_hash);
                                ui.add(
                                    egui::Label::new(egui::RichText::new(title).heading())
                                        .wrap_mode(egui::TextWrapMode::Truncate),
                                );
                            } else {
                                ui.add_sized(
                                    [title_width, 0.0],
                                    egui::Label::new(egui::RichText::new("No Session").heading())
                                        .wrap_mode(egui::TextWrapMode::Truncate),
                                );
                            }
                        });
                    },
                );
            });

            ui.add_space(6.0);
            render_workspace_picker(ui, workspaces, actions, state);

            if let Some(catalog) = model_catalog {
                ui.add_space(4.0);
                render_model_picker(ui, catalog, actions, state);
            }
        });
    });
    response.response.rect
}

fn show_header_row<R>(
    ui: &mut egui::Ui,
    add_contents: impl FnOnce(&mut egui::Ui) -> R,
) -> egui::InnerResponse<R> {
    egui::Frame::none()
        .inner_margin(egui::Margin::same(6.0))
        .show(ui, add_contents)
}

fn render_workspace_picker(
    ui: &mut egui::Ui,
    workspaces: Option<&dyn WorkspaceList>,
    actions: &mut dyn UiActions,
    _state: &mut super::MainPanelState,
) {
    let Some(workspaces) = workspaces else {
        return;
    };

    show_header_row(ui, |ui| {
        ui.label("Workspace");
        let selected = workspaces.selected();
        let display = selected
            .map(|s| s.split('/').last().unwrap_or(s).to_string())
            .unwrap_or_else(|| "Select workspace".to_string());

        egui::ComboBox::from_id_salt("workspace_picker")
            .selected_text(display)
            .show_ui(ui, |ui| {
                for dir in workspaces.workspaces() {
                    let short = dir.split('/').last().unwrap_or(&dir).to_string();
                    if ui
                        .selectable_label(false, short)
                        .on_hover_text(&dir)
                        .clicked()
                    {
                        actions.select_workspace(&dir);
                    }
                }
            });
    });
}

fn render_model_picker(
    ui: &mut egui::Ui,
    catalog: &dyn ModelCatalog,
    actions: &mut dyn UiActions,
    state: &mut super::MainPanelState,
) {
    let models = catalog.available_models();
    let selected_id = catalog.selected_model_id();
    let selected_label = models
        .iter()
        .find(|m| m.0 == selected_id)
        .map(|m| m.1)
        .unwrap_or(selected_id);

    show_header_row(ui, |ui| {
        ui.label("Model");
        egui::ComboBox::from_id_salt("model_picker")
            .selected_text(selected_label)
            .show_ui(ui, |ui| {
                for (id, label) in models {
                    if ui
                        .selectable_value(&mut state.input_text.clone(), id.to_string(), label)
                        .clicked()
                    {
                        actions.select_model(id);
                    }
                }
            });
    });
}

fn clamp_content_width(ui: &mut egui::Ui, width: f32) {
    ui.set_width(width);
    ui.set_min_width(width);
    ui.set_max_width(width);
}
