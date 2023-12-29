use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Asset {
    #[serde(skip_serializing_if = "Option::is_none")]
    large_image: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    large_text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    small_image: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    small_text: Option<String>,
}

impl Asset {
    pub fn new(
        large_image: Option<String>,
        large_text: Option<String>,
        small_image: Option<String>,
        small_text: Option<String>,
    ) -> Asset {
        Self {
            large_image,
            large_text,
            small_image,
            small_text,
        }
    }
}
