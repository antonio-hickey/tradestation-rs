use crate::{
    execution::{ActivationTrigger, Order, OrderConfirmation},
    Error, Route,
};
use serde::{Deserialize, Serialize};

use super::ApiError;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// The TradeStation API Response for confirming
/// an order, but not actually placing it.
pub struct OrderRespRaw {
    /// The orders modified, placed, or canceled.
    orders: Option<Vec<Order>>,

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
/// The TradeStation API Response for confirming
/// an order, but not actually placing it.
pub struct OrderResp {
    /// The order confirmations.
    pub orders: Option<Vec<Order>>,

    /// The error from TradeStation's API.
    ///
    /// NOTE: Will be None if there was no error.
    pub error: Option<Error>,
}
impl From<OrderRespRaw> for OrderResp {
    fn from(raw: OrderRespRaw) -> Self {
        let error_enum =
            if let (Some(err), Some(msg)) = (raw.error.as_deref(), raw.message.as_deref()) {
                Some(Error::from_api_error(ApiError {
                    error: err.into(),
                    message: msg.into(),
                }))
            } else {
                None
            };

        OrderResp {
            orders: raw.orders,
            error: error_enum,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// The TradeStation API Response for
/// canceling or replacing an order.
pub struct ModifyOrderRespRaw {
    #[serde(rename = "OrderID")]
    /// The order id of the modified `Order`.
    order_id: String,

    /// The message related to the `Order` modification.
    message: Option<String>,

    /// The error type from TradeStation's API.
    ///
    /// NOTE: Will be None if there was no error.
    error: Option<String>,
}
#[derive(Debug)]
/// The TradeStation API Response for
/// canceling or replacing an order.
pub struct ModifyOrderResp {
    /// The order confirmations.
    pub order_id: String,

    /// The message related to the `Order` modification.
    message: Option<String>,

    /// The error from TradeStation's API.
    ///
    /// NOTE: Will be None if there was no error.
    pub error: Option<Error>,
}
impl From<ModifyOrderRespRaw> for ModifyOrderResp {
    fn from(raw: ModifyOrderRespRaw) -> Self {
        let error_enum =
            if let (Some(err), Some(msg)) = (raw.error.as_deref(), raw.message.as_deref()) {
                Some(Error::from_api_error(ApiError {
                    error: err.into(),
                    message: msg.into(),
                }))
            } else {
                None
            };

        ModifyOrderResp {
            order_id: raw.order_id,
            message: raw.message,
            error: error_enum,
        }
    }
}
impl From<ModifyOrderResp> for Order {
    fn from(raw: ModifyOrderResp) -> Self {
        let error = raw.error.map(|err| err.to_string());

        Order {
            order_id: raw.order_id,
            message: raw.message.unwrap_or_default(),
            error,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// The TradeStation API Response for confirming
/// an order, but not actually placing it.
pub struct ConfirmOrderRespRaw {
    /// The order confirmations.
    confirmations: Option<Vec<OrderConfirmation>>,
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
/// The TradeStation API Response for confirming
/// an order, but not actually placing it.
pub struct ConfirmOrderResp {
    /// The order confirmations.
    pub confirmations: Option<Vec<OrderConfirmation>>,
    /// The error from TradeStation's API.
    ///
    /// NOTE: Will be None if there was no error.
    pub error: Option<Error>,
}
impl From<ConfirmOrderRespRaw> for ConfirmOrderResp {
    fn from(raw: ConfirmOrderRespRaw) -> Self {
        let error_enum =
            if let (Some(err), Some(msg)) = (raw.error.as_deref(), raw.message.as_deref()) {
                Some(Error::from_api_error(ApiError {
                    error: err.into(),
                    message: msg.into(),
                }))
            } else {
                None
            };

        ConfirmOrderResp {
            confirmations: raw.confirmations,
            error: error_enum,
        }
    }
}

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
        let error_enum =
            if let (Some(err), Some(msg)) = (raw.error.as_deref(), raw.message.as_deref()) {
                Some(Error::from_api_error(ApiError {
                    error: err.into(),
                    message: msg.into(),
                }))
            } else {
                None
            };

        GetExecutionRoutesResp {
            routes: raw.routes,
            error: error_enum,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// The TradeStation API Response for fetching valid
/// activation triggers and their corresponding key.
pub struct GetActivationTriggersRespRaw {
    /// Activation Triggers.
    activation_triggers: Option<Vec<ActivationTrigger>>,

    /// The error type from TradeStation's API.
    ///
    /// NOTE: Will be None if there was no error.
    error: Option<String>,

    /// The error message from TradeStation's API.
    ///
    /// NOTE: Will be None if there was no error.
    message: Option<String>,
}
#[derive(Debug)]
/// The TradeStation API Response for fetching symbol details.
pub struct GetActivationTriggersResp {
    /// The Activation Triggers.
    pub activation_triggers: Option<Vec<ActivationTrigger>>,

    /// The error from TradeStation's API.
    ///
    /// NOTE: Will be None if there was no error.
    pub error: Option<Error>,
}
impl From<GetActivationTriggersRespRaw> for GetActivationTriggersResp {
    fn from(raw: GetActivationTriggersRespRaw) -> Self {
        let error_enum =
            if let (Some(err), Some(msg)) = (raw.error.as_deref(), raw.message.as_deref()) {
                Some(Error::from_api_error(ApiError {
                    error: err.into(),
                    message: msg.into(),
                }))
            } else {
                None
            };

        GetActivationTriggersResp {
            activation_triggers: raw.activation_triggers,
            error: error_enum,
        }
    }
}
