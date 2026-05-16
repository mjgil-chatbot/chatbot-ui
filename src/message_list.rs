//! Virtualized message list rendering and height cache management.

use std::sync::Arc;
use std::time::{Duration, Instant};
use chrono::Utc;
use eframe::egui;
const MESSAGE_OVERSCAN_PX: f32 = 200.0;
const DEFAULT_MESSAGE_ROW_HEIGHT: f32 = 140.0;
const MIN_MESSAGE_ROW_HEIGHT: f32 = 28.0;
const HEIGHT_UPDATE_EPSILON: f32 = 1.0;
pub struct MessageListState {
    pub row_heights: Vec<f32>,
    pub scroll_offset: f32,
    pub scroll_offset_session_id: Option<String>,
    pub scroll_offset_override: Option<f32>,
    pub height_cache_scan: bool,
}
impl MessageListState {
    pub fn new() -> Self {
        Self {
            row_heights: Vec::new(),
            scroll_offset: 0.0,
            scroll_offset_session_id: None,
            scroll_offset_override: None,
            height_cache_scan: false,
        }
    }}
impl Default for MessageListState {
    fn default() -> Self {
        Self::new()
    }
}
pub fn render_empty_message_area(ui: &mut egui::Ui, session_id: &str) {
    let available_height = ui.available_height() - 80.0;
    let available_width = ui.available_width();
    egui::ScrollArea::vertical()
        .max_height(available_height)
        .max_width(available_width)
        .auto_shrink(egui::Vec2b::new(false, true))
        .stick_to_bottom(true)
        .id_salt(session_id)
        .horizontal_scroll_offset(0.0)
        .show_viewport(ui, |ui, _viewport| {
            ui.set_width(available_width);
        });
}
fn sync_row_heights(
    state: &mut MessageListState,
    session_id: &str,
    message_count: usize,
    available_width: f32,
) {
    if state.scroll_offset_session_id.as_deref() != Some(session_id) {
        state.row_heights.clear();
        state
            .row_heights
            .resize(message_count, DEFAULT_MESSAGE_ROW_HEIGHT);
        state.scroll_offset_session_id = Some(session_id.to_string());
    } else if state.row_heights.len() != message_count {
        let default_height = if state.row_heights.is_empty() {
            DEFAULT_MESSAGE_ROW_HEIGHT
        } else {
            state.row_heights.iter().sum::<f32>() / state.row_heights.len() as f32
        };
        state.row_heights.resize(message_count, default_height);
    }
}
fn build_row_prefix_sums(row_heights: &[f32]) -> Vec<f32> {
    let mut prefix = Vec::with_capacity(row_heights.len() + 1);
    prefix.push(0.0);
    for height in row_heights {
        let clamped = height.max(MIN_MESSAGE_ROW_HEIGHT);
        let next = prefix.last().copied().unwrap_or(0.0) + clamped;
        prefix.push(next);
    }
    prefix
}
fn viewport_message_range(
    row_prefix: &[f32],
    viewport: egui::Rect,
    message_count: usize,
) -> (usize, usize) {
    if message_count == 0 {
        return (0, 0);
    }

    let start_y = (viewport.min.y - MESSAGE_OVERSCAN_PX).max(0.0);
    let end_y = viewport.max.y + MESSAGE_OVERSCAN_PX;

    let mut start_idx = lower_bound(row_prefix, start_y).saturating_sub(1);
    if start_idx >= message_count {
        start_idx = message_count - 1;
    }

    let mut end_idx = lower_bound(row_prefix, end_y).min(message_count);
    if end_idx <= start_idx {
        end_idx = (start_idx + 1).min(message_count);
    }

    (start_idx, end_idx)
}
fn lower_bound(values: &[f32], target: f32) -> usize {
    let mut left = 0;
    let mut right = values.len();
    while left < right {
        let mid = (left + right) / 2;
        if values[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left
}
