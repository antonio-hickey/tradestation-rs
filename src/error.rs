/// TradeStation API Client Error
#[derive(Debug)]
pub enum Error {
    InvalidToken,
    AccountNotFound,
    Request(reqwest::Error),
}
/// Implement display trait for `Error`
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InvalidToken => write!(f, "Invalid `Token` may be expired, bad, or `None`"),
            Self::AccountNotFound => {
                write!(f, "Couldn't find an account registered to you with that id")
            }
            Self::Request(e) => write!(f, "{e:?}"),
        }
    }
}
/// Implement error conversion (`reqwest::Error` -> `Error`)
impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::Request(err)
    }
}
