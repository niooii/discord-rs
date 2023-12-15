use crate::http;
use crate::http::validate_ratelimit;
use crate::message::Message;
use http::QueryError;
use serde::Serialize;
use crate::user_structs::*;
use crate::channel;
use channel::*;
use crate::context::DiscordContext;
use crate::endpoints;

// wrapper implementations
pub async fn me(
    discord_context: &DiscordContext,
) -> Result<Me, QueryError> {
    http::get_data(
        &discord_context.req_client(),
        endpoints::ME,
        None,
    ).await
}

pub async fn user_from_id(
    discord_context: &DiscordContext,
    id: &str,
) -> Result<User, QueryError> {
    http::get_data(
        &discord_context.req_client(),
        &endpoints::USER(id),
        None,
    ).await
}

// TEXTCHANNEL STUFF
pub async fn guild_text_channel(
    discord_context: &DiscordContext,
    id: &str,
) -> Result<GuildTextChannel, QueryError> {
    channel::text_channel_from_id::<GuildTextChannel>(
        discord_context.req_client(),
        ChannelType::GuildText,
        id,
    ).await
}

pub async fn dm_text_channel(
    discord_context: &DiscordContext,
    id: &str,
) -> Result<DmChannel, QueryError> {
    channel::text_channel_from_id::<DmChannel>(
        discord_context.req_client(),
        ChannelType::Dm,
        id,
    ).await
}

pub async fn group_text_channel(
    discord_context: &DiscordContext,
    id: &str,
) -> Result<GroupDmChannel, QueryError> {
    channel::text_channel_from_id::<GroupDmChannel>(
        discord_context.req_client(),
        ChannelType::GroupDm,
        id,
    ).await
}

// messaging utilities
#[derive(Serialize)]
struct MessagePostData {
    content: String
}

impl MessagePostData {
    fn new(content: String) -> MessagePostData {
        MessagePostData {
            content
        }
    }
}

pub async fn send_message(
    discord_context: &DiscordContext,
    channel_id: &String,
    content: String
) -> Result<(), QueryError> {
    let res = discord_context.req_client().post(endpoints::SEND_MESSAGE(&channel_id))
        .json(&MessagePostData::new(content.to_string()))
        .send().await;

    // handle errors better MONKEY
    if let Err(e) = res {
        return  Err(QueryError::ReqwestError { err: e });
    }
    
    validate_ratelimit(res.unwrap()).await?;

    Ok(())
}

// getting content
pub async fn messages_before(
    discord_context: &DiscordContext,
    channel_id: &String,
    before_message_id: &String,
    limit: u8
) -> Result<Vec<Message>, QueryError> {
    let res = discord_context.req_client().get(endpoints::MESSAGES_BEFORE(&channel_id, &before_message_id, limit))
        .send().await;

    // handle errors better MONKEY
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