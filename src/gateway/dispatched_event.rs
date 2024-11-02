use message::MessageAttachment;
use serde::Deserialize;
use crate::model::*;

use crate::model;
use model::{guild::GuildMemberData, message::{GeneralMessageData, Message, MessageComponent, MessageEmbed}, voice::UserVoiceState, channel::Channel, user::{activity::*, GatewayUserData, MainUserData, RelationshipAddEvent, RelationshipAddType, RelationshipRemoveEvent, RelationshipRemoveType, UserDataLimited}};

use super::{error::GatewayError, events::GatewayReceiveEventRaw};

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "SCREAMING_SNAKE_CASE"))]
#[serde(tag = "t", content = "d")]
pub enum DispatchedEvent {
    /// Message was created
    MessageCreate {
        #[serde(flatten)]
        message: Message
    },
    /// Message was edited
    MessageUpdate {
        #[serde(rename = "id")]
        message_id: Snowflake,
        channel_id: Snowflake,
        guild_id: Snowflake,
        embeds: Option<Vec<MessageEmbed>>,
        components: Option<Vec<MessageComponent>>,
        attachments: Option<Vec<MessageAttachment>>,
    },
    /// Contains the initial state information
    // TODO! this contains a bit too much information. Maybe handle full deserialization another day. 
    Ready {
        user: GatewayUserData
    },
    /// Defines the heartbeat interval
    Hello {
        
    },
    /// Response to Resume
    Resumed {
        
    },
    /// Server is going away, client should reconnect to gateway and resume
    Reconnect {
        
    },
    /// Failure response to Identify or Resume or invalid active session
    InvalidSession {
        
    },
    /// Application command permission was updated
    ApplicationCommandPermissionsUpdate {
        
    },
    /// Auto Moderation rule was created
    AutoModerationRuleCreate {
        
    },
    /// Auto Moderation rule was updated
    AutoModerationRuleUpdate {
        
    },
    /// Auto Moderation rule was deleted
    AutoModerationRuleDelete {
        
    },
    /// Auto Moderation rule was triggered and an action was executed (e.g. a message was blocked)
    AutoModerationActionExecution {
        
    },
    /// New guild channel created
    ChannelCreate {
        
    },
    /// Channel was updated
    ChannelUpdate {
        #[serde(flatten)]
        channel: Channel,
        version: u64
    },
    /// Channel was deleted
    ChannelDelete {
        #[serde(flatten)]
        channel: Channel,
        version: u64  
    },
    /// Message was pinned or unpinned
    ChannelPinsUpdate {
        
    },
    /// Thread created, also sent when being added to a private thread
    ThreadCreate {
        
    },
    /// Thread was updated
    ThreadUpdate {
        
    },
    /// Thread was deleted
    ThreadDelete {
        
    },
    /// Sent when gaining access to a channel, contains all active threads in that channel
    ThreadListSync {
        
    },
    /// Thread member for the current user was updated
    ThreadMemberUpdate {
        
    },
    /// Some user(s) were added to or removed from a thread
    ThreadMembersUpdate {
        
    },
    /// Entitlement was created
    EntitlementCreate {
        
    },
    /// Entitlement was updated or renewed
    EntitlementUpdate {
        
    },
    /// Entitlement was deleted
    EntitlementDelete {
        
    },
    /// Lazy-load for unavailable guild, guild became available, or user joined a new guild
    GuildCreate {
        
    },
    /// Guild was updated
    GuildUpdate {
        
    },
    /// Guild became unavailable, or user left/was removed from a guild
    GuildDelete {
        
    },
    /// A guild audit log entry was created
    GuildAuditLogEntryCreate {
        
    },
    /// User was banned from a guild
    GuildBanAdd {
        
    },
    /// User was unbanned from a guild
    GuildBanRemove {
        
    },
    /// Guild emojis were updated
    GuildEmojisUpdate {
        
    },
    /// Guild stickers were updated
    GuildStickersUpdate {
        
    },
    /// Guild integration was updated
    GuildIntegrationsUpdate {
        
    },
    /// New user joined a guild
    GuildMemberAdd {
        
    },
    /// User was removed from a guild
    GuildMemberRemove {
        
    },
    /// Guild member was updated
    GuildMemberUpdate {
        
    },
    /// Response to Request Guild Members
    GuildMembersChunk {
        
    },
    /// Guild role was created
    GuildRoleCreate {
        
    },
    /// Guild role was updated
    GuildRoleUpdate {
        
    },
    /// Guild role was deleted
    GuildRoleDelete {
        
    },
    /// Guild scheduled event was created
    GuildScheduledEventCreate {
        
    },
    /// Guild scheduled event was updated
    GuildScheduledEventUpdate {
        
    },
    /// Guild scheduled event was deleted
    GuildScheduledEventDelete {
        
    },
    /// User subscribed to a guild scheduled event
    GuildScheduledEventUserAdd {
        
    },
    /// User unsubscribed from a guild scheduled event
    GuildScheduledEventUserRemove {
        
    },
    /// Guild integration was created
    IntegrationCreate {
        
    },
    /// Guild integration was updated
    IntegrationUpdate {
        
    },
    /// Guild integration was deleted
    IntegrationDelete {
        
    },
    /// User used an interaction, such as an Application Command
    InteractionCreate {
        
    },
    /// Invite to a channel was created
    InviteCreate {
        
    },
    /// Invite to a channel was deleted
    InviteDelete {
        
    },
    /// Message was deleted
    MessageDelete {
        
    },
    /// Multiple messages were deleted at once
    MessageDeleteBulk {
        
    },
    /// User reacted to a message
    MessageReactionAdd {
        
    },
    /// User removed a reaction from a message
    MessageReactionRemove {
        
    },
    /// All reactions were explicitly removed from a message
    MessageReactionRemoveAll {
        
    },
    /// All reactions for a given emoji were explicitly removed from a message
    MessageReactionRemoveEmoji {
        
    },
    /// User was updated
    PresenceUpdate {
        // activities: Vec<Activity>,
        // broadcast: Option<bool>,
        // guild_id: Option<Snowflake>,
        // user: UserDataLimited,
        // status: String
    },
    /// Stage instance was created
    StageInstanceCreate {
        
    },
    /// Stage instance was updated
    StageInstanceUpdate {
        
    },
    /// Stage instance was deleted or closed
    StageInstanceDelete {
        
    },
    /// User started typing in a channel
    TypingStart {
        
    },
    /// Properties about the user changed
    UserUpdate {
        
    },
    /// Someone joined, left, or moved a voice channel
    VoiceStateUpdate {
        #[serde(flatten)]
        new_state: UserVoiceState,
        guild_id: Snowflake,
        member: Option<GuildMemberData>
    },
    /// Guild's voice server was updated
    VoiceServerUpdate {
        
    },
    /// Guild channel webhook was created, updated, or deleted
    WebhooksUpdate {
        
    },
    /// User voted on a poll
    MessagePollVoteAdd {
        answer_id: u16,
        channel_id: Snowflake,
        guild_id: Snowflake,
        message_id: Snowflake,
        user_id: Snowflake,
    },
    /// User removed a vote on a poll
    MessagePollVoteRemove {
        
    },
    /* ========================================================================================= */
    /* ----- THE FOLLOWING ARE ONLY FOR USER ACCOUNTS, AND ARE NOT DEFIEND IN THE API DOCS ----- */
    /* ========================================================================================= */
    /// A call has been created in the user's dms somewhere. (user only)
    CallCreate {
        // TODO!
        // pub embedded_activities: [],
        channel_id: Snowflake,
        message_id: Snowflake,
        region: String,
        #[serde(rename = "ringing")]
        ringing_user_ids: Vec<String>,
        #[serde(rename = "voice_states")]
        user_voice_states: Vec<UserVoiceState>
    },
    // /// A friend request has been sent to or from the user. (user only)
    RelationshipAdd {
        #[serde(flatten)]
        relationship_add_event: RelationshipAddEvent,
    },
    /// A friend has been removed by the user or their friend. (user only)
    RelationshipRemove {
        #[serde(flatten)]
        relationship_remove_event: RelationshipRemoveEvent,
    },
    /// Someone joined/left a vc in a server (I think). (user only)
    VoiceChannelStatusUpdate {
        // TODO!
    },
    /// (user only)
    ConversationSummaryUpdate {
        
    },
    /// (user only)
    PassiveUpdateV2 {
        
    },
    /// Supplemental ready event (too huge to worry about tbh) (user only)
    ReadySupplemental {
        
    },
    /// A message has been sent and acknowledged. (user only)
    MessageAck {
        channel_id: Snowflake,
        flags: Option<u64>,
        last_viewed: Option<u64>,
        message_id: Snowflake,
        version: u64
    },
    UserSettingsProtoUpdate {

    },
    GuildApplicationCommandIndexUpdate {

    },
    ChannelUnreadUpdate {

    },
    ContentInventoryInboxStale {
        
    },
    AudioSettingsUpdate {

    }
}