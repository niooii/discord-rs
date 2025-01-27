use arboard::Clipboard;
use num::FromPrimitive;
use num_derive::FromPrimitive;
use serde::{de::Error, Deserialize};
use serde_json::Value;
use time::{Duration, OffsetDateTime};
use crate::model::{guild::{GuildMemberData, interaction::*}, user::UserData, voice::PrivateCallData};

use super::{Snowflake, ID};

#[derive(Deserialize, Debug)]
pub struct Emoji {
    animated: bool,
    id: Snowflake,
    name: String
}

/// Refer to the discord documentation for more info: 
/// https://discord.com/developers/docs/resources/channel#message-object-message-types
#[derive(FromPrimitive, Debug)]
#[repr(u8)]
enum MessageType {
    Default = 0,
    RecipientAdd = 1,
    RecipientRemove = 2,
    Call = 3,
    ChannelNameChange = 4,
    ChannelIconChange = 5,
    ChannelPinnedMessage = 6,
    UserJoin = 7,
    GuildBoost = 8,
    GuildBoostTier1 = 9,
    GuildBoostTier2 = 10,
    GuildBoostTier3 = 11,
    ChannelFollowAdd = 12,
    GuildDiscoveryDisqualified = 14,
    GuildDiscoveryRequalified = 15,
    GuildDiscoveryGracePeriodInitialWarning = 16,
    GuildDiscoveryGracePeriodFinalWarning = 17,
    ThreadCreated = 18,
    Reply = 19,
    ChatInputCommand = 20,
    ThreadStarterMessage = 21,
    GuildInviteReminder = 22,
    ContextMenuCommand = 23,
    AutoModerationAction = 24,
    RoleSubscriptionPurchase = 25,
    InteractionPremiumUpsell = 26,
    StageStart = 27,
    StageEnd = 28,
    StageSpeaker = 29,
    StageTopic = 31,
    GuildApplicationPremiumSubscription = 32,
    GuildIncidentAlertModeEnabled = 36,
    GuildIncidentAlertModeDisabled = 37,
    GuildIncidentReportRaid = 38,
    GuildIncidentReportFalseAlarm = 39,
    PurchaseNotification = 44,
}

// TODO! fill these types
#[derive(Deserialize, Debug)]
pub struct MessageAttachment {
    id: Snowflake,
}
#[derive(Deserialize, Debug)]
pub struct MessageComponent {
    r#type: u64
}
#[derive(Deserialize, Debug)]
pub struct MessageEmbed {
    r#type: String,
}

#[derive(Deserialize, Debug)]
pub struct GeneralMessageData {
    pub id: Snowflake,
    pub channel_id: Snowflake,
    #[serde(with = "time::serde::iso8601")]
    pub timestamp: OffsetDateTime,
    pub flags: u64,
    pub attachments: Vec<MessageAttachment>,
    pub embeds: Vec<MessageEmbed>,
    pub components: Vec<MessageComponent>,
}

#[derive(Deserialize, Debug)]
pub struct DefaultMessageData {
    #[serde(flatten)]
    pub general: GeneralMessageData,
    pub content: String,
    pub author: UserData,
    // pub mentions: Vec<String>,
    // pub mention_roles: Vec<String>,
    pub pinned: bool,
    pub mention_everyone: bool,
    pub tts: bool,
    #[serde(default, with = "time::serde::iso8601::option")]
    pub edited_timestamp: Option<OffsetDateTime>,
}

#[derive(Deserialize, Debug)]
pub struct UserJoinData {
    #[serde(flatten)]
    pub general: GeneralMessageData,
    #[serde(rename = "author")]
    pub user: UserData,
    /// This will be None if it is accessed from the referenced_message in a Reply,
    /// otherwise it will be Some.
    pub member: Option<GuildMemberData>,
    /// This will be none if the message was recieved from the Gateway API, as it can be
    /// accessed from the dispatched event variants themselves.
    /// Otherwise, it is guarenteed to be Some from the discord API.
    pub guild_id: Option<Snowflake>,
}

#[derive(Deserialize, Debug)]
/// I don't think you can reply with another message that's not the default message type. 
pub struct ReplyMessageData {
    #[serde(flatten)]
    pub message: DefaultMessageData,
    pub referenced_message: Option<Box<Message>>,
}

#[derive(Debug, Deserialize)]
pub struct ChatInputCommandData {
    #[serde(flatten)]
    pub general: GeneralMessageData,
    pub application_id: Snowflake,
    pub author: UserData,
    pub content: String,
    #[serde(default, with = "time::serde::iso8601::option")]
    pub edited_timestamp: Option<OffsetDateTime>,
    /// This will be none if the message was recieved from the Gateway API, as it can be
    /// accessed from the dispatched event variants themselves.
    /// Otherwise, it is guarenteed to be Some from the discord API.
    pub guild_id: Option<Snowflake>,
    pub interaction: Data,
    pub interaction_metadata: Metadata,
    pub member: Option<GuildMemberData>,
    pub mention_everyone: bool,
    pub mention_roles: Vec<String>,
    pub mentions: Vec<String>,
    pub nonce: String,
    pub pinned: bool,
    pub position: u64,
    pub tts: bool,
    pub r#type: u64,
    pub webhook_id: Snowflake,
}

#[derive(Deserialize, Debug)]
pub struct CallMessageData {
    #[serde(flatten)]
    pub general: GeneralMessageData,
    #[serde(rename = "author")]
    pub caller: UserData,
    pub call: PrivateCallData
}

#[derive(Debug)]
pub enum Message {
    Default(DefaultMessageData),
    Call(CallMessageData),
    UserJoin(UserJoinData),
    Reply(ReplyMessageData),
    ChatInputCommand(ChatInputCommandData),

    Unknown(GeneralMessageData)
}

#[derive(Deserialize)]
struct MessageTypeHelper {
    #[serde(rename = "type")]
    msg_type: u8
}

// TODO! FIX ATROCIOUSNESS
impl<'de> serde::Deserialize<'de> for Message {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let value = Value::deserialize(d)?;
        let helper = MessageTypeHelper::deserialize(&value).map_err(D::Error::custom)?;
        
        let message_type = MessageType::from_u8(helper.msg_type)
        .ok_or_else(|| Error::custom(format!("invalid or unimplemented message type: {}", helper.msg_type)))?;
    
        let message = match message_type {
            MessageType::Default => {
                let default_msg_data = DefaultMessageData::deserialize(value).map_err(D::Error::custom)?;
                Message::Default(default_msg_data)
            }
            // MessageType::RecipientAdd => todo!(),
            // MessageType::RecipientRemove => todo!(),
            MessageType::Call => {
                let call_msg_data = CallMessageData::deserialize(value).map_err(D::Error::custom)?;
                Message::Call(call_msg_data)
            },
            // MessageType::ChannelNameChange => todo!(),
            // MessageType::ChannelIconChange => todo!(),
            // MessageType::ChannelPinnedMessage => todo!(),
            MessageType::UserJoin => {
                // TODO! for some reason this new value
                // is missing the guild_id in its json for user
                let user_join_data = UserJoinData::deserialize(value).map_err(D::Error::custom)?;
                Message::UserJoin(user_join_data)
            },
            // MessageType::GuildBoost => todo!(),
            // MessageType::GuildBoostTier1 => todo!(),
            // MessageType::GuildBoostTier2 => todo!(),
            // MessageType::GuildBoostTier3 => todo!(),
            // MessageType::ChannelFollowAdd => todo!(),
            // MessageType::GuildDiscoveryDisqualified => todo!(),
            // MessageType::GuildDiscoveryRequalified => todo!(),
            // MessageType::GuildDiscoveryGracePeriodInitialWarning => todo!(),
            // MessageType::GuildDiscoveryGracePeriodFinalWarning => todo!(),
            // MessageType::ThreadCreated => todo!(),
            MessageType::Reply => {
                let msg_reply_data = ReplyMessageData::deserialize(value).map_err(D::Error::custom)?;
                Message::Reply(msg_reply_data)
            },
            MessageType::ChatInputCommand => {
                let chat_input_cmd_data = ChatInputCommandData::deserialize(value).map_err(D::Error::custom)?;
                Message::ChatInputCommand(chat_input_cmd_data)
            },
            // MessageType::ThreadStarterMessage => todo!(),
            // MessageType::GuildInviteReminder => todo!(),
            // MessageType::ContextMenuCommand => todo!(),
            // MessageType::AutoModerationAction => todo!(),
            // MessageType::RoleSubscriptionPurchase => todo!(),
            // MessageType::InteractionPremiumUpsell => todo!(),
            // MessageType::StageStart => todo!(),
            // MessageType::StageEnd => todo!(),
            // MessageType::StageSpeaker => todo!(),
            // MessageType::StageTopic => todo!(),
            // MessageType::GuildApplicationPremiumSubscription => todo!(),
            // MessageType::GuildIncidentAlertModeEnabled => todo!(),
            // MessageType::GuildIncidentAlertModeDisabled => todo!(),
            // MessageType::GuildIncidentReportRaid => todo!(),
            // MessageType::GuildIncidentReportFalseAlarm => todo!(),
            // MessageType::PurchaseNotification => todo!(),
            _ => {
                let general_msg_data = GeneralMessageData::deserialize(value).map_err(D::Error::custom)?;
                Message::Unknown(general_msg_data)
            },
        };
        Ok(message)
    }
}

impl ID for Message {
    fn id(&self) -> &Snowflake {
        match self {
            Message::Default(default_message_data) => &default_message_data.general.id,
            Message::Call(call_message_data) => &call_message_data.general.id,
            Message::UserJoin(user_join_data) => &user_join_data.general.id,
            Message::Reply(reply_message_data) => &reply_message_data.message.general.id,
            Message::ChatInputCommand(chat_input_command_data) => &chat_input_command_data.general.id,
            Message::Unknown(general_message_data) =>&general_message_data.id,
        }
    }
}