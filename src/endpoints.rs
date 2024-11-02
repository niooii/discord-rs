use crate::model::Snowflake;

pub const ME: &str = "https://discord.com/api/v9/users/@me";
pub const PRIVATE_CHANNELS: &str = "https://discord.com/api/v9/users/@me/channels";
pub const GUILDS: &str = "https://discord.com/api/v9/users/@me/guilds";

pub fn channel(channel_id: &Snowflake) -> String {
    format!("https://discord.com/api/v9/channels/{}", channel_id)
}

pub fn guild_channels(guild_id: &Snowflake) -> String {
    format!("https://discord.com/api/v9/guilds/{}/channels", guild_id)
}

pub fn send_message(channel_id: &Snowflake) -> String {
    format!("{}/messages", channel(channel_id))
}

pub fn start_typing(channel_id: &Snowflake) -> String {
    format!("{}/typing", channel(channel_id))
}

pub fn user(user_id: &Snowflake) -> String {
    format!("https://discord.com/api/v9/users/{}", user_id)
}

pub fn messages(channel_id: &Snowflake, before_message_id: Option<&Snowflake>, limit: u8) -> String {
    match before_message_id {
        Some(before_msg_id) => format!("https://discord.com/api/v9/channels/{channel_id}/messages?before={before_msg_id}&limit={limit}"),
        None => format!("https://discord.com/api/v9/channels/{channel_id}/messages?&limit={limit}")
    }
}

pub fn message(channel_id: &Snowflake, message_id: &Snowflake) -> String {
    format!("https://discord.com/api/v9/channels/{channel_id}/messages/{message_id}")
}

