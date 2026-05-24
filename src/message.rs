//! Message rendering UI components
//!
//! Renders individual chat messages with markdown formatting and copy functionality.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

use chrono::{DateTime, Utc};
use eframe::egui;
use egui_extras::syntax_highlighting::{self, CodeTheme};
use tmc::egui::MarkdownCache;
use tmc::egui::render_markdown;

use crate::traits::{CachedMarkdown, MessageView};

pub struct CopyFeedback {
    pub feedback: HashMap<i64, Instant>,
}

impl CopyFeedback {
    pub fn new() -> Self {
        Self {
            feedback: HashMap::new(),
        }
    }

    pub fn get_and_cleanup(&mut self, msg_id: i64) -> bool {
        match self.feedback.get(&msg_id) {
            Some(t) if t.elapsed().as_secs_f32() < 2.0 => true,
            Some(_) => {
                self.feedback.remove(&msg_id);
                false
            }
            None => false,
        }
    }

    pub fn set(&mut self, msg_id: i64) {
        self.feedback.insert(msg_id, Instant::now());
    }
}

impl Default for CopyFeedback {
    fn default() -> Self {
        Self::new()
    }
}

/// Render a single chat message.
pub fn render_message(
    ui: &mut egui::Ui,
    message: &Arc<dyn MessageView>,
    markdown_cache: &mut MarkdownCache,
    copy_feedback: &mut CopyFeedback,
) -> egui::Rect {
    let response = ui.group(|ui| {
        let inner_w = ui.available_width();
        ui.set_width(inner_w);
        ui.set_min_width(inner_w);
        ui.set_max_width(inner_w);

        ui.horizontal(|ui| {
            if message.is_user() {
                ui.label("🧑 You:");
            } else {
                ui.label("🤖 Assistant:");
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let msg_id = message.timestamp().timestamp_millis();
                let show_checkmark = copy_feedback.get_and_cleanup(msg_id);

                let button_text = if show_checkmark {
                    "✓ Copied"
                } else {
                    "📋 Copy"
                };
                if ui.small_button(button_text).clicked() {
                    ui.ctx().copy_text(message.content().to_string());
                    copy_feedback.set(msg_id);
                }
                ui.label(
                    message
                        .timestamp()
                        .with_timezone(&chrono::Local)
                        .format("%b %-d %I:%M%P")
                        .to_string(),
                );
            });
        });

        ui.separator();

        if message.is_user() {
            let color = egui::Color32::from_rgb(70, 130, 180);
            ui.add(
                egui::Label::new(egui::RichText::new(message.content()).color(color))
                    .wrap_mode(egui::TextWrapMode::Wrap),
            );
        } else {
            render_assistant_markdown(ui, markdown_cache, message, inner_w);
        }
    });
    response.response.rect
}

fn render_assistant_markdown(
    ui: &mut egui::Ui,
    markdown_cache: &mut MarkdownCache,
    message: &Arc<dyn MessageView>,
    inner_w: f32,
) {
    let content = message.content();
    let width_key = inner_w.round().max(0.0) as i64;
    let id_salt = format!(
        "msg_{}_w{}_len{}",
        message.timestamp().timestamp_millis(),
        width_key,
        content.len()
    );

    if let Some(pretty_json) = format_standalone_json_message(content) {
        let fenced_json = format!("```json\n{}\n```", pretty_json);
        let render_start = Instant::now();
        render_markdown(ui, markdown_cache, &fenced_json, &id_salt);
        let elapsed = render_start.elapsed();
    } else {
        let escaped = escape_underscores_in_tool_names(content);
        let render_start = Instant::now();
        render_markdown(ui, markdown_cache, &escaped, &id_salt);
        let elapsed = render_start.elapsed();
    }
}

fn format_standalone_json_message(content: &str) -> Option<String> {
    let trimmed = content.trim();
    let parsed = serde_json::from_str::<serde_json::Value>(trimmed).ok()?;
    serde_json::to_string_pretty(&parsed).ok()
}

fn escape_underscores_in_tool_names(content: &str) -> String {
    content.replace("__", "\\_\\_")
}

/// Render the processing indicator
pub fn render_processing_indicator(ui: &mut egui::Ui) {
    ui.group(|ui| {
        ui.horizontal(|ui| {
            ui.label("🤖 Assistant:");
            ui.spinner();
            ui.label("Thinking...");
        });
    });
}

fn is_standalone_json_message(content: &str) -> bool {
    let trimmed = content.trim();
    if !(trimmed.starts_with('{') || trimmed.starts_with('[')) {
        return false;
    }
    serde_json::from_str::<serde_json::Value>(trimmed).is_ok()
}

#[cfg(test)]
mod tests {
    use super::{
        escape_underscores_in_tool_names, format_standalone_json_message,
        is_standalone_json_message,
    };

    #[test]
    fn standalone_json_object_is_detected() {
        assert!(is_standalone_json_message("{\"ok\":true}"));
    }

    #[test]
    fn markdown_wrapped_json_is_not_treated_as_raw_json() {
        assert!(!is_standalone_json_message("```json\n{\"ok\":true}\n```"));
    }

    #[test]
    fn normal_markdown_message_is_not_treated_as_raw_json() {
        assert!(!is_standalone_json_message(
            "Here is `code` and **markdown**."
        ));
    }

    #[test]
    fn standalone_json_is_pretty_printed() {
        let formatted = format_standalone_json_message("{\"z\":true,\"a\":[1,2]}")
            .expect("valid standalone JSON should be formatted");
        assert!(formatted.contains("\n  "));
    }

    #[test]
    fn escape_underscores_in_tool_names_basic() {
        let input = "mcp__bash__run";
        let escaped = escape_underscores_in_tool_names(input);
        assert_eq!(escaped, "mcp\\_\\_bash\\_\\_run");
    }

    #[test]
    fn escape_underscores_preserves_single_underscores() {
        let input = "some_variable_name";
        let escaped = escape_underscores_in_tool_names(input);
        assert_eq!(escaped, "some_variable_name");
    }
}
