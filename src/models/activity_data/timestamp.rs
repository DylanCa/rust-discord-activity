use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Timestamp {
    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<u128>,

    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<u128>,
}

impl Timestamp {
    pub fn new(start: Option<u128>,
               end: Option<u128>) -> Timestamp {
        Self {
            start,
            end,
        }
    }
}