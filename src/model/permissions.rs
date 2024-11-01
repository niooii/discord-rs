use serde::Deserialize;
use serde_repr::Deserialize_repr;

use super::Snowflake;


// https://discord.com/developers/docs/topics/permissions
pub mod permission_bit_flag {
    pub const CREATE_INSTANT_INVITE: u128 = 1 << 0;
    pub const KICK_MEMBERS: u128 = 1 << 1;
    pub const BAN_MEMBERS: u128 = 1 << 2;
    pub const ADMINISTRATOR: u128 = 1 << 3;
    pub const MANAGE_CHANNELS: u128 = 1 << 4;
    pub const MANAGE_GUILD: u128 = 1 << 5;
    pub const ADD_REACTIONS: u128 = 1 << 6;
    pub const VIEW_AUDIT_LOG: u128 = 1 << 7;
    pub const PRIORITY_SPEAKER: u128 = 1 << 8;
    pub const STREAM: u128 = 1 << 9;
    pub const VIEW_CHANNEL: u128 = 1 << 10;
    pub const SEND_MESSAGES: u128 = 1 << 11;
    pub const SEND_TTS_MESSAGES: u128 = 1 << 12;
    pub const MANAGE_MESSAGES: u128 = 1 << 13;
    pub const EMBED_LINKS: u128 = 1 << 14;
    pub const ATTACH_FILES: u128 = 1 << 15;
    pub const READ_MESSAGE_HISTORY: u128 = 1 << 16;
    pub const MENTION_EVERYONE: u128 = 1 << 17;
    pub const USE_EXTERNAL_EMOJIS: u128 = 1 << 18;
    pub const VIEW_GUILD_INSIGHTS: u128 = 1 << 19;
    pub const CONNECT: u128 = 1 << 20;
    pub const SPEAK: u128 = 1 << 21;
    pub const MUTE_MEMBERS: u128 = 1 << 22;
    pub const DEAFEN_MEMBERS: u128 = 1 << 23;
    pub const MOVE_MEMBERS: u128 = 1 << 24;
    pub const USE_VAD: u128 = 1 << 25;
    pub const CHANGE_NICKNAME: u128 = 1 << 26;
    pub const MANAGE_NICKNAMES: u128 = 1 << 27;
    pub const MANAGE_ROLES: u128 = 1 << 28;
    pub const MANAGE_WEBHOOKS: u128 = 1 << 29;
    pub const MANAGE_GUILD_EXPRESSIONS: u128 = 1 << 30;
    pub const USE_APPLICATION_COMMANDS: u128 = 1 << 31;
    pub const REQUEST_TO_SPEAK: u128 = 1 << 32;
    pub const MANAGE_EVENTS: u128 = 1 << 33;
    pub const MANAGE_THREADS: u128 = 1 << 34;
    pub const CREATE_PUBLIC_THREADS: u128 = 1 << 35;
    pub const CREATE_PRIVATE_THREADS: u128 = 1 << 36;
    pub const USE_EXTERNAL_STICKERS: u128 = 1 << 37;
    pub const SEND_MESSAGES_IN_THREADS: u128 = 1 << 38;
    pub const USE_EMBEDDED_ACTIVITIES: u128 = 1 << 39;
    pub const MODERATE_MEMBERS: u128 = 1 << 40;
    pub const VIEW_CREATOR_MONETIZATION_ANALYTICS: u128 = 1 << 41;
    pub const USE_SOUNDBOARD: u128 = 1 << 42;
    pub const CREATE_GUILD_EXPRESSIONS: u128 = 1 << 43;
    pub const CREATE_EVENTS: u128 = 1 << 44;
    pub const USE_EXTERNAL_SOUNDS: u128 = 1 << 45;
    pub const SEND_VOICE_MESSAGES: u128 = 1 << 46;
    pub const SEND_POLLS: u128 = 1 << 49;
    pub const USE_EXTERNAL_APPS: u128 = 1 << 50;
}


pub struct Permissions {
    bit_field: u128
}

impl Permissions {
    pub fn has_permission(&self, permission_bit_flag: u128) -> bool {
        self.bit_field & permission_bit_flag == permission_bit_flag
    }
}

#[derive(Deserialize_repr, Debug)]
#[repr(u8)]
pub enum PermissionOverwriteType {
    Role = 0,
    Member = 1,
}

#[derive(Deserialize, Debug)]
pub struct PermissionOverwrite {
    id: Snowflake,
    #[serde(rename = "type")]
    r#type: PermissionOverwriteType,
    allow: String,
    deny: String
}