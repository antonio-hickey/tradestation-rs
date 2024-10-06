use crate::market_data::SymbolDetails;
use crate::market_data::{
    OptionChain, OptionExpiration, OptionQuote, OptionRiskRewardAnalysis, OptionSpreadStrikes,
    OptionSpreadType,
};
use crate::{responses::stream, Error, MarketData::Bar};
use serde::{de, Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// The TradeStation API Response for running risk vs reward
/// analysis on an options trade.
pub struct GetOptionsRiskRewardRespRaw {
    /// Indicates whether the maximum gain can be infinite.
    pub max_gain_is_infinite: Option<bool>,
    /// The adjusted maximum gain (if it is not infinite).
    pub adjusted_max_gain: Option<String>,
    /// Indicates whether the maximum loss can be infinite.
    pub max_loss_is_infinite: Option<bool>,
    /// The adjusted maximum loss (if it is not infinite).
    pub adjusted_max_loss: Option<String>,
    /// Market price that the underlying security must reach
    /// for the trade to avoid a loss.
    pub breakeven_points: Option<Vec<String>>,
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
pub struct GetOptionsRiskRewardResp {
    /// The option expirations for a symbol.
    pub analysis: Option<OptionRiskRewardAnalysis>,
    /// The error from TradeStation's API.
    ///
    /// NOTE: Will be None if there was no error.
    pub error: Option<Error>,
}
impl From<GetOptionsRiskRewardRespRaw> for GetOptionsRiskRewardResp {
    fn from(raw: GetOptionsRiskRewardRespRaw) -> Self {
        let error_enum =
            if let (Some(err), Some(msg)) = (raw.error.as_deref(), raw.message.as_deref()) {
                Error::from_tradestation_api_error(err, msg)
            } else {
                None
            };

        // NOTE: If one of these is some they all are some, same vice versa.
        let analysis = if let (
            Some(max_gain_is_infinite),
            Some(adjusted_max_gain),
            Some(max_loss_is_infinite),
            Some(adjusted_max_loss),
            Some(breakeven_points),
        ) = (
            raw.max_gain_is_infinite,
            raw.adjusted_max_gain.as_deref(),
            raw.max_loss_is_infinite,
            raw.adjusted_max_loss.as_deref(),
            raw.breakeven_points.as_deref(),
        ) {
            Some(OptionRiskRewardAnalysis {
                max_gain_is_infinite,
                adjusted_max_gain: adjusted_max_gain.to_owned(),
                max_loss_is_infinite,
                adjusted_max_loss: adjusted_max_loss.to_owned(),
                breakeven_points: breakeven_points.to_vec(),
            })
        } else {
            None
        };

        GetOptionsRiskRewardResp {
            analysis,
            error: error_enum,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// The TradeStation API Response for fetching symbol details.
pub struct GetOptionExpirationsRespRaw {
    /// The option expirations for a symbol.
    pub expirations: Option<Vec<OptionExpiration>>,
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
pub struct GetOptionExpirationsResp {
    /// The option expirations for a symbol.
    pub expirations: Option<Vec<OptionExpiration>>,
    /// The error from TradeStation's API.
    ///
    /// NOTE: Will be None if there was no error.
    pub error: Option<Error>,
}
impl From<GetOptionExpirationsRespRaw> for GetOptionExpirationsResp {
    fn from(raw: GetOptionExpirationsRespRaw) -> Self {
        let error_enum =
            if let (Some(err), Some(msg)) = (raw.error.as_deref(), raw.message.as_deref()) {
                Error::from_tradestation_api_error(err, msg)
            } else {
                None
            };

        GetOptionExpirationsResp {
            expirations: raw.expirations,
            error: error_enum,
        }
    }
}

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

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// The TradeStation API Response for running risk vs reward
/// analysis on an options trade.
pub struct OptionSpreadStrikesRespRaw {
    /// Indicates whether the maximum gain can be infinite.
    pub spread_type: Option<OptionSpreadType>,
    pub strikes: Option<Vec<Vec<String>>>,
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
pub struct OptionSpreadStrikesResp {
    /// The option expirations for a symbol.
    pub spread_strikes: Option<OptionSpreadStrikes>,
    /// The error from TradeStation's API.
    ///
    /// NOTE: Will be None if there was no error.
    pub error: Option<Error>,
}
impl From<OptionSpreadStrikesRespRaw> for OptionSpreadStrikesResp {
    fn from(raw: OptionSpreadStrikesRespRaw) -> Self {
        println!("{raw:?}");
        let error_enum =
            if let (Some(err), Some(msg)) = (raw.error.as_deref(), raw.message.as_deref()) {
                Error::from_tradestation_api_error(err, msg)
            } else {
                None
            };

        // NOTE: If one of these is some they all are some, same vice versa.
        let spread_strikes =
            if let (Some(spread_type), Some(strikes)) = (raw.spread_type, raw.strikes) {
                Some(OptionSpreadStrikes {
                    spread_type,
                    strikes,
                })
            } else {
                None
            };

        OptionSpreadStrikesResp {
            spread_strikes,
            error: error_enum,
        }
    }
}

/// The TradeStation API Response for streaming an options chain.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum StreamOptionChainResp {
    /// The main response which contains the option chain data.
    OptionChain(Box<self::OptionChain>),
    /// Periodic signal to know the connection is still alive.
    Heartbeat(stream::Heartbeat),
    /// Signal sent on state changes in the stream (closed, opened, paused, resumed).
    Status(stream::StreamStatus),
    /// Response for when an error was encountered, with details on the error.
    Error(stream::ErrorResp),
}
impl<'de> Deserialize<'de> for StreamOptionChainResp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;

        if value.get("Open").is_some() {
            // Deserialize into the `OptionChain` variant
            let option_chain = serde_json::from_value(value).map_err(de::Error::custom)?;
            Ok(StreamOptionChainResp::OptionChain(Box::new(option_chain)))
        } else if value.get("StreamStatus").is_some() {
            // Deserialize into the `Status` variant
            let status = serde_json::from_value(value).map_err(de::Error::custom)?;
            Ok(StreamOptionChainResp::Status(status))
        } else if value.get("Heartbeat").is_some() {
            // Deserialize into the `Heartbeat` variant
            let heartbeat = serde_json::from_value(value).map_err(de::Error::custom)?;
            Ok(StreamOptionChainResp::Heartbeat(heartbeat))
        } else {
            // Default to `Error` variant if nothing else matches
            let error = serde_json::from_value(value).map_err(de::Error::custom)?;
            Ok(StreamOptionChainResp::Error(error))
        }
    }
}

/// The TradeStation API Response for streaming an options quotes.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum StreamOptionQuotesResp {
    /// The main response which contains the option chain data.
    OptionQuotes(Box<self::OptionQuote>),
    /// Periodic signal to know the connection is still alive.
    Heartbeat(stream::Heartbeat),
    /// Signal sent on state changes in the stream (closed, opened, paused, resumed).
    Status(stream::StreamStatus),
    /// Response for when an error was encountered, with details on the error.
    Error(stream::ErrorResp),
}
impl<'de> Deserialize<'de> for StreamOptionQuotesResp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;

        if value.get("Open").is_some() {
            // Deserialize into the `OptionQuotes` variant
            let option_quote = serde_json::from_value(value).map_err(de::Error::custom)?;
            Ok(StreamOptionQuotesResp::OptionQuotes(Box::new(option_quote)))
        } else if value.get("StreamStatus").is_some() {
            // Deserialize into the `Status` variant
            let status = serde_json::from_value(value).map_err(de::Error::custom)?;
            Ok(StreamOptionQuotesResp::Status(status))
        } else if value.get("Heartbeat").is_some() {
            // Deserialize into the `Heartbeat` variant
            let heartbeat = serde_json::from_value(value).map_err(de::Error::custom)?;
            Ok(StreamOptionQuotesResp::Heartbeat(heartbeat))
        } else {
            // Default to `Error` variant if nothing else matches
            let error = serde_json::from_value(value).map_err(de::Error::custom)?;
            Ok(StreamOptionQuotesResp::Error(error))
        }
    }
}
