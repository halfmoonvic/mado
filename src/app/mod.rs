use std::sync::Arc;
use std::sync::atomic::{AtomicI32, Ordering};

use eframe::egui;

use crate::cli::{Cli, DialogKind};
use crate::exit::Outcome;

/// Shared window shell: runs an eframe window and reports the outcome.
pub struct Shell {
    outcome: Arc<AtomicI32>,
    title: String,
}

impl Shell {
    fn new(outcome: Arc<AtomicI32>, title: String) -> Self {
        Self { outcome, title }
    }

    fn finish(&self, ctx: &egui::Context, outcome: Outcome) {
        self.outcome.store(outcome.code(), Ordering::SeqCst);
        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
    }
}

impl eframe::App for Shell {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.finish(&ctx, Outcome::Cancel);
        }

        egui::Frame::central_panel(ui.style()).show(ui, |ui| {
            ui.heading(&self.title);
            ui.label("mado scaffold — dialogs coming next");
        });
    }
}

/// Run the dialog window described by `cli` and return its outcome code.
pub fn run(cli: &Cli) -> i32 {
    let title = cli.title.clone().unwrap_or_else(|| "mado".to_owned());
    let (default_width, default_height) = match cli.dialog_kind() {
        DialogKind::TextInfo => (650.0, 360.0),
        DialogKind::Info | DialogKind::Warning | DialogKind::Error => (380.0, 150.0),
    };
    let width = cli.width.unwrap_or(default_width);
    let height = cli.height.unwrap_or(default_height);

    let viewport = egui::ViewportBuilder::default()
        .with_title(&title)
        .with_inner_size([width, height])
        .with_always_on_top_maybe(cli.always_on_top);

    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    // Closing the window without cancelling counts as a normal close (0).
    let outcome = Arc::new(AtomicI32::new(Outcome::Ok.code()));
    let app_outcome = outcome.clone();

    let app_name = title.clone();
    let result = eframe::run_native(
        &app_name,
        options,
        Box::new(move |_cc| Ok(Box::new(Shell::new(app_outcome, title)))),
    );

    if result.is_err() {
        return 255;
    }
    outcome.load(Ordering::SeqCst)
}

trait ViewportBuilderExt {
    fn with_always_on_top_maybe(self, on_top: bool) -> Self;
}

impl ViewportBuilderExt for egui::ViewportBuilder {
    fn with_always_on_top_maybe(self, on_top: bool) -> Self {
        if on_top {
            self.with_always_on_top()
        } else {
            self
        }
    }
}
