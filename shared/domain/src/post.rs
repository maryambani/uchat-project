use nutype::nutype;

use crate::UserFacingError;

#[nutype(validate(max_len = 30))]
#[derive(AsRef, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Headline(String);

impl Headline {
    pub const MAX_CHARS: usize = 30;
}

impl UserFacingError for HeadlineError {
    fn formatted_error(&self) -> &'static str {
        match self {
            HeadlineError::TooLong => "Headline is too long. Must be at most 30 characters.",
        }
    }
}

#[nutype(validate(max_len = 100))]
#[derive(AsRef, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Message(String);

impl Message {
    pub const MAX_CHARS: usize = 100;
}

impl UserFacingError for MessageError {
    fn formatted_error(&self) -> &'static str {
        match self {
            MessageError::TooLong => "Message is too long. Must be at most 100 characters.",
        }
    }
}
