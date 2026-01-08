use std::borrow::Cow;
use std::fmt;
use std::fmt::{Display, Formatter};

/// Custom Error list for the library.
#[derive(Debug)]
pub enum Error {
    DiscordNotFound,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let msg = match self {
            Error::DiscordNotFound => {
                Cow::Borrowed("Could not connect to client. Is Discord running ?")
            }
        };

        f.write_str(&msg)
    }
}
