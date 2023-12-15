use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
pub struct Me {
    id: String,
    username: String,
    avatar: String,
    discriminator: String,
    public_flags: u64,
    premium_type: u64,
    flags: u64,
    banner: Option<String>,
    accent_color: Option<String>,
    global_name: Option<String>,
    avatar_decoration_data: Option<String>,
    banner_color: Option<String>,
    mfa_enabled: bool,
    locale: String,
    email: String,
    verified: bool,
    phone: Option<String>,
    nsfw_allowed: bool,
    linked_users: Vec<String>,
    bio: String,
    authenticator_types: Vec<String>,
}

// impl getters
impl Me {
    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn username(&self) -> &String {
        &self.username
    }

    pub fn avatar(&self) -> &String {
        &self.avatar
    }

    pub fn discriminator(&self) -> &String {
        &self.discriminator
    }

    pub fn public_flags(&self) -> u64 {
        self.public_flags
    }

    pub fn premium_type(&self) -> u64 {
        self.premium_type
    }

    pub fn flags(&self) -> u64 {
        self.flags
    }

    pub fn banner(&self) -> &Option<String> {
        &self.banner
    }

    pub fn accent_color(&self) -> &Option<String> {
        &self.accent_color
    }

    pub fn global_name(&self) -> &Option<String> {
        &self.global_name
    }

    pub fn avatar_decoration_data(&self) -> &Option<String> {
        &self.avatar_decoration_data
    }

    pub fn banner_color(&self) -> &Option<String> {
        &self.banner_color
    }

    pub fn mfa_enabled(&self) -> bool {
        self.mfa_enabled
    }

    pub fn locale(&self) -> &String {
        &self.locale
    }

    pub fn email(&self) -> &String {
        &self.email
    }

    pub fn verified(&self) -> bool {
        self.verified
    }

    pub fn phone(&self) -> &Option<String> {
        &self.phone
    }

    pub fn nsfw_allowed(&self) -> bool {
        self.nsfw_allowed
    }

    pub fn linked_users(&self) -> &Vec<String> {
        &self.linked_users
    }

    pub fn bio(&self) -> &String {
        &self.bio
    }

    pub fn authenticator_types(&self) -> &Vec<String> {
        &self.authenticator_types
    }
}

#[derive(Debug)]
pub struct User {
    id: String,
    username: String,
    avatar: String,
    discriminator: String,
    public_flags: u64,
    extra_info: Option<ExtraInfo>, // ExtraInfo as an Option
}

impl User {
    // Getter methods for User fields

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn username(&self) -> &String {
        &self.username
    }

    pub fn avatar(&self) -> &String {
        &self.avatar
    }

    pub fn discriminator(&self) -> &String {
        &self.discriminator
    }

    pub fn public_flags(&self) -> u64 {
        self.public_flags
    }

    pub fn extra_info(&self) -> &Option<ExtraInfo> {
        &self.extra_info
    }

    // Setter method for extra_info
    pub fn set_extra_info(&mut self, extra_info: Option<ExtraInfo>) {
        self.extra_info = extra_info;
    }
}

// custom deserialize for User
impl<'de> Deserialize<'de> for User {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct UserHelper {
            id: String,
            username: String,
            avatar: String,
            discriminator: String,
            public_flags: u64,
            premium_type: Option<u64>,
            flags: u64,
            banner: Option<String>,
            accent_color: Option<u64>,
            banner_color: Option<String>,
        }

        let helper = UserHelper::deserialize(deserializer)?;

        let mut extra_info: Option<ExtraInfo> = None;

        if helper.premium_type.is_some() {
            extra_info = Some(ExtraInfo {
                premium_type: helper.premium_type.unwrap(),
                flags: helper.flags,
                banner: helper.banner,
                accent_color: helper.accent_color,
                banner_color: helper.banner_color,
            });
        }

        Ok(User {
            id: helper.id,
            username: helper.username,
            avatar: helper.avatar,
            discriminator: helper.discriminator,
            public_flags: helper.public_flags,
            extra_info: extra_info, // Set ExtraInfo
        })
    }
}

// Define the ExtraInfo struct with additional fields
#[derive(Debug, Deserialize)]
pub struct ExtraInfo {
    premium_type: u64,
    flags: u64,
    banner: Option<String>,
    accent_color: Option<u64>, // Change to u64 for accent_color
    banner_color: Option<String>,
}

impl ExtraInfo {
    // Getter methods for ExtraInfo fields
    pub fn premium_type(&self) -> u64 {
        self.premium_type
    }

    pub fn flags(&self) -> u64 {
        self.flags
    }

    pub fn banner(&self) -> &Option<String> {
        &self.banner
    }

    pub fn accent_color(&self) -> &Option<u64> {
        &self.accent_color
    }

    pub fn banner_color(&self) -> &Option<String> {
        &self.banner_color
    }
}
