use arboard::Clipboard;
use num::FromPrimitive;
use num_derive::FromPrimitive;
use serde::Deserialize;
use serde_json::Value;
use time::OffsetDateTime;

use crate::user::UserData;

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
    id: String,
}
#[derive(Deserialize, Debug)]
pub struct MessageComponent;
#[derive(Deserialize, Debug)]
pub struct MessageEmbed {
    r#type: String,
}

#[derive(Deserialize, Debug)]
pub struct DefaultMessageData {
    id: String,
    content: String,
    channel_id: String,
    author: UserData,
    attachments: Vec<MessageAttachment>,
    embeds: Vec<MessageEmbed>,
    // mentions: Vec<String>,
    // mention_roles: Vec<String>,
    pinned: bool,
    mention_everyone: bool,
    tts: bool,
    #[serde(with = "time::serde::iso8601")]
    timestamp: OffsetDateTime,
    #[serde(default, with = "time::serde::iso8601::option")]
    edited_timestamp: Option<OffsetDateTime>,
    flags: u64,
    components: Vec<MessageComponent>,
}

#[derive(Deserialize, Debug)]
/// I don't think you can reply with another message that's not the default message type. 
pub struct ReplyMessageData {
    #[serde(flatten)]
    message: DefaultMessageData,
    referenced_message: Option<Box<Message>>,
}

#[derive(Debug)]
pub enum Message {
    Default(DefaultMessageData),
    Reply(ReplyMessageData)
}

impl<'de> serde::Deserialize<'de> for Message {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let value = Value::deserialize(d)?;
        let message_type = value.get("type").unwrap();
        let message_type = message_type.as_u64().unwrap();
        let message_type = FromPrimitive::from_u8(message_type as u8).unwrap();
        let mut clipboard = Clipboard::new().unwrap();
        clipboard.set_text(serde_json::to_string_pretty(&value).unwrap());
        let message = match message_type {
            MessageType::Default => {
                let data = DefaultMessageData::deserialize(value).unwrap();
                Message::Default(data)
            }
            MessageType::RecipientAdd => todo!(),
            MessageType::RecipientRemove => todo!(),
            MessageType::Call => todo!(),
            MessageType::ChannelNameChange => todo!(),
            MessageType::ChannelIconChange => todo!(),
            MessageType::ChannelPinnedMessage => todo!(),
            MessageType::UserJoin => todo!(),
            MessageType::GuildBoost => todo!(),
            MessageType::GuildBoostTier1 => todo!(),
            MessageType::GuildBoostTier2 => todo!(),
            MessageType::GuildBoostTier3 => todo!(),
            MessageType::ChannelFollowAdd => todo!(),
            MessageType::GuildDiscoveryDisqualified => todo!(),
            MessageType::GuildDiscoveryRequalified => todo!(),
            MessageType::GuildDiscoveryGracePeriodInitialWarning => todo!(),
            MessageType::GuildDiscoveryGracePeriodFinalWarning => todo!(),
            MessageType::ThreadCreated => todo!(),
            MessageType::Reply => {
                let data = ReplyMessageData::deserialize(value).unwrap();
                Message::Reply(data)
            },
            MessageType::ChatInputCommand => todo!(),
            MessageType::ThreadStarterMessage => todo!(),
            MessageType::GuildInviteReminder => todo!(),
            MessageType::ContextMenuCommand => todo!(),
            MessageType::AutoModerationAction => todo!(),
            MessageType::RoleSubscriptionPurchase => todo!(),
            MessageType::InteractionPremiumUpsell => todo!(),
            MessageType::StageStart => todo!(),
            MessageType::StageEnd => todo!(),
            MessageType::StageSpeaker => todo!(),
            MessageType::StageTopic => todo!(),
            MessageType::GuildApplicationPremiumSubscription => todo!(),
            MessageType::GuildIncidentAlertModeEnabled => todo!(),
            MessageType::GuildIncidentAlertModeDisabled => todo!(),
            MessageType::GuildIncidentReportRaid => todo!(),
            MessageType::GuildIncidentReportFalseAlarm => todo!(),
            MessageType::PurchaseNotification => todo!(),
        };
        Ok(message)
    }
}