use std::sync::mpsc::Receiver;

use eframe::egui;
use egui::{Button, CornerRadius, RichText, ScrollArea, Stroke, TextEdit};

use super::DialogView;
use crate::exit::Outcome;
use crate::source::SourceEvent;
use crate::theme::Tokens;

/// The text-info dialog: read-only text display with Copy / Close.
/// Content is either static or fed by a background source (stdin / --watch).
pub struct TextInfoView {
    content: String,
    source: Option<Receiver<SourceEvent>>,
    stream_active: bool,
    initial: Option<String>,
    stick_to_bottom: bool,
    wrap: bool,
    ok_label: String,
}

impl TextInfoView {
    pub fn new_static(content: String, wrap: bool, ok_label: String) -> Self {
        Self {
            content,
            source: None,
            stream_active: false,
            initial: None,
            stick_to_bottom: false,
            wrap,
            ok_label,
        }
    }

    /// `stream` = true for stdin (has an end; closing earlier means Cancel),
    /// false for --watch (endless polling; closing is a normal close).
    pub fn new_streaming(
        source: Receiver<SourceEvent>,
        stream: bool,
        initial: Option<String>,
        wrap: bool,
        ok_label: String,
    ) -> Self {
        Self {
            content: String::new(),
            source: Some(source),
            stream_active: stream,
            initial,
            stick_to_bottom: true,
            wrap,
            ok_label,
        }
    }

    fn drain_source(&mut self) {
        let Some(source) = &self.source else {
            return;
        };
        let mut disconnected = false;
        loop {
            match source.try_recv() {
                Ok(SourceEvent::Append(chunk)) => self.content.push_str(&chunk),
                Ok(SourceEvent::Replace(content)) => self.content = content,
                Ok(SourceEvent::Finished) => {
                    disconnected = true;
                    break;
                }
                Err(std::sync::mpsc::TryRecvError::Empty) => break,
                Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                    disconnected = true;
                    break;
                }
            }
        }
        if disconnected {
            self.source = None;
            self.stream_active = false;
        }
    }
}

impl DialogView for TextInfoView {
    fn ui(&mut self, ui: &mut egui::Ui, t: &Tokens) -> Option<Outcome> {
        self.drain_source();

        let frame = egui::Frame::new()
            .fill(t.input_bg)
            .stroke(Stroke::new(1.0, t.input_border_color))
            .corner_radius(CornerRadius::same(t.input_radius.clamp(0.0, 255.0) as u8))
            .inner_margin(12.0);

        let show_placeholder = self.content.is_empty() && self.initial.is_some();

        ui.allocate_ui(ui.available_size(), |ui| {
            frame.show(ui, |ui| {
                ui.set_min_size(ui.available_size());
                if show_placeholder {
                    ui.label(
                        RichText::new(self.initial.as_deref().unwrap_or_default()).color(t.muted),
                    );
                    return;
                }
                let scroll = if self.wrap {
                    ScrollArea::vertical()
                } else {
                    ScrollArea::both()
                };
                scroll
                    .auto_shrink(false)
                    .stick_to_bottom(self.stick_to_bottom)
                    .show(ui, |ui| {
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

        None
    }

    fn footer(&mut self, ui: &mut egui::Ui, t: &Tokens) -> Option<Outcome> {
        let mut result = None;
        let close = Button::new(RichText::new(&self.ok_label).color(t.button_primary_fg))
            .fill(t.button_primary_bg);
        if ui.add(close).clicked() {
            result = Some(if self.stream_active {
                Outcome::Cancel
            } else {
                Outcome::Ok
            });
        }
        if ui.button("Copy").clicked() {
            ui.ctx().copy_text(self.content.clone());
        }
        result
    }

    fn stream_active(&self) -> bool {
        self.stream_active
    }
}
