use std::ops::Range;

const DEFAULT_GUICONFIG: GuiConfig = GuiConfig {
    cell_size: 50.,
    text_size: 25.,
    initial_rows: 5,
    initial_cols: 5,
    row_range: 1..10,
    col_range: 1..10,
    exit_shortcut: egui::KeyboardShortcut::new(egui::Modifiers::NONE, egui::Key::Q),
    reset_shortcut: egui::KeyboardShortcut::new(egui::Modifiers::SHIFT, egui::Key::R),
    solve_shortcut: egui::KeyboardShortcut::new(egui::Modifiers::SHIFT, egui::Key::S),
};

pub struct GuiConfig {
    pub cell_size: f32,
    pub text_size: f32,
    pub initial_rows: usize,
    pub initial_cols: usize,
    pub row_range: Range<usize>,
    pub col_range: Range<usize>,
    pub exit_shortcut: egui::KeyboardShortcut,
    pub reset_shortcut: egui::KeyboardShortcut,
    pub solve_shortcut: egui::KeyboardShortcut,
}

#[allow(dead_code)]
impl GuiConfig {
    // This method will help users to discover the builder
    pub fn builder() -> GuiConfigBuilder {
        GuiConfigBuilder::default()
    }
}

#[derive(Default)]
pub struct GuiConfigBuilder {
    cell_size: Option<f32>,
    text_size: Option<f32>,
    initial_rows: Option<usize>,
    initial_cols: Option<usize>,
    row_range: Option<Range<usize>>,
    col_range: Option<Range<usize>>,
    exit_shortcut: Option<egui::KeyboardShortcut>,
    reset_shortcut: Option<egui::KeyboardShortcut>,
    solve_shortcut: Option<egui::KeyboardShortcut>,
}

#[allow(dead_code)]
impl GuiConfigBuilder {
    pub fn new(/* ... */) -> Self {
        // Set the minimally required fields.
        Self {
            ..Default::default()
        }
    }

    pub fn cell_size(mut self, cell_size: impl Into<f32>) -> GuiConfigBuilder {
        self.cell_size = Some(cell_size.into());
        self
    }

    pub fn text_size(mut self, text_size: impl Into<f32>) -> GuiConfigBuilder {
        self.text_size = Some(text_size.into());
        self
    }

    pub fn initial_rows(mut self, initial_rows: impl Into<usize>) -> GuiConfigBuilder {
        self.initial_rows = Some(initial_rows.into());
        self
    }

    pub fn initial_cols(mut self, initial_cols: impl Into<usize>) -> GuiConfigBuilder {
        self.initial_cols = Some(initial_cols.into());
        self
    }

    pub fn row_range(mut self, row_range: impl Into<Range<usize>>) -> GuiConfigBuilder {
        self.row_range = Some(row_range.into());
        self
    }

    pub fn col_range(mut self, col_range: impl Into<Range<usize>>) -> GuiConfigBuilder {
        self.col_range = Some(col_range.into());
        self
    }

    pub fn exit_shortcut(
        mut self,
        exit_shortcut: impl Into<egui::KeyboardShortcut>,
    ) -> GuiConfigBuilder {
        self.exit_shortcut = Some(exit_shortcut.into());
        self
    }

    pub fn reset_shortcut(
        mut self,
        reset_shortcut: impl Into<egui::KeyboardShortcut>,
    ) -> GuiConfigBuilder {
        self.reset_shortcut = Some(reset_shortcut.into());
        self
    }

    pub fn solve_shortcut(
        mut self,
        solve_shortcut: impl Into<egui::KeyboardShortcut>,
    ) -> GuiConfigBuilder {
        self.solve_shortcut = Some(solve_shortcut.into());
        self
    }

    pub fn build(&self) -> GuiConfig {
        GuiConfig {
            cell_size: self.cell_size.unwrap_or(DEFAULT_GUICONFIG.cell_size),
            text_size: self.text_size.unwrap_or(DEFAULT_GUICONFIG.text_size),
            initial_rows: self.initial_rows.unwrap_or(DEFAULT_GUICONFIG.initial_rows),
            initial_cols: self.initial_cols.unwrap_or(DEFAULT_GUICONFIG.initial_cols),
            row_range: self
                .row_range
                .clone()
                .unwrap_or(DEFAULT_GUICONFIG.row_range),
            col_range: self
                .col_range
                .clone()
                .unwrap_or(DEFAULT_GUICONFIG.col_range),
            exit_shortcut: self
                .exit_shortcut
                .unwrap_or(DEFAULT_GUICONFIG.exit_shortcut),
            reset_shortcut: self
                .reset_shortcut
                .unwrap_or(DEFAULT_GUICONFIG.reset_shortcut),
            solve_shortcut: self
                .solve_shortcut
                .unwrap_or(DEFAULT_GUICONFIG.solve_shortcut),
        }
    }
}
