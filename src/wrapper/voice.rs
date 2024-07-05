use serde::Deserialize;
use time::OffsetDateTime;

use crate::guild::GuildMemberData;

#[derive(Deserialize, Debug)]
pub struct PrivateCallData {
    #[serde(with = "time::serde::iso8601::option")]
    pub ended_timestamp: Option<OffsetDateTime>,
    #[serde(rename = "participants")]
    pub participant_ids: Vec<String>
}

#[derive(Deserialize, Debug)]
pub struct UserVoiceState {
    channel_id: Option<String>,
    deaf: bool,
    mute: bool,
    #[serde(with = "time::serde::iso8601::option")]
    pub request_to_speak_timestamp: Option<OffsetDateTime>,
    self_deaf: bool,
    self_mute: bool,
    self_video: bool,
    session_id: String,
    suppress: bool,
    user_id: String,
}