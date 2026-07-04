use std::path::PathBuf;

use clap::Parser;

/// mado (窓) — a cross-platform dialog window for your command line.
#[derive(Debug, Parser)]
#[command(name = "mado", version, about)]
#[command(group = clap::ArgGroup::new("dialog").required(true))]
pub struct Cli {
    /// Display a text information dialog
    #[arg(long, group = "dialog")]
    pub text_info: bool,

    /// Display an info message dialog
    #[arg(long, group = "dialog")]
    pub info: bool,

    /// Display a warning message dialog
    #[arg(long, group = "dialog")]
    pub warning: bool,

    /// Display an error message dialog
    #[arg(long, group = "dialog")]
    pub error: bool,

    /// Set the dialog text (static content for --text-info)
    #[arg(long)]
    pub text: Option<String>,

    /// Read content from a file (--text-info only)
    #[arg(long, conflicts_with = "text")]
    pub filename: Option<PathBuf>,

    /// Watch a file and refresh when its content changes (--text-info only)
    #[arg(long, conflicts_with_all = ["text", "filename"])]
    pub watch: Option<PathBuf>,

    /// Placeholder text shown before any data arrives (--text-info only)
    #[arg(long)]
    pub initial: Option<String>,

    /// Polling interval in milliseconds for --watch
    #[arg(long, default_value_t = 200)]
    pub poll_interval: u64,

    /// Do not wrap long lines (--text-info only)
    #[arg(long)]
    pub no_wrap: bool,

    /// Set the window title
    #[arg(long)]
    pub title: Option<String>,

    /// Set the window width
    #[arg(long)]
    pub width: Option<f32>,

    /// Set the window height
    #[arg(long)]
    pub height: Option<f32>,

    /// Close the dialog automatically after N seconds (exit code 5)
    #[arg(long)]
    pub timeout: Option<u64>,

    /// Set the label of the OK button
    #[arg(long)]
    pub ok_label: Option<String>,

    /// Set the label of the Cancel button
    #[arg(long)]
    pub cancel_label: Option<String>,

    /// Keep the window above all other windows
    #[arg(long)]
    pub always_on_top: bool,

    /// Font family (overrides theme)
    #[arg(long)]
    pub font: Option<String>,

    /// Font size (overrides theme)
    #[arg(long)]
    pub font_size: Option<f32>,

    /// Path to a theme file (overrides the default location)
    #[arg(long)]
    pub theme: Option<PathBuf>,
}

/// Which dialog was requested on the command line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogKind {
    TextInfo,
    Info,
    Warning,
    Error,
}

impl Cli {
    pub fn dialog_kind(&self) -> DialogKind {
        if self.text_info {
            DialogKind::TextInfo
        } else if self.info {
            DialogKind::Info
        } else if self.warning {
            DialogKind::Warning
        } else {
            DialogKind::Error
        }
    }
}
