//! Built-in default tokens, extracted from the "1c ink header" design
//! direction (ui concept, light + dark).

use eframe::egui::Color32;

use super::Tokens;

const ACCENT: Color32 = Color32::from_rgb(0xE8, 0x6A, 0x33);
const INK: Color32 = Color32::from_rgb(0x1C, 0x20, 0x33);
const PAPER: Color32 = Color32::from_rgb(0xFA, 0xF6, 0xEF);

pub fn light() -> Tokens {
    Tokens {
        dark: false,

        window_bg: PAPER,
        window_radius: 14.0,
        window_border_color: Color32::from_rgb(0xE8, 0xDF, 0xCE),
        window_border_width: 1.0,
        window_padding: 20.0,

        titlebar_show: true,
        titlebar_bg: INK,
        titlebar_fg: PAPER,
        titlebar_height: 44.0,
        titlebar_icon: "✦".to_owned(),
        titlebar_separator_color: Color32::TRANSPARENT,
        titlebar_separator_width: 0.0,

        footer_bg: PAPER,
        footer_separator_color: Color32::TRANSPARENT,
        footer_separator_width: 0.0,

        foreground: INK,
        muted: Color32::from_rgb(0x5A, 0x64, 0x78),
        accent: ACCENT,
        warning: Color32::from_rgb(0xC8, 0x92, 0x33),
        danger: Color32::from_rgb(0xB0, 0x4A, 0x3A),

        font_family: None,
        font_size: 14.0,
        heading_size: 20.0,

        banner_warning_bg: Color32::from_rgb(0xF5, 0xE6, 0xC6),
        banner_warning_fg: Color32::from_rgb(0x6E, 0x50, 0x11),

        input_bg: Color32::from_rgb(0xF3, 0xED, 0xE1),
        input_border_color: Color32::from_rgb(0xE8, 0xDF, 0xCE),
        input_radius: 8.0,
        input_focus_border: ACCENT,

        mono_bg: INK,
        mono_fg: Color32::from_rgb(0xC6, 0xCC, 0xDB),

        button_radius: 999.0,
        button_padding: [18.0, 8.0],
        button_primary_bg: ACCENT,
        button_primary_fg: PAPER,
        button_secondary_bg: Color32::TRANSPARENT,
        button_secondary_border: Color32::from_rgb(0xD8, 0xCC, 0xB4),
        button_secondary_fg: INK,
    }
}

pub fn dark() -> Tokens {
    let border = Color32::from_rgba_unmultiplied(0xFA, 0xF6, 0xEF, 31); // rgba(250,246,239,0.12)
    Tokens {
        dark: true,

        window_bg: Color32::from_rgb(0x1F, 0x23, 0x37),
        window_border_color: border,

        titlebar_bg: Color32::from_rgb(0x16, 0x1A, 0x2B),

        footer_bg: Color32::from_rgb(0x1F, 0x23, 0x37),

        foreground: PAPER,
        muted: Color32::from_rgb(0x8B, 0x94, 0xA6),

        banner_warning_bg: Color32::from_rgba_unmultiplied(0xC8, 0x92, 0x33, 36), // rgba(200,146,51,0.14)
        banner_warning_fg: Color32::from_rgb(0xE3, 0xC2, 0x7E),

        input_bg: Color32::from_rgb(0x16, 0x1A, 0x2B),
        input_border_color: border,

        mono_bg: Color32::from_rgb(0x16, 0x1A, 0x2B),

        warning: Color32::from_rgb(0xE3, 0xC2, 0x7E),

        button_secondary_border: Color32::from_rgba_unmultiplied(0xFA, 0xF6, 0xEF, 64),
        button_secondary_fg: PAPER,

        ..light()
    }
}
