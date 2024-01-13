use crate::models::client::event::{EventData, EventName};
use serde::Serialize;

/// List of OpCode to send to Discord App through IPC.
pub enum OpCode {
    HANDSHAKE,
    MESSAGE,
}

/// Payload object used to encapsulate data to send to Discord Client.
#[derive(Serialize, Debug)]
pub struct Payload {
    pub event_name: String,
    pub event_data: EventData,
}

impl Payload {
    pub fn new(event_name: EventName, event_data: EventData) -> Self {
        Self {
            event_name: event_name.as_string(),
            event_data,
        }
    }
}
