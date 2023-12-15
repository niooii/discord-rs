pub static ME: &str = "https://discord.com/api/v9/users/@me";

pub fn CHANNEL(channel_id: &str) -> String {
    format!("https://discord.com/api/v9/channels/{}", channel_id)
}

pub fn SEND_MESSAGE(channel_id: &str) -> String {
    format!("{}/messages", CHANNEL(channel_id))
}

pub fn USER(user_id: &str) -> String {
    format!("https://discord.com/api/v9/users/{}", user_id)
}

pub fn MESSAGES_BEFORE(channel_id: &str, before_message_id: &str, limit: u8) -> String {
    format!("https://discord.com/api/v9/channels/{channel_id}/messages?before={before_message_id}&limit={limit}")
}