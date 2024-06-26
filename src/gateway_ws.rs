#![allow(dead_code)]

use eetf::Term;
use num_derive::FromPrimitive;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use tokio_tungstenite::tungstenite::Message;
use zstd::stream::read::Decoder;
use zstd::zstd_safe::WriteBuf;
use crate::http;
use crate::http::validate_ratelimit;
use http::QueryError;
use serde::Serialize;
use crate::user_structs::*;
use crate::channel;
use channel::*;
use crate::endpoints;
use crate::context::DiscordContext;
use futures_util::{future, pin_mut, SinkExt, StreamExt};
use num_traits::FromPrimitive;
use std::collections::HashMap;
use std::io::Cursor;
use std::io::Read;
use std::sync::Arc;
use std::sync::Mutex;
use serde_eetf::{to_bytes, from_bytes};
use anyhow::Result;

pub enum GatewayError {
    Custom { text: String }
}

// https://discord.com/developers/docs/topics/opcodes-and-status-codes#gateway-gateway-close-event-codes
#[derive(Debug, FromPrimitive, PartialEq, Eq, Copy, Clone)]
#[repr(u32)]
pub enum GatewayOpCode {
    /*
    TYPE: Receive
    An event was dispatched.
    */
    Dispatch = 0,
    
    /*
    TYPE: Send/Receive
    Fired periodically by the client to keep the connection alive.
    */
    Heartbeat = 1,
    
    /*
    TYPE: Send
    Starts a new session during the initial handshake.
    */
    Identify = 2,
    
    /*
    TYPE: Send
    Update the client's presence.
    */
    PresenceUpdate = 3,
    
    /*
    TYPE: Send
    Used to join/leave or move between voice channels.
    */
    VoiceStateUpdate = 4,
    
    /*
    TYPE: Send
    Resume a previous session that was disconnected.
    */
    Resume = 6,
    
    /*
    TYPE: Receive
    You should attempt to reconnect and resume immediately.
    */
    Reconnect = 7,
    
    /*
    TYPE: Send
    Request information about offline guild members in a large guild.
    */
    RequestGuildMembers = 8,
    
    /*
    TYPE: Receive
    The session has been invalidated. You should reconnect and identify/resume accordingly.
    */
    InvalidSession = 9,
    
    /*
    TYPE: Receive
    Sent immediately after connecting, contains the heartbeat_interval to use.
    */
    Hello = 10,
    
    /*
    TYPE: Receive
    Sent in response to receiving a heartbeat to acknowledge that it has been received.
    */
    HeartbeatAck = 11,
}
#[derive(Serialize, PartialEq, Debug)]
struct GatewaySendEventRaw
{
    op: u32,
    d: serde_json::Value,
}

#[derive(Deserialize, PartialEq, Debug)]
struct GatewayReceiveEventRaw
{
    t: Option<String>,
    op: u32,
    s: u32,
    d: serde_json::Value,
}

#[derive(Deserialize, PartialEq, Debug)]
// #[serde(rename_all(deserialize = "PascalCase"))]
// TODO! me in tommorow please move everything related to dispatch events into its own file its too much for this one thanks
pub enum GatewayDispatchEvent {

}

impl GatewayDispatchEvent {
    fn from_raw(data_raw: GatewayReceiveEventRaw) -> Result<Self, GatewayError> {
        let dispatch_type = data_raw.t.unwrap();   
        // TODO!  
        // https://stackoverflow.com/questions/45059538/how-to-deserialize-into-a-enum-variant-based-on-a-key-name
        // #[serde(flatten)]
        match dispatch_type.as_str() {
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
        }
    }
}

pub enum GatewayRecieveEvent {
    DispatchEvent{ e: GatewayDispatchEvent }
}

impl GatewayRecieveEvent {
    

    pub fn from_raw(data_raw: GatewayReceiveEventRaw) -> Result<Self, GatewayError> {
        match FromPrimitive::from_u32(data_raw.op).unwrap() {
            GatewayOpCode::Dispatch => {
                Ok(Self::into_dispatch_event(data_raw)?)
            }
            _ => {
                Err(
                    GatewayError::Custom {text: "Unhandled op code".to_string()}
                )
            }
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
struct Properties {
    os: String,
    browser: String,
    release_channel: String,
    client_version: String,
    os_version: String,
    os_arch: String,
    app_arch: String,
    system_locale: String,
    browser_user_agent: String,
    browser_version: String,
    client_build_number: u32,
    native_build_number: u32,
    client_event_source: Option<String>,
    design_id: u32,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
struct Presence {
    status: String,
    since: u64,
    activities: Vec<String>,
    afk: bool,
    broadcast: Option<String>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
struct ClientState {
    guild_versions: std::collections::HashMap<String, u32>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
struct D {
    token: String,
    capabilities: u32,
    properties: Properties,
    presence: Presence,
    compress: bool,
    client_state: ClientState,
}

impl GatewaySendEventRaw {
    pub fn login(token: String, capabilities: u32, properties: Properties, presence: Presence, compress: bool, client_state: ClientState) -> Result<Self> {
        use serde_json::Value;
        let mut map = HashMap::<String, Value>::new();
        map.insert("token".to_string(), Value::String(token));
        map.insert("capabilities".to_string(), Value::Number(capabilities.into()));
        map.insert("properties".to_string(), serde_json::to_value(properties)?);
        map.insert("presence".to_string(), serde_json::to_value(presence)?);
        map.insert("compress".to_string(), Value::Bool(compress));
        map.insert("client_state".to_string(), serde_json::to_value(client_state)?);
        let json = serde_json::to_value(map)?;
        Ok(
            Self {
                op: 2, 
                d: json
            }
        )
    }
}
pub async fn test(token: &str) -> Result<()> {
    // let cursor = Cursor::new(Vec::new());
    // let decoder = Arc::new(Mutex::new(Decoder::new(cursor)?));
    // TODO! "wss://gateway.discord.gg/?encoding=etf&v=9&compress=zstd-stream"
    let ws = "wss://gateway.discord.gg/?encoding=json&v=9";
    let (ws_stream, _) = tokio_tungstenite::connect_async(ws).await?;

    let (mut write, read) = ws_stream.split();
    
    let decompresser = Arc::new(Mutex::new(
        zstd::bulk::Decompressor::new()?
    ));

    let properties = Properties {
        os: "Windows".to_string(),
        browser: "Discord Client".to_string(),
        release_channel: "stable".to_string(),
        client_version: "1.0.9151".to_string(),
        os_version: "10.0.19045".to_string(),
        os_arch: "x64".to_string(),
        app_arch: "x64".to_string(),
        system_locale: "en-US".to_string(),
        browser_user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) discord/1.0.9151 Chrome/120.0.6099.291 Electron/28.2.10 Safari/537.36".to_string(),
        browser_version: "28.2.10".to_string(),
        client_build_number: 304683,
        native_build_number: 48891,
        client_event_source: None,
        design_id: 0,
    };

    let presence = Presence {
        status: "unknown".to_string(),
        since: 0,
        activities: vec![],
        afk: false,
        broadcast: None,
    };

    let client_state = ClientState {
        guild_versions: std::collections::HashMap::new(),
    };

    // Create a new GatewayLogin instance
    let gateway_login = GatewaySendEventRaw::login(
        token.to_string(),
        30717,
        properties,
        presence,
        false,
        client_state,
    )?;

    let gateway_login_json = serde_json::to_value(&gateway_login)?;

    write.send(Message::Text(gateway_login_json.to_string())).await?;

    let ws_to_stdout = {
        read.for_each(|message| async {
            let json = serde_json::to_value(
                message.unwrap().into_text().unwrap()
            ).unwrap();
            // let data = message.unwrap().into_data();
            // let mut decompresser = decompresser.lock().unwrap();
            // let mut decompressed: Vec<u8> = decompresser.decompress(data.as_slice(), 12000).expect("AWFAJFNAWF");
            println!("\n\n");
            println!("\n\n");
            println!("{:?}", json);
            println!("\n\n");
            println!("\n\n");
            // println!("{data}");            
        })
    };

    pin_mut!(ws_to_stdout);
    ws_to_stdout.await;

    Ok(())
}