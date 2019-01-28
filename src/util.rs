use failure::{Error, Fail};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};
use url::percent_encoding::{utf8_percent_encode, DEFAULT_ENCODE_SET};


pub fn format_param<T: Display>(key: &str, value: T) -> String {
    format!("{}={}", key, value)
}
 
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    reason:String,
    message:String
}

#[derive(Debug, Fail)]
pub enum TagError {
    #[fail(display = "Empty clan tag")]
    EmptyTag,
    #[fail(display = "Missing `#` : {}", tag)]
    MissingHash { tag: String },
    #[fail(display = "Invalid Tag {}", tag)]
    NonAlphaNumericCharacter { tag: String },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchResponse<T: Serialize> {
    items: Vec<T>,
    #[serde(skip_deserializing)]
    paging: Paging,
}

#[derive(Debug, Serialize, Clone, Default)]
pub struct Paging {
    cursors: Vec<()>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag(String);

impl Tag {
    pub fn new(tag: &str) -> Result<Tag, Error> {
        if tag.is_empty() {
            return Err(TagError::EmptyTag.into());
        }

        if &tag[0..1] != "#" {
            return Err(TagError::MissingHash {
                tag: tag.to_string(),
            }
            .into());
        }

        if !(tag.chars().skip(1).all(Tag::validation)) {
            return Err(TagError::NonAlphaNumericCharacter {
                tag: tag.to_string(),
            }
            .into());
        }

        Ok(Tag(utf8_percent_encode(tag, DEFAULT_ENCODE_SET).collect()))
    }

    fn validation(c: char) -> bool {
        //Hashtags should only contain these characters
        //Numbers: 0, 2, 8, 9
        //Letters: P, Y, L, Q, G, R, J, C, U, V
        match c {
            '0' | '2' | '8' | '9' | 'P' | 'Y' | 'L' | 'Q' | 'G' | 'R' | 'J' | 'C' | 'U' | 'V' => {
                true
            }
            _ => false,
        }
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
