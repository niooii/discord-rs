#![allow(dead_code)]

use num_derive::FromPrimitive;
use serde::Deserialize;
use serde_json::error::Category;
use serde_json::Value;
use tokio_tungstenite::tungstenite::Message;
use zstd::stream::read::Decoder;
use zstd::zstd_safe::WriteBuf;
use crate::http;
use crate::http::validate_ratelimit;
use http::QueryError;
use serde::Serialize;
use crate::user::*;
use crate::channel;
use channel::*;
use crate::endpoints;
use crate::client::DiscordClient;
use futures_util::{future, pin_mut, SinkExt, StreamExt};
use num_traits::FromPrimitive;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::io::Cursor;
use std::io::Read;
use std::sync::Arc;
use std::sync::Mutex;
use serde_eetf::{to_bytes, from_bytes};
use anyhow::Result;

use super::dispatched_event::DispatchedEvent;
use super::error::GatewayError;
use super::events::*;

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
    guild_versions: HashMap<String, u32>,
}

pub struct GatewayConnection {
    queued_events: BinaryHeap<GatewayRecieveEvent>,

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
            let json: Value = serde_json::from_str(
                &message.unwrap().into_text().unwrap()
            ).unwrap();
            let recv_event = GatewayRecieveEvent::deserialize(json);
            if let Err(e) = &recv_event {
                // why must i do this chat
                if e.is_data() && e.to_string().contains("UnwantedEventError") {
                    eprintln!("recieved unwanted event but all good..");
                }
                // eprintln!("Recieved unwanted event type: {event_name}")
            }
            // let data = message.unwrap().into_data();
            // let mut decompresser = decompresser.lock().unwrap();
            // let mut decompressed: Vec<u8> = decompresser.decompress(data.as_slice(), 12000).expect("AWFAJFNAWF");
            println!("{:?}", recv_event);
            // println!("{data}");            
        })
    };

    pin_mut!(ws_to_stdout);
    ws_to_stdout.await;

    Ok(())
}