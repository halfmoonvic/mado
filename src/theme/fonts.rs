//! System font discovery. egui's bundled fonts have no CJK coverage, and the
//! primary mado use case is displaying Chinese/Japanese translation results,
//! so a platform CJK font is always loaded as fallback.

use std::sync::Arc;

use eframe::egui;
use egui::{FontData, FontDefinitions, FontFamily};

#[cfg(target_os = "windows")]
const BODY: &[&str] = &["Segoe UI"];
#[cfg(target_os = "macos")]
const BODY: &[&str] = &["Helvetica Neue"];
#[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
const BODY: &[&str] = &["Noto Sans", "DejaVu Sans"];

#[cfg(target_os = "windows")]
const CJK: &[&str] = &[
    "Microsoft YaHei UI",
    "Microsoft YaHei",
    "Yu Gothic UI",
    "Meiryo",
];
#[cfg(target_os = "macos")]
const CJK: &[&str] = &[
    "PingFang SC",
    "Hiragino Sans GB",
    "Hiragino Kaku Gothic ProN",
];
#[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
const CJK: &[&str] = &[
    "Noto Sans CJK SC",
    "Noto Sans SC",
    "Source Han Sans SC",
    "WenQuanYi Micro Hei",
];

#[cfg(target_os = "windows")]
const SERIF: &[&str] = &["Fraunces", "Georgia", "Constantia"];
#[cfg(target_os = "macos")]
const SERIF: &[&str] = &["Fraunces", "Georgia", "New York"];
#[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
const SERIF: &[&str] = &["Fraunces", "Noto Serif", "DejaVu Serif"];

#[cfg(target_os = "windows")]
const MONO: &[&str] = &["JetBrains Mono", "Cascadia Mono", "Consolas"];
#[cfg(target_os = "macos")]
const MONO: &[&str] = &["JetBrains Mono", "SF Mono", "Menlo"];
#[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
const MONO: &[&str] = &["JetBrains Mono", "Noto Sans Mono", "DejaVu Sans Mono"];

/// Family name for headings (serif per the design; falls back to body).
pub const HEADING_FAMILY: &str = "mado-heading";

/// Install system fonts into the egui context. `family` is the theme /
/// CLI-requested body font, tried before the platform defaults.
pub fn install_fonts(ctx: &egui::Context, family: Option<&str>) {
    let mut db = fontdb::Database::new();
    db.load_system_fonts();

    let mut defs = FontDefinitions::default();

    let body = load_first(
        &mut defs,
        &db,
        family.into_iter().chain(BODY.iter().copied()),
    );
    let cjk = load_first(&mut defs, &db, CJK.iter().copied());
    let serif = load_first(&mut defs, &db, SERIF.iter().copied());
    let mono = load_first(&mut defs, &db, MONO.iter().copied());

    let prop = defs.families.entry(FontFamily::Proportional).or_default();
    for name in [&cjk, &body].into_iter().flatten() {
        prop.insert(0, name.clone());
    }

    let mono_family = defs.families.entry(FontFamily::Monospace).or_default();
    for name in [&cjk, &mono].into_iter().flatten() {
        mono_family.insert(0, name.clone());
    }

    // Headings: serif first, then the whole proportional stack as fallback.
    let mut heading = Vec::new();
    heading.extend(serif.clone());
    heading.extend(defs.families[&FontFamily::Proportional].iter().cloned());
    defs.families
        .insert(FontFamily::Name(HEADING_FAMILY.into()), heading);

    ctx.set_fonts(defs);
}

/// Load the first available family from `candidates` into `defs`;
/// returns the registered font name.
fn load_first<'a>(
    defs: &mut FontDefinitions,
    db: &fontdb::Database,
    candidates: impl Iterator<Item = &'a str>,
) -> Option<String> {
    for name in candidates {
        let query = fontdb::Query {
            families: &[fontdb::Family::Name(name)],
            ..Default::default()
        };
        let Some(id) = db.query(&query) else {
            continue;
        };
        let Some((source, index)) = db.face(id).map(|f| (f.source.clone(), f.index)) else {
            continue;
        };
        let bytes = match source {
            fontdb::Source::Binary(data) | fontdb::Source::SharedFile(_, data) => {
                data.as_ref().as_ref().to_vec()
            }
            fontdb::Source::File(path) => match std::fs::read(&path) {
                Ok(bytes) => bytes,
                Err(_) => continue,
            },
        };
        let mut data = FontData::from_owned(bytes);
        data.index = index;
        defs.font_data.insert(name.to_owned(), Arc::new(data));
        return Some(name.to_owned());
    }
    None
}
