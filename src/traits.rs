//! Trait definitions for the chatbot UI.
//! The UI crate only knows these interfaces — all domain types live in the host app.
//! The host implements these traits for its concrete types and passes trait objects to the UI.

use chrono::{DateTime, Utc};
use egui::Vec2;
use std::collections::HashMap;
use std::sync::Arc;

/// A lightweight session entry for the sidebar list.
pub trait SessionListItem: Send + Sync {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn message_count(&self) -> usize;
    fn created_at(&self) -> DateTime<Utc>;
    fn updated_at(&self) -> DateTime<Utc>;
    fn hidden(&self) -> bool;
}

/// A single message in a chat session.
pub trait MessageView: Send + Sync {
    fn content(&self) -> &str;
    fn is_user(&self) -> bool;
    fn timestamp(&self) -> DateTime<Utc>;
    fn command(&self) -> Option<&str>;
}

/// The currently loaded session, providing access to messages and metadata.
pub trait SessionView: Send + Sync {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn messages(&self) -> Vec<Arc<dyn MessageView>>;
    fn system_prompt(&self) -> Option<&str>;
    fn json_summary(&self) -> Option<Arc<dyn JsonSummaryView>>;
}

/// Summary info shown in the JSON summary panel.
pub trait JsonSummaryView: Send + Sync {
    fn turn_count(&self) -> usize;
    fn total_tokens(&self) -> u64;
    fn estimated_cost(&self) -> Option<f64>;
    fn model(&self) -> &str;
    fn to_full_json(&self) -> String;
}

/// Available models for the model selector in the header.
pub trait ModelCatalog: Send + Sync {
    fn available_models(&self) -> Vec<(&'static str, &'static str)>;
    fn selected_model_id(&self) -> &str;
    fn select_model(&mut self, model_id: &str);
}

/// Basic provider information for display.
pub trait ProviderInfo: Send + Sync {
    fn display_name(&self) -> &str;
    fn provider_key(&self) -> &str;
}

/// Processing state for status indicators.
pub trait ProcessingState: Send + Sync {
    fn is_processing(&self) -> bool;
    fn is_local_processing(&self) -> bool;
}

/// Workspace directories available for selection.
pub trait WorkspaceList: Send + Sync {
    fn workspaces(&self) -> Vec<String>;
    fn selected(&self) -> Option<&str>;
    fn select(&mut self, dir: &str);
}

/// Runsc overrides for container execution.
pub trait RunscConfig: Send + Sync {
    fn runsc_image(&self) -> Option<&str>;
    fn runsc_workspace_includes(&self) -> Vec<String>;
    fn runsc_dirs(&self) -> Vec<String>;
}

/// Actions the UI can trigger on the host application.
pub trait UiActions: Send {
    fn submit_message(&mut self, text: String);
    fn stop_processing(&mut self);
    fn load_session(&mut self, id: &str);
    fn create_session(&mut self, name: String, system_prompt: Option<String>);
    fn delete_session(&mut self, id: &str);
    fn rename_session(&mut self, id: &str, name: String);
    fn select_workspace(&mut self, dir: &str);
    fn select_model(&mut self, model_id: &str);
    fn set_system_prompt(&mut self, prompt: Option<String>);
    fn toggle_system_prompt(&mut self);
    fn show_new_session_dialog(&mut self);
    fn show_rename_dialog(&mut self, session_id: String);
    fn hide_dialogs(&mut self);
    fn get_sessions(&self) -> Vec<Arc<dyn SessionListItem>>;
}

/// Height cache entry for virtualized scroll.
#[derive(Debug, Clone)]
pub struct HeightCacheEntry {
    pub row_heights: Vec<f32>,
    pub total_height: f32,
    pub message_count: usize,
    pub updated_at: DateTime<Utc>,
}

/// Persistent height cache per session.
pub trait HeightCacheStore: Send {
    fn save(&mut self, session_id: &str, widths: HashMap<u32, HeightCacheEntry>);
    fn load(&mut self, session_id: &str) -> Option<HashMap<u32, HeightCacheEntry>>;
}

/// Slash command for the input field.
#[derive(Debug, Clone)]
pub struct SlashCommand {
    pub name: String,
    pub description: String,
}

pub trait SlashCommandSource: Send + Sync {
    fn commands(&self) -> Vec<SlashCommand>;
}

/// Input history navigation.
pub trait InputHistory: Send {
    fn history(&self) -> Vec<String>;
    fn push(&mut self, text: String);
    fn clear(&mut self);
}

/// MCP server info for status display.
#[derive(Debug, Clone)]
pub struct McpServer {
    pub name: String,
    pub enabled: bool,
}

pub trait McpRegistry: Send + Sync {
    fn connected_servers(&self) -> Vec<McpServer>;
}

/// Cached rendered markdown.
#[derive(Debug, Clone)]
pub struct CachedMarkdown {
    pub html: String,
    pub render_time_ms: f32,
}
