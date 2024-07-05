use serde::Deserialize;
use time::OffsetDateTime;
use crate::UserData;

#[derive(Debug, Deserialize)]
pub struct GuildMemberData {
    pub avatar: Option<String>,
    #[serde(default, with = "time::serde::iso8601::option")]
    pub communication_disabled_until: Option<OffsetDateTime>,
    pub deaf: bool,
    pub flags: u64,
    #[serde(with = "time::serde::iso8601")]
    pub joined_at: OffsetDateTime,
    pub mute: bool,
    pub nick: Option<String>,
    pub pending: bool,
    pub premium_since: Option<String>,
    pub roles: Vec<String>,
    pub user: Option<UserData>
}

pub mod interaction {
    use serde::Deserialize;
    use super::GuildMemberData;
    use crate::UserData;

    /// Pretty sure users don't really need to know about this...
    #[derive(Debug, Deserialize)]
    pub struct Metadata {
        // pub authorizing_integration_owners: HashMap<String, String>,
        // pub id: String,
        // pub name: String,
        // pub r#type: u64,
        // pub user: UserData,
        // pub user_id: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct Data {
        pub id: String,
        pub member: GuildMemberData,
        pub name: String,
        pub r#type: u64,
        pub user: UserData,
    }
}

pub mod audit_log {
    use serde::Deserialize;
    
    #[derive(Deserialize, Debug)]
    pub enum EventType {
        /// Server settings were updated
        GuildUpdate = 1,
        /// Channel was created
        ChannelCreate = 10,
        /// Channel settings were updated
        ChannelUpdate = 11,
        /// Channel was deleted
        ChannelDelete = 12,
        /// Permission overwrite was added to a channel
        ChannelOverwriteCreate = 13,
        /// Permission overwrite was updated for a channel
        ChannelOverwriteUpdate = 14,
        /// Permission overwrite was deleted from a channel
        ChannelOverwriteDelete = 15,
        /// Member was removed from server
        MemberKick = 20,
        /// Members were pruned from server
        MemberPrune = 21,
        /// Member was banned from server
        MemberBanAdd = 22,
        /// Server ban was lifted for a member
        MemberBanRemove = 23,
        /// Member was updated in server
        MemberUpdate = 24,
        /// Member was added or removed from a role
        MemberRoleUpdate = 25,
        /// Member was moved to a different voice channel
        MemberMove = 26,
        /// Member was disconnected from a voice channel
        MemberDisconnect = 27,
        /// Bot user was added to server
        BotAdd = 28,
        /// Role was created
        RoleCreate = 30,
        /// Role was edited
        RoleUpdate = 31,
        /// Role was deleted
        RoleDelete = 32,
        /// Server invite was created
        InviteCreate = 40,
        /// Server invite was updated
        InviteUpdate = 41,
        /// Server invite was deleted
        InviteDelete = 42,
        /// Webhook was created
        WebhookCreate = 50,
        /// Webhook properties or channel were updated
        WebhookUpdate = 51,
        /// Webhook was deleted
        WebhookDelete = 52,
        /// Emoji was created
        EmojiCreate = 60,
        /// Emoji name was updated
        EmojiUpdate = 61,
        /// Emoji was deleted
        EmojiDelete = 62,
        /// Single message was deleted
        MessageDelete = 72,
        /// Multiple messages were deleted
        MessageBulkDelete = 73,
        /// Message was pinned to a channel
        MessagePin = 74,
        /// Message was unpinned from a channel
        MessageUnpin = 75,
        /// App was added to server
        IntegrationCreate = 80,
        /// App was updated (as an example, its scopes were updated)
        IntegrationUpdate = 81,
        /// App was removed from server
        IntegrationDelete = 82,
        /// Stage instance was created (stage channel becomes live)
        StageInstanceCreate = 83,
        /// Stage instance details were updated
        StageInstanceUpdate = 84,
        /// Stage instance was deleted (stage channel no longer live)
        StageInstanceDelete = 85,
        /// Sticker was created
        StickerCreate = 90,
        /// Sticker details were updated
        StickerUpdate = 91,
        /// Sticker was deleted
        StickerDelete = 92,
        /// Event was created
        GuildScheduledEventCreate = 100,
        /// Event was updated
        GuildScheduledEventUpdate = 101,
        /// Event was cancelled
        GuildScheduledEventDelete = 102,
        /// Thread was created in a channel
        ThreadCreate = 110,
        /// Thread was updated
        ThreadUpdate = 111,
        /// Thread was deleted
        ThreadDelete = 112,
        /// Permissions were updated for a command
        ApplicationCommandPermissionUpdate = 121,
        /// Auto Moderation rule was created
        AutoModerationRuleCreate = 140,
        /// Auto Moderation rule was updated
        AutoModerationRuleUpdate = 141,
        /// Auto Moderation rule was deleted
        AutoModerationRuleDelete = 142,
        /// Message was blocked by Auto Moderation
        AutoModerationBlockMessage = 143,
        /// Message was flagged by Auto Moderation
        AutoModerationFlagToChannel = 144,
        /// Member was timed out by Auto Moderation
        AutoModerationUserCommunicationDisabled = 145,
        /// Creator monetization request was created
        CreatorMonetizationRequestCreated = 150,
        /// Creator monetization terms were accepted
        CreatorMonetizationTermsAccepted = 151,
        /// Guild Onboarding Question was created
        OnboardingPromptCreate = 163,
        /// Guild Onboarding Question was updated
        OnboardingPromptUpdate = 164,
        /// Guild Onboarding Question was deleted
        OnboardingPromptDelete = 165,
        /// Guild Onboarding was created
        OnboardingCreate = 166,
        /// Guild Onboarding was updated
        OnboardingUpdate = 167,
        /// Guild Server Guide was created
        HomeSettingsCreate = 190,
        /// Guild Server Guide was updated
        HomeSettingsUpdate = 191,
    }
}