use serde::Deserialize;
use crate::user::*;

// https://discord.com/developers/docs/resources/channel
#[derive(Deserialize, Debug)]
pub enum ChannelType {
    // type 0
    GuildText,
    // type 1
    Dm {
        id: String,
        last_message_id: Option<String>,
        flags: u64,
        recipients: Vec<UserData>,
        name: Option<String>,
        icon: Option<String>,
        owner_id: Option<String>,
    },
    // type 2
    GuildVoice,
    // type 3
    GroupDm,
    // type 4
    GuildCategory,
    // type 5
    GuildAnnouncement,
    // type 10
    AnnouncementThread,
    // type 11
    PublicThread,
    // type 12
    PrivateThread,
    // type 13
    GuildStageVoice,
    // type 14
    GuildDirectory,
    // type 15
    GuildForum,
    // type 16
    GuildMedia,
}