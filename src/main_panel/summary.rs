//! JSON summary rendering.

use eframe::egui;

pub fn render_json_summary_area(ui: &mut egui::Ui, summary_json: &str) -> egui::Rect {
    let response = ui.group(|ui| {
        ui.label("Conversation Summary:");

        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(summary_json) {
            if let Some(obj) = parsed.as_object() {
                if let Some(response_count) = obj.get("response_count").and_then(|v| v.as_u64()) {
                    ui.label(format!("Responses: {}", response_count));
                }
                if let Some(model) = obj.get("model").and_then(|v| v.as_str()) {
                    ui.label(format!("Model: {}", model));
                }
                if let (Some(input), Some(output)) = (
                    obj.get("total_input_tokens").and_then(|v| v.as_u64()),
                    obj.get("total_output_tokens").and_then(|v| v.as_u64()),
                ) {
                    ui.horizontal(|ui| {
                        ui.label(format!("Input: {} tokens", input));
                        ui.label(format!("Output: {} tokens", output));
                    });
                }
                if let Some(cost) = obj.get("total_cost_usd").and_then(|v| v.as_f64()) {
                    if cost > 0.0 {
                        ui.label(format!("Total cost: ${:.4}", cost));
                    }
                }
                if let Some(turns) = obj.get("turn_history").and_then(|v| v.as_array()) {
                    ui.label(format!("Turn History ({} calls):", turns.len()));
                    ui.add_space(4.0);

                    use egui::text::LayoutJob;
                    let mono = egui::TextFormat {
                        font_id: egui::FontId::monospace(11.0),
                        ..Default::default()
                    };

                    egui::ScrollArea::vertical()
                        .max_height(300.0)
                        .stick_to_bottom(true)
                        .show(ui, |ui| {
                            for (i, turn) in turns.iter().enumerate() {
                                let line = format!("Turn {}: {}", i + 1, turn);
                                let mut job = LayoutJob::default();
                                job.append(&line, 0.0, mono.clone());
                                ui.label(job);
                            }
                        });
                }
            } else {
                ui.label(format!("Summary: {}", summary_json));
            }
        } else {
            ui.label("No conversation data available yet.");
        }
    });
    response.response.rect
}
