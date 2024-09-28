use crate::market_data::SymbolDetails;
use crate::{responses::stream, Error, MarketData::Bar};
use serde::{de, Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// The TradeStation API Response for fetching symbol details.
pub struct GetSymbolDetailsRespRaw {
    /// The symbol details.
    pub symbols: Option<Vec<SymbolDetails>>,
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
/// The TradeStation API Response for fetching symbol details.
pub struct GetSymbolDetailsResp {
    /// The symbol details.
    pub symbols: Option<Vec<SymbolDetails>>,
    /// The error from TradeStation's API.
    ///
    /// NOTE: Will be None if there was no error.
    pub error: Option<Error>,
}
impl From<GetSymbolDetailsRespRaw> for GetSymbolDetailsResp {
    fn from(raw: GetSymbolDetailsRespRaw) -> Self {
        let error_enum =
            if let (Some(err), Some(msg)) = (raw.error.as_deref(), raw.message.as_deref()) {
                Error::from_tradestation_api_error(err, msg)
            } else {
                None
            };

        GetSymbolDetailsResp {
            symbols: raw.symbols,
            error: error_enum,
        }
    }
}

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

/// The TradeStation API Response for streaming orders.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum StreamBarsResp {
    /// The main response which contains order data
    Bar(Box<self::Bar>),
    /// Periodic signal to know the connection is still alive
    Heartbeat(stream::Heartbeat),
    /// Signal sent on state changes in the stream (closed, opened, paused, resumed)
    Status(stream::StreamStatus),
    /// Response for when an error was encountered, with details on the error
    Error(stream::ErrorResp),
}
impl<'de> Deserialize<'de> for StreamBarsResp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;

        if value.get("Open").is_some() {
            // Deserialize into the `Bar` variant
            let order = serde_json::from_value(value).map_err(de::Error::custom)?;
            Ok(StreamBarsResp::Bar(Box::new(order)))
        } else if value.get("StreamStatus").is_some() {
            // Deserialize into the `Status` variant
            let status = serde_json::from_value(value).map_err(de::Error::custom)?;
            Ok(StreamBarsResp::Status(status))
        } else if value.get("Heartbeat").is_some() {
            // Deserialize into the `Heartbeat` variant
            let heartbeat = serde_json::from_value(value).map_err(de::Error::custom)?;
            Ok(StreamBarsResp::Heartbeat(heartbeat))
        } else {
            // Default to `Error` variant if nothing else matches
            let error = serde_json::from_value(value).map_err(de::Error::custom)?;
            Ok(StreamBarsResp::Error(error))
        }
    }
}
