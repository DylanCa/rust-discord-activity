use serde::Serialize;

/// Contains Secrets URIs to join, spectate or instantiate a match through Discord Chat.
#[derive(Serialize, Debug)]
pub struct Secret {
    #[serde(skip_serializing_if = "Option::is_none")]
    join: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    spectate: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    instanced_match: Option<String>,
}

impl Secret {
    pub fn new(
        join: Option<String>,
        spectate: Option<String>,
        instanced_match: Option<String>,
    ) -> Secret {
        Self {
            join,
            spectate,
            instanced_match,
        }
    }
}
