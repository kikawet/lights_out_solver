const DEFAULT_GUICONFIG: GuiConfig = GuiConfig { cell_size: 50. };

pub struct GuiConfig {
    pub cell_size: f32,
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
}

#[allow(dead_code)]
impl GuiConfigBuilder {
    pub fn new(/* ... */) -> Self {
        // Set the minimally required fields.
        Self { cell_size: None }
    }

    pub fn cell_size(mut self, cell_size: f32) -> GuiConfigBuilder {
        self.cell_size = Some(cell_size);
        self
    }

    pub fn build(&self) -> GuiConfig {
        GuiConfig {
            cell_size: self.cell_size.unwrap_or(DEFAULT_GUICONFIG.cell_size),
        }
    }
}
