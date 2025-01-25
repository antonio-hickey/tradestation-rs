use std::error::Error as StdErrorTrait;

#[derive(Debug)]
/// TradeStation API Client Error
pub enum Error {
    /// Issue with your current `Token` the `Client` is using.
    InvalidToken,
    /// An `Account` was not found for a given account id.
    AccountNotFound,
    /// A `Position` was not found for a given position id.
    PositionNotFound(String, String),
    /// An HTTP request error.
    Request(reqwest::Error),
    BoxedError(Box<dyn StdErrorTrait + Send + Sync>),
    /// Error while in stream
    StreamIssue(String),
    /// Use this to stop a stream connection.
    StopStream,
    /// Error with JSON serializing or deseializing.
    Json(serde_json::Error),
    /// No symbol set when one was required.
    SymbolNotSet,
    /// Account Id not set when one was required.
    AccountIdNotSet,
    /// Trade Action not set when one was required.
    TradeActionNotSet,
    /// Time In Force not set when one was required.
    TimeInForceNotSet,
    /// Order Type not set when one was required.
    OrderTypeNotSet,
    /// Quantity not set when one was required.
    QuantityNotSet,
    /// No Option legs set when they were required.
    OptionLegsNotSet,
    /// Order Requests not set when they're required.
    OrderRequestsNotSet,
    /// Order Group Type not set when it's required.
    OrderGroupTypeNotSet,
    /// TradeStation API Error for a bad request
    BadRequest(String),
    /// TradeStation API Error for an unauthorized request.
    Unauthorized(String),
    /// TradeStation API Error for a forbidden request.
    Forbidden(String),
    /// TradeStation API Error for too many requests.
    TooManyRequests(String),
    /// TradeStation API Error for an internal server error.
    InternalServerError(String),
    /// TradeStation API Error for a gateway timeout.
    GatewayTimeout(String),
    /// TradeStation API Error for an unkown error.
    UnknownTradeStationAPIError,
}
impl Error {
    /// Convert a error from the tradestation api to `Some(Error)` or `None` if not supported.
    pub fn from_tradestation_api_error(
        ts_api_err_type: &str,
        ts_api_err_msg: &str,
    ) -> Option<Error> {
        match ts_api_err_type {
            "BadRequest" => Some(Error::BadRequest(ts_api_err_msg.into())),
            "Unauthorized" => Some(Error::Unauthorized(ts_api_err_msg.into())),
            "Forbidden" => Some(Error::Forbidden(ts_api_err_msg.into())),
            "TooManyRequests" => Some(Error::TooManyRequests(ts_api_err_msg.into())),
            "InternalServerError" => Some(Error::InternalServerError(ts_api_err_msg.into())),
            "GatewayTimeout" => Some(Error::GatewayTimeout(ts_api_err_msg.into())),
            _ => None,
        }
    }
}
impl StdErrorTrait for Error {}
/// Implement display trait for `Error`
impl std::fmt::Display for Error {
    /// The error message display format
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InvalidToken => write!(f, "Invalid `Token` may be expired, bad, or `None`"),
            Self::AccountNotFound => {
                write!(f, "Couldn't find an account registered to you with that id")
            }
            Self::PositionNotFound(position_id, account_id) => {
                write!(
                    f,
                    "Couldn't find a position with id: {} in account with id: {}",
                    position_id, account_id
                )
            }
            Self::Request(e) => write!(f, "{e:?}"),
            Self::BoxedError(e) => write!(f, "{e:?}"),
            Self::StreamIssue(e) => write!(f, "Issue during stream: {e}"),
            Self::StopStream => write!(f, "WARNING: You've stopped a stream!"),
            Self::Json(e) => write!(f, "JSON Error: {e:?}"),
            Self::SymbolNotSet => write!(f, "ERROR: You need to set the symbol."),
            Self::OptionLegsNotSet => write!(f, "ERROR: You need to set the option legs."),
            Self::BadRequest(msg) => write!(f, "TradeStation API ERROR: {msg}"),
            Self::Unauthorized(msg) => write!(f, "TradeStation API ERROR: {msg}"),
            Self::Forbidden(msg) => write!(f, "TradeStation API ERROR: {msg}"),
            Self::TooManyRequests(msg) => write!(f, "TradeStation API ERROR: {msg}"),
            Self::InternalServerError(msg) => write!(f, "TradeStation API ERROR: {msg}"),
            Self::GatewayTimeout(msg) => write!(f, "TradeStation API ERROR: {msg}"),
            Self::UnknownTradeStationAPIError => {
                write!(f, "TradeStation API ERROR: Unknown Error.")
            }
            Self::AccountIdNotSet => write!(f, "ERROR: account_id not set when it's required."),
            Self::TradeActionNotSet => write!(f, "ERROR: trade_action not set when it's required."),
            Self::OrderTypeNotSet => write!(f, "ERROR: order_type not set when it's required."),
            Self::TimeInForceNotSet => {
                write!(f, "ERROR: time_in_force not set when it's required.")
            }
            Self::QuantityNotSet => write!(f, "ERROR: quantity not set when it's required."),
            Self::OrderRequestsNotSet => {
                write!(f, "ERROR: order requests not set when they're required.")
            }
            Self::OrderGroupTypeNotSet => {
                write!(f, "ERROR: order group type not set when it's required.")
            }
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
