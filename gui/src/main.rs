#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use config::GuiConfigBuilder;
use simple_logger::SimpleLogger;

use crate::gui::LOSGui;

mod config;
mod gui;
mod lazy;

fn main() {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Debug)
        .init()
        .expect("Unable to start logger");

    //TODO: read config from env or file
    let config = GuiConfigBuilder::default()
        .build()
        .expect("Configuration must be valid");

    let window_size = egui::ViewportBuilder::default()
        .inner_size
        .map(|size| size.x.max(size.y))
        .map(|size| egui::Vec2::splat(size))
        .unwrap_or(egui::Vec2::new(1280., 720.));

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(window_size),
        ..Default::default()
    };

    eframe::run_native(
        "Lights Out Solver",
        options,
        Box::new(|_cc| Box::new(LOSGui::new(config))),
    )
    .expect("Error creating window");
}
