use crate::models::activity_data::{
    activity_flag::ActivityFlag, activity_type::ActivityType, asset::Asset, button::Button,
    emoji::Emoji, party::Party, secret::Secret, timestamp::Timestamp,
};
use serde::Serialize;

/// Represents a Discord Activity object to be send to Discord application.
/// See <https://discord.com/developers/docs/game-sdk/activities#data-models> for more information.
#[derive(Serialize, Debug)]
pub struct Activity {
    /// Name of the Discord Application - Read Only.
    #[serde(skip_serializing)]
    name: String,

    /// Type of Activity - Will be discarded by Discord App.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    activity_type: Option<i8>,

    /// Livestream URL, accepts only Twitch and Youtube links - Will be discarded by Discord App.
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,

    /// Time of creation of the Activity.
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<u128>,

    /// Timestamps for the Activity. Used to set the "elapsed / remaining" countdown on Discord Activity.
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamps: Option<Timestamp>,

    /// ID of Discord Application provided when instantiating DiscordClient.
    #[serde(skip_serializing_if = "Option::is_none")]
    application_id: Option<i32>,

    /// First line of Discord Activity.
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<String>,

    /// Second line of Discord Activity.
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<String>,

    /// Sets a custom Emoji on the Discord Activity - Will be discarded by Discord App.
    #[serde(skip_serializing_if = "Option::is_none")]
    emoji: Option<Emoji>,

    /// Adds a player count after the State.
    #[serde(skip_serializing_if = "Option::is_none")]
    party: Option<Party>,

    /// Activity Large image and Small image.
    #[serde(skip_serializing_if = "Option::is_none")]
    assets: Option<Asset>,

    /// Adds a Secret URI to the activity to enable Chat Join messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    secrets: Option<Secret>,

    /// Whether activity is in an Instance context, like an ongoing match.
    #[serde(skip_serializing_if = "Option::is_none")]
    instance: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    flags: Option<i8>,

    /// List of buttons added at the bottom of the Activity. Up to 2 buttons are supported by Discord.
    #[serde(skip_serializing_if = "Option::is_none")]
    buttons: Option<Vec<Button>>,
}

impl Activity {
    /// Instantiates a new Activity.
    pub fn new() -> Activity {
        Self {
            name: "".to_string(),
            activity_type: None,
            url: None,
            created_at: None,
            timestamps: None,
            application_id: None,
            details: None,
            state: None,
            emoji: None,
            party: None,
            assets: None,
            secrets: None,
            instance: None,
            flags: None,
            buttons: None,
        }
    }

    /// Sets a name for the Activity - Will be discarded by Discord App.
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }

    /// Sets a new ActivityType - Will be discarded by Discord App.
    pub fn set_activity_type(&mut self, activity_type: Option<ActivityType>) -> &mut Self {
        match activity_type {
            Some(val) => self.activity_type = Some(val as i8),
            _ => self.activity_type = None,
        }

        self
    }

    /// Sets a streaming URL.
    pub fn set_url(&mut self, url: Option<String>) -> &mut Self {
        self.url = url;
        self
    }

    /// Sets a created_at for the Activity.
    pub fn set_created_at(&mut self, created_at: Option<u128>) -> &mut Self {
        self.created_at = created_at;
        self
    }

    /// Sets new Timestamps for the Activity.
    pub fn set_timestamps(&mut self, timestamps: Option<Timestamp>) -> &mut Self {
        self.timestamps = timestamps;
        self
    }

    /// Sets an Application ID for the Activity - Will be discarded by Discord App.
    pub fn set_application_id(&mut self, application_id: Option<i32>) -> &mut Self {
        self.application_id = application_id;
        self
    }

    /// Sets the details for the current Activity.
    pub fn set_details(&mut self, details: Option<String>) -> &mut Self {
        self.details = details;
        self
    }

    /// Sets a state for the current Activity.
    pub fn set_state(&mut self, state: Option<String>) -> &mut Self {
        self.state = state;
        self
    }

    /// Sets an emoji for the current Activity.
    pub fn set_emoji(&mut self, emoji: Option<Emoji>) -> &mut Self {
        self.emoji = emoji;
        self
    }

    /// Sets the party count for the current Activity.
    pub fn set_party(&mut self, party: Option<Party>) -> &mut Self {
        self.party = party;
        self
    }

    /// Sets the Image Assets for the current Activity.
    pub fn set_assets(&mut self, assets: Option<Asset>) -> &mut Self {
        self.assets = assets;
        self
    }

    /// Sets a Secret for the current Activity.
    pub fn set_secrets(&mut self, secrets: Option<Secret>) -> &mut Self {
        self.secrets = secrets;
        self
    }

    /// Sets the instance boolean for the current Activity.
    pub fn set_instance(&mut self, instance: Option<bool>) -> &mut Self {
        self.instance = instance;
        self
    }

    /// Sets the flags for the current Activity.
    pub fn set_flags(&mut self, flag: Option<ActivityFlag>) -> &mut Self {
        match flag {
            Some(val) => self.flags = Some(val as i8),
            _ => self.flags = None,
        }

        self
    }

    /// Sets the Buttons for the current Activity. Up to 2 buttons are supported by Discord.
    pub fn set_buttons(&mut self, buttons: Option<Vec<Button>>) -> &mut Self {
        self.buttons = buttons;
        self
    }
}
