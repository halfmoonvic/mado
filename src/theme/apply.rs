use eframe::egui;
use egui::{
    Color32, CornerRadius, FontFamily, FontId, Stroke, TextStyle, Visuals, style::Selection,
};

use super::Tokens;

fn corner_radius(radius: f32) -> CornerRadius {
    CornerRadius::same(radius.clamp(0.0, 255.0) as u8)
}

/// Map resolved tokens onto the egui style.
pub fn apply(ctx: &egui::Context, t: &Tokens) {
    let mut style = egui::Style::default();

    let mut v = if t.dark {
        Visuals::dark()
    } else {
        Visuals::light()
    };

    v.override_text_color = Some(t.foreground);
    v.weak_text_color = Some(t.muted);

    v.window_fill = t.window_bg;
    v.panel_fill = t.window_bg;
    v.window_corner_radius = corner_radius(t.window_radius);
    v.window_stroke = Stroke::new(t.window_border_width, t.window_border_color);

    v.extreme_bg_color = t.input_bg;
    v.text_edit_bg_color = Some(t.input_bg);
    v.code_bg_color = t.mono_bg;
    v.warn_fg_color = t.warning;
    v.error_fg_color = t.danger;
    v.hyperlink_color = t.accent;
    v.selection = Selection {
        bg_fill: t.accent.gamma_multiply(0.35),
        stroke: Stroke::new(1.0, t.foreground),
    };

    // Widget states: secondary-button look is the baseline widget style;
    // primary buttons are filled explicitly at the call site.
    let radius = corner_radius(t.button_radius);
    let hover_tint = t.accent.gamma_multiply(0.08);

    v.widgets.noninteractive.bg_stroke = Stroke::new(1.0, t.input_border_color);
    v.widgets.noninteractive.fg_stroke = Stroke::new(1.0, t.foreground);
    v.widgets.noninteractive.corner_radius = corner_radius(t.input_radius);

    v.widgets.inactive.bg_fill = t.button_secondary_bg;
    v.widgets.inactive.weak_bg_fill = t.button_secondary_bg;
    v.widgets.inactive.bg_stroke = Stroke::new(1.0, t.button_secondary_border);
    v.widgets.inactive.fg_stroke = Stroke::new(1.0, t.button_secondary_fg);
    v.widgets.inactive.corner_radius = radius;

    v.widgets.hovered.bg_fill = hover_tint;
    v.widgets.hovered.weak_bg_fill = hover_tint;
    v.widgets.hovered.bg_stroke = Stroke::new(1.0, t.accent);
    v.widgets.hovered.fg_stroke = Stroke::new(1.0, t.foreground);
    v.widgets.hovered.corner_radius = radius;

    v.widgets.active.bg_fill = t.accent.gamma_multiply(0.18);
    v.widgets.active.weak_bg_fill = t.accent.gamma_multiply(0.18);
    v.widgets.active.bg_stroke = Stroke::new(1.0, t.accent);
    v.widgets.active.fg_stroke = Stroke::new(1.0, t.foreground);
    v.widgets.active.corner_radius = radius;

    v.widgets.open = v.widgets.active;

    style.visuals = v;

    style.spacing.button_padding = t.button_padding.into();
    style.spacing.item_spacing = egui::vec2(8.0, 8.0);

    style.text_styles = [
        (
            TextStyle::Heading,
            FontId::new(t.heading_size, FontFamily::Proportional),
        ),
        (
            TextStyle::Body,
            FontId::new(t.font_size, FontFamily::Proportional),
        ),
        (
            TextStyle::Button,
            FontId::new(t.font_size - 0.5, FontFamily::Proportional),
        ),
        (
            TextStyle::Small,
            FontId::new(t.font_size - 2.0, FontFamily::Proportional),
        ),
        (
            TextStyle::Monospace,
            FontId::new(t.font_size - 1.5, FontFamily::Monospace),
        ),
    ]
    .into();

    // mado resolves dark/light itself (tokens), so pin egui's theme and use
    // the same style for both slots.
    ctx.set_theme(if t.dark {
        egui::Theme::Dark
    } else {
        egui::Theme::Light
    });
    ctx.set_style_of(egui::Theme::Dark, style.clone());
    ctx.set_style_of(egui::Theme::Light, style);
}

/// Semi-transparent hover fill for the titlebar close button.
pub fn titlebar_hover_fill(t: &Tokens) -> Color32 {
    t.titlebar_fg.gamma_multiply(0.12)
}
