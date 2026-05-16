//! Message rendering UI components
//!
//! Renders individual chat messages with markdown formatting and copy functionality.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use chrono::{DateTime, Utc};
use eframe::egui;
use egui_extras::syntax_highlighting::{self, CodeTheme};
pub struct CopyFeedback {
    pub feedback: HashMap<i64, Instant>,
}
impl CopyFeedback {
    pub fn new() -> Self {
        Self {
            feedback: HashMap::new(),
        }
    }}
impl CopyFeedback {
    pub fn get_and_cleanup(&mut self, msg_id: i64) -> bool {
        match self.feedback.get(&msg_id) {
            Some(t) if t.elapsed().as_secs_f32() < 2.0 => true,
            Some(_) => {
                self.feedback.remove(&msg_id);
                false
            }
            None => false,
        }
    }}
impl CopyFeedback {
    pub fn set(&mut self, msg_id: i64) {
        self.feedback.insert(msg_id, Instant::now());
    }}
impl Default for CopyFeedback {
    fn default() -> Self {
        Self::new()
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
