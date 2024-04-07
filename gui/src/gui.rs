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

    fn draw_board(&mut self, ui: &mut egui::Ui) {
        let cell_space = 5.;

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
    }

    fn update_rows(&mut self, increase: bool) {
        let rows = if increase {
            self.board.rows() + 1
        } else {
            self.board.rows() - 1
        };

        self.board = Box::new(Binary::new_blank(self.board.cols(), rows));
    }

    fn render(&mut self, ctx: &egui::Context) {
        let style = ctx.style();
        let mut frame = egui::Frame::window(&style)
            .inner_margin(egui::Margin::same(self.config.cell_size / 2.));

        if ctx.debug_on_hover() {
            frame = frame.fill(egui::Color32::RED);
        }

        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                let window_margin = ui.spacing().window_margin;
                ui.spacing_mut().item_spacing = egui::Vec2::splat(window_margin.left);

                ui.horizontal_top(|ui| {
                    self.draw_board(ui);

                    let button_inc = egui::Button::new(">");
                    let button_dec = egui::Button::new("<");
                    ui.add_sized([self.config.cell_size, self.config.cell_size], button_dec)
                        .highlight()
                        .on_hover_cursor(egui::CursorIcon::PointingHand);
                    ui.add_sized([self.config.cell_size, self.config.cell_size], button_inc)
                        .highlight()
                        .on_hover_cursor(egui::CursorIcon::PointingHand);
                });

                self.draw_row_controls(ui);
            });
        });
    }

    fn draw_row_controls(&mut self, ui: &mut egui::Ui) {
        let button_dec = egui::Button::new("^");
        let button_inc = egui::Button::new("v");

        ui.add_enabled_ui(self.board.rows() > self.config.row_range.start, |ui| {
            let clicked = ui
                .add_sized([self.config.cell_size, self.config.cell_size], button_dec)
                .highlight()
                .on_hover_cursor(egui::CursorIcon::PointingHand)
                .clicked();

            if clicked {
                self.update_rows(false);
            }
        });
        ui.add_enabled_ui(self.board.rows() < self.config.row_range.end, |ui| {
            let clicked = ui
                .add_sized([self.config.cell_size, self.config.cell_size], button_inc)
                .highlight()
                .on_hover_cursor(egui::CursorIcon::PointingHand)
                .clicked();

            if clicked {
                self.update_rows(true);
            }
        });
    }

    fn handle_exit(&mut self, ctx: &egui::Context) {
        if self.exit {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            debug!("Exiting requested");
            self.exit = false;
        }

        ctx.input_mut(|reader| {
            if reader.consume_shortcut(&self.config.exit_shortcut) {
                self.exit = true;
            }
        });
    }
}

impl eframe::App for LOSGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle_exit(ctx);

        self.render(ctx);
    }
}
