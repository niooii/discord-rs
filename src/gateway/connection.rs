#![allow(dead_code)]

use tokio::task::JoinHandle;
use serde::Deserialize;
use serde_json::Value;
use tokio_tungstenite::tungstenite::Message;
use tokio::time::sleep;
use serde::Serialize;
use futures_util::{future, pin_mut, SinkExt, StreamExt};
use std::collections::HashMap;
use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use anyhow::Result;

use super::dispatched_event::DispatchedEvent;
use super::error::GatewayError;
use super::events::*;

#[derive(Deserialize, Serialize, PartialEq, Debug)]
struct Properties {
    os: String,
    browser: String,
    // release_channel: String,
    // client_version: String,
    // os_version: String,
    os_arch: String,
    app_arch: String,
    system_locale: String,
    browser_user_agent: String,
    // browser_version: String,
    // client_build_number: u32,
    // native_build_number: u32,
    // client_event_source: Option<String>,
    // design_id: u32,
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
    curr_sequence: Arc<Mutex<u64>>,
    // will be 0 when heartbeats shouldn't be sent. 
    heartbeat_interval_ms: Arc<Mutex<u64>>,
    ws_data_read_loop: JoinHandle<()>,
    ws_heartbeat_send_loop: JoinHandle<()>,
    event_q: Arc<Mutex<VecDeque<DispatchedEvent>>>
}

impl GatewayConnection { 
    pub async fn new(token: &str) -> Result<GatewayConnection> {
        // TODO! "wss://gateway.discord.gg/?encoding=etf&v=9&compress=zstd-stream"
        let ws = "wss://gateway.discord.gg/?encoding=json&v=9";
        let (ws_stream, _) = tokio_tungstenite::connect_async(ws).await?;

        let (mut write, read) = ws_stream.split();
        
        let properties = Properties {
            os: "Windows".to_string(),
            browser: "Discord Client".to_string(),
            // release_channel: "stable".to_string(),
            // client_version: "1.0.9151".to_string(),
            // os_version: "10.0.19045".to_string(),
            os_arch: "x64".to_string(),
            app_arch: "x64".to_string(),
            system_locale: "en-US".to_string(),
            browser_user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) discord/1.0.9151 Chrome/120.0.6099.291 Electron/28.2.10 Safari/537.36".to_string(),
            // browser_version: "28.2.10".to_string(),
            // client_build_number: 304683,
            // native_build_number: 48891,
            // client_event_source: None,
            // design_id: 0,
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

        let curr_sequence = Arc::new(Mutex::new(0));
        let heartbeat_interval_ms = Arc::new(Mutex::new(0));
        let event_q = Arc::new(Mutex::new(VecDeque::new()));

        let connection = GatewayConnection {
            curr_sequence: curr_sequence.clone(),
            heartbeat_interval_ms: heartbeat_interval_ms.clone(),
            event_q: event_q.clone(),
            ws_data_read_loop: {
                let heartbeat_interval = heartbeat_interval_ms.clone();
                let curr_sequence = curr_sequence.clone();
                let event_q = event_q.clone();
                let read_task = tokio::spawn(
                    read.for_each(move |message| {
                    let heartbeat_interval = heartbeat_interval.clone();
                    let curr_sequence = curr_sequence.clone();
                    let event_q = event_q.clone();
                    async move {
                        let recv_data = message.unwrap().into_text().unwrap();
                        if recv_data.is_empty() {
                            println!("Data is empty.. returning..");
                            return;
                        }
                        let json: Value = serde_json::from_str(
                            &recv_data
                        ).unwrap();
                        let recv_event = GatewayRecieveEvent::deserialize(json);
                        match recv_event {
                            Ok(e) => {
                                let mut curr_sequence = curr_sequence.lock().unwrap();
                                let mut update_sequence = |common: &GatewayRecieveEventInfo| {
                                    *curr_sequence = common.sequence;
                                };
                                match e {
                                    // Heartbeats are handled automatically.
                                    GatewayRecieveEvent::Hello { heartbeat_info, common } => {
                                        update_sequence(&common);
                                        let mut heartbeat_interval = heartbeat_interval.lock().unwrap();
                                        *heartbeat_interval = heartbeat_info.heartbeat_interval;
                                    },
                                    // TODO! be sure to handle the RESUME event, as it sends a list of events
                                    // the only events that the user should be notified about.
                                    GatewayRecieveEvent::GeneralEvent { dispatched_event, common } => {
                                        update_sequence(&common);
                                        let mut event_q = event_q.lock().unwrap();
                                        event_q.push_back(dispatched_event);
                                    }
                                    GatewayRecieveEvent::HeartbeatAck { common } => {
                                        update_sequence(&common);
                                    },
                                    GatewayRecieveEvent::UnwantedEvent { common } => {
                                        update_sequence(&common);
                                    },
                                };
                            },
                            Err(e) => {
                                // why must i do this chat
                                if e.is_data() && e.to_string().contains("UnwantedEventError") {
                                    eprintln!("Ignoring unwanted event..");
                                }
                                // eprintln!("Recieved unwanted event type: {event_name}")
                            },
                        }
                    }
                    })
                );
        
                read_task
            },
            ws_heartbeat_send_loop: {
                let heartbeat_interval = heartbeat_interval_ms.clone();
                let curr_sequence = curr_sequence.clone();
                let heartbeat_task = tokio::spawn(async move {
                    'send_loop: loop {
                        // TODO! check if a heartbeat is sent immediately after sending the login data. 
                        let ms = match heartbeat_interval.lock() {
                            Ok(ms) => *ms,
                            Err(poison) => {
                                eprintln!("Websocket read thread has panicked, no longer sending heartbeats.");
                                break 'send_loop;
                            }
                        };

                        if ms != 0 {
                            let heartbeat_payload = {
                                let curr_sequence = curr_sequence.lock().unwrap();
                                serde_json::json!({
                                    "op": 1,
                                    "d": *curr_sequence
                                })
                            };

                            if let Err(e) = write.send(Message::Text(heartbeat_payload.to_string())).await {
                                eprintln!("Failed to send heartbeat: {:?}", e);
                            } else {
                                println!("heartbeat sent.");
                            }
                            sleep(Duration::from_millis(ms)).await;
                        }
                        
                        
                    }
                });

                heartbeat_task
            }
        };

        Ok(connection)
    }

    pub fn poll_events(&mut self) -> Option<DispatchedEvent> {
        let mut event_q = self.event_q.lock().unwrap();
        event_q.pop_front()
    }
    
    pub async fn wait_until_finish(self) {
        self.ws_data_read_loop.await.unwrap();
    }
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