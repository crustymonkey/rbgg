use std::{error::Error, fmt};

#[derive(Debug)]
pub struct InvalidBGGType {
    invalid_type: String,
    msg: String,
}

impl InvalidBGGType {
    pub fn new(invalid_type: &str, msg: &str) -> Self {
        return Self {
            invalid_type: invalid_type.to_string(),
            msg: msg.to_string()
        };
    }
}

impl Error for InvalidBGGType {}

impl fmt::Display for InvalidBGGType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(
            f,
            "InvalidBGGType: {{ type: {}, message: {} }}",
            &self.invalid_type, &self.msg
        );
    }
}

