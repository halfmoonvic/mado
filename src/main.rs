#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod cli;
mod exit;

use clap::Parser;

fn main() {
    let cli = cli::Cli::parse();
    let code = app::run(&cli);
    std::process::exit(code);
}
