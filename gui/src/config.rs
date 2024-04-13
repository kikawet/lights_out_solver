use std::ops::Range;

use derive_builder::Builder;
use json_gettext::JSONGetText;

#[derive(Debug, Builder)]
#[builder(
    default,
    setter(into),
    build_fn(validate = "Self::validate"),
    pattern = "owned"
)]
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
    pub tranlation_ctx: JSONGetText<'static>,
    pub initial_language: &'static str,
}

impl Default for GuiConfig {
    fn default() -> Self {
        Self {
            cell_size: 50.,
            text_size: 25.,
            initial_rows: 5,
            initial_cols: 5,
            row_range: 1..10,
            col_range: 1..10,
            exit_shortcut: egui::KeyboardShortcut::new(egui::Modifiers::NONE, egui::Key::Q),
            reset_shortcut: egui::KeyboardShortcut::new(egui::Modifiers::SHIFT, egui::Key::R),
            solve_shortcut: egui::KeyboardShortcut::new(egui::Modifiers::SHIFT, egui::Key::S),
            tranlation_ctx: static_json_gettext_build!(
                "en_UK";
                "en_UK" => "locales/en_UK.json",
                "es_ES" => "locales/es_ES.json"
            )
            .unwrap(),
            initial_language: "en_UK",
        }
    }
}

impl PartialEq for GuiConfig {
    fn eq(&self, other: &Self) -> bool {
        self.cell_size == other.cell_size
            && self.text_size == other.text_size
            && self.initial_rows == other.initial_rows
            && self.initial_cols == other.initial_cols
            && self.row_range == other.row_range
            && self.col_range == other.col_range
            && self.exit_shortcut == other.exit_shortcut
            && self.reset_shortcut == other.reset_shortcut
            && self.solve_shortcut == other.solve_shortcut
            // && self.tranlation_ctx.get_keys() == other.tranlation_ctx.get_keys()
            && self.tranlation_ctx.get_default_key() == other.tranlation_ctx.get_default_key()
            && self.initial_language == other.initial_language
    }
}

impl GuiConfigBuilder {
    fn validate(&self) -> Result<(), GuiConfigBuilderError> {
        if let Some(0) = self.initial_cols {
            return Err(GuiConfigBuilderError::ValidationError(format!(
                "Initial number of columns ({}) must be greater than 0",
                self.initial_cols.as_ref().unwrap()
            )));
        }

        if let Some(0) = self.initial_rows {
            return Err(GuiConfigBuilderError::ValidationError(format!(
                "Initial number of rows ({}) must be greater than 0",
                self.initial_rows.as_ref().unwrap()
            )));
        }

        if self.cell_size.is_some_and(|size| size <= 0.) {
            return Err(GuiConfigBuilderError::ValidationError(format!(
                "Cell size ({}) must be greater than 0",
                self.cell_size.unwrap()
            )));
        }

        if self.text_size.is_some_and(|size| size <= 0.) {
            return Err(GuiConfigBuilderError::ValidationError(format!(
                "Text size ({}) must be greater than 0",
                self.text_size.unwrap()
            )));
        }

        if self
            .row_range
            .as_ref()
            .is_some_and(|range| range.start == 0)
        {
            return Err(GuiConfigBuilderError::ValidationError(format!(
                "Start of row range ({}) must be greater than 0",
                self.row_range.as_ref().unwrap().start
            )));
        }

        if self
            .col_range
            .as_ref()
            .is_some_and(|range| range.start == 0)
        {
            return Err(GuiConfigBuilderError::ValidationError(format!(
                "Start of column range ({}) must be greater than 0",
                self.col_range.as_ref().unwrap().start
            )));
        }

        let default_config = GuiConfig::default();
        let initial_rows = self.initial_rows.unwrap_or(default_config.initial_rows);
        let initial_cols = self.initial_cols.unwrap_or(default_config.initial_cols);
        let initial_language = self
            .initial_language
            .unwrap_or(default_config.initial_language);

        if self
            .row_range
            .as_ref()
            .is_some_and(|range| !range.contains(&initial_rows))
        {
            return Err(GuiConfigBuilderError::ValidationError(format!(
                "Initial number of rows ({initial_rows}) must be in the range {:?}",
                self.row_range.as_ref().unwrap()
            )));
        }

        if self
            .col_range
            .as_ref()
            .is_some_and(|range| !range.contains(&initial_cols))
        {
            return Err(GuiConfigBuilderError::ValidationError(format!(
                "Initial number of columns ({initial_cols}) must be in the range {:?}",
                self.col_range.as_ref().unwrap()
            )));
        }

        if self
            .tranlation_ctx
            .as_ref()
            .is_some_and(|ctx| !ctx.contains_key(initial_language))
        {
            return Err(GuiConfigBuilderError::ValidationError(format!(
                "Initial language ({initial_language}) must be available in the translation context {:?}",
                self.tranlation_ctx.as_ref().unwrap().get_keys()
            )));
        }

        Ok(())
    }
}

#[cfg(test)]
mod config_tests {
    use json_gettext::JSONGetTextBuilder;

    use crate::config::{GuiConfig, GuiConfigBuilder, GuiConfigBuilderError};

    #[test]
    fn verify_default() {
        assert_eq!(
            GuiConfig::default(),
            GuiConfigBuilder::create_empty().build().unwrap()
        );

        assert_eq!(
            GuiConfigBuilder::default().build().unwrap(),
            GuiConfigBuilder::create_empty().build().unwrap()
        );
    }

    #[test]
    fn test_validation_initial_cols() {
        assert!(GuiConfigBuilder::create_empty()
            .initial_cols(0u8)
            .build()
            .is_err_and(|err| {
                match err {
                    GuiConfigBuilderError::UninitializedField(_) => false,
                    GuiConfigBuilderError::ValidationError(msg) => {
                        msg == "Initial number of columns (0) must be greater than 0"
                    }
                }
            }));
    }

    #[test]
    fn test_validation_initial_rows() {
        assert!(GuiConfigBuilder::create_empty()
            .initial_rows(0u8)
            .build()
            .is_err_and(|err| {
                match err {
                    GuiConfigBuilderError::UninitializedField(_) => false,
                    GuiConfigBuilderError::ValidationError(msg) => {
                        msg == "Initial number of rows (0) must be greater than 0"
                    }
                }
            }));
    }

    #[test]
    fn test_validation_cell_size() {
        assert!(GuiConfigBuilder::create_empty()
            .cell_size(0.)
            .build()
            .is_err_and(|err| {
                match err {
                    GuiConfigBuilderError::UninitializedField(_) => false,
                    GuiConfigBuilderError::ValidationError(msg) => {
                        msg == "Cell size (0) must be greater than 0"
                    }
                }
            }));
    }

    #[test]
    fn test_validation_text_size() {
        assert!(GuiConfigBuilder::create_empty()
            .text_size(0.)
            .build()
            .is_err_and(|err| {
                match err {
                    GuiConfigBuilderError::UninitializedField(_) => false,
                    GuiConfigBuilderError::ValidationError(msg) => {
                        msg == "Text size (0) must be greater than 0"
                    }
                }
            }));
    }

    #[test]
    fn test_validation_col_range() {
        assert!(GuiConfigBuilder::create_empty()
            .col_range(0..1)
            .build()
            .is_err_and(|err| {
                match err {
                    GuiConfigBuilderError::UninitializedField(_) => false,
                    GuiConfigBuilderError::ValidationError(msg) => {
                        msg == "Start of column range (0) must be greater than 0"
                    }
                }
            }));

        let initial_cols = GuiConfig::default().initial_cols;
        assert!(GuiConfigBuilder::create_empty()
            .col_range(initial_cols..initial_cols)
            .build()
            .is_err_and(|err| {
                match err {
                    GuiConfigBuilderError::UninitializedField(_) => false,
                    GuiConfigBuilderError::ValidationError(msg) => msg.contains(&format!(
                        "Initial number of columns ({}) must be in the range",
                        initial_cols
                    )),
                }
            }));
    }

    #[test]
    fn test_validation_row_range() {
        assert!(GuiConfigBuilder::create_empty()
            .row_range(0..69)
            .build()
            .is_err_and(|err| {
                match err {
                    GuiConfigBuilderError::UninitializedField(_) => false,
                    GuiConfigBuilderError::ValidationError(msg) => {
                        msg == "Start of row range (0) must be greater than 0"
                    }
                }
            }));

        let initial_rows = GuiConfig::default().initial_rows;
        assert!(GuiConfigBuilder::create_empty()
            .row_range(initial_rows..initial_rows)
            .build()
            .is_err_and(|err| {
                match err {
                    GuiConfigBuilderError::UninitializedField(_) => false,
                    GuiConfigBuilderError::ValidationError(msg) => msg.contains(&format!(
                        "Initial number of rows ({}) must be in the range",
                        initial_rows
                    )),
                }
            }));
    }

    #[test]
    fn test_validation_initial_language() {
        let mut ctx_builder = JSONGetTextBuilder::new("en_UK");
        ctx_builder
            .add_json_owned("en_UK", stringify!({"title": "Tests"}))
            .unwrap();

        let initial_language = "sk_SK";
        assert!(GuiConfigBuilder::create_empty()
            .initial_language(initial_language)
            .tranlation_ctx(ctx_builder.build().unwrap())
            .build()
            .is_err_and(|err| {
                match err {
                    GuiConfigBuilderError::UninitializedField(_) => false,
                    GuiConfigBuilderError::ValidationError(msg) => {
                        msg.contains(&format!("Initial language ({initial_language}) must be available in the translation context"))
                    }
                }
            }));
    }
}
