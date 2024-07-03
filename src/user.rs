use num::FromPrimitive;
use num_derive::FromPrimitive;
use serde::{Deserialize, Deserializer};
use serde_json::Value;
use serde_repr::Deserialize_repr;
use time::OffsetDateTime;
use serde::{de::Error, Deserialize};

#[derive(Debug, Deserialize)]
pub struct MainUserData {
    pub id: String,
    pub username: String,
    pub avatar: Option<String>,
    pub discriminator: String,
    pub public_flags: u64,
    pub premium_type: u64,
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

#[derive(Deserialize, Debug)]
pub struct UserData {
    pub id: String,
    pub username: String,
    // This is the avatar hash. 
    pub avatar: Option<String>,
    pub discriminator: String,
    pub public_flags: Option<u64>,
    // pub extra_info: Option<ExtraInfo>, // ExtraInfo as an Option
}

#[derive(Deserialize, Debug)]
pub struct AcceptedFriendRequest {
    pub nickname: Option<String>,
    pub should_notify: bool,
    #[serde(rename = "since")]
    pub friend_request_sent_date: OffsetDateTime,
    #[serde(rename = "user")]
    pub other_user: UserData
}

#[derive(Deserialize, Debug)]
pub struct IncomingFriendRequest {
    pub nickname: Option<String>,
    pub should_notify: bool,
    #[serde(rename = "since")]
    pub friend_request_sent_date: OffsetDateTime,
    #[serde(rename = "user")]
    pub from_user: UserData
}

#[derive(Deserialize, Debug)]
pub struct OutgoingFriendRequest {
    pub nickname: Option<String>,
    pub should_notify: bool,
    #[serde(rename = "since")]
    pub friend_request_sent_date: OffsetDateTime,
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
pub enum RelationshipAddInfo {
    Accepted(AcceptedFriendRequest),
    NewIncoming(IncomingFriendRequest),
    NewOutgoing(OutgoingFriendRequest)
}

impl<'de> serde::Deserialize<'de> for RelationshipAddInfo {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let value = Value::deserialize(d)?;
        let radd_type = value.get("type").unwrap();
        let radd_type = radd_type.as_u64().unwrap();

        let relationship_add_info = match FromPrimitive::from_u8(radd_type as u8).unwrap() {
            RelationshipAddType::Accepted => {
                let accepted_data = AcceptedFriendRequest::deserialize(value)
                .map_err(D::Error::custom)?;
                RelationshipAddInfo::Accepted(accepted_data)
            },
            RelationshipAddType::NewIncoming => {
                let incoming_data = IncomingFriendRequest::deserialize(value)
                .map_err(D::Error::custom)?;
                RelationshipAddInfo::NewIncoming(incoming_data)
            },
            RelationshipAddType::NewOutgoing => {
                let outgoing_data = OutgoingFriendRequest::deserialize(value)
                .map_err(D::Error::custom)?;
                RelationshipAddInfo::NewOutgoing(outgoing_data)
            },
        };

        Ok(relationship_add_info)
    }
}

// TODO! everythign here 
#[derive(Debug, FromPrimitive)]
#[repr(u8)]
pub enum RelationshipRemoveType {
    Removed = 1,
    /// wtf discord
    DeclinedIncomingOrOtherPersonCancelsOutgoing = 3,
    CancelOutgoing = 4,
}

// // Define the ExtraInfo struct with additional fields
// #[derive(Debug, Deserialize)]
// pub struct ExtraInfo {
//     bot: Option<bool>,
//     premium_type: Option<u64>,
//     flags: Option<u64>,
//     banner: Option<String>,
//     accent_color: Option<u64>, // Change to u64 for accent_color
//     banner_color: Option<String>,
// }
