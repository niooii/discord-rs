#![allow(dead_code)]
use arboard::Clipboard;
use serde::{Deserialize, Serialize};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use anyhow::Result;
use serde_json::Value;
use super::dispatched_event::DispatchedEvent;
use super::error::GatewayError;
use serde::de::Error;

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
    // worry about sequence field later
    // pub s: u32,
    pub d: serde_json::Value,
}

// https://discord.com/developers/docs/topics/opcodes-and-status-codes#gateway-gateway-close-event-codes
#[derive(Debug, FromPrimitive, PartialEq, Eq, Copy, Clone)]
#[repr(u8)]
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

#[derive(Deserialize, Debug)]
pub struct HeartbeatInfo {
    pub heartbeat_interval: u64
}

#[derive(Debug, Clone, Copy)]
pub struct GatewayRecieveEventInfo {
    pub sequence: u64
}

#[derive(Debug)]
pub enum GatewayRecieveEvent {
    GeneralEvent {
        dispatched_event: DispatchedEvent, 
        common: GatewayRecieveEventInfo
    },
    /// Heartbeat sending is handled automatically.
    Hello {
        heartbeat_info: HeartbeatInfo, 
        common: GatewayRecieveEventInfo
    },
    HeartbeatAck,
}
 
impl<'de> serde::Deserialize<'de> for GatewayRecieveEvent {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let value = Value::deserialize(d)?;
        // TODO! comment out someday
        let recieved_json = serde_json::to_string_pretty(&value).unwrap();
        let opcode = value.get("op").unwrap();
        let opcode = opcode.as_u64().unwrap();
        let opcode = FromPrimitive::from_u8(opcode as u8).unwrap();
        let sequence = value.get("s").unwrap().as_u64().unwrap_or(0);
        // handled separately bc ownership stuff
        if opcode == GatewayOpCode::Dispatch {
            if value.get("d").unwrap().is_array() {
                return Err(
                    D::Error::custom(format!("UnwantedEventError: got event {}", value.get("t").unwrap().to_string()))
                );
            }
            let data = DispatchedEvent::deserialize(value);
            if let Ok(e) = data {
                return Ok(
                    GatewayRecieveEvent::GeneralEvent {
                        dispatched_event: e,
                        common: GatewayRecieveEventInfo { sequence }
                    }
                );
            } else {
                eprintln!("Error: {data:?}");
                let mut clipboard = Clipboard::new().expect("Failed to create clipboard.");
                clipboard.set_text(recieved_json).expect("Failed to copy recieved json to clipboard.");
                eprintln!("Recieved json has been copied to your clipboard.");
                panic!();
            }
        }
        let raw = GatewayReceiveEventRaw::deserialize(value).unwrap();
        let gateway_recv_event = match opcode {
            GatewayOpCode::Heartbeat => {
                todo!()
                // let data = HeartbeatInfo::deserialize(raw.d).unwrap();
                // GatewayRecieveEvent::Hello(data)
            },
            GatewayOpCode::Identify => todo!(),
            GatewayOpCode::PresenceUpdate => todo!(),
            GatewayOpCode::VoiceStateUpdate => todo!(),
            GatewayOpCode::Resume => todo!(),
            GatewayOpCode::Reconnect => todo!(),
            GatewayOpCode::RequestGuildMembers => todo!(),
            GatewayOpCode::InvalidSession => todo!(),
            GatewayOpCode::Hello => {
                let heartbeat_info = HeartbeatInfo::deserialize(raw.d).unwrap();
                let common = GatewayRecieveEventInfo {
                    sequence
                };
                GatewayRecieveEvent::Hello { 
                    heartbeat_info,
                    common,
                }
            },
            GatewayOpCode::HeartbeatAck => {
                GatewayRecieveEvent::HeartbeatAck
            },
            _ => {
                panic!("Invalid opcode not handled...");
            }
        };
        Ok(gateway_recv_event)
    }
}