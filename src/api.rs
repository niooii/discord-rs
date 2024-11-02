use reqwest::{Client, Method};
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use thiserror::Error;
use crate::model::channel::Channel;
use crate::model::guild::Guild;
use crate::model::message::DefaultMessageData;
use crate::model::message::Message;
use crate::model::user::MainUserData;
use crate::model::user::UserData;

use crate::model::Snowflake;
use crate::{endpoints, http::{self, QueryError}};
pub type Result<T> = core::result::Result<T, QueryError>;

#[derive(Error, Debug)]
pub enum DiscordError {
    #[error("Ratelimit reached: try again after {retry_after} seconds")]
    RateLimitReached { retry_after: f64 },
    #[error("Authentication failed")]
    AuthenticationFail,
    #[error("Error {code}: {message}")]
    Other {
        message: String,
        code: u64
    }
}

pub(crate) async fn get_authenticated_user_data(
    client: Client
) -> Result<MainUserData> {
    http::get_struct(
        client,
        endpoints::ME,
        Method::GET
    ).await
}

pub(crate) async fn get_user_from_id(
    client: Client,
    id: &Snowflake,
) -> Result<UserData> {
    http::get_struct(
        client,
        &endpoints::user(id),
        Method::GET
    ).await
}

pub(crate) async fn get_private_channels(
    client: Client
) -> Result<Vec<Channel>> {
    Ok(
        http::get_struct::<Vec<Channel>>(client, &endpoints::PRIVATE_CHANNELS, Method::GET).await?
    )
}

pub(crate) async fn get_guilds(
    client: Client
) -> Result<Vec<Guild>> {
    Ok(
        http::get_struct::<Vec<Guild>>(client, &endpoints::GUILDS, Method::GET).await?
    )
}

pub(crate) async fn get_channels_in_guild(
    client: Client,
    guild_id: &Snowflake,
) -> Result<Vec<Channel>> {
    http::get_struct::<Vec<Channel>>(client, &endpoints::guild_channels(guild_id), Method::GET).await
}

// messaging utilities
#[derive(Serialize)]
struct MessagePostData {
    content: String
}

impl MessagePostData {
    fn new(content: &String) -> MessagePostData {
        MessagePostData {
            content: content.clone()
        }
    }
}

pub async fn start_typing(
    client: Client,
    channel_id: &Snowflake
) -> Result<()> {
    http::send(client, &endpoints::start_typing(&channel_id), Method::POST)
        .await
}

pub async fn send_message(
    client: Client,
    channel_id: &Snowflake,
    content: &String
) -> Result<DefaultMessageData> {
    let post_data = MessagePostData::new(content);
    http::get_struct_body(
        client, 
        &endpoints::send_message(channel_id), 
        &post_data, 
        Method::POST
    ).await
}

pub async fn messages(
    client: Client,
    channel_id: &Snowflake,
    before_message_id: Option<&Snowflake>,
    limit: u8
) -> Result<Vec<Message>> {
    http::get_struct(
        client, 
        &endpoints::messages(channel_id, before_message_id, limit), 
        Method::GET
    ).await
}

pub async fn message_from_id(
    client: Client,
    channel_id: &Snowflake,
    message_id: &Snowflake
) -> Result<Message> {
    http::get_struct(
        client, 
        &endpoints::message(&channel_id, message_id), 
        Method::GET
    ).await
}

pub async fn open_channel(
    client: Client,
    recipient_ids: &[Snowflake]
) -> Result<Channel> {
    let body = json!({
        "recipients": recipient_ids
    });
    http::get_struct_body(client, &endpoints::PRIVATE_CHANNELS, &body, Method::POST).await
}

pub async fn close_channel(
    client: Client,
    channel_id: &Snowflake
) -> Result<Channel> {
    http::get_struct(
        client,
        // TODO! silent parameter seems interesting ngl
        &format!("{}?silent=false", endpoints::channel(channel_id)), 
        Method::DELETE
    ).await
}