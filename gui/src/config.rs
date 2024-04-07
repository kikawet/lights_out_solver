use std::ops::Range;

const DEFAULT_GUICONFIG: GuiConfig = GuiConfig {
    cell_size: 50.,
    exit_shortcut: egui::KeyboardShortcut::new(egui::Modifiers::NONE, egui::Key::Q),
    row_range: 1..10,
    col_range: 1..10,
};

pub struct GuiConfig {
    pub cell_size: f32,
    pub exit_shortcut: egui::KeyboardShortcut,
    pub row_range: Range<usize>,
    pub col_range: Range<usize>,
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
    exit_shortcut: Option<egui::KeyboardShortcut>,
    row_range: Option<Range<usize>>,
    col_range: Option<Range<usize>>,
}

#[allow(dead_code)]
impl GuiConfigBuilder {
    pub fn new(/* ... */) -> Self {
        // Set the minimally required fields.
        Self {
            cell_size: None,
            exit_shortcut: None,
            row_range: None,
            col_range: None,
        }
    }

    pub fn cell_size(mut self, cell_size: impl Into<f32>) -> GuiConfigBuilder {
        self.cell_size = Some(cell_size.into());
        self
    }

    pub fn exit_shortcut(
        mut self,
        exit_shortcut: impl Into<egui::KeyboardShortcut>,
    ) -> GuiConfigBuilder {
        self.exit_shortcut = Some(exit_shortcut.into());
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

    pub fn build(&self) -> GuiConfig {
        GuiConfig {
            cell_size: self.cell_size.unwrap_or(DEFAULT_GUICONFIG.cell_size),
            exit_shortcut: self
                .exit_shortcut
                .unwrap_or(DEFAULT_GUICONFIG.exit_shortcut),
            row_range: self
                .row_range
                .clone()
                .unwrap_or(DEFAULT_GUICONFIG.row_range),
            col_range: self
                .col_range
                .clone()
                .unwrap_or(DEFAULT_GUICONFIG.col_range),
        }
    }
}
