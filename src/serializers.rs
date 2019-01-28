use serde::de::{self, Deserialize, Deserializer};
use serde::ser::Serializer;
use std::fmt::Display;
use std::str::FromStr;

pub(crate) fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(de::Error::custom)
}

pub(crate) fn join_ids<S>(maybe_ids: &Option<Vec<i64>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&match maybe_ids {
        Some(ids) => ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(","),
        None => "".to_owned(),
    })
}
