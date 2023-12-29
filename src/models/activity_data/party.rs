use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Party {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<(i8, i8)>,
}

impl Party {
    pub fn new(id: Option<String>, size: Option<(i8, i8)>) -> Party {
        Self { id, size }
    }
}
