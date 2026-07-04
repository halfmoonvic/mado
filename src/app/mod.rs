mod message;
mod text_info;

use std::sync::Arc;
use std::sync::atomic::{AtomicI32, Ordering};

use eframe::egui;
use egui::{
    Align2, CornerRadius, CursorIcon, FontFamily, FontId, Id, PointerButton, Rect, ResizeDirection,
    Sense, Stroke, UiBuilder, ViewportCommand, vec2,
};

use crate::cli::{Cli, DialogKind};
use crate::exit::Outcome;
use crate::theme::{self, Tokens};

/// A dialog body rendered inside the shared window shell.
pub trait DialogView {
    /// Render the dialog content; return `Some` to close the window.
    fn ui(&mut self, ui: &mut egui::Ui, tokens: &Tokens) -> Option<Outcome>;

    /// Whether a data stream is still running (closing then means Cancel).
    fn stream_active(&self) -> bool {
        false
    }
}

/// Static content resolved before the window opens, so that an unreadable
/// --filename fails fast on stderr instead of flashing a window.
fn prefetch_content(cli: &Cli) -> Result<Option<String>, String> {
    if cli.dialog_kind() == DialogKind::TextInfo
        && let Some(path) = &cli.filename
    {
        return std::fs::read_to_string(path)
            .map(Some)
            .map_err(|err| format!("cannot read {}: {err}", path.display()));
    }
    Ok(cli.text.clone())
}

/// Build the dialog view for the parsed command line. Needs the egui context
/// so streaming sources can wake the UI.
fn build_view(cli: &Cli, prefetched: Option<String>, ctx: &egui::Context) -> Box<dyn DialogView> {
    match cli.dialog_kind() {
        DialogKind::TextInfo => {
            let wrap = !cli.no_wrap;
            let ok_label = cli.ok_label.clone().unwrap_or_else(|| "Close".to_owned());
            if let Some(content) = prefetched {
                Box::new(text_info::TextInfoView::new_static(content, wrap, ok_label))
            } else if let Some(path) = &cli.watch {
                let interval = std::time::Duration::from_millis(cli.poll_interval.max(16));
                let source = crate::source::spawn_watch(ctx.clone(), path.clone(), interval);
                Box::new(text_info::TextInfoView::new_streaming(
                    source,
                    false,
                    cli.initial.clone(),
                    wrap,
                    ok_label,
                ))
            } else {
                let source = crate::source::spawn_stdin(ctx.clone());
                Box::new(text_info::TextInfoView::new_streaming(
                    source,
                    true,
                    cli.initial.clone(),
                    wrap,
                    ok_label,
                ))
            }
        }
        kind @ (DialogKind::Info | DialogKind::Warning | DialogKind::Error) => {
            let ok_label = cli.ok_label.clone().unwrap_or_else(|| "OK".to_owned());
            Box::new(message::MessageView::new(
                kind,
                prefetched.unwrap_or_default(),
                ok_label,
            ))
        }
    }
}

/// Shared window shell: custom frame, titlebar, outcome and exit handling.
pub struct Shell {
    outcome: Arc<AtomicI32>,
    title: String,
    tokens: Tokens,
    view: Box<dyn DialogView>,
    deadline: Option<std::time::Instant>,
}

impl Shell {
    fn finish(&self, ctx: &egui::Context, outcome: Outcome) {
        self.outcome.store(outcome.code(), Ordering::SeqCst);
        ctx.send_viewport_cmd(ViewportCommand::Close);
    }

    fn close_outcome(&self) -> Outcome {
        if self.view.stream_active() {
            Outcome::Cancel
        } else {
            Outcome::Ok
        }
    }

    fn titlebar(&self, ui: &mut egui::Ui, rect: Rect) -> Option<Outcome> {
        let t = &self.tokens;
        let mut result = None;

        let top_radius = CornerRadius {
            nw: t.window_radius.clamp(0.0, 255.0) as u8,
            ne: t.window_radius.clamp(0.0, 255.0) as u8,
            sw: 0,
            se: 0,
        };
        ui.painter().rect_filled(rect, top_radius, t.titlebar_bg);
        if t.titlebar_separator_width > 0.0 {
            ui.painter().line_segment(
                [rect.left_bottom(), rect.right_bottom()],
                Stroke::new(t.titlebar_separator_width, t.titlebar_separator_color),
            );
        }

        // Spark icon + title.
        let mut text_x = rect.left() + 16.0;
        if !t.titlebar_icon.is_empty() {
            let icon_rect = ui.painter().text(
                egui::pos2(text_x, rect.center().y),
                Align2::LEFT_CENTER,
                &t.titlebar_icon,
                FontId::new(15.0, FontFamily::Proportional),
                t.accent,
            );
            text_x = icon_rect.right() + 9.0;
        }
        ui.painter().text(
            egui::pos2(text_x, rect.center().y),
            Align2::LEFT_CENTER,
            &self.title,
            FontId::new(16.0, FontFamily::Name(theme::HEADING_FAMILY.into())),
            t.titlebar_fg,
        );

        // Drag to move, double-click to maximize.
        let response = ui.interact(rect, Id::new("mado_titlebar"), Sense::click_and_drag());
        if response.double_clicked() {
            let maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));
            ui.ctx()
                .send_viewport_cmd(ViewportCommand::Maximized(!maximized));
        }
        if response.drag_started_by(PointerButton::Primary) {
            ui.ctx().send_viewport_cmd(ViewportCommand::StartDrag);
        }

        // Close button.
        let button_size = 26.0;
        let button_rect = Rect::from_center_size(
            egui::pos2(rect.right() - 12.0 - button_size / 2.0, rect.center().y),
            vec2(button_size, button_size),
        );
        let close = ui.interact(button_rect, Id::new("mado_close"), Sense::click());
        if close.hovered() {
            ui.painter().rect_filled(
                button_rect,
                CornerRadius::same(7),
                theme::titlebar_hover_fill(t),
            );
        }
        ui.painter().text(
            button_rect.center(),
            Align2::CENTER_CENTER,
            "✕",
            FontId::new(13.0, FontFamily::Proportional),
            t.titlebar_fg
                .gamma_multiply(if close.hovered() { 1.0 } else { 0.55 }),
        );
        if close.clicked() {
            result = Some(self.close_outcome());
        }

        result
    }

    /// Borderless windows have no native resize border; emulate one with
    /// grip zones along the window edges.
    fn resize_grips(&self, ui: &egui::Ui, rect: Rect) {
        const GRIP: f32 = 6.0;
        let maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));
        if maximized {
            return;
        }

        use ResizeDirection::*;
        let corner = GRIP * 2.0;
        let zones: [(ResizeDirection, Rect, CursorIcon); 8] = [
            (
                NorthWest,
                Rect::from_min_size(rect.min, vec2(corner, corner)),
                CursorIcon::ResizeNwSe,
            ),
            (
                NorthEast,
                Rect::from_min_size(rect.right_top() - vec2(corner, 0.0), vec2(corner, corner)),
                CursorIcon::ResizeNeSw,
            ),
            (
                SouthWest,
                Rect::from_min_size(rect.left_bottom() - vec2(0.0, corner), vec2(corner, corner)),
                CursorIcon::ResizeNeSw,
            ),
            (
                SouthEast,
                Rect::from_min_size(rect.max - vec2(corner, corner), vec2(corner, corner)),
                CursorIcon::ResizeNwSe,
            ),
            (
                North,
                Rect::from_min_max(rect.min, egui::pos2(rect.right(), rect.top() + GRIP)),
                CursorIcon::ResizeVertical,
            ),
            (
                South,
                Rect::from_min_max(egui::pos2(rect.left(), rect.bottom() - GRIP), rect.max),
                CursorIcon::ResizeVertical,
            ),
            (
                West,
                Rect::from_min_max(rect.min, egui::pos2(rect.left() + GRIP, rect.bottom())),
                CursorIcon::ResizeHorizontal,
            ),
            (
                East,
                Rect::from_min_max(egui::pos2(rect.right() - GRIP, rect.top()), rect.max),
                CursorIcon::ResizeHorizontal,
            ),
        ];

        // Corners first so they win over the edges.
        for (direction, zone, cursor) in zones {
            let id = Id::new("mado_resize").with(direction as u8);
            let response = ui.interact(zone, id, Sense::drag());
            if response.hovered() {
                ui.ctx().set_cursor_icon(cursor);
            }
            if response.drag_started_by(PointerButton::Primary) {
                ui.ctx()
                    .send_viewport_cmd(ViewportCommand::BeginResize(direction));
            }
            if response.hovered() || response.dragged() {
                break;
            }
        }
    }
}

impl eframe::App for Shell {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();

        if let Some(deadline) = self.deadline {
            let now = std::time::Instant::now();
            if now >= deadline {
                self.finish(&ctx, Outcome::Timeout);
                return;
            }
            ctx.request_repaint_after(deadline - now);
        }

        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.finish(&ctx, Outcome::Cancel);
            return;
        }
        // External close (Alt+F4, taskbar): honor the stream state.
        if ctx.input(|i| i.viewport().close_requested()) {
            self.outcome
                .store(self.close_outcome().code(), Ordering::SeqCst);
        }

        let t = self.tokens.clone();
        let frame = egui::Frame::new()
            .fill(t.window_bg)
            .corner_radius(CornerRadius::same(t.window_radius.clamp(0.0, 255.0) as u8))
            .stroke(Stroke::new(t.window_border_width, t.window_border_color))
            .outer_margin(1.0);

        let mut finish: Option<Outcome> = None;

        frame.show(ui, |ui| {
            let app_rect = ui.max_rect();
            ui.expand_to_include_rect(app_rect);

            let titlebar_height = if t.titlebar_show {
                t.titlebar_height
            } else {
                0.0
            };
            if t.titlebar_show {
                let titlebar_rect = Rect::from_min_max(
                    app_rect.min,
                    egui::pos2(app_rect.right(), app_rect.top() + titlebar_height),
                );
                if let Some(outcome) = self.titlebar(ui, titlebar_rect) {
                    finish = Some(outcome);
                }
            }

            let content_rect = Rect::from_min_max(
                egui::pos2(app_rect.left(), app_rect.top() + titlebar_height),
                app_rect.max,
            )
            .shrink(t.window_padding);
            let mut content_ui = ui.new_child(UiBuilder::new().max_rect(content_rect));
            if let Some(outcome) = self.view.ui(&mut content_ui, &t) {
                finish = Some(outcome);
            }

            self.resize_grips(ui, app_rect);
        });

        if let Some(outcome) = finish {
            self.finish(&ctx, outcome);
        }
    }
}

/// Run the dialog window described by `cli` and return its exit code.
pub fn run(cli: Cli) -> i32 {
    let title = cli.title.clone().unwrap_or_else(|| "mado".to_owned());
    let (default_width, default_height) = match cli.dialog_kind() {
        DialogKind::TextInfo => (650.0, 360.0),
        DialogKind::Info | DialogKind::Warning | DialogKind::Error => (380.0, 150.0),
    };
    let width = cli.width.unwrap_or(default_width);
    let height = cli.height.unwrap_or(default_height);

    let mut viewport = egui::ViewportBuilder::default()
        .with_title(&title)
        .with_inner_size([width, height])
        .with_min_inner_size([240.0, 100.0])
        .with_decorations(false)
        .with_transparent(true);
    if cli.always_on_top {
        viewport = viewport.with_always_on_top();
    }

    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    let prefetched = match prefetch_content(&cli) {
        Ok(prefetched) => prefetched,
        Err(message) => {
            eprintln!("mado: {message}");
            return 255;
        }
    };

    // Closing the window without cancelling counts as a normal close (0).
    let outcome = Arc::new(AtomicI32::new(Outcome::Ok.code()));
    let app_outcome = outcome.clone();

    let app_name = title.clone();
    let result = eframe::run_native(
        &app_name,
        options,
        Box::new(move |cc| {
            let dark = cc
                .egui_ctx
                .system_theme()
                .map(|theme| theme == egui::Theme::Dark)
                .unwrap_or(false);
            let tokens =
                theme::resolve(dark, cli.theme.as_ref(), cli.font.as_deref(), cli.font_size);
            theme::install_fonts(&cc.egui_ctx, tokens.font_family.as_deref());
            theme::apply(&cc.egui_ctx, &tokens);
            let view = build_view(&cli, prefetched, &cc.egui_ctx);
            let deadline = cli
                .timeout
                .map(|secs| std::time::Instant::now() + std::time::Duration::from_secs(secs));
            Ok(Box::new(Shell {
                outcome: app_outcome,
                title,
                tokens,
                view,
                deadline,
            }))
        }),
    );

    if result.is_err() {
        return 255;
    }
    outcome.load(Ordering::SeqCst)
}
