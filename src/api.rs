use reqwest::{Client, Method};
use serde::Serialize;
use thiserror::Error;
use crate::model::channel::Channel;
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
    #[error("")]
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

// TEXTCHANNEL STUFF
pub(crate) async fn get_channels_in_guild(
    client: Client,
    guild_id: &Snowflake,
) -> Result<Vec<Channel>> {
    http::get_struct::<Vec<Channel>>(client, &endpoints::guild_channels(guild_id), Method::GET).await
}

pub(crate) async fn get_private_channels(
    client: Client
) -> Result<Vec<Channel>> {
    Ok(
        http::get_struct::<Vec<Channel>>(client, &endpoints::PRIVATE_CHANNELS, Method::GET).await?
    )
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
    let res = client.post(endpoints::start_typing(&channel_id))
        .send().await;

    // handle errors better MONKEY
    if let Err(e) = res {
        return  Err(QueryError::ReqwestError { err: e });
    }
    
    // http::validate_ratelimit(res.unwrap()).await?;

    Ok(())
}

pub async fn send_message(
    client: Client,
    channel_id: &Snowflake,
    content: &String
) -> Result<DefaultMessageData> {
    let post_data = MessagePostData::new(content);
    http::get_struct_body(client, &endpoints::send_message(channel_id), post_data, Method::POST).await
}

// getting content
pub async fn messages_before(
    client: Client,
    channel_id: &Snowflake,
    before_message_id: &Snowflake,
    limit: u8
) -> Result<Vec<Message>> {
    let res = client.get(endpoints::messages_before(&channel_id, &before_message_id, limit))
        .send().await;

    if let Err(e) = res {
        Err(QueryError::ReqwestError { err: e })
    }
    else {
        let res = res.unwrap();
        let messages = res.json::<Vec<Message>>().await
            .map_err(|e| QueryError::ReqwestError { err: e })?;
        Ok(messages)
    }
}

pub async fn message_from_id(
    client: Client,
    channel_id: &Snowflake,
    message_id: &Snowflake
) -> Result<Message> {
    let res = client.get(endpoints::message(&channel_id, message_id))
        .send().await;

    if let Err(e) = res {
        Err(QueryError::ReqwestError { err: e })
    }
    else {
        let res = res.unwrap();
        let message = res.json::<Message>().await
            .map_err(|e| QueryError::ReqwestError { err: e })?;
        Ok(message)
    }
}