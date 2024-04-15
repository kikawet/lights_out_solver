use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use serde::{
    de::{self, Unexpected},
    Deserialize,
};

#[derive(Debug)]
pub struct AdapterDeserializer<'a, F, I>
where
    F: TryInto<I, Error = DeserializeError<'a>> + Deserialize<'a>,
{
    phantom: PhantomData<F>,
    inner: I,
}

pub enum DeserializeError<'a> {
    MissingField(&'static str),
    InvalidValue(Unexpected<'a>, &'a str),
    Custom(String),
}

impl<'de, F, I> Deserialize<'de> for AdapterDeserializer<'de, F, I>
where
    F: TryInto<I, Error = DeserializeError<'de>> + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        fn map_deserialize_error<'de, V>(error: DeserializeError<'de>) -> V::Error
        where
            V: serde::Deserializer<'de>,
        {
            match error {
                DeserializeError::InvalidValue(unexpected, expected) => {
                    de::Error::invalid_value(unexpected, &expected)
                }
                DeserializeError::MissingField(field) => de::Error::missing_field(field),
                DeserializeError::Custom(msg) => de::Error::custom(msg),
            }
        }

        let from = F::deserialize(deserializer)?;
        let into: I = from.try_into().map_err(map_deserialize_error::<D>)?;

        Ok(AdapterDeserializer {
            inner: into,
            phantom: PhantomData,
        })
    }
}

impl<'a, F, I> PartialEq<I> for AdapterDeserializer<'a, F, I>
where
    F: TryInto<I, Error = DeserializeError<'a>> + Deserialize<'a>,
    I: PartialEq,
{
    fn eq(&self, other: &I) -> bool {
        self.inner == *other
    }
}

impl<'a, F, I> Deref for AdapterDeserializer<'a, F, I>
where
    F: TryInto<I, Error = DeserializeError<'a>> + Deserialize<'a>,
{
    type Target = I;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a, F, I> DerefMut for AdapterDeserializer<'a, F, I>
where
    F: TryInto<I, Error = DeserializeError<'a>> + Deserialize<'a>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<'a, F, I> From<I> for AdapterDeserializer<'a, F, I>
where
    F: TryInto<I, Error = DeserializeError<'a>> + Deserialize<'a>,
{
    fn from(value: I) -> Self {
        Self {
            inner: value,
            phantom: PhantomData,
        }
    }
}

#[cfg(test)]
mod adapter_tests {
    use serde::Deserialize;

    use super::{AdapterDeserializer, DeserializeError};

    #[derive(Debug, PartialEq)]
    struct NonDeserialize(u8);

    impl NonDeserialize {
        fn get_content(&self) -> u8 {
            self.0
        }

        fn mut_content(&mut self) -> &mut u8 {
            &mut self.0
        }
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct Deserializable(u8);

    impl TryInto<NonDeserialize> for Deserializable {
        type Error = DeserializeError<'static>;

        fn try_into(self) -> Result<NonDeserialize, Self::Error> {
            Ok(NonDeserialize(self.0))
        }
    }

    #[derive(Debug, Deserialize)]
    struct TestAdapter {
        #[serde(borrow)]
        non: AdapterDeserializer<'static, Deserializable, NonDeserialize>,
        yes: Deserializable,
    }

    #[test]
    fn test_deserialize_adapter() {
        let adapter: TestAdapter =
            serde_json::from_str(stringify!({"yes": 69, "non": 70})).unwrap();

        assert_eq!(adapter.yes, Deserializable(69));
        assert_eq!(adapter.non, NonDeserialize(70));
    }

    #[test]
    fn test_deref_adapter() {
        let content = 70;
        let adapter: AdapterDeserializer<Deserializable, _> = NonDeserialize(content).into();

        assert_eq!(adapter.get_content(), content);
    }

    #[test]
    fn test_derefmut_adapter() {
        let content = 69;
        let mut adapter: AdapterDeserializer<Deserializable, _> = NonDeserialize(content).into();
        assert_eq!(adapter.get_content(), content);
        *adapter.mut_content() = 1;
        assert_eq!(adapter.get_content(), 1);
    }
}
