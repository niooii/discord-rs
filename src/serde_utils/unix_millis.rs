use time::OffsetDateTime;
use serde::{de, Deserializer, Deserialize};

/// Deserialize an `OffsetDateTime` from its Unix timestamp with milliseconds
/// A workaround for the time::serde::timestamp modules because i keep getting an error regarding i128 support.
pub fn deserialize<'a, D: Deserializer<'a>>(deserializer: D) -> Result<OffsetDateTime, D::Error> {
    let value: i64 = <_>::deserialize(deserializer)?;
    OffsetDateTime::from_unix_timestamp_nanos((value * 1_000_000) as i128)
        .map_err(|err| de::Error::invalid_value(de::Unexpected::Str("no data available"), &err))
}

pub mod option {
    use time::OffsetDateTime;
use serde::{de, Deserializer, Deserialize};
    /// Deserialize an `Option<OffsetDateTime>` from its Unix timestamp with milliseconds
    pub fn deserialize<'a, D: Deserializer<'a>>(
        deserializer: D,
    ) -> Result<Option<OffsetDateTime>, D::Error> {
        Option::deserialize(deserializer)?
            .map(|value: i64| OffsetDateTime::from_unix_timestamp_nanos((value * 1_000_000) as i128))
            .transpose()
            .map_err(|err| de::Error::invalid_value(de::Unexpected::Str("no data available"), &err))
    }
}