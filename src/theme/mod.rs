mod apply;
mod defaults;
mod fonts;

use std::collections::BTreeMap;
use std::path::PathBuf;

use eframe::egui::Color32;
use serde::Deserialize;

pub use apply::{apply, titlebar_hover_fill};
pub use fonts::{HEADING_FAMILY, install_fonts};

/// Fully resolved design tokens consumed by the renderer.
#[derive(Debug, Clone)]
pub struct Tokens {
    pub dark: bool,

    pub window_bg: Color32,
    pub window_radius: f32,
    pub window_border_color: Color32,
    pub window_border_width: f32,
    pub window_padding: f32,

    pub titlebar_show: bool,
    pub titlebar_bg: Color32,
    pub titlebar_fg: Color32,
    pub titlebar_height: f32,
    pub titlebar_icon: String,
    pub titlebar_separator_color: Color32,
    pub titlebar_separator_width: f32,

    pub foreground: Color32,
    pub muted: Color32,
    pub accent: Color32,
    pub warning: Color32,
    pub danger: Color32,

    pub font_family: Option<String>,
    pub font_size: f32,
    pub heading_size: f32,

    pub banner_warning_bg: Color32,
    pub banner_warning_fg: Color32,

    pub input_bg: Color32,
    pub input_border_color: Color32,
    pub input_radius: f32,
    pub input_focus_border: Color32,

    pub mono_bg: Color32,
    // Used by the text-info mono content area.
    #[allow(dead_code)]
    pub mono_fg: Color32,

    pub button_radius: f32,
    pub button_padding: [f32; 2],
    pub button_primary_bg: Color32,
    pub button_primary_fg: Color32,
    pub button_secondary_bg: Color32,
    pub button_secondary_border: Color32,
    pub button_secondary_fg: Color32,
}

/// Resolve the effective tokens: built-in defaults for the given mode,
/// overridden by the theme file (if any), then by CLI options.
pub fn resolve(
    dark: bool,
    theme_path: Option<&PathBuf>,
    font: Option<&str>,
    font_size: Option<f32>,
) -> Tokens {
    let mut tokens = if dark {
        defaults::dark()
    } else {
        defaults::light()
    };

    if let Some(file) = load_theme_file(theme_path) {
        merge(&mut tokens, &file);
    }

    if let Some(family) = font {
        tokens.font_family = Some(family.to_owned());
    }
    if let Some(size) = font_size {
        tokens.font_size = size;
    }

    tokens
}

fn theme_file_path(explicit: Option<&PathBuf>) -> Option<PathBuf> {
    if let Some(path) = explicit {
        return Some(path.clone());
    }
    // Unified path on all three platforms (mado-plan §4.1).
    let default = dirs::home_dir()?
        .join(".config")
        .join("mado")
        .join("theme.toml");
    default.exists().then_some(default)
}

fn load_theme_file(explicit: Option<&PathBuf>) -> Option<ThemeFile> {
    let path = theme_file_path(explicit)?;
    let content = match std::fs::read_to_string(&path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("mado: cannot read theme file {}: {err}", path.display());
            return None;
        }
    };
    match toml::from_str(&content) {
        Ok(file) => Some(file),
        Err(err) => {
            eprintln!("mado: invalid theme file {}: {err}", path.display());
            None
        }
    }
}

// --- theme.toml model (mado-plan §4.3); unknown sections/keys are ignored ---

#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
struct ThemeFile {
    window: WindowSection,
    font: FontSection,
    colors: BTreeMap<String, String>,
    titlebar: TitlebarSection,
    heading: HeadingSection,
    banner: BannerSection,
    input: InputSection,
    button: ButtonSection,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
struct WindowSection {
    background: Option<Background>,
    radius: Option<f32>,
    padding: Option<f32>,
    border: Option<BorderSpec>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
struct FontSection {
    family: Option<String>,
    size: Option<f32>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
struct TitlebarSection {
    show: Option<bool>,
    height: Option<f32>,
    icon: Option<String>,
    background: Option<Background>,
    color: Option<String>,
    separator: Option<BorderSpec>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
struct HeadingSection {
    size: Option<f32>,
    color: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
struct BannerSection {
    warning: BannerStyle,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
struct BannerStyle {
    background: Option<Background>,
    color: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
struct InputSection {
    radius: Option<f32>,
    background: Option<Background>,
    border: Option<BorderSpec>,
    border_focus: Option<BorderSpec>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
struct ButtonSection {
    radius: Option<f32>,
    padding: Option<PaddingSpec>,
    primary: ButtonStyle,
    secondary: ButtonStyle,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
struct ButtonStyle {
    background: Option<Background>,
    color: Option<String>,
    border: Option<BorderSpec>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
struct BorderSpec {
    color: Option<String>,
    width: Option<f32>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
struct PaddingSpec {
    x: Option<f32>,
    y: Option<f32>,
}

/// A background is either a plain color string or a gradient table.
/// Gradient rendering lands in v0.3; until then the first stop is used.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Background {
    Color(String),
    Gradient {
        gradient: Vec<String>,
        #[serde(default)]
        #[allow(dead_code)]
        angle: Option<f32>,
    },
}

impl Background {
    fn first_color(&self) -> Option<&str> {
        match self {
            Background::Color(color) => Some(color),
            Background::Gradient { gradient, .. } => gradient.first().map(String::as_str),
        }
    }
}

// --- merging ---

fn merge(tokens: &mut Tokens, file: &ThemeFile) {
    // Semantic palette: defaults, overridden by [colors].
    let mut palette: BTreeMap<&str, Color32> = BTreeMap::from([
        ("foreground", tokens.foreground),
        ("muted", tokens.muted),
        ("accent", tokens.accent),
        ("warning", tokens.warning),
        ("danger", tokens.danger),
    ]);
    for (name, value) in &file.colors {
        if let Some(color) = parse_color(value) {
            if let Some(slot) = palette.get_mut(name.as_str()) {
                *slot = color;
            }
        } else {
            eprintln!("mado: invalid color for [colors].{name}: {value:?}");
        }
    }
    tokens.foreground = palette["foreground"];
    tokens.muted = palette["muted"];
    tokens.accent = palette["accent"];
    tokens.warning = palette["warning"];
    tokens.danger = palette["danger"];

    let resolve = |value: &str| -> Option<Color32> {
        palette.get(value).copied().or_else(|| parse_color(value))
    };
    let resolve_bg = |bg: &Background| -> Option<Color32> { bg.first_color().and_then(resolve) };

    let w = &file.window;
    set_color_opt(&mut tokens.window_bg, w.background.as_ref(), resolve_bg);
    set_opt(&mut tokens.window_radius, w.radius);
    set_opt(&mut tokens.window_padding, w.padding);
    if let Some(border) = &w.border {
        set_color_opt(
            &mut tokens.window_border_color,
            border.color.as_deref(),
            resolve,
        );
        set_opt(&mut tokens.window_border_width, border.width);
    }

    set_opt(&mut tokens.font_family, file.font.family.clone().map(Some));
    set_opt(&mut tokens.font_size, file.font.size);

    let t = &file.titlebar;
    set_opt(&mut tokens.titlebar_show, t.show);
    set_opt(&mut tokens.titlebar_height, t.height);
    set_opt(&mut tokens.titlebar_icon, t.icon.clone());
    set_color_opt(&mut tokens.titlebar_bg, t.background.as_ref(), resolve_bg);
    set_color_opt(&mut tokens.titlebar_fg, t.color.as_deref(), resolve);
    if let Some(sep) = &t.separator {
        set_color_opt(
            &mut tokens.titlebar_separator_color,
            sep.color.as_deref(),
            resolve,
        );
        set_opt(&mut tokens.titlebar_separator_width, sep.width);
    }

    set_opt(&mut tokens.heading_size, file.heading.size);

    let banner = &file.banner.warning;
    set_color_opt(
        &mut tokens.banner_warning_bg,
        banner.background.as_ref(),
        resolve_bg,
    );
    set_color_opt(
        &mut tokens.banner_warning_fg,
        banner.color.as_deref(),
        resolve,
    );

    let input = &file.input;
    set_opt(&mut tokens.input_radius, input.radius);
    set_color_opt(&mut tokens.input_bg, input.background.as_ref(), resolve_bg);
    if let Some(border) = &input.border {
        set_color_opt(
            &mut tokens.input_border_color,
            border.color.as_deref(),
            resolve,
        );
    }
    if let Some(focus) = &input.border_focus {
        set_color_opt(
            &mut tokens.input_focus_border,
            focus.color.as_deref(),
            resolve,
        );
    }

    let button = &file.button;
    set_opt(&mut tokens.button_radius, button.radius);
    if let Some(padding) = &button.padding {
        if let Some(x) = padding.x {
            tokens.button_padding[0] = x;
        }
        if let Some(y) = padding.y {
            tokens.button_padding[1] = y;
        }
    }
    set_color_opt(
        &mut tokens.button_primary_bg,
        button.primary.background.as_ref(),
        resolve_bg,
    );
    set_color_opt(
        &mut tokens.button_primary_fg,
        button.primary.color.as_deref(),
        resolve,
    );
    set_color_opt(
        &mut tokens.button_secondary_bg,
        button.secondary.background.as_ref(),
        resolve_bg,
    );
    set_color_opt(
        &mut tokens.button_secondary_fg,
        button.secondary.color.as_deref(),
        resolve,
    );
    if let Some(border) = &button.secondary.border {
        set_color_opt(
            &mut tokens.button_secondary_border,
            border.color.as_deref(),
            resolve,
        );
    }
}

fn set_opt<T>(slot: &mut T, value: Option<T>) {
    if let Some(value) = value {
        *slot = value;
    }
}

fn set_color_opt<S>(slot: &mut Color32, value: Option<S>, resolve: impl Fn(S) -> Option<Color32>) {
    if let Some(color) = value.and_then(resolve) {
        *slot = color;
    }
}

/// Parse "#rgb", "#rrggbb", "#rrggbbaa", "rgba(r,g,b,a)" or "transparent".
pub fn parse_color(value: &str) -> Option<Color32> {
    let value = value.trim();
    if value.eq_ignore_ascii_case("transparent") {
        return Some(Color32::TRANSPARENT);
    }
    if let Some(hex) = value.strip_prefix('#') {
        return match hex.len() {
            3 => {
                let mut bytes = [0u8; 3];
                for (i, c) in hex.chars().enumerate() {
                    let d = c.to_digit(16)? as u8;
                    bytes[i] = d * 17;
                }
                Some(Color32::from_rgb(bytes[0], bytes[1], bytes[2]))
            }
            6 | 8 => {
                let mut bytes = [0u8; 4];
                bytes[3] = 255;
                for i in 0..hex.len() / 2 {
                    bytes[i] = u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16).ok()?;
                }
                Some(Color32::from_rgba_unmultiplied(
                    bytes[0], bytes[1], bytes[2], bytes[3],
                ))
            }
            _ => None,
        };
    }
    if let Some(inner) = value
        .strip_prefix("rgba(")
        .or_else(|| value.strip_prefix("rgb("))
        .and_then(|s| s.strip_suffix(')'))
    {
        let parts: Vec<&str> = inner.split(',').map(str::trim).collect();
        if parts.len() < 3 {
            return None;
        }
        let r: u8 = parts[0].parse().ok()?;
        let g: u8 = parts[1].parse().ok()?;
        let b: u8 = parts[2].parse().ok()?;
        let a: f32 = if parts.len() > 3 {
            parts[3].parse().ok()?
        } else {
            1.0
        };
        return Some(Color32::from_rgba_unmultiplied(
            r,
            g,
            b,
            (a.clamp(0.0, 1.0) * 255.0).round() as u8,
        ));
    }
    None
}
