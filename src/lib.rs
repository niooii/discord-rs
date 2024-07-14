#![allow(dead_code)]
pub mod client;
pub mod endpoints;
pub mod http;
pub mod util;
pub mod api;
pub mod gateway;
pub mod model;

#[macro_use]
pub mod serde_utils;

use zstd::stream::read::Decoder;
use crate::http::validate_ratelimit;
use http::QueryError;
use serde::Serialize;
use crate::client::DiscordClient;
use futures_util::{future, pin_mut, SinkExt, StreamExt};
use num_traits::FromPrimitive;
use std::io::Cursor;
use std::io::Read;
use serde_eetf::{to_bytes, from_bytes};

impl DiscordClient {
    // // wrapper implementations
    // pub async fn me(
    //     &self
    // ) -> Result<MainUserData, QueryError> {
    //     http::get_data(
    //         &self.req_client(),
    //         endpoints::ME,
    //         None,
    //     ).await
    // }

    // pub async fn user_from_id(
    //     &self,
    //     id: &str,
    // ) -> Result<UserData, QueryError> {
    //     http::get_data(
    //         &self.req_client(),
    //         &endpoints::USER(id),
    //         None,
    //     ).await
    // }

    // // TEXTCHANNEL STUFF
    // pub async fn get_text_channels_in_guild(
    //     &self,
    //     guild_id: &str,
    // ) -> Result<GuildTextChannel, QueryError> {
    //     todo!();
    // }

    // pub async fn get_private_text_channels(
    //     &self
    // ) -> Result<Vec<PrivateTextChannel>, QueryError> {
    //     let mut channels: Vec<PrivateTextChannel> = Vec::new();

    //     let req_client = self.req_client();

    //     let json = http::get_as_json(&req_client, &endpoints::CHANNELS).await?;

    //     let json_array = json.as_array().unwrap();

    //     for json in json_array {
    //         if !json["type"].is_number() {
    //             return Err(QueryError::Other { error: "type field is not number (fix this message later)".to_string() });
    //         }

    //         let channel_type = json["type"].as_number().unwrap().as_u64().unwrap();
        
    //         match FromPrimitive::from_u64(channel_type).unwrap() {
    //             ChannelType::Dm | ChannelType::GroupDm => {
    //                 channels.push(
    //                     PrivateTextChannel::__(
    //                         req_client.clone(),
    //                         serde_json::from_value::<PrivateTextChannelData>(json.clone()).unwrap()
    //                     )
    //                 )
    //             },
    //             _ => {
    //                 return Err(QueryError::Other { error: "unrecognized channel type, check new discord api docs.".to_string() });
    //             }
    //         }
    //     }

    //     Ok(channels)
    // }

    // /* 
    // Try to use get_private_text_channels and cache the results to minimize api calls.
    // */
    // pub async fn private_text_channel(
    //     &self,
    //     id: &str,
    // ) -> Result<PrivateTextChannel, QueryError> {
    //     todo!();
    // }

    // /* 
    // Try to use get_guild_text_channels and cache the results to minimize api calls.
    // */
    // pub async fn guild_text_channel(
    //     &self,
    //     id: &str,
    // ) -> Result<PrivateTextChannel, QueryError> {
    //     todo!();
    // }

    // pub async fn send_message(
    //     &self,
    //     channel_id: &String,
    //     content: &String
    // ) -> Result<(), QueryError> {
    //     let res = self.req_client().post(endpoints::SEND_MESSAGE(&channel_id))
    //         .json(&MessagePostData::new(content.clone()))
    //         .send().await;

    //     // handle errors better MONKEY
    //     if let Err(e) = res {
    //         return  Err(QueryError::ReqwestError { err: e });
    //     }
        
    //     validate_ratelimit(res.unwrap()).await?;

    //     Ok(())
    // }

    // // getting content
    // pub async fn messages_before(
    //     &self,
    //     channel_id: &String,
    //     before_message_id: &String,
    //     limit: u8
    // ) -> Result<Vec<Message>, QueryError> {
    //     let res = self.req_client().get(endpoints::MESSAGES_BEFORE(&channel_id, &before_message_id, limit))
    //         .send().await;

    //     // handle errors better MONKEY
    //     if let Err(e) = res {
    //         Err(QueryError::ReqwestError { err: e })
    //     }
    //     else {
    //         let res = res.unwrap();
    //         let messages = res.json::<Vec<Message>>().await
    //             .map_err(|e| QueryError::ReqwestError { err: e })?;
    //         Ok(messages)
    //     }
    // }

    // pub async fn message_from_id(
    //     &self,
    //     channel_id: &String,
    //     message_id: &String
    // ) -> Result<Message, QueryError> {
    //     let res = self.req_client().get(endpoints::MESSAGE(&channel_id, &message_id))
    //         .send().await;

    //     // handle errors better MONKEY
    //     if let Err(e) = res {
    //         Err(QueryError::ReqwestError { err: e })
    //     }
    //     else {
    //         let res = res.unwrap();
    //         let message = res.json::<Message>().await
    //             .map_err(|e| QueryError::ReqwestError { err: e })?;
    //         Ok(message)
    //     }
    // }
}