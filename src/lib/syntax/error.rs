use std::fmt;

pub struct Error {
    // TODO: Add position information
    msg: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error {
    pub fn new(msg: String) -> Error {
        Error { msg }
    }

    pub fn get_msg(&self) -> &str {
        &self.msg
    }
}