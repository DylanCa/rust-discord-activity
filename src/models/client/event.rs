use serde::Serialize;

use crate::models::activity::Activity;

/// List of EventData to send to Discord - Currently only supports Activity.
#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum EventData {
    Activity(Activity),
}

/// List of EventName to send to Discord - Currently only supports Activity.
pub enum EventName {
    Activity,
}

impl EventName {
    pub fn as_string(&self) -> String {
        match self {
            EventName::Activity => "activity".into(),
        }
    }
}
