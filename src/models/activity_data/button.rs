use serde::Serialize;

/// Simple structure containing a label and an URL to form a Discord Activity button.
#[derive(Serialize, Debug)]
pub struct Button {
    label: String,
    url: String,
}

impl Button {
    pub fn new(label: String, url: String) -> Button {
        Self { label, url }
    }
}
