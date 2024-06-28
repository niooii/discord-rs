#![allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Serialize, PartialEq, Debug)]
pub struct GatewaySendEventRaw
{
    pub op: u32,
    pub d: serde_json::Value,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct GatewayReceiveEventRaw
{
    pub t: Option<String>,
    pub op: u32,
    pub s: u32,
    pub d: serde_json::Value,
}

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use anyhow::Result;

use super::dispatched_event::DispatchedEvent;
use super::error::GatewayError;

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

pub enum GatewayRecieveEvent {
    DispatchEvent{ e: DispatchedEvent }
}

impl GatewayRecieveEvent {
    

    pub fn from_raw(data_raw: GatewayReceiveEventRaw) -> Result<Self, GatewayError> {
        match FromPrimitive::from_u32(data_raw.op).unwrap() {
            GatewayOpCode::Dispatch => {
                todo!();
                // Ok(DispatchedEvent::from_raw(data_raw)?)
            }
            _ => {
                Err(
                    GatewayError::Custom {text: "Unhandled op code".to_string()}
                )
            }
        }
    }
}