use serde::Serialize;

use crate::models::activity::Activity;

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum EventData {
    Activity(Activity),
}

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
