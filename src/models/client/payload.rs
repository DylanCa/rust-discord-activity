use serde::Serialize;
use crate::models::client::event::{EventData, EventName};

pub enum OpCode {
    HANDSHAKE,
    MESSAGE,
}

#[derive(Serialize, Debug)]
pub struct Payload {
    pub event_name: String,
    pub event_data: EventData,
}

impl Payload {
    pub fn new(event_name: EventName, event_data: EventData) -> Self {
        Self {
            event_name: event_name.as_string(),
            event_data
        }
    }
}