#![allow(dead_code)]
pub mod client;
pub mod endpoints;
pub mod http;
pub mod api;
pub mod gateway;
pub mod model;

#[macro_use]
pub mod serde_utils;

use model::{message::DefaultMessageData, user::MainUserData, Snowflake};
use crate::client::DiscordClient;
use api::Result;

impl DiscordClient {
    pub async fn me(&self) -> Result<MainUserData> {
        api::get_authenticated_user_data(self.req_client()).await
    }

    pub async fn send_message(&self, channel_id: &Snowflake, content: &String) -> Result<DefaultMessageData> {
        api::send_message(self.req_client(), channel_id, content).await
    }
}