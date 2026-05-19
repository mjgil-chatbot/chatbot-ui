//! Main chat panel UI
//!
//! Renders the main chat interface including the message list,
//! system prompt area, JSON summary, and input controls.

use eframe::egui;
use std::sync::Arc;
pub mod header;
pub mod input;
pub mod summary;
use crate::traits::ModelCatalog;
use crate::traits::ProcessingState;
use crate::traits::ProviderInfo;
use crate::traits::SessionView;
use crate::traits::UiActions;
pub struct MainPanelState {
    pub side_panel: crate::side_panel::SidePanelState,
    pub show_system_prompt: bool,
    pub show_json_summary: bool,
    pub system_prompt_text: String,
    pub system_prompt_dirty: bool,
    pub current_json_summary_full: Option<String>,
    pub input_text: String,
    pub should_focus_input: bool,
    pub pending_layout_log: bool,
}
impl MainPanelState {
    pub fn new() -> Self {
        Self {
            side_panel: crate::side_panel::SidePanelState::new(),
            show_system_prompt: false,
            show_json_summary: false,
            system_prompt_text: String::new(),
            system_prompt_dirty: false,
            current_json_summary_full: None,
            input_text: String::new(),
            should_focus_input: false,
            pending_layout_log: false,
        }
    }}
impl Default for MainPanelState {
    fn default() -> Self {
        Self::new()
    }
}
const CONTENT_MAX_WIDTH: f32 = 820.0;
fn clamp_content_width(ui: &mut egui::Ui, width: f32) {
    ui.set_width(width);
    ui.set_min_width(width);
    ui.set_max_width(width);
}
fn render_panel_separator(ui: &mut egui::Ui, width: f32) {
    let separator_height = ui.spacing().item_spacing.y.max(6.0);
    let (rect, _) =
        ui.allocate_exact_size(egui::vec2(width, separator_height), egui::Sense::hover());
    ui.painter().hline(
        rect.x_range(),
        rect.center().y,
        ui.visuals().widgets.noninteractive.bg_stroke,
    );
}
fn render_system_prompt_area(
    ui: &mut egui::Ui,
    state: &mut MainPanelState,
    actions: &mut dyn UiActions,
) -> egui::Rect {
    let response = ui.group(|ui| {
        ui.label("System Prompt:");
        ui.add_sized(
            [ui.available_width(), 60.0],
            egui::TextEdit::multiline(&mut state.system_prompt_text)
                .hint_text("Enter system prompt here (optional)..."),
        );

        if state.system_prompt_dirty {
            actions.set_system_prompt(Some(state.system_prompt_text.clone()));
            state.system_prompt_dirty = false;
        }
    });
    response.response.rect
}
fn render_status_bar(
    ui: &mut egui::Ui,
    session: Option<&Arc<dyn SessionView>>,
    provider: &dyn ProviderInfo,
    model_catalog: Option<&dyn ModelCatalog>,
    processing: &dyn ProcessingState,
) -> egui::Rect {
    let response = ui.horizontal(|ui| {
        ui.label(format!("Provider: {}", provider.display_name()));

        if let Some(catalog) = model_catalog {
            ui.label("Model:");
            let selected = catalog.selected_model_id();
            ui.label(selected);
        }

        if let Some(session) = session {
            ui.label(format!("Messages: {}", session.messages().len()));
        }

        if processing.is_processing() {
            ui.colored_label(egui::Color32::from_rgb(255, 165, 0), "| Processing...");
        }
    });
    response.response.rect
}
