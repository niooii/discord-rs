use num_derive::FromPrimitive;
use serde::Deserialize;
use serde_repr::Deserialize_repr;
use time::OffsetDateTime;
use serde::de::Error;
use crate::impl_deserialize_uint_tags;

use super::Snowflake;

#[derive(Deserialize_repr, Debug)]
#[repr(u8)]
pub enum NitroType {
    None = 0,
    NitroClassic = 1,
    Nitro = 2,
    NitroBasic = 3
}

#[derive(Debug, Deserialize)]
pub struct MainUserData {
    pub id: Snowflake,
    pub username: String,
    pub avatar: Option<String>,
    pub discriminator: String,
    pub public_flags: u64,
    pub premium_type: NitroType,
    pub flags: u64,
    pub banner: Option<String>,
    pub accent_color: Option<u32>,
    pub global_name: Option<String>,
    pub avatar_decoration_data: Option<String>,
    pub banner_color: Option<String>,
    pub mfa_enabled: bool,
    pub locale: String,
    pub email: String,
    pub verified: bool,
    pub phone: Option<String>,
    pub nsfw_allowed: bool,
    pub linked_users: Vec<String>,
    pub bio: String,
    pub authenticator_types: Vec<String>,
}

/// User data that is sent from the Gateway connection, which contains slightly different data.
#[derive(Debug, Deserialize)]
pub struct GatewayUserData {
    pub id: Snowflake,
    pub username: String,
    pub avatar: Option<String>,
    pub discriminator: String,
    // pub public_flags: u64,
    pub premium_type: NitroType,
    pub flags: u64,
    pub banner: Option<String>,
    pub accent_color: Option<u32>,
    pub global_name: Option<String>,
    pub avatar_decoration_data: Option<String>,
    pub banner_color: Option<String>,
    pub mfa_enabled: bool,
    pub email: String,
    pub verified: bool,
    pub phone: Option<String>,
    pub nsfw_allowed: bool,
    pub bio: String,
}

#[derive(Deserialize, Debug)]
pub struct UserData {
    pub id: Snowflake,
    pub username: String,
    // This is the avatar hash. 
    pub avatar: Option<String>,
    pub discriminator: String,
    pub public_flags: Option<u64>,
}

/// Used in responses where the api returns a partial user payload, which always should contain the id.
#[derive(Deserialize, Debug)]
pub struct UserDataLimited {
    pub id: Snowflake,
    pub username: Option<String>,
    // This is the avatar hash. 
    pub avatar: Option<String>,
    pub discriminator: Option<String>,
    pub public_flags: Option<u64>,
}

/* =========================================== */
/* ----- RELATIONSHIP STRUCT DEFINITIONS ----- */
/* =========================================== */

#[derive(Deserialize, Debug)]
pub struct AcceptedFriendRequest {
    pub nickname: Option<String>,
    #[serde(default)]
    pub should_notify: Option<bool>,
    #[serde(rename = "since", with = "time::serde::iso8601")]
    pub friend_request_sent_date: OffsetDateTime,
    #[serde(rename = "user")]
    pub other_user: UserData
}

#[derive(Deserialize, Debug)]
pub struct IncomingFriendRequest {
    pub nickname: Option<String>,
    pub should_notify: bool,
    #[serde(rename = "since", with = "time::serde::iso8601")]
    pub friend_request_sent_date: OffsetDateTime,
    #[serde(rename = "user")]
    pub from_user: UserData
}

#[derive(Deserialize, Debug)]
pub struct OutgoingFriendRequest {
    pub nickname: Option<String>,
    #[serde(rename = "user")]
    pub to_user: UserData
}

#[derive(Debug, FromPrimitive)]
#[repr(u8)]
pub enum RelationshipAddType {
    Accepted = 1,
    NewIncoming = 3,
    NewOutgoing = 4,
}

#[derive(Debug)]
pub enum RelationshipAddEvent {
    Accepted(AcceptedFriendRequest),
    NewIncoming(IncomingFriendRequest),
    NewOutgoing(OutgoingFriendRequest)
}

impl_deserialize_uint_tags!(
    "type",
    RelationshipAddType,
    RelationshipAddEvent,
    {
        Accepted => AcceptedFriendRequest,
        NewIncoming => IncomingFriendRequest,
        NewOutgoing => OutgoingFriendRequest,
    }
);

#[derive(Deserialize, Debug)]
pub struct FriendRemoved {
    #[serde(rename = "id")]
    other_user_id: Snowflake,
    nickname: Option<String>,
    #[serde(rename = "since", with = "time::serde::iso8601")]
    friends_since: OffsetDateTime
}

#[derive(Deserialize, Debug)]
pub struct IncomingRequestDeclinedOrCanceled {
    #[serde(rename = "id")]
    other_user_id: Snowflake,
    nickname: Option<String>,
    #[serde(rename = "since", with = "time::serde::iso8601")]
    friend_request_sent_date: OffsetDateTime
}

#[derive(Deserialize, Debug)]
pub struct OutgoingRequestCanceled {
    #[serde(rename = "id")]
    other_user_id: Snowflake,
    nickname: Option<String>,
}

#[derive(Debug, FromPrimitive)]
#[repr(u8)]
pub enum RelationshipRemoveType {
    Removed = 1,
    /// wtf discord
    IncomingDeclinedOrCanceled = 3,
    OutgoingCanceled = 4,
}

#[derive(Debug)]
pub enum RelationshipRemoveEvent {
    Removed(FriendRemoved),
    IncomingDeclinedOrCanceled(IncomingRequestDeclinedOrCanceled),
    OutgoingCanceled(OutgoingRequestCanceled)
}

impl_deserialize_uint_tags!(
    "type",
    RelationshipRemoveType,
    RelationshipRemoveEvent,
    {
        Removed => FriendRemoved,
        IncomingDeclinedOrCanceled => IncomingRequestDeclinedOrCanceled,
        OutgoingCanceled => OutgoingRequestCanceled,
    }
);

/* ================================================== */
/* ----- ACTIVITY / PRESENCE STRUCT DEFINITIONS ----- */
/* ================================================== */

pub mod activity {
    use serde::Deserialize;
    use serde_repr::Deserialize_repr;
    use time::OffsetDateTime;

    use crate::model::Snowflake;

    #[derive(Deserialize, Debug)]
    pub struct Assets {
        pub large_image: String,
        pub large_text: String,
        pub small_image: String,
        pub small_text: String
    }

    #[derive(Deserialize, Debug)]
    pub struct Timestamps {
        #[serde(with = "crate::serde_utils::unix_millis")]
        pub start: OffsetDateTime,
        #[serde(default, with = "crate::serde_utils::unix_millis::option")]
        pub end: Option<OffsetDateTime>,
    }

    #[derive(Deserialize_repr, Debug)]
    #[repr(u8)]
    pub enum Type {
        Game = 0,
        Streaming = 1,
        Listening = 2,
        Watching = 3,
        Custom = 4,
        Competing = 5,
        ChannelStatus = 6
    }

    // TODO! add better support for activity stuff
    #[derive(Deserialize, Debug)]
    pub struct Activity {
        pub r#type: Type,
        pub application_id: Option<Snowflake>,
        // pub emoji: Option<Emoji>,
        pub name: Option<String>,
        #[serde(rename = "state")]
        pub text: Option<String>,
        pub details: Option<String>,
        pub assets: Option<Assets>,
        #[serde(with = "crate::serde_utils::unix_millis")]
        pub created_at: OffsetDateTime,
        pub timestamps: Option<Timestamps>,
        pub buttons: Option<Vec<String>>,
        pub id: Option<Snowflake>,
        pub session_id: Option<Snowflake>,
        pub url: Option<String>
    }
}