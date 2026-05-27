//! Virtualized message list rendering and height cache management.

use std::sync::Arc;
use std::time::{Duration, Instant};

use chrono::Utc;
use eframe::egui;
use tmc::egui::MarkdownCache;

use crate::message::{CopyFeedback, render_message, render_processing_indicator};
use crate::traits::{HeightCacheEntry, MessageView, ProcessingState, SessionView};

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
    }
}

impl Default for MessageListState {
    fn default() -> Self {
        Self::new()
    }
}

/// Render the message list area
pub fn render_message_area(
    ui: &mut egui::Ui,
    session: &Arc<dyn SessionView>,
    processing: &dyn ProcessingState,
    markdown_cache: &mut MarkdownCache,
    copy_feedback: &mut CopyFeedback,
    state: &mut MessageListState,
) {
    let available_height = ui.available_height() - 80.0;
    let available_width = ui.available_width();
    let message_count = session.messages().len();
    let is_processing = processing.is_processing();

    sync_row_heights(state, session.id(), message_count, available_width);

    let mut rendered_rows: Vec<Option<egui::Rect>> = vec![None; message_count];

    let mut scroll_area = egui::ScrollArea::vertical()
        .max_height(available_height)
        .max_width(available_width)
        .auto_shrink(egui::Vec2b::new(false, true))
        .stick_to_bottom(true)
        .id_salt(session.id())
        .horizontal_scroll_offset(0.0);

    if state.scroll_offset_session_id.as_deref() == Some(session.id()) {
        if let Some(offset) = state.scroll_offset_override.take() {
            scroll_area = scroll_area.vertical_scroll_offset(offset);
        }
    } else {
        state.scroll_offset_session_id = Some(session.id().to_string());
    }

    scroll_area.show_viewport(ui, |ui, viewport| {
        ui.set_width(available_width);

        let messages = session.messages();
        if messages.is_empty() {
            if is_processing {
                render_processing_indicator(ui);
            }
            return;
        }

        let row_prefix = build_row_prefix_sums(&state.row_heights);
        let total_height = *row_prefix.last().unwrap_or(&0.0);
        ui.set_height(total_height);

        let (start_idx, end_idx) = viewport_message_range(&row_prefix, viewport, messages.len());

        let top_spacer = row_prefix[start_idx];
        if top_spacer > 0.0 {
            ui.add_space(top_spacer);
        }

        for idx in start_idx..end_idx {
            let row_start = ui.cursor().top();
            let msg = &messages[idx];

            let row_min = ui.cursor().left_top();
            let row_rect = egui::Rect::from_min_max(
                row_min,
                egui::pos2(row_min.x + available_width, ui.clip_rect().max.y),
            );
            let mut row_ui = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(row_rect)
                    .layout(*ui.layout()),
            );
            row_ui.set_clip_rect(ui.clip_rect().intersect(row_rect));
            row_ui.set_width(available_width);
            row_ui.set_min_width(available_width);
            row_ui.set_max_width(available_width);

            let message_rect = render_message(&mut row_ui, msg, markdown_cache, copy_feedback);

            let row_used = row_ui.min_rect();
            if row_used.is_positive() {
                let mut size = row_used.size();
                size.x = size.x.min(available_width);
                ui.advance_cursor_after_rect(egui::Rect::from_min_size(row_rect.min, size));
            }

            rendered_rows[idx] = Some(message_rect);
            ui.add_space(8.0);

            let measured_height = (ui.cursor().top() - row_start).max(MIN_MESSAGE_ROW_HEIGHT);
            let prev_height = state.row_heights[idx];
            if (prev_height - measured_height).abs() > HEIGHT_UPDATE_EPSILON {
                state.row_heights[idx] = measured_height;
            }
        }

        let bottom_spacer = (total_height - row_prefix[end_idx]).max(0.0);
        if bottom_spacer > 0.0 {
            ui.add_space(bottom_spacer);
        }

        if is_processing {
            render_processing_indicator(ui);
        }
    });
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
