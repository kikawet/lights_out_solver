use eframe::egui;
use log::debug;
use solvers::board::{Binary, Board};

use crate::config::GuiConfig;

pub struct LOSGui {
    board: Box<dyn Board>,
    exit: bool,
    config: GuiConfig,
}

enum RenderEvents {
    IncreaseRow,
    DecreaseRow,
    IncreaseCol,
    DecreaseCol,
    TriggerCell(usize),
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

    fn draw_board(&mut self, ui: &mut egui::Ui) -> Vec<RenderEvents> {
        let cell_space = 5.;
        let mut events = vec![];

        egui::Grid::new("board")
            .spacing([cell_space, cell_space])
            .show(ui, |ui| {
                for row in 0..self.board.rows() {
                    for col in 0..self.board.cols() {
                        let cell = self.draw_cell(col, row, ui);

                        if cell.clicked() {
                            events.push(RenderEvents::TriggerCell(self.board.get_index(col, row)));
                        }
                    }
                    ui.end_row();
                }
            });

        events
    }

    fn resize_board(&mut self, new_cols: usize, new_rows: usize) {
        // Replace with empty board bc otherwise may end with impossible state
        self.board = Box::new(Binary::new_blank(new_cols, new_rows));
    }

    fn render(&mut self, ctx: &egui::Context) -> Vec<RenderEvents> {
        let mut events = vec![];
        let style = ctx.style();
        let mut frame = egui::Frame::window(&style)
            .inner_margin(egui::Margin::same(self.config.cell_size / 2.));

        if ctx.debug_on_hover() {
            frame = frame.fill(egui::Color32::RED);
        }

        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            let window_margin = ui.spacing().window_margin;
            ui.spacing_mut().item_spacing = egui::Vec2::splat(window_margin.left);

            egui::ScrollArea::both().show(ui, |ui| {
                ui.horizontal_top(|ui| {
                    events.append(&mut self.draw_board(ui));

                    self.draw_control(
                        ui,
                        "⏴",
                        self.board.cols() > self.config.col_range.start,
                        || events.push(RenderEvents::DecreaseCol),
                    );
                    self.draw_control(
                        ui,
                        "⏵",
                        self.board.cols() < self.config.col_range.end,
                        || events.push(RenderEvents::IncreaseCol),
                    );
                });

                self.draw_control(
                    ui,
                    "⏶",
                    self.board.rows() > self.config.row_range.start,
                    || events.push(RenderEvents::DecreaseRow),
                );
                self.draw_control(
                    ui,
                    "⏷",
                    self.board.rows() < self.config.row_range.end,
                    || events.push(RenderEvents::IncreaseRow),
                );
            });
        });

        events
    }

    fn draw_control(
        &self,
        ui: &mut egui::Ui,
        text: impl Into<egui::WidgetText>,
        enabled: bool,
        clicked_action: impl FnOnce(),
    ) {
        ui.add_enabled_ui(enabled, |ui| {
            let clicked = ui
                .add_sized(
                    [self.config.cell_size, self.config.cell_size],
                    egui::Button::new(text),
                )
                .highlight()
                .on_hover_cursor(egui::CursorIcon::PointingHand)
                .clicked();

            if clicked {
                clicked_action()
            }
        });
    }

    fn handle_keys(&mut self, ctx: &egui::Context) {
        ctx.input_mut(|reader| {
            if reader.consume_shortcut(&self.config.exit_shortcut) {
                self.exit = true;
            }
        });
    }
}

impl eframe::App for LOSGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.exit {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            debug!("Exiting requested");
            self.exit = false;
        }

        self.handle_keys(ctx);

        let events = self.render(ctx);

        events.into_iter().for_each(|event| match event {
            RenderEvents::IncreaseRow => {
                self.resize_board(self.board.cols(), self.board.rows() + 1)
            }
            RenderEvents::DecreaseRow => {
                self.resize_board(self.board.cols(), self.board.rows() - 1)
            }
            RenderEvents::IncreaseCol => {
                self.resize_board(self.board.cols() + 1, self.board.rows())
            }
            RenderEvents::DecreaseCol => {
                self.resize_board(self.board.cols() - 1, self.board.rows())
            }
            RenderEvents::TriggerCell(index) => {
                self.board.trigger_index(index);
            }
        });
    }
}
