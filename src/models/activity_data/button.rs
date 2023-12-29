use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Button {
    label: String,
    url: String,
}

impl Button {
    pub fn new(label: String, url: String) -> Button {
        Self {
            label,
            url
        }
    }
}