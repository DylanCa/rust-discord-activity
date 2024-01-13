use serde::Serialize;

/// List of Activity Flags to send to Discord Client.
#[derive(Serialize, Debug)]
pub enum ActivityFlag {
    Instance = 1,
    Join = 2,
    Spectate = 4,
    JoinRequest = 8,
    Sync = 16,
    Play = 32,
    PartyPrivacyFriends = 64,
    PartyPrivacyVoiceChannel = 128,
    Embedded = 256,
}
