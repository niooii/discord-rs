#![allow(dead_code)]
pub mod client;
pub mod endpoints;
pub mod http;
pub(crate) mod api;
pub mod gateway;
pub mod model;
#[macro_use]
pub mod serde_utils;
use std::{ops::{Sub, SubAssign}, pin::Pin, task::{Context, Poll}};
use futures_util::Stream;
use model::{channel::Channel, guild::Guild, message::{DefaultMessageData, GeneralMessageData, Message}, user::{MainUserData, UserData}, Snowflake};
use pin_project_lite::pin_project;
use tokio::time::Duration;
use crate::client::DiscordClient;
use api::Result;
use async_stream::{stream, try_stream};
use model::ID;

pub enum MessageSendTime {
    /// The message will send without delay.
    Instant,
    /// 15 characters typed per second.
    Fast,
    /// 10 characters typed per second.
    Medium,
    /// 5 characters typed per second.
    Slow,
    /// Specify how long the message takes to type.
    CustomDur { duration: Duration },
    /// Specify characters per second.
    CustomCps { cps: u16 }
}

pub enum MessageFetchRate {
    // 50 per request, default for the official client.
    Default,
    // 100 per request, max that discord allows.
    Max,
    // Must be above 0 and under 100.
    Custom { per_request: u8 }
}

pin_project! {
    pub struct MessageStream<'a> {
        #[pin]
        inner: Pin<Box<dyn Stream<Item = Result<Vec<Message>>> + 'a>>
    }
}

impl<'a> Stream for MessageStream<'a> {
    type Item = Result<Vec<Message>>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.project().inner.poll_next(cx)
    }
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

    pub async fn guilds(&self) -> Result<Vec<Guild>> {
        api::get_guilds(self.req_client()).await
    }

    pub fn messages<'a>(
        &'a self, 
        channel_id: &'a Snowflake, 
        fetch_rate: MessageFetchRate
    ) -> impl Stream<Item = Result<Vec<Message>>> + 'a {
        let stream = try_stream! {
            let limit: u8 = match fetch_rate {
                MessageFetchRate::Default => 50,
                MessageFetchRate::Max => 100,
                MessageFetchRate::Custom { per_request } => per_request,
            };
            let mut curr_oldest_id: Option<Snowflake> = None;
            loop {
                let msgs = api::messages(
                    self.req_client(),
                    channel_id,
                    curr_oldest_id.as_ref(),
                    limit
                ).await?;
                curr_oldest_id = Some(
                    if let Some(msg) = msgs.last() {
                        msg.id().clone()
                    } else {
                        break;
                    }
                );
                yield msgs;
            }
        };

        MessageStream {
            inner: Box::pin(stream)
        }
    }

    pub async fn send_message(&self, channel_id: &Snowflake, content: &String, send_time: MessageSendTime) -> Result<DefaultMessageData> {
        let mut typing_duration = match send_time {
            MessageSendTime::Instant => return api::send_message(self.req_client(), channel_id, content).await,
            MessageSendTime::Fast => Duration::from_secs_f32(content.len() as f32 / 15_f32),
            MessageSendTime::Medium => Duration::from_secs_f32(content.len() as f32 / 10_f32),
            MessageSendTime::Slow => Duration::from_secs_f32(content.len() as f32 / 5_f32),
            MessageSendTime::CustomDur { duration } => duration,
            MessageSendTime::CustomCps { cps } => Duration::from_secs_f32(content.len() as f32 / cps as f32) 
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

// Convenience methods
impl DefaultMessageData {
    pub fn is_author(&self, user_id: &Snowflake) -> bool {
        user_id == &self.author.id
    }
}