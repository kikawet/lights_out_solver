#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod adapter;
mod gui;
mod lazy;

#[macro_use]
extern crate json_gettext;
//TODO: Remove everything about benchmark and use this crate since is already integrated with egui - https://github.com/EmbarkStudios/puffin/tree/main/puffin_egui
#[cfg(feature = "benchmark")]
use std::{
    process::exit,
    sync::mpsc::Sender,
    thread::{self, sleep},
    time::{Duration, Instant},
};

use gui::config;
#[cfg(feature = "benchmark")]
use gui::Events;
use simple_logger::SimpleLogger;

use crate::gui::Gui;

fn main() {
    //TODO: read CLI args to dynamically read config files
    let config = ::config::Config::builder()
        .add_source(::config::File::with_name("gui/config/default").required(false))
        .add_source(::config::File::with_name("gui/config/local").required(false))
        .add_source(::config::Environment::with_prefix("LOS"))
        .build()
        .expect("Unable to load config from external sources")
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

    #[cfg(not(feature = "benchmark"))]
    let benchmark = None;
    #[cfg(feature = "benchmark")]
    let benchmark = Some(setup_benchmark());

    let gui = Gui::new(config, benchmark);

    eframe::run_native(
        &gui.get_text("app.title"),
        options,
        Box::new(|_cc| Ok(Box::new(gui))),
    )
    .expect("Error creating window");
}

#[cfg(feature = "benchmark")]
fn setup_benchmark() -> Sender<Events> {
    let (tx, rx) = std::sync::mpsc::channel::<Events>();

    thread::spawn(|| {
        sleep(Duration::from_secs(5));
        // TODO: For some reason this code collects everything after 5 seconds
        // change it so is dynamic
        let timestamps = rx
            .into_iter()
            .filter_map(|event| {
                if let Events::TimeStamp(val) = event {
                    Some(val)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        println!("timestamps: {}", timestamps.len());
        let fps = calculate_frames_per_second(timestamps);

        println!("{fps:?}");
        exit(1);
    });

    tx
}

#[cfg(feature = "benchmark")]
fn calculate_frames_per_second(timestamps: Vec<Instant>) -> Vec<usize> {
    let frame_threshold = Duration::from_secs(1);
    let mut fps: Vec<usize> = Vec::new();
    let mut current_second: Instant = timestamps[0];
    let mut current_frame_count = 0usize;

    for timestamp in timestamps {
        if timestamp.duration_since(current_second) < frame_threshold {
            current_frame_count += 1;
        } else {
            current_second = timestamp;
            fps.push(current_frame_count);
            current_frame_count = 0;
        }
    }

    fps
}
