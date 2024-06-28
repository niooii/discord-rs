use serde::{Deserialize, Deserializer};

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
    pub public_flags: u64,
    // pub extra_info: Option<ExtraInfo>, // ExtraInfo as an Option
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
