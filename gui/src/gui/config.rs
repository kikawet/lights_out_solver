use std::{ops::Range, str::FromStr};

use derive_builder::Builder;
use json_gettext::{JSONGetText, JSONGetTextBuilder};

use serde::{de::Unexpected, Deserialize};

use crate::adapter::{Adapter, DeserializeError};

#[derive(Debug, Builder)]
#[builder(
    default,
    setter(into),
    build_fn(validate = "Self::validate"),
    pattern = "owned",
    derive(Deserialize)
)]
pub struct Config {
    pub cell_size: f32,
    pub text_size: f32,
    pub initial_rows: usize,
    pub initial_cols: usize,
    pub row_range: Range<usize>,
    pub col_range: Range<usize>,
    pub exit_shortcut: Adapter<'static, KeyboardShortcut, egui::KeyboardShortcut>,
    pub reset_shortcut: Adapter<'static, KeyboardShortcut, egui::KeyboardShortcut>,
    pub solve_shortcut: Adapter<'static, KeyboardShortcut, egui::KeyboardShortcut>,
    #[builder_field_attr(serde(borrow))]
    pub tranlation_ctx: Adapter<'static, TranslationCtx, JSONGetText<'static>>,
    pub initial_language: String,
    pub log_level: Adapter<'static, LevelFilter, log::LevelFilter>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            cell_size: 50.,
            text_size: 25.,
            initial_rows: 5,
            initial_cols: 5,
            row_range: 1..10,
            col_range: 1..10,
            exit_shortcut: egui::KeyboardShortcut::new(egui::Modifiers::NONE, egui::Key::Q).into(),
            reset_shortcut: egui::KeyboardShortcut::new(egui::Modifiers::SHIFT, egui::Key::R)
                .into(),
            solve_shortcut: egui::KeyboardShortcut::new(egui::Modifiers::SHIFT, egui::Key::S)
                .into(),
            tranlation_ctx: static_json_gettext_build!(
                "en_UK";
                "en_UK" => "locales/en_UK.json",
                "es_ES" => "locales/es_ES.json"
            )
            .unwrap()
            .into(),
            initial_language: "en_UK".to_owned(),
            log_level: log::LevelFilter::Debug.into(),
        }
    }
}

impl PartialEq for Config {
    fn eq(&self, other: &Self) -> bool {
        self.cell_size == other.cell_size
            && self.text_size == other.text_size
            && self.initial_rows == other.initial_rows
            && self.initial_cols == other.initial_cols
            && self.row_range == other.row_range
            && self.col_range == other.col_range
            && self.exit_shortcut == *other.exit_shortcut
            && self.reset_shortcut == *other.reset_shortcut
            && self.solve_shortcut == *other.solve_shortcut
            // && self.tranlation_ctx.get_keys() == other.tranlation_ctx.get_keys()
            && self.tranlation_ctx.get_default_key() == other.tranlation_ctx.get_default_key()
            && self.initial_language == other.initial_language
    }
}

impl ConfigBuilder {
    fn validate(&self) -> Result<(), ConfigBuilderError> {
        if let Some(0) = self.initial_cols {
            return Err(ConfigBuilderError::ValidationError(format!(
                "Initial number of columns ({}) must be greater than 0",
                self.initial_cols.as_ref().unwrap()
            )));
        }

        if let Some(0) = self.initial_rows {
            return Err(ConfigBuilderError::ValidationError(format!(
                "Initial number of rows ({}) must be greater than 0",
                self.initial_rows.as_ref().unwrap()
            )));
        }

        if self.cell_size.is_some_and(|size| size <= 0.) {
            return Err(ConfigBuilderError::ValidationError(format!(
                "Cell size ({}) must be greater than 0",
                self.cell_size.unwrap()
            )));
        }

        if self.text_size.is_some_and(|size| size <= 0.) {
            return Err(ConfigBuilderError::ValidationError(format!(
                "Text size ({}) must be greater than 0",
                self.text_size.unwrap()
            )));
        }

        if self
            .row_range
            .as_ref()
            .is_some_and(|range| range.start == 0)
        {
            return Err(ConfigBuilderError::ValidationError(format!(
                "Start of row range ({}) must be greater than 0",
                self.row_range.as_ref().unwrap().start
            )));
        }

        if self
            .col_range
            .as_ref()
            .is_some_and(|range| range.start == 0)
        {
            return Err(ConfigBuilderError::ValidationError(format!(
                "Start of column range ({}) must be greater than 0",
                self.col_range.as_ref().unwrap().start
            )));
        }

        let default_config = Config::default();
        let initial_rows = self.initial_rows.unwrap_or(default_config.initial_rows);
        let initial_cols = self.initial_cols.unwrap_or(default_config.initial_cols);
        let initial_language = self
            .initial_language
            .as_ref()
            .unwrap_or(&default_config.initial_language);

        if self
            .row_range
            .as_ref()
            .is_some_and(|range| !range.contains(&initial_rows))
        {
            return Err(ConfigBuilderError::ValidationError(format!(
                "Initial number of rows ({initial_rows}) must be in the range {:?}",
                self.row_range.as_ref().unwrap()
            )));
        }

        if self
            .col_range
            .as_ref()
            .is_some_and(|range| !range.contains(&initial_cols))
        {
            return Err(ConfigBuilderError::ValidationError(format!(
                "Initial number of columns ({initial_cols}) must be in the range {:?}",
                self.col_range.as_ref().unwrap()
            )));
        }

        if self
            .tranlation_ctx
            .as_ref()
            .is_some_and(|ctx| !ctx.contains_key(initial_language))
        {
            return Err(ConfigBuilderError::ValidationError(format!(
                "Initial language ({initial_language}) must be available in the translation context {:?}",
                self.tranlation_ctx.as_ref().unwrap().get_keys()
            )));
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct KeyboardShortcut {
    pub modifiers: Option<Vec<String>>,
    pub key: egui::Key,
}

impl TryInto<egui::KeyboardShortcut> for KeyboardShortcut {
    type Error = DeserializeError<'static>;

    fn try_into(self) -> Result<egui::KeyboardShortcut, Self::Error> {
        let modifiers = self
            .modifiers
            .unwrap_or_default()
            .into_iter()
            .map(|modifier| -> Result<egui::Modifiers, Self::Error> {
                match modifier {
                    name if egui::ModifierNames::NAMES.alt.to_uppercase()
                        == name.to_uppercase() =>
                    {
                        Ok(egui::Modifiers::ALT)
                    }
                    name if egui::ModifierNames::NAMES.ctrl.to_uppercase()
                        == name.to_uppercase() =>
                    {
                        Ok(egui::Modifiers::CTRL)
                    }
                    name if egui::ModifierNames::NAMES.shift.to_uppercase()
                        == name.to_uppercase() =>
                    {
                        Ok(egui::Modifiers::SHIFT)
                    }
                    name if egui::ModifierNames::NAMES.mac_cmd.to_uppercase()
                        == name.to_uppercase() =>
                    {
                        Ok(egui::Modifiers::MAC_CMD)
                    }
                    // name if egui::ModifierNames::NAMES.mac_alt == name => Ok(egui::Modifiers::),//Not supported
                    name => Err(DeserializeError::InvalidValue(
                        Unexpected::Str(Box::leak(name.into_boxed_str())),
                        "[ALT, CTRL, SHIFT, CMD]", //TODO: replace with list of valid modifiers
                    )),
                }
            })
            .collect::<Result<Vec<_>, _>>();

        Ok(egui::KeyboardShortcut::new(
            modifiers?
                .into_iter()
                .reduce(|acc, cur| acc | cur)
                .unwrap_or(egui::Modifiers::NONE),
            self.key,
        ))
    }
}

#[derive(Debug, Deserialize)]
pub struct TranslationCtx {
    default_key: String,
    translations: Vec<TranslationCtxItem>,
}

#[derive(Debug, Deserialize)]
struct TranslationCtxItem {
    key: String,
    path: String,
}

impl TranslationCtx {
    fn map_error<'a>(error: json_gettext::JSONGetTextBuildError) -> DeserializeError<'a> {
        use json_gettext::{JSONGetTextBuildError, Key};
        match error {
            JSONGetTextBuildError::DefaultKeyNotFound => {
                DeserializeError::MissingField("defaut_key")
            }
            JSONGetTextBuildError::TextInKeyNotInDefaultKey { key, text } => {
                DeserializeError::Custom(format!("text \"{text}\" not found in \"{key}\" "))
            }
            JSONGetTextBuildError::DuplicatedKey(Key(key)) => {
                DeserializeError::Custom(format!("translation \"{key}\" is already defined"))
            }
            JSONGetTextBuildError::IOError(error) => DeserializeError::Custom(format!("{error}")),
            JSONGetTextBuildError::SerdeJSONError(error) => {
                DeserializeError::Custom(format!("{error}"))
            }
        }
    }
}

impl<'a> TryInto<JSONGetText<'a>> for TranslationCtx {
    type Error = DeserializeError<'a>;

    fn try_into(self) -> Result<JSONGetText<'a>, Self::Error> {
        let mut builder = JSONGetTextBuilder::new(self.default_key);

        let builder = self
            .translations
            .into_iter()
            .try_fold(&mut builder, |builder, translation| {
                builder.add_json_file(translation.key, translation.path)
            })
            .map_err(TranslationCtx::map_error)?;

        builder
            .to_owned()
            .build()
            .map_err(TranslationCtx::map_error)
    }
}

#[derive(Debug, Deserialize)]
pub struct LevelFilter(String);

impl TryInto<log::LevelFilter> for LevelFilter {
    type Error = DeserializeError<'static>;

    fn try_into(self) -> Result<log::LevelFilter, Self::Error> {
        log::LevelFilter::from_str(self.0.as_str())
            .map_err(|err| DeserializeError::Custom(format!("{err}")))
    }
}

#[cfg(test)]
mod config_tests {
    use json_gettext::JSONGetTextBuilder;

    use crate::gui::config::{Config, ConfigBuilder, ConfigBuilderError};

    #[test]
    fn verify_default() {
        assert_eq!(
            Config::default(),
            ConfigBuilder::create_empty().build().unwrap()
        );

        assert_eq!(
            ConfigBuilder::default().build().unwrap(),
            ConfigBuilder::create_empty().build().unwrap()
        );
    }

    #[test]
    fn test_validation_initial_cols() {
        assert!(ConfigBuilder::create_empty()
            .initial_cols(0u8)
            .build()
            .is_err_and(|err| {
                match err {
                    ConfigBuilderError::UninitializedField(_) => false,
                    ConfigBuilderError::ValidationError(msg) => {
                        msg == "Initial number of columns (0) must be greater than 0"
                    }
                }
            }));
    }

    #[test]
    fn test_validation_initial_rows() {
        assert!(ConfigBuilder::create_empty()
            .initial_rows(0u8)
            .build()
            .is_err_and(|err| {
                match err {
                    ConfigBuilderError::UninitializedField(_) => false,
                    ConfigBuilderError::ValidationError(msg) => {
                        msg == "Initial number of rows (0) must be greater than 0"
                    }
                }
            }));
    }

    #[test]
    fn test_validation_cell_size() {
        assert!(ConfigBuilder::create_empty()
            .cell_size(0.)
            .build()
            .is_err_and(|err| {
                match err {
                    ConfigBuilderError::UninitializedField(_) => false,
                    ConfigBuilderError::ValidationError(msg) => {
                        msg == "Cell size (0) must be greater than 0"
                    }
                }
            }));
    }

    #[test]
    fn test_validation_text_size() {
        assert!(ConfigBuilder::create_empty()
            .text_size(0.)
            .build()
            .is_err_and(|err| {
                match err {
                    ConfigBuilderError::UninitializedField(_) => false,
                    ConfigBuilderError::ValidationError(msg) => {
                        msg == "Text size (0) must be greater than 0"
                    }
                }
            }));
    }

    #[test]
    fn test_validation_col_range() {
        assert!(ConfigBuilder::create_empty()
            .col_range(0..1)
            .build()
            .is_err_and(|err| {
                match err {
                    ConfigBuilderError::UninitializedField(_) => false,
                    ConfigBuilderError::ValidationError(msg) => {
                        msg == "Start of column range (0) must be greater than 0"
                    }
                }
            }));

        let initial_cols = Config::default().initial_cols;
        assert!(ConfigBuilder::create_empty()
            .col_range(initial_cols..initial_cols)
            .build()
            .is_err_and(|err| {
                match err {
                    ConfigBuilderError::UninitializedField(_) => false,
                    ConfigBuilderError::ValidationError(msg) => msg.contains(&format!(
                        "Initial number of columns ({initial_cols}) must be in the range"
                    )),
                }
            }));
    }

    #[test]
    fn test_validation_row_range() {
        assert!(ConfigBuilder::create_empty()
            .row_range(0..69)
            .build()
            .is_err_and(|err| {
                match err {
                    ConfigBuilderError::UninitializedField(_) => false,
                    ConfigBuilderError::ValidationError(msg) => {
                        msg == "Start of row range (0) must be greater than 0"
                    }
                }
            }));

        let initial_rows = Config::default().initial_rows;
        assert!(ConfigBuilder::create_empty()
            .row_range(initial_rows..initial_rows)
            .build()
            .is_err_and(|err| {
                match err {
                    ConfigBuilderError::UninitializedField(_) => false,
                    ConfigBuilderError::ValidationError(msg) => msg.contains(&format!(
                        "Initial number of rows ({initial_rows}) must be in the range"
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
        assert!(ConfigBuilder::create_empty()
            .initial_language(initial_language)
            .tranlation_ctx(ctx_builder.build().unwrap())
            .build()
            .is_err_and(|err| {
                match err {
                    ConfigBuilderError::UninitializedField(_) => false,
                    ConfigBuilderError::ValidationError(msg) => {
                        msg.contains(&format!("Initial language ({initial_language}) must be available in the translation context"))
                    }
                }
            }));
    }

    #[test]
    fn test_deserialize_empty_config() {
        let config_builder: ConfigBuilder = serde_json::from_str("{}").unwrap();
        assert_eq!(Config::default(), config_builder.build().unwrap());
    }

    #[test]
    fn test_deserialize_full_config() {
        let mut ctx_builder = JSONGetTextBuilder::new("en_UK");
        ctx_builder
            .add_json_file("en_UK", "locales/en_UK.json")
            .unwrap();

        let config_builder: ConfigBuilder = serde_json::from_str(stringify!({
            "cell_size": 69.0,
            "text_size": 420.0,
            "initial_rows": 15,
            "initial_cols": 17,
            "row_range": { "start": 10, "end": 20 },
            "col_range": { "start": 11, "end": 21 },
            "exit_shortcut": { "modifiers": ["ALT"], "key": "A" },
            "solve_shortcut": { "modifiers": ["SHIFT"], "key": "T" },
            "reset_shortcut": { "modifiers": ["CTRL"], "key": "X" },
            "tranlation_ctx": { "default_key": "en_UK", "translations": [{"key": "en_UK", "path": "locales/en_UK.json"}] },
            "initial_language": "en_UK",
            "log_level": "WARN"
        }))
        .unwrap();

        assert_eq!(
            Config {
                cell_size: 69.,
                text_size: 420.,
                initial_rows: 15,
                initial_cols: 17,
                row_range: 10..20,
                col_range: 11..21,
                exit_shortcut: egui::KeyboardShortcut::new(egui::Modifiers::ALT, egui::Key::A)
                    .into(),
                reset_shortcut: egui::KeyboardShortcut::new(egui::Modifiers::CTRL, egui::Key::X)
                    .into(),
                solve_shortcut: egui::KeyboardShortcut::new(egui::Modifiers::SHIFT, egui::Key::T)
                    .into(),
                tranlation_ctx: ctx_builder.build().unwrap().into(),
                initial_language: "en_UK".to_owned(),
                log_level: log::LevelFilter::Warn.into()
            },
            config_builder.build().unwrap()
        );
    }
}
