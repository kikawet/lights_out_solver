use std::{
    collections::HashSet,
    sync::mpsc::{Receiver, Sender},
    thread::{self},
};

use eframe::egui;
use log::debug;
use solvers::{
    board::{Binary, Board},
    gf2,
};

use crate::config::GuiConfig;

type Solution = Option<HashSet<usize>>;

pub struct LOSGui {
    board: Box<Binary>,
    exit: bool,
    config: GuiConfig,
    solution: Solution, //TODO: solution may be requested but no solution may exists
    tx: Sender<Events>,
    rx: Receiver<Events>,
}

enum Events {
    IncreaseRow,
    DecreaseRow,
    IncreaseCol,
    DecreaseCol,
    TriggerCell(usize),
    Reset,
    Solve,
    SolutionFound(Solution),
}

impl LOSGui {
    pub fn new(config: GuiConfig) -> Self {
        let (tx, rx) = std::sync::mpsc::channel();

        Self {
            board: Box::new(Binary::new_blank(5, 5)),
            exit: false,
            config,
            solution: None,
            tx,
            rx,
        }
    }

    pub fn draw_cell(&self, col: usize, row: usize, ui: &mut egui::Ui) -> egui::Response {
        let active = self.board.get(col, row).map_or(false, |val| val >= 1);
        let marked = self
            .solution
            .as_ref()
            .map(|solution| {
                let index = self.board.get_index(col, row);
                solution.contains(&index)
            })
            .unwrap_or_default();

        let color = match (marked, active) {
            (true, true) => egui::Color32::from_rgb(255, 165, 0), // Orange
            (true, false) => egui::Color32::GOLD,
            (false, true) => egui::Color32::RED,
            (false, false) => egui::Color32::DARK_GRAY,
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
                            self.queue_event(Events::TriggerCell(self.board.get_index(col, row)));
                        }
                    }
                    ui.end_row();
                }
            });
    }

    fn resize_board(&mut self, new_cols: usize, new_rows: usize) {
        // Replace with empty board bc otherwise may end with impossible state
        self.board = Box::new(Binary::new_blank(new_cols, new_rows));
    }

    fn render(&mut self, ctx: &egui::Context) {
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
                    self.draw_board(ui);

                    self.draw_control(
                        ui,
                        "⏴",
                        self.board.cols() > self.config.col_range.start,
                        || self.queue_event(Events::DecreaseCol),
                    );
                    self.draw_control(
                        ui,
                        "⏵",
                        self.board.cols() < self.config.col_range.end,
                        || self.queue_event(Events::IncreaseCol),
                    );
                });

                self.draw_control(
                    ui,
                    "⏶",
                    self.board.rows() > self.config.row_range.start,
                    || self.queue_event(Events::DecreaseRow),
                );
                self.draw_control(
                    ui,
                    "⏷",
                    self.board.rows() < self.config.row_range.end,
                    || self.queue_event(Events::IncreaseRow),
                );
            });
        });
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

            // arrows map to board resize
            if reader.key_released(egui::Key::ArrowLeft) || reader.key_released(egui::Key::A) {
                self.queue_event(Events::DecreaseCol);
            }
            if reader.key_released(egui::Key::ArrowRight) || reader.key_released(egui::Key::D) {
                self.queue_event(Events::IncreaseCol);
            }
            if reader.key_released(egui::Key::ArrowUp) || reader.key_released(egui::Key::W) {
                self.queue_event(Events::DecreaseRow);
            }
            if reader.key_released(egui::Key::ArrowDown) {
                // || reader.key_released(egui::Key::S) {
                self.queue_event(Events::IncreaseRow);
            }

            // r maps to reset
            if reader.key_released(egui::Key::R) {
                self.queue_event(Events::Reset);
            }

            // s maps to mark solve
            if reader.key_released(egui::Key::S) {
                self.queue_event(Events::Solve);
            }
        })
    }

    fn queue_event(&self, event: Events) {
        let _ = self.tx.send(event);
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
        self.render(ctx);

        if let Ok(event) = self.rx.try_recv() {
            match event {
                Events::IncreaseRow => {
                    if self.board.rows() < self.config.row_range.end {
                        self.resize_board(self.board.cols(), self.board.rows() + 1)
                    }
                }
                Events::DecreaseRow => {
                    if self.board.rows() > self.config.row_range.start {
                        self.resize_board(self.board.cols(), self.board.rows() - 1)
                    }
                }
                Events::IncreaseCol => {
                    if self.board.cols() < self.config.col_range.end {
                        self.resize_board(self.board.cols() + 1, self.board.rows())
                    }
                }
                Events::DecreaseCol => {
                    if self.board.cols() > self.config.col_range.start {
                        self.resize_board(self.board.cols() - 1, self.board.rows())
                    }
                }
                Events::TriggerCell(index) => {
                    self.board.trigger_index(index);
                    self.solution.as_mut().map(|sol| sol.remove(&index));
                }
                Events::Reset => {
                    self.resize_board(self.board.cols(), self.board.rows());
                    self.solution = None;
                }
                Events::Solve => {
                    self.solution =
                        gf2::solve(self.board.as_ref()).map(|vec| HashSet::from_iter(vec));
                    let board = self.board.as_ref().to_owned();
                    calculate_solution(board, self.tx.clone(), ctx.clone());
                }
                Events::SolutionFound(solution) => {
                    self.solution = solution;
                }
            }
        }
    }
}

fn calculate_solution(board: Binary, tx: Sender<Events>, ctx: egui::Context) {
    thread::spawn(move || {
        let solution = gf2::solve(&board).map(|vec| HashSet::from_iter(vec));
        let _ = tx.send(Events::SolutionFound(solution));
        ctx.request_repaint();
    });
}
