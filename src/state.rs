//! UI-only ephemeral state. No references to providers, sessions, or domain types.

use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

use crate::traits::{CachedMarkdown, SlashCommand};

/// Ephemeral UI state — input text, scroll, caches, feedback.
pub struct UiState {
    pub input_text: String,
    pub scroll_offset: f32,
    pub markdown_cache: HashMap<String, Arc<CachedMarkdown>>,
    pub copy_feedback: Option<(String, Instant)>,
    pub search_query: String,
    pub show_system_prompt: bool,
    pub current_json_summary_full: Option<String>,
    pub show_json_summary: bool,
    pub height_cache_scan: bool,
    pub slash_commands: Vec<SlashCommand>,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            input_text: String::new(),
            scroll_offset: 0.0,
            markdown_cache: HashMap::new(),
            copy_feedback: None,
            search_query: String::new(),
            show_system_prompt: false,
            current_json_summary_full: None,
            show_json_summary: false,
            height_cache_scan: false,
            slash_commands: Vec::new(),
        }
    }
}

impl UiState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn touch_copy_feedback(&mut self, msg_id: String) {
        self.copy_feedback = Some((msg_id, Instant::now()));
    }

    pub fn clear_copy_feedback(&mut self) {
        self.copy_feedback = None;
    }

    pub fn cache_markdown(&mut self, key: String, html: CachedMarkdown) {
        self.markdown_cache.insert(key, Arc::new(html));
    }

    pub fn get_cached_markdown(&self, key: &str) -> Option<Arc<CachedMarkdown>> {
        self.markdown_cache.get(key).cloned()
    }
}
