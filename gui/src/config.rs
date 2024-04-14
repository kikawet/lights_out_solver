use std::ops::Range;

use derive_builder::Builder;
use json_gettext::{JSONGetText, JSONGetTextBuilder};
use serde::{
    de::{self, MapAccess, Unexpected, Visitor},
    Deserialize,
};

#[derive(Debug, Builder)]
#[builder(
    default,
    setter(into),
    build_fn(validate = "Self::validate"),
    pattern = "owned"
)]
pub struct GuiConfig<'a> {
    pub cell_size: f32,
    pub text_size: f32,
    pub initial_rows: usize,
    pub initial_cols: usize,
    pub row_range: Range<usize>,
    pub col_range: Range<usize>,
    pub exit_shortcut: egui::KeyboardShortcut,
    pub reset_shortcut: egui::KeyboardShortcut,
    pub solve_shortcut: egui::KeyboardShortcut,
    pub tranlation_ctx: JSONGetText<'a>,
    pub initial_language: &'a str,
}

impl<'a> Default for GuiConfig<'a> {
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

impl<'a> PartialEq for GuiConfig<'a> {
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

impl<'a> GuiConfigBuilder<'a> {
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

impl<'de> Deserialize<'de> for GuiConfigBuilder<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        enum DeserializeError<'a> {
            MissingField(&'static str),
            InvalidValue(Unexpected<'a>, &'a str),
            Custom(String),
        }

        #[derive(Debug, Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum GuiConfigField {
            CellSize,
            TextSize,
            InitialRows,
            InitialCols,
            RowRange,
            ColRange,
            ExitShortcut,
            ResetShortcut,
            SolveShortcut,
            TranlationCtx,
            InitialLanguage,
        }

        #[derive(Debug, Deserialize)]
        struct KeyboardShortcut<'a> {
            #[serde(borrow)]
            pub modifiers: Vec<&'a str>,
            pub key: egui::Key,
        }

        impl<'a> TryInto<egui::KeyboardShortcut> for KeyboardShortcut<'a> {
            type Error = DeserializeError<'a>;

            fn try_into(self) -> Result<egui::KeyboardShortcut, Self::Error> {
                let modifiers = self
                    .modifiers
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
                                Unexpected::Str(name),
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
        struct TranslationCtx<'a> {
            #[serde(borrow)]
            default_key: &'a str,
            translations: Vec<TranslationCtxItem<'a>>,
        }

        #[derive(Debug, Deserialize)]
        struct TranslationCtxItem<'a> {
            #[serde(borrow)]
            key: &'a str,
            path: &'a str,
        }

        impl<'a> TranslationCtx<'a> {
            fn map_error(error: json_gettext::JSONGetTextBuildError) -> DeserializeError<'a> {
                use json_gettext::{JSONGetTextBuildError, Key};
                match error {
                    JSONGetTextBuildError::DefaultKeyNotFound => {
                        DeserializeError::MissingField("defaut_key")
                    }
                    JSONGetTextBuildError::TextInKeyNotInDefaultKey { key, text } => {
                        DeserializeError::Custom(format!("text \"{text}\" not found in \"{key}\" "))
                    }
                    JSONGetTextBuildError::DuplicatedKey(Key(key)) => DeserializeError::Custom(
                        format!("translation \"{key}\" was already defined"),
                    ),
                    JSONGetTextBuildError::IOError(error) => {
                        DeserializeError::Custom(format!("{}", error))
                    }
                    JSONGetTextBuildError::SerdeJSONError(error) => {
                        DeserializeError::Custom(format!("{}", error))
                    }
                }
            }
        }

        impl<'a> TryInto<JSONGetText<'a>> for TranslationCtx<'a> {
            type Error = DeserializeError<'a>;

            fn try_into(self) -> Result<JSONGetText<'a>, Self::Error> {
                let mut builder = JSONGetTextBuilder::new(self.default_key);

                let builder = self
                    .translations
                    .into_iter()
                    .fold(Ok(&mut builder), |builder: Result<_, _>, translation| {
                        builder.and_then(|builder: &mut JSONGetTextBuilder| {
                            builder.add_json_file(translation.key, translation.path)
                        })
                    })
                    .map_err(TranslationCtx::map_error)?;

                builder
                    .to_owned()
                    .build()
                    .map_err(TranslationCtx::map_error)
            }
        }

        struct GuiConfigBuilderVisitor;

        impl<'de> Visitor<'de> for GuiConfigBuilderVisitor {
            type Value = GuiConfigBuilder<'de>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct GuiConfigBuilder")
            }

            fn visit_map<V>(self, mut map: V) -> Result<GuiConfigBuilder<'de>, V::Error>
            where
                V: MapAccess<'de>,
            {
                fn map_deserialize_error<'de, V>(error: DeserializeError<'de>) -> V::Error
                where
                    V: MapAccess<'de>,
                {
                    match error {
                        DeserializeError::InvalidValue(unexpected, expected) => {
                            de::Error::invalid_value(unexpected, &expected)
                        }
                        DeserializeError::MissingField(field) => de::Error::missing_field(field),
                        DeserializeError::Custom(msg) => de::Error::custom(msg),
                    }
                }

                let mut builder = GuiConfigBuilder::create_empty();
                while let Some(key) = map.next_key::<GuiConfigField>()? {
                    match key {
                        GuiConfigField::CellSize => {
                            if builder.cell_size.is_some() {
                                return Err(de::Error::duplicate_field("cell_size"));
                            }

                            builder = builder.cell_size(map.next_value::<f32>()?);
                        }
                        GuiConfigField::TextSize => {
                            if builder.text_size.is_some() {
                                return Err(de::Error::duplicate_field("text_size"));
                            }

                            builder = builder.text_size(map.next_value::<f32>()?);
                        }
                        GuiConfigField::InitialRows => {
                            if builder.initial_rows.is_some() {
                                return Err(de::Error::duplicate_field("initial_rows"));
                            }

                            builder = builder.initial_rows(map.next_value::<usize>()?);
                        }
                        GuiConfigField::InitialCols => {
                            if builder.initial_cols.is_some() {
                                return Err(de::Error::duplicate_field("initial_cols"));
                            }

                            builder = builder.initial_cols(map.next_value::<usize>()?);
                        }
                        GuiConfigField::RowRange => {
                            if builder.row_range.is_some() {
                                return Err(de::Error::duplicate_field("row_range"));
                            }

                            builder = builder.row_range(map.next_value::<Range<usize>>()?);
                        }
                        GuiConfigField::ColRange => {
                            if builder.col_range.is_some() {
                                return Err(de::Error::duplicate_field("col_range"));
                            }

                            builder = builder.col_range(map.next_value::<Range<usize>>()?);
                        }
                        GuiConfigField::ExitShortcut => {
                            if builder.exit_shortcut.is_some() {
                                return Err(de::Error::duplicate_field("exit_shortcut"));
                            }

                            let shortcut: Result<egui::KeyboardShortcut, _> =
                                map.next_value::<KeyboardShortcut>()?.try_into();

                            match shortcut {
                                Ok(shortcut) => builder = builder.exit_shortcut(shortcut),
                                Err(err) => return Err(map_deserialize_error::<V>(err)),
                            };
                        }
                        GuiConfigField::ResetShortcut => {
                            if builder.reset_shortcut.is_some() {
                                return Err(de::Error::duplicate_field("reset_shortcut"));
                            }

                            let shortcut: Result<egui::KeyboardShortcut, _> =
                                map.next_value::<KeyboardShortcut>()?.try_into();

                            match shortcut {
                                Ok(shortcut) => builder = builder.reset_shortcut(shortcut),
                                Err(err) => return Err(map_deserialize_error::<V>(err)),
                            };
                        }
                        GuiConfigField::SolveShortcut => {
                            if builder.solve_shortcut.is_some() {
                                return Err(de::Error::duplicate_field("solve_shortcut"));
                            }

                            let shortcut: Result<egui::KeyboardShortcut, _> =
                                map.next_value::<KeyboardShortcut>()?.try_into();

                            match shortcut {
                                Ok(shortcut) => builder = builder.solve_shortcut(shortcut),
                                Err(err) => return Err(map_deserialize_error::<V>(err)),
                            };
                        }
                        GuiConfigField::TranlationCtx => {
                            if builder.tranlation_ctx.is_some() {
                                return Err(de::Error::duplicate_field("tranlation_ctx"));
                            }

                            let ctx: Result<JSONGetText, _> =
                                map.next_value::<TranslationCtx>()?.try_into();

                            match ctx {
                                Ok(ctx) => builder = builder.tranlation_ctx(ctx),
                                Err(err) => return Err(map_deserialize_error::<V>(err)),
                            };
                        }
                        GuiConfigField::InitialLanguage => {
                            if builder.initial_language.is_some() {
                                return Err(de::Error::duplicate_field("initial_language"));
                            }

                            builder = builder.initial_language(map.next_value::<&'de str>()?);
                        }
                    }
                }

                Ok(builder)
            }
        }

        const FIELDS: &[&str] = &[
            "cell_size",
            "text_size",
            "initial_rows",
            "initial_cols",
            "row_range",
            "col_range",
            "exit_shortcut",
            "reset_shortcut",
            "solve_shortcut",
            "tranlation_ctx",
            "initial_language",
        ];

        deserializer.deserialize_struct("GuiConfigBuilder", FIELDS, GuiConfigBuilderVisitor)
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

    #[test]
    fn test_deserialize_empty_config() {
        let config_builder: GuiConfigBuilder = serde_json::from_str("{}").unwrap();
        assert_eq!(GuiConfig::default(), config_builder.build().unwrap())
    }

    #[test]
    fn test_deserialize_full_config() {
        let mut ctx_builder = JSONGetTextBuilder::new("en_UK");
        ctx_builder
            .add_json_file("en_UK", "locales/en_UK.json")
            .unwrap();

        let config_builder: GuiConfigBuilder = serde_json::from_str(stringify!({
            "cell_size": 69.0,
            "text_size": 420.0,
            "initial_rows": 15,
            "initial_cols": 17,
            "row_range": { "start": 10, "end": 20 },
            "col_range": { "start": 11, "end": 21 },
            "exit_shortcut": { "modifiers": ["Alt"], "key": "A" },
            "solve_shortcut": { "modifiers": ["Shift"], "key": "T" },
            "reset_shortcut": { "modifiers": ["Ctrl"], "key": "X" },
            "tranlation_ctx": { "default_key": "en_UK", "translations": [{"key": "en_UK", "path": "locales/en_UK.json"}] },
            "initial_language": "en_UK"
        }))
        .unwrap();

        assert_eq!(
            GuiConfig {
                cell_size: 69.,
                text_size: 420.,
                initial_rows: 15,
                initial_cols: 17,
                row_range: 10..20,
                col_range: 11..21,
                exit_shortcut: egui::KeyboardShortcut::new(egui::Modifiers::ALT, egui::Key::A),
                reset_shortcut: egui::KeyboardShortcut::new(egui::Modifiers::CTRL, egui::Key::X),
                solve_shortcut: egui::KeyboardShortcut::new(egui::Modifiers::SHIFT, egui::Key::T),
                tranlation_ctx: ctx_builder.build().unwrap(),
                initial_language: "en_UK"
            },
            config_builder.build().unwrap()
        )
    }
}
