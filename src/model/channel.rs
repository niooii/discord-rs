use async_std::channel;
use num::FromPrimitive;
use num_derive::FromPrimitive;
use serde::Deserialize;
use serde_json::Value;
use time::OffsetDateTime;
use crate::{permissions::PermissionOverwrite, user::*};

#[derive(Deserialize, Debug)]
pub struct GuildForumTag {
    id: String, 
    name: String,
    moderated: bool,
    emoji_id: Option<String>,
    emoji_name: Option<String>
}

#[derive(Deserialize, Debug)]
pub struct GuildTextData {
    pub id: String,
    pub last_message_id: Option<String>,
    pub flags: u64,
    pub guild_id: String,
    #[serde(default, with = "time::serde::iso8601::option")]
    pub last_pin_timestamp: Option<OffsetDateTime>,
    pub name: String,
    #[serde(rename = "parent_id")]
    pub category_id: Option<String>,
    pub rate_limit_per_user: u32,
    pub topic: Option<String>,
    pub position: u32,
    pub permission_overwrites: Vec<PermissionOverwrite>,
    pub nsfw: bool,
}

#[derive(Deserialize, Debug)]
pub struct DmData {
    pub id: String,
    pub last_message_id: Option<String>,
    pub flags: u64,
    #[serde(default, with = "time::serde::iso8601::option")]
    pub last_pin_timestamp: Option<OffsetDateTime>,
    pub recipients: Vec<UserData>,
}

#[derive(Deserialize, Debug)]
pub struct GuildVoiceData {
    pub id: String,
    pub last_message_id: Option<String>,
    pub flags: u64,
    pub guild_id: String,
    pub name: String,
    #[serde(default, with = "time::serde::iso8601::option")]
    pub last_pin_timestamp: Option<OffsetDateTime>,
    #[serde(rename = "parent_id")]
    pub category_id: Option<String>,
    pub rate_limit_per_user: u32,
    pub bitrate: u32,
    pub user_limit: u32,
    pub rtc_region: Option<String>,
    pub position: u32,
    pub permission_overwrites: Vec<PermissionOverwrite>,
    pub nsfw: bool,
}

#[derive(Deserialize, Debug)]
pub struct GroupDmData {
    pub id: String,
    pub last_message_id: Option<String>,
    pub flags: u64,
    #[serde(default, with = "time::serde::iso8601::option")]
    pub last_pin_timestamp: Option<OffsetDateTime>,
    pub recipients: Vec<UserData>,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub owner_id: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct GuildCategoryData {
    pub id: String,
    pub flags: u64,
    pub guild_id: String,
    pub name: String,
    pub position: u32,
    pub permission_overwrites: Vec<PermissionOverwrite>,
}

#[derive(Deserialize, Debug)]
pub struct GuildAnnouncementData {
    pub id: String,
    pub last_message_id: String,
    pub flags: u64,
    #[serde(default, with = "time::serde::iso8601::option")]
    pub last_pin_timestamp: Option<OffsetDateTime>,
    pub guild_id: String,
    pub name: String,
    #[serde(rename = "parent_id")]
    pub category_id: Option<String>,
    pub rate_limit_per_user: u32,
    pub topic: String,
    pub position: u32,
    pub permission_overwrites: Vec<PermissionOverwrite>,
    pub nsfw: bool,
    pub theme_color: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct GuildForumData {
    pub id: String,
    pub last_message_id: Option<String>,
    pub flags: u64,
    pub guild_id: String,
    pub name: String,
    #[serde(rename = "parent_id")]
    pub category_id: String,
    pub rate_limit_per_user: u32,
    pub topic: String,
    pub position: u32,
    pub permission_overwrites: Vec<PermissionOverwrite>,
    pub nsfw: bool,
    pub available_tags: Vec<GuildForumTag>,
}

#[derive(FromPrimitive)]
#[repr(u8)]
enum ChannelType {
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

// https://discord.com/developers/docs/resources/channel
#[derive(Debug)]
pub enum Channel {
    // type 0
    GuildText(GuildTextData),
    // type 1
    Dm(DmData),
    // type 2
    GuildVoice(GuildVoiceData),
    // type 3
    GroupDm(GroupDmData),
    // type 4
    GuildCategory(GuildCategoryData),
    // type 5
    GuildAnnouncement(GuildAnnouncementData),
    // type 10
    AnnouncementThread,
    // type 11
    PublicThread,
    // type 12
    PrivateThread,
    // type 13
    GuildStageVoice,
    // type 14
    GuildDirectory,
    // type 15
    GuildForum(GuildForumData),
    // type 16
    GuildMedia,
}

impl<'de> serde::Deserialize<'de> for Channel {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let value = Value::deserialize(d)?;
        let channel_type = value.get("type").unwrap();
        let channel_type = channel_type.as_u64().unwrap();
        let channel_type = FromPrimitive::from_u8(channel_type as u8).unwrap();
        let channel = match channel_type {
            ChannelType::GuildText => {
                let data = GuildTextData::deserialize(value);
                Channel::GuildText(data.unwrap())
            },
            ChannelType::Dm => {
                todo!()
            },
            ChannelType::GuildVoice => {
                let data = GuildVoiceData::deserialize(value);
                Channel::GuildVoice(data.unwrap())
            },
            ChannelType::GroupDm => todo!(),
            ChannelType::GuildCategory => {
                let data = GuildCategoryData::deserialize(value);
                Channel::GuildCategory(data.unwrap())
            },
            ChannelType::GuildAnnouncement => {
                let data = GuildAnnouncementData::deserialize(value);
                Channel::GuildAnnouncement(data.unwrap())
            },
            ChannelType::AnnouncementThread => todo!(),
            ChannelType::PublicThread => todo!(),
            ChannelType::PrivateThread => todo!(),
            ChannelType::GuildStageVoice => todo!(),
            ChannelType::GuildDirectory => todo!(),
            ChannelType::GuildForum => todo!(),
            ChannelType::GuildMedia => todo!(),
        };
        Ok(channel)
    }
}