use std::error::Error as StdErrorTrait;

/// TradeStation API Client Error
#[derive(Debug)]
pub enum Error {
    InvalidToken,
    AccountNotFound,
    Request(reqwest::Error),
    BoxedError(Box<dyn StdErrorTrait + Send + Sync>),
    StreamIssue(String),
    Json(serde_json::Error),
}
impl StdErrorTrait for Error {}
/// Implement display trait for `Error`
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InvalidToken => write!(f, "Invalid `Token` may be expired, bad, or `None`"),
            Self::AccountNotFound => {
                write!(f, "Couldn't find an account registered to you with that id")
            }
            Self::Request(e) => write!(f, "{e:?}"),
            Self::BoxedError(e) => write!(f, "{e:?}"),
            Self::StreamIssue(e) => write!(f, "Issue during stream: {e}"),
            Self::Json(e) => write!(f, "JSON Error: {e:?}"),
        }
    }
}
/// Implement error conversion (`reqwest::Error` -> `Error`)
impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::Request(err)
    }
}
/// Implement error conversion (`<Box<dyn StdErrorTrait + Send + Sync>>` -> `Error`)
impl From<Box<dyn StdErrorTrait + Send + Sync>> for Error {
    fn from(err: Box<dyn StdErrorTrait + Send + Sync>) -> Self {
        Error::BoxedError(err)
    }
}
/// Implement error conversion (`serde_json::Error` -> `Error`)
impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::Json(err)
    }
}
