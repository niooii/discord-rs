use serde::{de, Deserializer, Deserialize};

pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Deserialize<'de>,
{
    let v: Vec<T> = Deserialize::deserialize(deserializer)?;
    v.into_iter().next().ok_or_else(|| serde::de::Error::custom("Expected an array with elements."))
}