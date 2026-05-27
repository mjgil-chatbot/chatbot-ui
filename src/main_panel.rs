//! Main chat panel UI
//!
//! Renders the main chat interface including the message list,
//! system prompt area, JSON summary, and input controls.

use eframe::egui;
use std::sync::Arc;
use tmc::egui::MarkdownCache;

pub mod header;
pub mod input;
pub mod summary;

use crate::dialogs::DialogState;
use crate::message::{CopyFeedback, render_processing_indicator};
use crate::message_list::{self, MessageListState};
use crate::traits::{
    JsonSummaryView, MessageView, ModelCatalog, ProcessingState, ProviderInfo, SessionView,
    UiActions, WorkspaceList,
};

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
    }
}

impl Default for MainPanelState {
    fn default() -> Self {
        Self::new()
    }
}

const CONTENT_MAX_WIDTH: f32 = 820.0;

pub fn render(
    ui: &mut egui::Ui,
    session: Option<&Arc<dyn SessionView>>,
    provider: &dyn ProviderInfo,
    model_catalog: Option<&dyn ModelCatalog>,
    processing: &dyn ProcessingState,
    workspaces: Option<&dyn WorkspaceList>,
    actions: &mut dyn UiActions,
    message_state: &mut MessageListState,
    dialog_state: &mut DialogState,
    panel_state: &mut MainPanelState,
    markdown_cache: &mut MarkdownCache,
    copy_feedback: &mut CopyFeedback,
) {
    let content_width = ui.available_width().min(CONTENT_MAX_WIDTH);
    let content_height = ui.available_height();
    let child_rect = egui::Rect::from_min_size(
        ui.cursor().left_top(),
        egui::vec2(content_width, content_height),
    );
    let mut content_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(child_rect)
            .layout(egui::Layout::top_down(egui::Align::LEFT)),
    );
    content_ui.set_clip_rect(ui.clip_rect().intersect(child_rect));
    content_ui.set_min_width(content_width);
    content_ui.set_max_width(content_width);
    content_ui.set_width(content_width);

    clamp_content_width(&mut content_ui, content_width);
    header::render_header(
        &mut content_ui,
        session,
        provider,
        model_catalog,
        workspaces,
        actions,
        panel_state,
    );
    render_panel_separator(&mut content_ui, content_width);
    clamp_content_width(&mut content_ui, content_width);

    if panel_state.show_system_prompt {
        render_system_prompt_area(&mut content_ui, panel_state, actions);
        content_ui.add_space(5.0);
        clamp_content_width(&mut content_ui, content_width);
    }

    if panel_state.show_json_summary {
        if panel_state.current_json_summary_full.is_none() {
            if let Some(ref session) = session {
                if let Some(summary) = session.json_summary() {
                    panel_state.current_json_summary_full = Some(summary.to_full_json());
                }
            }
        }
        if let Some(json) = panel_state.current_json_summary_full.as_ref() {
            summary::render_json_summary_area(&mut content_ui, json);
        }
        content_ui.add_space(5.0);
        clamp_content_width(&mut content_ui, content_width);
    }

    let body_rect = egui::Rect::from_min_size(
        content_ui.cursor().left_top(),
        egui::vec2(content_width, content_ui.available_height()),
    );
    let mut body_ui = content_ui.new_child(
        egui::UiBuilder::new()
            .max_rect(body_rect)
            .layout(egui::Layout::top_down(egui::Align::LEFT)),
    );
    body_ui.set_clip_rect(content_ui.clip_rect().intersect(body_rect));
    clamp_content_width(&mut body_ui, content_width);

    let has_session = session.is_some();
    let is_processing = processing.is_processing();

    match session {
        Some(session) if !session.messages().is_empty() => {
            message_list::render_message_area(
                &mut body_ui,
                session,
                processing,
                markdown_cache,
                copy_feedback,
                message_state,
            );
        }
        _ => {
            message_list::render_empty_message_area(&mut body_ui, "message_scroll");
        }
    }

    render_panel_separator(&mut body_ui, content_width);
    clamp_content_width(&mut body_ui, content_width);

    let input_hint = if is_processing {
        "This chat is processing...".to_string()
    } else {
        format!("Type your message to {}...", provider.display_name())
    };

    input::render_input_area(
        &mut body_ui,
        &mut panel_state.input_text,
        &mut panel_state.should_focus_input,
        actions,
        has_session && !is_processing,
        is_processing,
    );

    clamp_content_width(&mut body_ui, content_width);
    render_status_bar(&mut body_ui, session, provider, model_catalog, processing);
}

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
