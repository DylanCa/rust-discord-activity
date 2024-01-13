use serde::Serialize;

/// Contains start and end time for an Activity.
/// Must be in Milliseconds since UNIX_EPOCH time.
/// If only Start is set and is in the past, it will display "xx:xx elapsed"
/// Otherwise if End is set, it will display "xx:xx remaining"
#[derive(Serialize, Debug)]
pub struct Timestamp {
    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<u128>,

    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<u128>,
}

impl Timestamp {
    pub fn new(start: Option<u128>, end: Option<u128>) -> Timestamp {
        Self { start, end }
    }
}
