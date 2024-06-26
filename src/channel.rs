use std::any::Any;

use reqwest::Client;
use serde::{Deserialize, Deserializer, Serialize};
use serde_repr::{Serialize_repr, Deserialize_repr};

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

#[derive(Debug, Deserialize)]
pub struct PrivateTextChannelData {
    id: String,
    last_message_id: Option<String>,
    flags: u64,
    recipients: Vec<User>,
    name: Option<String>,
    icon: Option<String>,
    owner_id: Option<String>,
}

// TYPE 1
#[derive(Debug, Deserialize)]
pub struct PrivateTextChannel {
    #[serde(skip_deserializing)]
    client: Client, 
    data: PrivateTextChannelData
}

impl PrivateTextChannel {
    pub fn __(client: Client, data: PrivateTextChannelData) -> Self {
        Self {
            client,
            data
        }
    }

    pub fn id(&self) -> &String {
        &self.data.id
    }

    pub fn last_message(&self) -> &Option<String> {
        &self.data.last_message_id
    }

    pub fn flags(&self) -> u64 {
        self.data.flags
    }

    pub fn recipients(&self) -> &Vec<User> {
        &self.data.recipients
    }

    pub fn name(&self) -> &Option<String> {
        &self.data.name
    }

    pub fn icon(&self) -> &Option<String> {
        &self.data.icon
    }

    pub fn owner(&self) -> &Option<String> {
        &self.data.owner_id
    }
}

pub struct GuildTextChannelData {
    id: String,
    last_message_id: Option<String>,
    flags: u64,
    name: Option<String>,
}

pub struct GuildTextChannel {
    client: Client,
    data: GuildTextChannelData
}

impl GuildTextChannel {
    
}