use std::{
    collections::HashSet,
    sync::mpsc::{Receiver, Sender},
    thread::{self},
};

use eframe::egui;
use log::{debug, warn};
use solvers::{
    board::{Binary, Board},
    gf2,
};

use crate::{config::GuiConfig, lazy::Lazy};

pub struct LOSGui {
    board: Box<Binary>,
    config: GuiConfig,
    solution: Lazy<Solution>,
    tx: Sender<Events>,
    rx: Receiver<Events>,
    language: String,
}

type Solution = Option<HashSet<usize>>;

enum Events {
    IncreaseRow,
    DecreaseRow,
    IncreaseCol,
    DecreaseCol,
    TriggerCell(usize),
    Exit,
    Reset,
    Solve,
    SolutionFound(Solution),
}

impl LOSGui {
    pub fn new(config: GuiConfig) -> Self {
        let (tx, rx) = std::sync::mpsc::channel();

        Self {
            language: config.initial_language.to_string(),
            board: Box::new(Binary::new_blank(config.initial_cols, config.initial_rows)),
            config,
            solution: Lazy::Empty,
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
                solution
                    .as_ref()
                    .map(|s| s.contains(&index))
                    .unwrap_or_default()
            })
            .unwrap_or_default();
        let loading = matches!(self.solution, Lazy::Requested);

        ui.add_enabled_ui(!loading, |ui| {
            self.draw_cell_base(ui, marked, active)
                .on_hover_cursor(egui::CursorIcon::PointingHand)
        })
        .inner
    }

    fn draw_board(&mut self, ui: &mut egui::Ui) {
        let cell_space = 5.;
        let spinner_scale = 0.8;
        let loading = matches!(self.solution, Lazy::Requested);

        let board_rect = egui::Grid::new("board")
            .spacing([cell_space, cell_space])
            .show(ui, |ui| {
                for row in 0..self.board.rows() {
                    for col in 0..self.board.cols() {
                        let cell = self.draw_cell(col, row, ui);

                        if cell.clicked() && !loading {
                            self.queue_event(Events::TriggerCell(self.board.get_index(col, row)));
                        }
                    }
                    ui.end_row();
                }
            })
            .response
            .rect;

        if loading {
            let side_length = (board_rect.width() * spinner_scale)
                .min(board_rect.height() * spinner_scale)
                .min(200.);

            let spinner_rect =
                egui::Rect::from_center_size(board_rect.center(), egui::Vec2::splat(side_length));

            egui::Spinner::new().paint_at(ui, spinner_rect);
        }
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
                    ui.vertical(|ui| {
                        ui.horizontal_top(|ui| {
                            self.draw_board(ui);

                            self.draw_control(
                                ui,
                                self.get_text("control.left"),
                                self.board.cols() > self.config.col_range.start,
                                || self.queue_event(Events::DecreaseCol),
                            );
                            self.draw_control(
                                ui,
                                self.get_text("control.right"),
                                self.board.cols() < self.config.col_range.end,
                                || self.queue_event(Events::IncreaseCol),
                            );
                        });

                        self.draw_control(
                            ui,
                            self.get_text("control.up"),
                            self.board.rows() > self.config.row_range.start,
                            || self.queue_event(Events::DecreaseRow),
                        );
                        self.draw_control(
                            ui,
                            self.get_text("control.down"),
                            self.board.rows() < self.config.row_range.end,
                            || self.queue_event(Events::IncreaseRow),
                        );
                    });

                    ui.add_space(self.config.text_size);
                    self.print_instructions(ui, ctx);
                });
            });
        });
    }

    fn print_instructions(&self, ui: &mut egui::Ui, ctx: &egui::Context) {
        let is_mac = !matches!(ctx.os(), egui::os::OperatingSystem::Mac);

        ui.vertical(|ui| {
            ui.style_mut().wrap = Some(false);
            ui.spacing_mut().item_spacing = egui::Vec2::splat(self.config.text_size);

            ui.collapsing(self.text("instructions.goal.header"), |ui| {
                ui.style_mut().wrap = Some(true);
                let description = egui::text::LayoutJob::single_section(
                    self.get_text("instructions.goal.description"),
                    egui::TextFormat {
                        font_id: egui::FontId::proportional(self.config.text_size),
                        ..Default::default()
                    },
                );

                // NOTE: `Label` overrides some of the wrapping settings, e.g. wrap width
                ui.label(description);
            });

            ui.collapsing(self.text("instructions.shortcuts.header"), |ui| {
                ui.horizontal(|ui| {
                    ui.code(
                        self.text(
                            self.config
                                .solve_shortcut
                                .format(&egui::ModifierNames::NAMES, is_mac),
                        ),
                    );
                    self.label(ui, "instructions.shortcuts.description.solve");
                });

                ui.horizontal(|ui| {
                    ui.code(
                        self.text(
                            self.config
                                .reset_shortcut
                                .format(&egui::ModifierNames::NAMES, is_mac),
                        ),
                    );
                    self.label(ui, "instructions.shortcuts.description.reset");
                });

                ui.horizontal(|ui| {
                    ui.code(
                        self.text(
                            self.config
                                .exit_shortcut
                                .format(&egui::ModifierNames::NAMES, is_mac),
                        ),
                    );
                    self.label(ui, "instructions.shortcuts.description.exit");
                });
            });

            ui.collapsing(self.text("instructions.states.header"), |ui| {
                ui.horizontal(|ui| {
                    self.draw_cell_base(ui, true, true)
                        | self.label(ui, "instructions.states.mixed.name")
                })
                .inner
                .on_hover_ui(|ui| {
                    self.label(ui, "instructions.states.mixed.tooltip");
                });

                ui.horizontal(|ui| {
                    self.draw_cell_base(ui, true, false)
                        | self.label(ui, "instructions.states.marked.name")
                })
                .inner
                .on_hover_ui(|ui| {
                    self.label(ui, "instructions.states.marked.tooltip");
                });

                ui.horizontal(|ui| {
                    self.draw_cell_base(ui, false, true)
                        | self.label(ui, "instructions.states.active.name")
                })
                .inner
                .on_hover_ui(|ui| {
                    self.label(ui, "instructions.states.active.tooltip");
                });

                ui.horizontal(|ui| {
                    self.draw_cell_base(ui, false, false)
                        | self.label(ui, "instructions.states.default.name")
                })
                .inner
                .on_hover_ui_at_pointer(|ui| {
                    self.label(ui, "instructions.states.default.tooltip");
                });
            })
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

    ///
    /// Draw into [egui::Ui] the basic state of a button
    ///
    /// Color will be set based on marked (is part of the solution) and active (is triggered)
    ///
    fn draw_cell_base(&self, ui: &mut egui::Ui, marked: bool, active: bool) -> egui::Response {
        let color = match (marked, active) {
            (true, true) => egui::Color32::from_rgb(255, 165, 0), // Orange
            (true, false) => egui::Color32::GOLD,
            (false, true) => egui::Color32::RED,
            (false, false) => egui::Color32::DARK_GRAY,
        };

        let button = egui::Button::new("").fill(color);

        ui.add_sized([self.config.cell_size, self.config.cell_size], button)
            .highlight()
    }

    fn resize_board(&mut self, new_cols: usize, new_rows: usize) {
        // Replace with empty board bc otherwise may end up with impossible state
        self.board = Box::new(Binary::new_blank(new_cols, new_rows));
        self.solution = Lazy::Empty;
    }

    fn handle_keys(&mut self, ctx: &egui::Context) {
        ctx.input_mut(|reader| {
            if reader.consume_shortcut(&self.config.exit_shortcut) {
                self.queue_event(Events::Exit);
            }

            if reader.consume_shortcut(&self.config.reset_shortcut) {
                self.queue_event(Events::Reset);
            }

            if reader.consume_shortcut(&self.config.solve_shortcut) {
                self.queue_event(Events::Solve);
            }

            // arrows map to board resize
            if reader.key_pressed(egui::Key::ArrowLeft) || reader.key_pressed(egui::Key::A) {
                self.queue_event(Events::DecreaseCol);
            }
            if reader.key_pressed(egui::Key::ArrowRight) || reader.key_pressed(egui::Key::D) {
                self.queue_event(Events::IncreaseCol);
            }
            if reader.key_pressed(egui::Key::ArrowUp) || reader.key_pressed(egui::Key::W) {
                self.queue_event(Events::DecreaseRow);
            }
            if reader.key_pressed(egui::Key::ArrowDown) || reader.key_pressed(egui::Key::S) {
                self.queue_event(Events::IncreaseRow);
            }
        })
    }

    fn queue_event(&self, event: Events) {
        let _ = self.tx.send(event);
    }

    fn text(&self, translation_key: impl Into<String>) -> egui::RichText {
        egui::RichText::new(self.get_text(translation_key))
            .font(egui::FontId::proportional(self.config.text_size))
    }

    fn label(&self, ui: &mut egui::Ui, translation_key: impl Into<String>) -> egui::Response {
        ui.label(self.text(translation_key))
    }

    pub fn get_text(&self, translation_key: impl Into<String>) -> String {
        if !self.config.tranlation_ctx.contains_key(&self.language) {
            warn!(
                "Language {} not loaded, using default translation",
                &self.language
            );
        }

        let key = translation_key.into();
        self.config
            .tranlation_ctx
            .get_text_with_key(&self.language, &key)
            .map(|val| val.to_string())
            .unwrap_or_else(|| {
                warn!("Translation with key {} not found", key);
                key
            })
    }
}

impl eframe::App for LOSGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !matches!(self.solution, Lazy::Requested) {
            self.handle_keys(ctx);
        }
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
                    self.solution
                        .as_mut()
                        .map(|sol| sol.as_mut().map(|hash| hash.remove(&index)));
                }
                Events::Exit => {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    debug!("Exiting requested");
                }
                Events::Reset => {
                    self.resize_board(self.board.cols(), self.board.rows());
                }
                Events::Solve => {
                    let board = self.board.as_ref().to_owned();
                    self.solution = Lazy::Requested;
                    calculate_solution(board, self.tx.clone(), ctx.clone());
                }
                Events::SolutionFound(solution) => {
                    self.solution = Lazy::Completed(solution);
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
