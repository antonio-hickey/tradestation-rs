use crate::{
    accounting::{Account, BODBalance, Balance, Order, Position},
    responses::stream,
};
use serde::{de, Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// The error type for partially successful operations around accounts.
/// For example if getting balances on multiple accounts, but 1 of the account
/// id's is incorrect and the other 5 account id's were correct, so it was partially
/// successful with 1 error which would be of this type.
pub(crate) struct AccountApiError {
    /// The Account ID of the error.
    ///
    /// NOTE: May contain multiple Account IDs in a comma seperated string.
    #[serde(rename = "AccountID")]
    pub account_id: String,

    /// The error.
    pub error: String,

    /// The error message.
    pub message: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// The TradeStation API Response for getting accounts.
pub(crate) struct GetAccountsResp {
    pub accounts: Vec<Account>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// The TradeStation API Response for getting account's balance.
pub(crate) struct GetBalanceResp {
    pub balances: Vec<Balance>,

    #[serde(default)]
    pub errors: Vec<AccountApiError>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// The TradeStation API Response for getting account's balance.
pub(crate) struct GetBODBalanceResp {
    #[serde(rename = "BODBalances")]
    pub bod_balances: Vec<BODBalance>,

    #[serde(default)]
    pub errors: Vec<AccountApiError>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// The TradeStation API Response for getting account's balance.
// TODO: This also gives a `nextToken`, look into using this.
pub(crate) struct GetOrdersResp {
    pub orders: Vec<Order>,

    #[serde(default)]
    pub errors: Vec<AccountApiError>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// The TradeStation API Response for getting account's balance.
pub(crate) struct GetPositionsResp {
    pub positions: Vec<Position>,

    #[serde(default)]
    pub errors: Vec<AccountApiError>,
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
