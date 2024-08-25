/// TradeStation API Client Error
#[derive(Debug)]
pub enum Error {
    Request(reqwest::Error),
}
/// Implement display trait for `Error`
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
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
