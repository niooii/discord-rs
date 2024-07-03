use std::collections::HashMap;

use serde::Deserialize;
use time::OffsetDateTime;

use crate::UserData;

#[derive(Debug, Deserialize)]
pub struct GuildMemberData {
    pub avatar: Option<String>,
    #[serde(default, with = "time::serde::iso8601::option")]
    pub communication_disabled_until: Option<OffsetDateTime>,
    pub deaf: bool,
    pub flags: u64,
    #[serde(with = "time::serde::iso8601")]
    pub joined_at: OffsetDateTime,
    pub mute: bool,
    pub nick: Option<String>,
    pub pending: bool,
    pub premium_since: Option<String>,
    pub roles: Vec<String>,
}

/// Pretty sure users don't really need to know about this...
#[derive(Debug, Deserialize)]
pub struct InteractionMetadata {
    // pub authorizing_integration_owners: HashMap<String, String>,
    // pub id: String,
    // pub name: String,
    // pub r#type: u64,
    // pub user: UserData,
    // pub user_id: String,
}

#[derive(Debug, Deserialize)]
pub struct InteractionData {
    pub id: String,
    pub member: GuildMemberData,
    pub name: String,
    pub r#type: u64,
    pub user: UserData,
}