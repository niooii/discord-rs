use reqwest::{Client, Method};
use serde::Serialize;
use crate::model::channel::Channel;
use crate::model::user::MainUserData;
use crate::model::{user::UserData};

use crate::{endpoints, http::{self, QueryError}};

pub async fn get_authenticated_user_data(
    client: Client
) -> Result<MainUserData, QueryError> {
    http::get_data(
        client,
        endpoints::ME,
        None,
        Method::GET
    ).await
}

pub async fn get_user_from_id(
    client: Client,
    id: &str,
) -> Result<UserData, QueryError> {
    http::get_data(
        client,
        &endpoints::USER(id),
        None,
        Method::GET
    ).await
}

// TEXTCHANNEL STUFF
pub async fn get_channels_in_guild(
    client: Client,
    guild_id: &str,
) -> Result<Vec<Channel>, QueryError> {
    http::get_data::<Vec<Channel>>(client, &endpoints::GUILD_CHANNELS(guild_id), None, Method::GET).await
}

pub async fn get_private_channels(
    client: Client
) -> Result<Vec<Channel>, QueryError> {
    let json: serde_json::Value = http::get_json(client, &endpoints::PRIVATE_CHANNELS, None, Method::GET).await?;

    // println!("{:?}", serde_json::to_string_pretty(&json));

    Ok(serde_json::from_value::<Vec<Channel>>(json).unwrap())
}

// /* 
// Try to use get_private_text_channels and cache the results to minimize api calls.
// */
// pub async fn private_text_channel(
//     client: Client,
//     id: &str,
// ) -> Result<PrivateTextChannel, QueryError> {
//     todo!();
// }

// /* 
// Try to use get_guild_text_channels and cache the results to minimize api calls.
// */
// pub async fn guild_text_channel(
//     client: Client,
//     id: &str,
// ) -> Result<PrivateTextChannel, QueryError> {
//     todo!();
// }

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

pub async fn start_typing(
    client: Client,
    channel_id: &String
) -> Result<(), QueryError> {
    let res = client.post(endpoints::START_TYPING(&channel_id))
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
    channel_id: &String,
    content: &String
) -> Result<(), QueryError> {
    let res = client.post(endpoints::SEND_MESSAGE(&channel_id))
        .json(&MessagePostData::new(content.clone()))
        .send().await;

    // handle errors better MONKEY
    if let Err(e) = res {
        return  Err(QueryError::ReqwestError { err: e });
    }
    
    http::validate_ratelimit(res.unwrap()).await?;

    Ok(())
}

// // getting content
// pub async fn messages_before(
//     client: Client,
//     channel_id: &String,
//     before_message_id: &String,
//     limit: u8
// ) -> Result<Vec<Message>, QueryError> {
//     let res = client.get(endpoints::MESSAGES_BEFORE(&channel_id, &before_message_id, limit))
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
//     client: Client,
//     channel_id: &String,
//     message_id: &String
// ) -> Result<Message, QueryError> {
//     let res = client.get(endpoints::MESSAGE(&channel_id, &message_id))
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