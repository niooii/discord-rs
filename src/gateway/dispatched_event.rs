use serde::Deserialize;

use crate::message::MessageData;

use super::{error::GatewayError, events::GatewayReceiveEventRaw};

#[derive(Deserialize, Debug)]
// #[serde(rename_all(deserialize = "PascalCase"))]
pub enum DispatchedEvent {
    MessageCreate {
        message_data: MessageData
    },
    
}

impl DispatchedEvent {
    pub fn from_raw(data_raw: GatewayReceiveEventRaw) -> Result<Self, GatewayError> {
        let dispatch_type = data_raw.t.unwrap();   
        let data_json = data_raw.d;
        // TODO!  
        // https://stackoverflow.com/questions/45059538/how-to-deserialize-into-a-enum-variant-based-on-a-key-name
        // #[serde(flatten)]
        let dispatched_event: Result<Self, GatewayError> = match dispatch_type.as_str() {
            // Defines the heartbeat interval
            "HELLO" => {
                todo!()
            },
            // Contains the initial state information
            "READY" => {
                todo!()
            },
            // Response to Resume
            "RESUMED" => {
                todo!()
            },
            // Server is going away, client should reconnect to gateway and resume
            "RECONNECT" => {
                todo!()
            },
            // Failure response to Identify or Resume or invalid active session
            "INVALID_SESSION" => {
                todo!()
            },
            // Application command permission was updated
            "APPLICATION_COMMAND_PERMISSIONS_UPDATE" => {
                todo!()
            },
            // Auto Moderation rule was created
            "AUTO_MODERATION_RULE_CREATE" => {
                todo!()
            },
            // Auto Moderation rule was updated
            "AUTO_MODERATION_RULE_UPDATE" => {
                todo!()
            },
            // Auto Moderation rule was deleted
            "AUTO_MODERATION_RULE_DELETE" => {
                todo!()
            },
            // Auto Moderation rule was triggered and an action was executed
            "AUTO_MODERATION_ACTION_EXECUTION" => {
                todo!()
            },
            // New guild channel created
            "CHANNEL_CREATE" => {
                todo!()
            },
            // Channel was updated
            "CHANNEL_UPDATE" => {
                todo!()
            },
            // Channel was deleted
            "CHANNEL_DELETE" => {
                todo!()
            },
            // Message was pinned or unpinned
            "CHANNEL_PINS_UPDATE" => {
                todo!()
            },
            // Thread created, also sent when being added to a private thread
            "THREAD_CREATE" => {
                todo!()
            },
            // Thread was updated
            "THREAD_UPDATE" => {
                todo!()
            },
            // Thread was deleted
            "THREAD_DELETE" => {
                todo!()
            },
            // Sent when gaining access to a channel, contains all active threads in that channel
            "THREAD_LIST_SYNC" => {
                todo!()
            },
            // Thread member for the current user was updated
            "THREAD_MEMBER_UPDATE" => {
                todo!()
            },
            // Some user(s) were added to or removed from a thread
            "THREAD_MEMBERS_UPDATE" => {
                todo!()
            },
            // Entitlement was created
            "ENTITLEMENT_CREATE" => {
                todo!()
            },
            // Entitlement was updated or renewed
            "ENTITLEMENT_UPDATE" => {
                todo!()
            },
            // Entitlement was deleted
            "ENTITLEMENT_DELETE" => {
                todo!()
            },
            // Lazy-load for unavailable guild, guild became available, or user joined a new guild
            "GUILD_CREATE" => {
                todo!()
            },
            // Guild was updated
            "GUILD_UPDATE" => {
                todo!()
            },
            // Guild became unavailable, or user left/was removed from a guild
            "GUILD_DELETE" => {
                todo!()
            },
            // A guild audit log entry was created
            "GUILD_AUDIT_LOG_ENTRY_CREATE" => {
                todo!()
            },
            // User was banned from a guild
            "GUILD_BAN_ADD" => {
                todo!()
            },
            // User was unbanned from a guild
            "GUILD_BAN_REMOVE" => {
                todo!()
            },
            // Guild emojis were updated
            "GUILD_EMOJIS_UPDATE" => {
                todo!()
            },
            // Guild stickers were updated
            "GUILD_STICKERS_UPDATE" => {
                todo!()
            },
            // Guild integration was updated
            "GUILD_INTEGRATIONS_UPDATE" => {
                todo!()
            },
            // New user joined a guild
            "GUILD_MEMBER_ADD" => {
                todo!()
            },
            // User was removed from a guild
            "GUILD_MEMBER_REMOVE" => {
                todo!()
            },
            // Guild member was updated
            "GUILD_MEMBER_UPDATE" => {
                todo!()
            },
            // Response to Request Guild Members
            "GUILD_MEMBERS_CHUNK" => {
                todo!()
            },
            // Guild role was created
            "GUILD_ROLE_CREATE" => {
                todo!()
            },
            // Guild role was updated
            "GUILD_ROLE_UPDATE" => {
                todo!()
            },
            // Guild role was deleted
            "GUILD_ROLE_DELETE" => {
                todo!()
            },
            // Guild scheduled event was created
            "GUILD_SCHEDULED_EVENT_CREATE" => {
                todo!()
            },
            // Guild scheduled event was updated
            "GUILD_SCHEDULED_EVENT_UPDATE" => {
                todo!()
            },
            // Guild scheduled event was deleted
            "GUILD_SCHEDULED_EVENT_DELETE" => {
                todo!()
            },
            // User subscribed to a guild scheduled event
            "GUILD_SCHEDULED_EVENT_USER_ADD" => {
                todo!()
            },
            // User unsubscribed from a guild scheduled event
            "GUILD_SCHEDULED_EVENT_USER_REMOVE" => {
                todo!()
            },
            // Guild integration was created
            "INTEGRATION_CREATE" => {
                todo!()
            },
            // Guild integration was updated
            "INTEGRATION_UPDATE" => {
                todo!()
            },
            // Guild integration was deleted
            "INTEGRATION_DELETE" => {
                todo!()
            },
            // User used an interaction, such as an Application Command
            "INTERACTION_CREATE" => {
                todo!()
            },
            // Invite to a channel was created
            "INVITE_CREATE" => {
                todo!()
            },
            // Invite to a channel was deleted
            "INVITE_DELETE" => {
                todo!()
            },
            // Message was created
            "MESSAGE_CREATE" => {
                todo!()
            },
            // Message was edited
            "MESSAGE_UPDATE" => {
                todo!()
            },
            // Message was deleted
            "MESSAGE_DELETE" => {
                todo!()
            },
            // Multiple messages were deleted at once
            "MESSAGE_DELETE_BULK" => {
                todo!()
            },
            // User reacted to a message
            "MESSAGE_REACTION_ADD" => {
                todo!()
            },
            // User removed a reaction from a message
            "MESSAGE_REACTION_REMOVE" => {
                todo!()
            },
            // All reactions were explicitly removed from a message
            "MESSAGE_REACTION_REMOVE_ALL" => {
                todo!()
            },
            // All reactions for a given emoji were explicitly removed from a message
            "MESSAGE_REACTION_REMOVE_EMOJI" => {
                todo!()
            },
            // User was updated
            "PRESENCE_UPDATE" => {
                todo!()
            },
            // Stage instance was created
            "STAGE_INSTANCE_CREATE" => {
                todo!()
            },
            // Stage instance was updated
            "STAGE_INSTANCE_UPDATE" => {
                todo!()
            },
            // Stage instance was deleted or closed
            "STAGE_INSTANCE_DELETE" => {
                todo!()
            },
            // User started typing in a channel
            "TYPING_START" => {
                todo!()
            },
            // Properties about the user changed
            "USER_UPDATE" => {
                todo!()
            },
            // Someone joined, left, or moved a voice channel
            "VOICE_STATE_UPDATE" => {
                todo!()
            },
            // Guild's voice server was updated
            "VOICE_SERVER_UPDATE" => {
                todo!()
            },
            // Guild channel webhook was created, updated, or deleted
            "WEBHOOKS_UPDATE" => {
                todo!()
            },
            // User voted on a poll
            "MESSAGE_POLL_VOTE_ADD" => {
                todo!()
            },
            // User removed a vote on a poll
            "MESSAGE_POLL_VOTE_REMOVE" => {
                todo!()
            },
            _ => {
                Err(
                    GatewayError::Custom { text: format!("Unknown dispatch type: {}", dispatch_type) }
                )
            }
        };
        dispatched_event
    }
}