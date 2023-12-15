use reqwest::Client;
use serde::{Deserialize, Deserializer, Serialize};

extern crate num;
use num_derive::FromPrimitive;
use serde_json::Value;
use serde::de::DeserializeOwned;

use async_trait::async_trait;

use crate::{http::QueryError, user_structs::*, endpoints};
use crate::http::{*, self};

#[derive(Debug, FromPrimitive, PartialEq, Eq, Copy, Clone)]
#[repr(u8)]
pub enum ChannelType {
    GuildText = 0,
    Dm = 1,
    GuildVoice = 2,
    GroupDm = 3,
    GuildCategory = 4,
    GuildAnnouncement = 5,
    AnnouncementThread = 10,
    PublicThread = 11,
    PrivateThread = 12,
    GuildStageVoice = 13,
    GuildDirectory = 14,
    GuildForum = 15,
    GuildMedia = 16,
}

#[async_trait]
pub trait TextChannel {
    fn id(&self) -> &String;
}

// TYPE 1
#[derive(Debug, Deserialize)]
pub struct DmChannel {
    #[serde(skip_deserializing)]
    // for internal use only. PLEASE REFACTOR SOMEDAY
    pub client: Option<Client>,
    id: String,
    last_message_id: Option<String>,
    flags: u64,
    #[serde(rename = "recipients", deserialize_with = "deserialize_recipient")]
    recipient: User,
}

impl TextChannel for DmChannel {
    fn id(&self) -> &String {
        &self.id
    }
}

impl DmChannel {
    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn last_message_id(&self) -> Option<&String> {
        self.last_message_id.as_ref()
    }

    pub fn flags(&self) -> u64 {
        self.flags
    }

    pub fn recipient(&self) -> &User {
        &self.recipient
    }
}

// TYPE 3
#[derive(Debug, Deserialize)]
pub struct GroupDmChannel {
    #[serde(skip_deserializing)]
    // for internal use only. PLEASE REFACTOR SOMEDAY
    pub client: Option<Client>,
    id: String,
    last_message_id: Option<String>,
    flags: u64,
    recipients: Vec<User>,
    name: Option<String>,
    icon: Option<String>,
    owner_id: String,
}

impl GroupDmChannel {
    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn last_message(&self) -> &Option<String> {
        &self.last_message_id
    }

    pub fn flags(&self) -> u64 {
        self.flags
    }

    pub fn recipients(&self) -> &Vec<User> {
        &self.recipients
    }

    pub fn name(&self) -> &Option<String> {
        &self.name
    }

    pub fn icon(&self) -> &Option<String> {
        &self.icon
    }

    pub fn owner(&self) -> &String {
        &self.owner_id
    }
}

// LORD HAVE MERCY
fn deserialize_recipient<'de, D>(deserializer: D) -> Result<User, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Value = Deserialize::deserialize(deserializer)?;

    Ok(serde_json::from_value::<User>(value.as_array().unwrap().first().unwrap().clone()).unwrap())
}

impl TextChannel for GroupDmChannel {
    fn id(&self) -> &String {
        &self.id
    }
}

// TYPE 0
#[derive(Debug, Deserialize)]
pub struct GuildTextChannel {
    #[serde(skip_deserializing)]
    // for internal use only. PLEASE REFACTOR SOMEDAY
    pub client: Option<Client>,
    id: String,
}

impl TextChannel for GuildTextChannel {
    fn id(&self) -> &String {
        &self.id
    }
}

// helpers
pub async fn text_channel_from_id<T>(
    req_client: Client,
    target_type: ChannelType,
    id: &str,
) -> Result<T, QueryError>
where
    T: TextChannel + DeserializeOwned,
{
    let json = http::get_as_json(&req_client, &endpoints::CHANNEL(id)).await?;

    if json["type"] != target_type as u32 {
        return Err(QueryError::WrongChannelType {
            correct_type: num::FromPrimitive::from_u8(json["type"].as_u64().unwrap() as u8)
                .unwrap(),
        });
    }

    let channel = serde_json::from_value::<T>(json).map_err(|e| QueryError::SerdeError { err: e });

    return channel;
}