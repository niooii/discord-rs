#![allow(dead_code)]
pub mod client;
pub mod endpoints;
pub mod http;
pub mod api;
pub mod gateway;
pub mod model;
#[macro_use]
pub mod serde_utils;
use std::ops::{Sub, SubAssign};

use model::{channel::Channel, message::DefaultMessageData, user::{MainUserData, UserData}, Snowflake};
use tokio::time::Duration;
use crate::client::DiscordClient;
use api::Result;

pub enum MessageSendTime {
    INSTANT,
    FAST,
    MEDIUM,
    SLOW,
    CUSTOM { duration: Duration }
}

impl DiscordClient {
    pub async fn me(&self) -> Result<MainUserData> {
        api::get_authenticated_user_data(self.req_client()).await
    }

    pub async fn user(&self, user_id: &Snowflake) -> Result<UserData> {
        api::get_user_from_id(self.req_client(), user_id).await
    } 

    pub async fn dm_channels(&self) -> Result<Vec<Channel>> {
        api::get_private_channels(self.req_client()).await
    }

    pub async fn send_message(&self, channel_id: &Snowflake, content: &String, send_time: MessageSendTime) -> Result<DefaultMessageData> {
        let mut typing_duration = match send_time {
            MessageSendTime::INSTANT => return api::send_message(self.req_client(), channel_id, content).await,
            MessageSendTime::FAST => todo!(),
            MessageSendTime::MEDIUM => todo!(),
            MessageSendTime::SLOW => todo!(),
            MessageSendTime::CUSTOM { duration } => duration,
        };

        let wait_cycle_duration = Duration::from_secs(11);
        loop {
            api::start_typing(self.req_client(), channel_id).await?;
            if typing_duration.le(&wait_cycle_duration) {
                tokio::time::sleep(typing_duration).await;
                break;
            } else {
                tokio::time::sleep(wait_cycle_duration).await;
                typing_duration.sub_assign(wait_cycle_duration);
            }
        }

        api::send_message(self.req_client(), channel_id, content).await
    }
}