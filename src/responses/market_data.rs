use crate::{Error, MarketData::Bar};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// The TradeStation API Response for fetching bars.
pub struct GetBarsRespRaw {
    /// The bars fetched from your query
    ///
    /// NOTE: Will be None if there was an error
    /// at TradeStation's API level.
    pub bars: Option<Vec<Bar>>,
    /// The error type from TradeStation's API
    ///
    /// NOTE: Will be None if there was no error
    pub error: Option<String>,
    /// The error message from TradeStation's API
    ///
    /// NOTE: Will be None if there was no error
    pub message: Option<String>,
}
#[derive(Debug)]
/// The TradeStation API Response for fetching bars.
pub struct GetBarsResp {
    /// The bars fetched from your query
    ///
    /// NOTE: Will be None if there was an error
    /// at TradeStation's API level.
    pub bars: Option<Vec<Bar>>,
    /// The error from TradeStation's API
    ///
    /// NOTE: Will be None if there was no error
    pub error: Option<Error>,
}
impl From<GetBarsRespRaw> for GetBarsResp {
    fn from(raw: GetBarsRespRaw) -> Self {
        let error_enum =
            if let (Some(err), Some(msg)) = (raw.error.as_deref(), raw.message.as_deref()) {
                Error::from_tradestation_api_error(err, msg)
            } else {
                None
            };

        GetBarsResp {
            bars: raw.bars,
            error: error_enum,
        }
    }
}
