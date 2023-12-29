use crate::models::activity_data::{
    activity_flag::ActivityFlag, activity_type::ActivityType, asset::Asset, button::Button,
    emoji::Emoji, party::Party, secret::Secret, timestamp::Timestamp,
};
use serde::Serialize;

/// Test Doc
#[derive(Serialize, Debug)]
pub struct Activity {
    #[serde(skip_serializing)]
    name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    activity_type: Option<i8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<u128>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamps: Option<Timestamp>,

    #[serde(skip_serializing_if = "Option::is_none")]
    application_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    emoji: Option<Emoji>,

    #[serde(skip_serializing_if = "Option::is_none")]
    party: Option<Party>,

    #[serde(skip_serializing_if = "Option::is_none")]
    assets: Option<Asset>,

    #[serde(skip_serializing_if = "Option::is_none")]
    secrets: Option<Secret>,

    #[serde(skip_serializing_if = "Option::is_none")]
    instance: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    flags: Option<i8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    buttons: Option<Vec<Button>>,
}

impl Activity {
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

    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }

    pub fn set_activity_type(&mut self, activity_type: Option<ActivityType>) -> &mut Self {
        match activity_type {
            Some(val) => self.activity_type = Some(val as i8),
            _ => self.activity_type = None,
        }

        self
    }

    pub fn set_url(&mut self, url: Option<String>) -> &mut Self {
        self.url = url;
        self
    }

    pub fn set_created_at(&mut self, created_at: Option<u128>) -> &mut Self {
        self.created_at = created_at;
        self
    }

    pub fn set_timestamps(&mut self, timestamps: Option<Timestamp>) -> &mut Self {
        self.timestamps = timestamps;
        self
    }

    pub fn set_application_id(&mut self, application_id: Option<i32>) -> &mut Self {
        self.application_id = application_id;
        self
    }

    pub fn set_details(&mut self, details: Option<String>) -> &mut Self {
        self.details = details;
        self
    }

    pub fn set_state(&mut self, state: Option<String>) -> &mut Self {
        self.state = state;
        self
    }

    pub fn set_emoji(&mut self, emoji: Option<Emoji>) -> &mut Self {
        self.emoji = emoji;
        self
    }

    pub fn set_party(&mut self, party: Option<Party>) -> &mut Self {
        self.party = party;
        self
    }

    pub fn set_assets(&mut self, assets: Option<Asset>) -> &mut Self {
        self.assets = assets;
        self
    }

    pub fn set_secrets(&mut self, secrets: Option<Secret>) -> &mut Self {
        self.secrets = secrets;
        self
    }

    pub fn set_instance(&mut self, instance: Option<bool>) -> &mut Self {
        self.instance = instance;
        self
    }

    pub fn set_flags(&mut self, flag: Option<ActivityFlag>) -> &mut Self {
        match flag {
            Some(val) => self.flags = Some(val as i8),
            _ => self.flags = None,
        }

        self
    }

    pub fn set_buttons(&mut self, buttons: Option<Vec<Button>>) -> &mut Self {
        self.buttons = buttons;
        self
    }
}
