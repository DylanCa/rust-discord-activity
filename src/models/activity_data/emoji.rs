use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Emoji {
    name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    animated: Option<bool>
}

impl Emoji {
    pub fn new(name: String,
               id: Option<i64>,
               animated: Option<bool>) -> Emoji {
        Self {
            name,
            id,
            animated
        }
    }
}