#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use config::GuiConfigBuilder;
use simple_logger::SimpleLogger;

use crate::gui::LOSGui;

mod config;
mod gui;

fn main() {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Debug)
        .init()
        .expect("Unable to start logger");

    //TODO: read config from env or file
    let config = GuiConfigBuilder::new().build();

    eframe::run_native(
        "Lights Out Solver",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Box::new(LOSGui::new(config))),
    )
    .expect("Error creating window");
}
