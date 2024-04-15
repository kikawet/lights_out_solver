#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod adapter;
mod gui;
mod lazy;

#[macro_use]
extern crate json_gettext;

use gui::config;
use simple_logger::SimpleLogger;

use crate::gui::Gui;

fn main() {
    //TODO: read CLI args to dinamically read config files
    let config = ::config::Config::builder()
        .add_source(::config::File::with_name("gui/config/release").required(false))
        .add_source(::config::Environment::with_prefix("LOS"))
        .build()
        .expect("Unable to load config from extenal sources")
        .try_deserialize::<config::ConfigBuilder>()
        .expect("Unable to deserialize GUI Config")
        .build()
        .expect("Configuration must be valid");

    SimpleLogger::new()
        .with_level(*config.log_level)
        .init()
        .expect("Unable to start logger");

    let window_size = egui::ViewportBuilder::default()
        .inner_size
        .map(|size| size.x.max(size.y))
        .map_or(egui::Vec2::new(1280., 720.), egui::Vec2::splat);

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(window_size),
        ..Default::default()
    };

    let gui = Gui::new(config);

    eframe::run_native(
        &gui.get_text("app.title"),
        options,
        Box::new(|_cc| Box::new(gui)),
    )
    .expect("Error creating window");
}
