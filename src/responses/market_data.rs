use crate::{
    market_data::{
        Bar, MarketDepthAggregates, MarketDepthQuotes, OptionChain, OptionExpiration, OptionQuote,
        OptionRiskRewardAnalysis, OptionSpreadStrikes, OptionSpreadType, Quote, SymbolDetails,
    },
    responses::{stream, ApiError},
    Error,
};
use serde::{de, Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// The TradeStation API Response for running risk vs reward
/// analysis on an options trade.
pub(crate) struct GetOptionsRiskRewardRespRaw {
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
pub(crate) struct GetOptionsRiskRewardResp {
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
                Some(Error::from_api_error(super::ApiError {
                    error: err.into(),
                    message: msg.into(),
                }))
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
pub(crate) struct GetOptionExpirationsRespRaw {
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
pub(crate) struct GetOptionExpirationsResp {
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
                Some(Error::from_api_error(ApiError {
                    error: err.into(),
                    message: msg.into(),
                }))
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
pub(crate) struct GetSymbolDetailsRespRaw {
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
pub(crate) struct GetSymbolDetailsResp {
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
                Some(Error::from_api_error(ApiError {
                    error: err.into(),
                    message: msg.into(),
                }))
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
pub(crate) struct GetBarsRespRaw {
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
pub(crate) struct GetBarsResp {
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
                Some(Error::from_api_error(ApiError {
                    error: err.into(),
                    message: msg.into(),
                }))
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
pub(crate) struct OptionSpreadStrikesRespRaw {
    /// The name of the spread type for these strikes.
    pub spread_type: Option<OptionSpreadType>,

    /// The strike prices for this spread type.
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
pub(crate) struct OptionSpreadStrikesResp {
    /// The option expirations for a symbol.
    pub spread_strikes: Option<OptionSpreadStrikes>,

    /// The error from TradeStation's API.
    ///
    /// NOTE: Will be None if there was no error.
    pub error: Option<Error>,
}
impl From<OptionSpreadStrikesRespRaw> for OptionSpreadStrikesResp {
    fn from(raw: OptionSpreadStrikesRespRaw) -> Self {
        let error_enum =
            if let (Some(err), Some(msg)) = (raw.error.as_deref(), raw.message.as_deref()) {
                Some(Error::from_api_error(ApiError {
                    error: err.into(),
                    message: msg.into(),
                }))
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

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// The TradeStation API Response for fetching quote snapshots.
pub(crate) struct GetQuoteSnapshotsRespRaw {
    /// The quotes snapshots.
    pub quotes: Option<Vec<Quote>>,

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
/// The TradeStation API Response for fetching quote snapshots.
pub(crate) struct GetQuoteSnapshotsResp {
    /// The quote snapshots.
    pub quotes: Option<Vec<Quote>>,

    /// The error from TradeStation's API.
    ///
    /// NOTE: Will be None if there was no error.
    pub error: Option<Error>,
}
impl From<GetQuoteSnapshotsRespRaw> for GetQuoteSnapshotsResp {
    fn from(raw: GetQuoteSnapshotsRespRaw) -> Self {
        let error_enum =
            if let (Some(err), Some(msg)) = (raw.error.as_deref(), raw.message.as_deref()) {
                Some(Error::from_api_error(ApiError {
                    error: err.into(),
                    message: msg.into(),
                }))
            } else {
                None
            };

        GetQuoteSnapshotsResp {
            quotes: raw.quotes,
            error: error_enum,
        }
    }
}

/// The TradeStation API Response for streaming quotes.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum StreamQuotesResp {
    /// The main response which contains the option chain data.
    Quote(Box<self::Quote>),

    /// Periodic signal to know the connection is still alive.
    Heartbeat(stream::Heartbeat),

    /// Signal sent on state changes in the stream (closed, opened, paused, resumed).
    Status(stream::StreamStatus),

    /// Response for when an error was encountered, with details on the error.
    Error(stream::ErrorResp),
}
impl<'de> Deserialize<'de> for StreamQuotesResp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;

        if value.get("Open").is_some() {
            // Deserialize into the `Quotes` variant
            let quote = serde_json::from_value(value).map_err(de::Error::custom)?;
            Ok(StreamQuotesResp::Quote(Box::new(quote)))
        } else if value.get("StreamStatus").is_some() {
            // Deserialize into the `Status` variant
            let status = serde_json::from_value(value).map_err(de::Error::custom)?;
            Ok(StreamQuotesResp::Status(status))
        } else if value.get("Heartbeat").is_some() {
            // Deserialize into the `Heartbeat` variant
            let heartbeat = serde_json::from_value(value).map_err(de::Error::custom)?;
            Ok(StreamQuotesResp::Heartbeat(heartbeat))
        } else {
            // Default to `Error` variant if nothing else matches
            let error = serde_json::from_value(value).map_err(de::Error::custom)?;
            Ok(StreamQuotesResp::Error(error))
        }
    }
}

/// The TradeStation API Response for streaming market depth quotes.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum StreamMarketDepthQuotesResp {
    /// The main response which contains the bid data.
    Quote(Box<self::MarketDepthQuotes>),

    /// Periodic signal to know the connection is still alive.
    Heartbeat(stream::Heartbeat),

    /// Signal sent on state changes in the stream (closed, opened, paused, resumed).
    Status(stream::StreamStatus),

    /// Response for when an error was encountered, with details on the error.
    Error(stream::ErrorResp),
}
impl<'de> Deserialize<'de> for StreamMarketDepthQuotesResp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;

        if value.get("Asks").is_some() {
            // Deserialize into the `Quote` variant
            let quote = serde_json::from_value(value).map_err(de::Error::custom)?;
            Ok(StreamMarketDepthQuotesResp::Quote(Box::new(quote)))
        } else if value.get("StreamStatus").is_some() {
            // Deserialize into the `Status` variant
            let status = serde_json::from_value(value).map_err(de::Error::custom)?;
            Ok(StreamMarketDepthQuotesResp::Status(status))
        } else if value.get("Heartbeat").is_some() {
            // Deserialize into the `Heartbeat` variant
            let heartbeat = serde_json::from_value(value).map_err(de::Error::custom)?;
            Ok(StreamMarketDepthQuotesResp::Heartbeat(heartbeat))
        } else {
            // Default to `Error` variant if nothing else matches
            let error = serde_json::from_value(value).map_err(de::Error::custom)?;
            Ok(StreamMarketDepthQuotesResp::Error(error))
        }
    }
}

/// The TradeStation API Response for streaming market depth aggregates.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum StreamMarketDepthAggregatesResp {
    /// The main response which contains the bid/ask data for
    /// a market depth level.
    Aggregate(Box<self::MarketDepthAggregates>),

    /// Periodic signal to know the connection is still alive.
    Heartbeat(stream::Heartbeat),

    /// Signal sent on state changes in the stream (closed, opened, paused, resumed).
    Status(stream::StreamStatus),

    /// Response for when an error was encountered, with details on the error.
    Error(stream::ErrorResp),
}
impl<'de> Deserialize<'de> for StreamMarketDepthAggregatesResp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        println!("{value:?}");

        if value.get("Asks").is_some() {
            // Deserialize into the `Quote` variant
            let quote = serde_json::from_value(value).map_err(de::Error::custom)?;
            Ok(StreamMarketDepthAggregatesResp::Aggregate(Box::new(quote)))
        } else if value.get("StreamStatus").is_some() {
            // Deserialize into the `Status` variant
            let status = serde_json::from_value(value).map_err(de::Error::custom)?;
            Ok(StreamMarketDepthAggregatesResp::Status(status))
        } else if value.get("Heartbeat").is_some() {
            // Deserialize into the `Heartbeat` variant
            let heartbeat = serde_json::from_value(value).map_err(de::Error::custom)?;
            Ok(StreamMarketDepthAggregatesResp::Heartbeat(heartbeat))
        } else {
            // Default to `Error` variant if nothing else matches
            let error = serde_json::from_value(value).map_err(de::Error::custom)?;
            Ok(StreamMarketDepthAggregatesResp::Error(error))
        }
    }
}
