//! Side panel UI for session management
//!
//! Renders the session list and provides controls for creating,
//! selecting, and deleting chat sessions.

use eframe::egui;
use std::sync::Arc;
pub struct SidePanelState {
    pub search_filter: String,
}
