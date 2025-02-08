use crate::{
    accounting::{Account, BODBalance, Balance, Order, Position},
    responses::stream,
};
use serde::{de, Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// The TradeStation API Response for getting accounts.
pub struct GetAccountsResp {
    pub accounts: Vec<Account>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// The TradeStation API Response for getting account's balance.
// TODO: This also gives a key for errors, look into using these.
pub struct GetBalanceResp {
    pub balances: Vec<Balance>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// The TradeStation API Response for getting account's balance.
// TODO: This also gives a key for errors, look into using these.
pub struct GetBODBalanceResp {
    #[serde(rename = "BODBalances")]
    pub bod_balances: Vec<BODBalance>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// The TradeStation API Response for getting account's balance.
// TODO: This also gives a key for errors, look into using these.
// TODO: This also gives a `nextToken`, look into using this.
pub struct GetOrdersResp {
    pub orders: Vec<Order>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// The TradeStation API Response for getting account's balance.
// TODO: This also gives a key for errors, look into using these.
pub struct GetPositionsResp {
    pub positions: Vec<Position>,
}

/// The TradeStation API Response for streaming orders.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum StreamOrdersResp {
    /// The main response which contains order data
    Order(Box<self::Order>),
    /// Periodic signal to know the connection is still alive
    Heartbeat(stream::Heartbeat),
    /// Signal sent on state changes in the stream (closed, opened, paused, resumed)
    Status(stream::StreamStatus),
    /// Response for when an error was encountered, with details on the error
    Error(stream::ErrorResp),
}
impl<'de> Deserialize<'de> for StreamOrdersResp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;

        if value.get("AccountID").is_some() {
            // Deserialize into the `Order` variant
            let order = serde_json::from_value(value).map_err(de::Error::custom)?;
            Ok(StreamOrdersResp::Order(Box::new(order)))
        } else if value.get("StreamStatus").is_some() {
            // Deserialize into the `Status` variant
            let status = serde_json::from_value(value).map_err(de::Error::custom)?;
            Ok(StreamOrdersResp::Status(status))
        } else if value.get("Heartbeat").is_some() {
            // Deserialize into the `Heartbeat` variant
            let heartbeat = serde_json::from_value(value).map_err(de::Error::custom)?;
            Ok(StreamOrdersResp::Heartbeat(heartbeat))
        } else {
            // Default to `Error` variant if nothing else matches
            let error = serde_json::from_value(value).map_err(de::Error::custom)?;
            Ok(StreamOrdersResp::Error(error))
        }
    }
}

/// The TradeStation API Response for streaming `Position`(s).
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum StreamPositionsResp {
    /// The main response which contains position data
    Position(Box<self::Position>),
    /// Periodic signal to know the connection is still alive
    Heartbeat(stream::Heartbeat),
    /// Signal sent on state changes in the stream (closed, opened, paused, resumed)
    Status(stream::StreamStatus),
    /// Response for when an error was encountered, with details on the error
    Error(stream::ErrorResp),
}
impl<'de> Deserialize<'de> for StreamPositionsResp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;

        if value.get("AccountID").is_some() {
            // Deserialize into the `Position` variant
            let position = serde_json::from_value(value).map_err(de::Error::custom)?;
            Ok(StreamPositionsResp::Position(Box::new(position)))
        } else if value.get("StreamStatus").is_some() {
            // Deserialize into the `Status` variant
            let status = serde_json::from_value(value).map_err(de::Error::custom)?;
            Ok(StreamPositionsResp::Status(status))
        } else if value.get("Heartbeat").is_some() {
            // Deserialize into the `Heartbeat` variant
            let heartbeat = serde_json::from_value(value).map_err(de::Error::custom)?;
            Ok(StreamPositionsResp::Heartbeat(heartbeat))
        } else {
            // Default to `Error` variant if nothing else matches
            let error = serde_json::from_value(value).map_err(de::Error::custom)?;
            Ok(StreamPositionsResp::Error(error))
        }
    }
}
