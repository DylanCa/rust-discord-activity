pub enum Commands {
    SetActivity
}

impl Commands {
    pub fn as_string(&self) -> String {
        match self {
            Commands::SetActivity => "SET_ACTIVITY".into(),
        }
    }
}
