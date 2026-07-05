//! Built-in default tokens: the "1b compact tool" design direction on the
//! zenwritten_light / zenwritten_dark palettes.

use eframe::egui::Color32;

use super::Tokens;

// zenwritten_light
const ACCENT: Color32 = Color32::from_rgb(0xA8, 0x33, 0x4C); // rose
const FG: Color32 = Color32::from_rgb(0x35, 0x35, 0x35);
const BG: Color32 = Color32::from_rgb(0xEE, 0xEE, 0xEE);
const BAR: Color32 = Color32::from_rgb(0xE4, 0xE4, 0xE4); // sunken title/footer bars
const BORDER: Color32 = Color32::from_rgb(0xB4, 0xB4, 0xB4);

pub fn light() -> Tokens {
    Tokens {
        dark: false,

        window_bg: BG,
        window_radius: 8.0,
        window_border_color: BORDER,
        window_border_width: 1.0,
        window_padding: 14.0,

        titlebar_show: true,
        titlebar_bg: BAR,
        titlebar_fg: FG,
        titlebar_height: 36.0,
        titlebar_icon: String::new(),
        titlebar_separator_color: BORDER,
        titlebar_separator_width: 1.0,

        footer_bg: BAR,
        footer_separator_color: BORDER,
        footer_separator_width: 1.0,

        foreground: FG,
        muted: Color32::from_rgb(0x63, 0x63, 0x63),
        accent: ACCENT,
        info: Color32::from_rgb(0x28, 0x64, 0x86), // water
        warning: Color32::from_rgb(0x94, 0x49, 0x27), // wood
        danger: ACCENT,

        font_family: None,
        font_size: 13.0,
        heading_size: 15.0,

        banner_warning_bg: Color32::from_rgba_unmultiplied(0x35, 0x35, 0x35, 18), // rgba(53,53,53,0.07)
        banner_warning_fg: Color32::from_rgb(0x53, 0x53, 0x53),

        input_bg: Color32::from_rgb(0xE7, 0xE7, 0xE7),
        input_border_color: BORDER,
        input_radius: 4.0,
        input_focus_border: Color32::from_rgb(0x28, 0x64, 0x86),

        mono_bg: Color32::from_rgb(0x19, 0x19, 0x19),
        mono_fg: Color32::from_rgb(0xB4, 0xB4, 0xB4),

        button_radius: 5.0,
        button_padding: [12.0, 5.0],
        button_primary_bg: ACCENT,
        button_primary_fg: BG,
        button_secondary_bg: BG,
        button_secondary_border: BORDER,
        button_secondary_fg: FG,
    }
}

// zenwritten_dark
pub fn dark() -> Tokens {
    let accent = Color32::from_rgb(0xDE, 0x6E, 0x7C); // rose
    let fg = Color32::from_rgb(0xBB, 0xBB, 0xBB);
    let bg = Color32::from_rgb(0x19, 0x19, 0x19);
    let bar = Color32::from_rgb(0x15, 0x15, 0x15);
    // Borders are translucent foreground: rgba(187,187,187,α)
    let border = |alpha: u8| Color32::from_rgba_unmultiplied(0xBB, 0xBB, 0xBB, alpha);

    Tokens {
        dark: true,

        window_bg: bg,
        window_border_color: border(41), // 0.16

        titlebar_bg: bar,
        titlebar_fg: fg,
        titlebar_separator_color: border(31), // 0.12

        footer_bg: bar,
        footer_separator_color: border(31),

        foreground: fg,
        muted: Color32::from_rgb(0x8C, 0x8C, 0x8C),
        accent,
        info: Color32::from_rgb(0x60, 0x99, 0xC0), // water
        warning: Color32::from_rgb(0xB7, 0x7E, 0x64), // wood
        danger: accent,

        banner_warning_bg: border(23), // rgba(187,187,187,0.09)
        banner_warning_fg: Color32::from_rgb(0xA6, 0xA6, 0xA6),

        input_bg: Color32::from_rgb(0x14, 0x14, 0x14),
        input_border_color: border(38), // 0.15
        input_focus_border: Color32::from_rgb(0x60, 0x99, 0xC0),

        mono_fg: fg,

        button_primary_bg: accent,
        button_primary_fg: bg,
        button_secondary_bg: Color32::TRANSPARENT,
        button_secondary_border: border(64), // 0.25
        button_secondary_fg: fg,

        ..light()
    }
}
