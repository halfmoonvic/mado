use eframe::egui;
use egui::{Button, CornerRadius, RichText, ScrollArea, Stroke, TextEdit};

use super::DialogView;
use crate::exit::Outcome;
use crate::theme::Tokens;

/// The text-info dialog: read-only text display with Copy / Close.
pub struct TextInfoView {
    content: String,
    wrap: bool,
    ok_label: String,
}

impl TextInfoView {
    pub fn new(content: String, wrap: bool, ok_label: String) -> Self {
        Self {
            content,
            wrap,
            ok_label,
        }
    }
}

impl DialogView for TextInfoView {
    fn ui(&mut self, ui: &mut egui::Ui, t: &Tokens) -> Option<Outcome> {
        let mut result = None;

        let footer_height = 44.0;
        let text_area_height = (ui.available_height() - footer_height).max(0.0);

        let frame = egui::Frame::new()
            .fill(t.input_bg)
            .stroke(Stroke::new(1.0, t.input_border_color))
            .corner_radius(CornerRadius::same(t.input_radius.clamp(0.0, 255.0) as u8))
            .inner_margin(12.0);

        ui.allocate_ui(egui::vec2(ui.available_width(), text_area_height), |ui| {
            frame.show(ui, |ui| {
                ui.set_min_size(ui.available_size());
                let scroll = if self.wrap {
                    ScrollArea::vertical()
                } else {
                    ScrollArea::both()
                };
                scroll.auto_shrink(false).show(ui, |ui| {
                    // Read-only TextEdit keeps the text selectable.
                    let mut text = self.content.as_str();
                    let edit = TextEdit::multiline(&mut text)
                        .frame(egui::Frame::NONE)
                        .desired_width(if self.wrap {
                            ui.available_width()
                        } else {
                            f32::INFINITY
                        });
                    ui.add(edit);
                });
            });
        });

        ui.add_space(8.0);
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let close = Button::new(RichText::new(&self.ok_label).color(t.button_primary_fg))
                .fill(t.button_primary_bg);
            if ui.add(close).clicked() {
                result = Some(Outcome::Ok);
            }
            if ui.button("Copy").clicked() {
                ui.ctx().copy_text(self.content.clone());
            }
        });

        result
    }
}
