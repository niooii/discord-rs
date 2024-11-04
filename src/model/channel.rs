use async_std::channel;
use num::FromPrimitive;
use num_derive::FromPrimitive;
use serde::Deserialize;
use serde_json::Value;
use time::OffsetDateTime;
use crate::http::QueryError;
use crate::serde_utils;
use crate::model;
use serde::de::Error;
use model::{permissions::PermissionOverwrite, user::*};

use super::guild;
use super::Snowflake;
use super::ID;

#[derive(Deserialize, Debug)]
pub struct GuildForumTag {
    id: Snowflake, 
    name: String,
    moderated: bool,
    emoji_id: Option<Snowflake>,
    emoji_name: Option<String>
}

#[derive(Deserialize, Debug)]
pub struct GuildTextData {
    pub id: Snowflake,
    pub last_message_id: Option<Snowflake>,
    pub flags: u64,
    pub guild_id: Snowflake,
    #[serde(default, with = "time::serde::iso8601::option")]
    pub last_pin_timestamp: Option<OffsetDateTime>,
    pub name: String,
    #[serde(rename = "parent_id")]
    pub category_id: Option<Snowflake>,
    pub rate_limit_per_user: u32,
    pub topic: Option<String>,
    pub position: u32,
    pub permission_overwrites: Vec<PermissionOverwrite>,
    pub nsfw: bool,
}

#[derive(Deserialize, Debug)]
pub struct DmData {
    pub id: Snowflake,
    pub last_message_id: Option<Snowflake>,
    pub flags: u64,
    #[serde(default, with = "time::serde::iso8601::option")]
    pub last_pin_timestamp: Option<OffsetDateTime>,
    #[serde(rename = "recipients", with = "serde_utils::deserialize_single_element")]
    pub recipient: UserData,
}

#[derive(Deserialize, Debug)]
pub struct GuildVoiceData {
    pub id: Snowflake,
    pub last_message_id: Option<Snowflake>,
    pub flags: u64,
    pub guild_id: Snowflake,
    pub name: String,
    #[serde(default, with = "time::serde::iso8601::option")]
    pub last_pin_timestamp: Option<OffsetDateTime>,
    #[serde(rename = "parent_id")]
    pub category_id: Option<Snowflake>,
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
    pub id: Snowflake,
    pub last_message_id: Option<Snowflake>,
    pub flags: u64,
    #[serde(default, with = "time::serde::iso8601::option")]
    pub last_pin_timestamp: Option<OffsetDateTime>,
    pub recipients: Vec<UserData>,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub owner_id: Option<Snowflake>,
}

#[derive(Deserialize, Debug)]
pub struct GuildCategoryData {
    pub id: Snowflake,
    pub flags: u64,
    pub guild_id: Snowflake,
    pub name: String,
    pub position: u32,
    pub permission_overwrites: Vec<PermissionOverwrite>,
}

#[derive(Deserialize, Debug)]
pub struct GuildAnnouncementData {
    pub id: Snowflake,
    pub last_message_id: Snowflake,
    pub flags: u64,
    #[serde(default, with = "time::serde::iso8601::option")]
    pub last_pin_timestamp: Option<OffsetDateTime>,
    pub guild_id: Snowflake,
    pub name: String,
    #[serde(rename = "parent_id")]
    pub category_id: Option<Snowflake>,
    pub rate_limit_per_user: u32,
    pub topic: Option<String>,
    pub position: u32,
    pub permission_overwrites: Vec<PermissionOverwrite>,
    pub nsfw: bool,
    pub theme_color: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct GuildForumData {
    pub id: Snowflake,
    pub last_message_id: Option<Snowflake>,
    pub flags: u64,
    pub guild_id: Snowflake,
    pub name: String,
    #[serde(rename = "parent_id")]
    pub category_id: Snowflake,
    pub rate_limit_per_user: u32,
    pub topic: Option<String>,
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

#[derive(Deserialize)]
struct ChannelTypeHelper {
    #[serde(rename = "type")]
    channel_type: u8,
}

impl<'de> serde::Deserialize<'de> for Channel {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let value = Value::deserialize(d)?;
        let helper = ChannelTypeHelper::deserialize(&value).map_err(D::Error::custom)?;

        let channel_type = ChannelType::from_u8(helper.channel_type)
            .ok_or_else(|| Error::custom(format!("invalid or unimplemented channel type: {}", helper.channel_type)))?;

        Ok(
            match channel_type {
                ChannelType::GuildText => Channel::GuildText(GuildTextData::deserialize(value).map_err(D::Error::custom)?),
                ChannelType::Dm => Channel::Dm(DmData::deserialize(value).map_err(D::Error::custom)?),
                ChannelType::GuildVoice => Channel::GuildVoice(GuildVoiceData::deserialize(value).map_err(D::Error::custom)?),
                ChannelType::GroupDm => Channel::GroupDm(GroupDmData::deserialize(value).map_err(D::Error::custom)?),
                ChannelType::GuildCategory => Channel::GuildCategory(GuildCategoryData::deserialize(value).map_err(D::Error::custom)?),
                ChannelType::GuildAnnouncement => Channel::GuildAnnouncement(GuildAnnouncementData::deserialize(value).map_err(D::Error::custom)?),
                ChannelType::GuildForum => Channel::GuildForum(GuildForumData::deserialize(value).map_err(D::Error::custom)?),
                ChannelType::AnnouncementThread => Channel::AnnouncementThread,
                ChannelType::PublicThread => Channel::PublicThread,
                ChannelType::PrivateThread => Channel::PrivateThread,
                ChannelType::GuildStageVoice => Channel::GuildStageVoice,
                ChannelType::GuildDirectory => Channel::GuildDirectory,
                ChannelType::GuildMedia => Channel::GuildMedia,
            }
        )
    }
}

impl ID for Channel {
    fn id(&self) -> &Snowflake {
        match self {
            Channel::Dm(dm_data) => &dm_data.id,
            Channel::GroupDm(group_dm_data) => &group_dm_data.id,
            Channel::GuildText(guild_text_data) => &guild_text_data.id,
            Channel::GuildVoice(guild_voice_data) => &guild_voice_data.id,
            Channel::GuildCategory(guild_category_data) => &guild_category_data.id,
            Channel::GuildAnnouncement(guild_announcement_data) => &guild_announcement_data.id,
            Channel::GuildForum(guild_forum_data) => &guild_forum_data.id,
            Channel::AnnouncementThread => todo!(),
            Channel::PublicThread => todo!(),
            Channel::PrivateThread => todo!(),
            Channel::GuildStageVoice => todo!(),
            Channel::GuildDirectory => todo!(),
            Channel::GuildMedia => todo!(),
        }
    }
}