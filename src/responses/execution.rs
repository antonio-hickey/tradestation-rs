use crate::{Error, Route};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// The TradeStation API Response for running risk vs reward
/// analysis on an options trade.
pub struct GetExecutionRoutesRespRaw {
    routes: Option<Vec<Route>>,
    /// The error type from TradeStation's API
    ///
    /// NOTE: Will be None if there was no error
    error: Option<String>,
    /// The error message from TradeStation's API
    ///
    /// NOTE: Will be None if there was no error
    message: Option<String>,
}
#[derive(Debug)]
/// The TradeStation API Response for fetching symbol details.
pub struct GetExecutionRoutesResp {
    /// The option expirations for a symbol.
    pub routes: Option<Vec<Route>>,
    /// The error from TradeStation's API.
    ///
    /// NOTE: Will be None if there was no error.
    pub error: Option<Error>,
}
impl From<GetExecutionRoutesRespRaw> for GetExecutionRoutesResp {
    fn from(raw: GetExecutionRoutesRespRaw) -> Self {
        println!("{raw:?}");
        let error_enum =
            if let (Some(err), Some(msg)) = (raw.error.as_deref(), raw.message.as_deref()) {
                Error::from_tradestation_api_error(err, msg)
            } else {
                None
            };

        GetExecutionRoutesResp {
            routes: raw.routes,
            error: error_enum,
        }
    }
}