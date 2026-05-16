//! Chat input field with history and slash commands.

use eframe::egui;
use eframe::egui::TextBuffer as _;

use crate::traits::{SlashCommandSource, UiActions};

const MAIN_CHAT_INPUT_ID: &str = "main_chat_input";

pub struct InputState {
    pub input_text: String,
    pub should_focus_input: bool,
    pub slash_command_index: Option<usize>,
    pub input_history_draft: String,
    pub input_history_index: Option<usize>,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            input_text: String::new(),
            should_focus_input: false,
            slash_command_index: None,
            input_history_draft: String::new(),
            input_history_index: None,
        }
    }
}

impl Default for InputState {
    fn default() -> Self {
        Self::new()
    }
}

pub struct InputAreaSnapshot {
    pub rect: egui::Rect,
    pub input_focus_active: bool,
}

pub fn render_input_area(
    ui: &mut egui::Ui,
    input_text: &mut String,
    should_focus_input: &mut bool,
    actions: &mut dyn UiActions,
    submit_enabled: bool,
    is_processing: bool,
) -> InputAreaSnapshot {
    let available_width = ui.available_width();
    let can_stop = !is_processing;

    let inline_button_width = 70.0;
    let inline_button_gap = ui.spacing().item_spacing.x * 0.5;

    let hint_text = if is_processing {
        "This chat is processing...".to_string()
    } else {
        "Type your message...".to_string()
    };

    let response = ui.allocate_ui_with_layout(
        egui::vec2(available_width, 0.0),
        egui::Layout::top_down(egui::Align::LEFT),
        |ui| {
            ui.group(|ui| {
                let can_render_inline =
                    available_width >= inline_button_width + inline_button_gap + 160.0;

                if can_render_inline {
                    let text_width =
                        (available_width - inline_button_width - inline_button_gap).max(0.0);
                    let text_edit_response = render_text_edit(
                        ui,
                        input_text,
                        should_focus_input,
                        text_width,
                        &hint_text,
                    );

                    ui.add_enabled_ui(!is_processing && submit_enabled, |ui| {
                        if ui
                            .add_sized([inline_button_width, 25.0], egui::Button::new("Send"))
                            .clicked()
                        {
                            actions.submit_message(input_text.clone());
                            *input_text = String::new();
                        }
                    });
                } else {
                    let text_edit_response = render_text_edit(
                        ui,
                        input_text,
                        should_focus_input,
                        available_width,
                        &hint_text,
                    );

                    ui.add_space(inline_button_gap);
                    ui.add_enabled_ui(!is_processing && submit_enabled, |ui| {
                        if ui
                            .add_sized(
                                [inline_button_width.min(available_width), 25.0],
                                egui::Button::new("Send"),
                            )
                            .clicked()
                        {
                            actions.submit_message(input_text.clone());
                            *input_text = String::new();
                        }
                    });
                }
            });
        },
    );

    InputAreaSnapshot {
        rect: response.response.rect,
        input_focus_active: false,
    }
}

fn render_text_edit(
    ui: &mut egui::Ui,
    input_text: &mut String,
    should_focus_input: &mut bool,
    width: f32,
    hint_text: &str,
) -> egui::Response {
    let text_edit_response = ui.add_sized(
        [width, 25.0],
        egui::TextEdit::singleline(input_text)
            .id(main_chat_input_id())
            .hint_text(hint_text)
            .lock_focus(true)
            .return_key(None),
    );

    if *should_focus_input {
        text_edit_response.request_focus();
        *should_focus_input = false;
    }

    text_edit_response
}

pub fn handle_global_input(
    ctx: &egui::Context,
    input_text: &str,
    submit_enabled: bool,
    submit_fn: impl FnOnce(),
) {
    let input_id = main_chat_input_id();
    if !ctx.memory(|mem| mem.has_focus(input_id)) {
        return;
    }

    let submit_via_enter = submit_enabled && ctx.input(|input| input.key_pressed(egui::Key::Enter));

    if submit_via_enter {
        submit_fn();
    }
}

fn main_chat_input_id() -> egui::Id {
    egui::Id::new(MAIN_CHAT_INPUT_ID)
}
