use std::option::Option;

use serde::{de, Deserialize, Deserializer};

///
/// # Usage
///
/// In the case you have an external struct you need to serialize/deserialize,
/// and doesn't implement the trait you can use this method to deserialize from a know struct.
///
/// Is like using `#[serde(from = "FromType")]` but for fields.
///
/// It is also really useful if you don't like how a struct is deserialized.
/// ```
#[cfg(test)] // The plan was to use this but since config requires optionals ...
fn generic_deserializer<'de, D, F, I>(deserializer: D) -> Result<I, D::Error>
where
    D: Deserializer<'de>,
    F: TryInto<I, Error: de::Error> + Deserialize<'de>,
{
    let from = F::deserialize(deserializer)?;
    let into: I = from.try_into().map_err(de::Error::custom)?;

    Ok(into)
}

pub fn option_deserializer<'de, D, F, I>(deserializer: D) -> Result<Option<I>, D::Error>
where
    D: Deserializer<'de>,
    F: TryInto<I, Error: de::Error> + Deserialize<'de>,
{
    if let Some(from) = Option::<F>::deserialize(deserializer)? {
        let into: I = from.try_into().map_err(de::Error::custom)?;
        Ok(Some(into))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod deserializer_tests {
    use serde::{de, Deserialize};

    use super::generic_deserializer;

    #[derive(Debug, PartialEq)]
    struct NonDeserialize(u8);

    #[derive(Debug, Deserialize, PartialEq)]
    struct Deserializable(u8);

    impl TryInto<NonDeserialize> for Deserializable {
        type Error = de::value::Error;

        fn try_into(self) -> Result<NonDeserialize, Self::Error> {
            Ok(NonDeserialize(self.0))
        }
    }

    #[derive(Debug, Deserialize)]
    struct TestAdapter {
        #[serde(deserialize_with = "generic_deserializer::<_, Deserializable, NonDeserialize>")]
        non: NonDeserialize,
        yes: Deserializable,
    }

    #[test]
    fn test_deserialize_adapter() {
        let adapter: TestAdapter =
            serde_json::from_str(stringify!({"yes": 69, "non": 70})).unwrap();

        assert_eq!(adapter.yes, Deserializable(69));
        assert_eq!(adapter.non, NonDeserialize(70));
    }
}
