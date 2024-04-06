use eframe::egui;
use log::debug;
use solvers::board::{Binary, Board};

use crate::config::GuiConfig;

pub struct LOSGui {
    board: Box<dyn Board>,
    exit: bool,
    config: GuiConfig,
}

impl LOSGui {
    pub fn new(config: GuiConfig) -> Self {
        Self {
            board: Box::new(Binary::new_blank(5, 5)),
            exit: false,
            config,
        }
    }

    pub fn draw_cell(&self, col: usize, row: usize, ui: &mut egui::Ui) -> egui::Response {
        let checked = self.board.get(col, row).map_or(false, |val| val >= 1);
        let color = if checked {
            egui::Color32::RED
        } else {
            egui::Color32::DARK_GRAY
        };
        let button = egui::Button::new("").fill(color);

        ui.add_sized([self.config.cell_size, self.config.cell_size], button)
            .highlight()
            .on_hover_cursor(egui::CursorIcon::PointingHand)
    }
}

impl eframe::App for LOSGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.exit {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            debug!("Exiting requested");
            self.exit = false;
        }

        ctx.input(|reader| {
            if reader.key_released(egui::Key::Q) {
                self.exit = true;
            }
        });
        let cell_space = 5.;
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Grid::new("board")
                .spacing([cell_space, cell_space])
                .show(ui, |ui| {
                    for row in 0..self.board.rows() {
                        for col in 0..self.board.cols() {
                            let cell = self.draw_cell(col, row, ui);

                            if cell.clicked() {
                                self.board.trigger_coord(col, row);
                            }
                        }
                        ui.end_row();
                    }
                });
        });
    }
}
