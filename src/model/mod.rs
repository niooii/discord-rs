pub mod channel;
pub mod guild;
pub mod message;
pub mod permissions;
pub mod user;
pub mod voice;

use std::fmt::{Debug, Display};

/// Will be implemented for all discord objects that have a snowflake id.
pub trait ID {
    fn id(&self) -> &Snowflake;
}

use serde::{Serialize, Deserialize};

// Maybe add more someday
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(transparent)]
pub struct Snowflake {
    pub(crate) snowflake_str: String
}

impl Debug for Snowflake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Snowflake({})", self.snowflake_str))
    }
}

impl Display for Snowflake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.snowflake_str)
    }
}

impl Snowflake {
    pub fn new(string: &str) -> Self {
        Self {
            snowflake_str: string.to_string()
        }
    }
}

