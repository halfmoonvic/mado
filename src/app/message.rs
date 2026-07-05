use eframe::egui;
use egui::{Align2, Button, Color32, FontId, RichText, Sense, Stroke, vec2};

use super::DialogView;
use crate::cli::DialogKind;
use crate::exit::Outcome;
use crate::theme::Tokens;

/// Message dialogs: --info / --warning / --error.
pub struct MessageView {
    kind: DialogKind,
    text: String,
    ok_label: String,
}

impl MessageView {
    pub fn new(kind: DialogKind, text: String, ok_label: String) -> Self {
        Self {
            kind,
            text,
            ok_label,
        }
    }

    fn icon(&self, t: &Tokens) -> (Option<&'static str>, Color32) {
        match self.kind {
            DialogKind::Warning => (Some("!"), t.warning),
            // The cross is painted as a vector shape (glyph coverage varies).
            DialogKind::Error => (None, t.danger),
            _ => (Some("i"), t.info),
        }
    }
}

impl DialogView for MessageView {
    fn ui(&mut self, ui: &mut egui::Ui, t: &Tokens) -> Option<Outcome> {
        ui.allocate_ui(ui.available_size(), |ui| {
            ui.horizontal_top(|ui| {
                // Outlined circle icon.
                let (glyph, color) = self.icon(t);
                let (rect, _) = ui.allocate_exact_size(vec2(22.0, 22.0), Sense::hover());
                ui.painter()
                    .circle_stroke(rect.center(), 10.0, Stroke::new(1.5, color));
                if let Some(glyph) = glyph {
                    ui.painter().text(
                        rect.center(),
                        Align2::CENTER_CENTER,
                        glyph,
                        FontId::proportional(13.0),
                        color,
                    );
                } else {
                    super::paint_cross(ui.painter(), rect.center(), 3.5, Stroke::new(1.5, color));
                }

                ui.add_space(4.0);
                ui.vertical(|ui| {
                    egui::ScrollArea::vertical()
                        .auto_shrink(false)
                        .show(ui, |ui| {
                            ui.label(RichText::new(&self.text).size(t.font_size + 0.5));
                        });
                });
            });
        });

        None
    }

    fn footer(&mut self, ui: &mut egui::Ui, t: &Tokens) -> Option<Outcome> {
        let ok = Button::new(RichText::new(&self.ok_label).color(t.button_primary_fg))
            .fill(t.button_primary_bg);
        if ui.add(ok).clicked() {
            return Some(Outcome::Ok);
        }
        None
    }
}
