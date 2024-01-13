/// List of Commands to send through IPC to Discord Client.
/// Currently only supports SET_ACTIVITY.
pub enum Commands {
    SetActivity,
}

impl Commands {
    pub fn as_string(&self) -> String {
        match self {
            Commands::SetActivity => "SET_ACTIVITY".into(),
        }
    }
}
