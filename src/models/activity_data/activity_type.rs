use serde::Serialize;

#[derive(Serialize, Debug)]
pub enum ActivityType {
    GAME = 0,
    STREAMING = 1,
    LISTENING = 2,
    WATCHING = 3,
    CUSTOM = 4,
    COMPETING = 5,
}
