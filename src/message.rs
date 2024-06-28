use serde::Deserialize;

use crate::user::UserData;

#[derive(Deserialize, Debug)]
pub enum MessageData {
    Default {
        id: String,
        content: String,
        channel_id: String,
        author: UserData,
        attachments: Vec<Attachment>,
        embeds: Vec<String>, // Or replace String with an appropriate type
        mentions: Vec<String>, // Or replace String with an appropriate type
        mention_roles: Vec<String>, // Or replace String with an appropriate type
        pinned: bool,
        mention_everyone: bool,
        tts: bool,
        timestamp: String,
        edited_timestamp: Option<String>,
        flags: u64,
        components: Vec<String>, // Or replace String with an appropriate type
    }
}

#[derive(Debug, Deserialize)]
// consider using enums
pub struct Attachment {
    id: String,
    filename: String,
    size: u64,
    url: String,
    proxy_url: String,
    content_type: String,
    content_scan_version: u64,
    width: Option<u64>,
    height: Option<u64>,
    placeholder: Option<String>,
    placeholder_version: Option<u64>,
}

impl Attachment {
    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn filename(&self) -> &String {
        &self.filename
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn url(&self) -> &String {
        &self.url
    }

    pub fn proxy_url(&self) -> &String {
        &self.proxy_url
    }

    pub fn content_type(&self) -> &String {
        &self.content_type
    }

    pub fn content_scan_version(&self) -> u64 {
        self.content_scan_version
    }

    pub fn width(&self) -> Option<u64> {
        self.width
    }

    pub fn height(&self) -> Option<u64> {
        self.height
    }

    pub fn placeholder(&self) -> Option<&String> {
        self.placeholder.as_ref()
    }

    pub fn placeholder_version(&self) -> Option<u64> {
        self.placeholder_version
    }
}