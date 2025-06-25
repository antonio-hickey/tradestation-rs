use crate::{
    accounting::{
        orders::{AssetType, MarketActivationRule, OrderType, TimeActivationRule, TrailingStop},
        OptionType,
    },
    market_data::OptionSpreadType,
    responses::{
        execution::{
            ConfirmOrderResp, ConfirmOrderRespRaw, GetActivationTriggersResp,
            GetActivationTriggersRespRaw, GetExecutionRoutesResp, GetExecutionRoutesRespRaw,
            ModifyOrderResp, ModifyOrderRespRaw, OrderResp, OrderRespRaw,
        },
        ApiResponse,
    },
    Client, Error,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// An Active Order
pub struct Order {
    /// Short text summary / description of the order.
    pub message: String,

    #[serde(rename = "OrderID")]
    /// The id of the order.
    pub order_id: String,

    /// The error for the order.
    pub error: Option<String>,
}
impl Order {
    /// Instantiate an `Order` using a provided order id.
    ///
    /// NOTE: The created `Order` is NOT guaranteed to be valid
    /// for use. The order id provided must be valid to do anything
    /// with the `Order` instance.
    ///
    /// # Example
    /// ---
    ///
    /// Create an instance of `Order` for an order id `11111111`
    /// which you can then use to cancel or replace the order.
    ///
    /// ```
    /// use tradestation::execution::Order;
    ///
    /// let order = Order::from_id("11111111");
    /// println!("{order:?}");
    /// ```
    pub fn from_id<S: Into<String>>(order_id: S) -> Order {
        Order {
            message: "".into(),
            order_id: order_id.into(),
            error: None,
        }
    }

    /// Confirm an order getting back an estimated cost
    /// and commission information for the order without
    /// actually placing the order.
    ///
    /// NOTE: Only valid for `Market Limit`, `Stop Market`,
    /// `Options`, and `Order Sends Order (OSO)` order types.
    ///
    /// # Example
    /// ---
    ///
    /// Confirm a limit buy order for 3 Month SOFR Futures at the
    /// March 2025 contract @ 96.0725 with a quantity of 50 contracts
    /// and a duration of Good Till Close (GTC).
    ///
    /// ```ignore
    /// use tradestation::{
    ///     ClientBuilder, Error, Token,
    ///     execution::{Duration, OrderRequestBuilder, Order},
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     // Create client
    ///     let client = ClientBuilder::new()?
    ///         .credentials("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")?
    ///         .token(Token { /* YOUR BEARER AUTH TOKEN */ })?
    ///         .build()
    ///         .await?;
    ///
    ///     let order_req = OrderRequestBuilder::new()
    ///         .account_id("YOUR_FUTURES_ACCOUNT_ID")
    ///         .symbol("SR3H25")
    ///         .trade_action(TradeAction::Buy)
    ///         .quantity("50")
    ///         .order_type(OrderType::Limit)
    ///         .limit_price("96.0725")
    ///         .time_in_force(OrderTimeInForce {
    ///             duration: Duration::GTC,
    ///             expiration: None,
    ///         })
    ///         .build()?;
    ///
    ///     match Order::confirm(&order_req, &client).await {
    ///         Ok(confirmation) => println!("Confirmed Order: {confirmation:?}"),
    ///         Err(e) => println!("Issue Confirming Order: {e:?}"),
    ///     };
    ///     Ok(())
    /// }
    ///```
    pub async fn confirm(
        order_request: &OrderRequest,
        client: &Client,
    ) -> Result<Vec<OrderConfirmation>, Error> {
        let endpoint = String::from("orderexecution/orderconfirm");

        match client
            .post(&endpoint, order_request)
            .await?
            .json::<ApiResponse<ConfirmOrderRespRaw>>()
            .await?
        {
            ApiResponse::Success(resp_raw) => {
                let resp: ConfirmOrderResp = resp_raw.into();
                if let Some(confirmations) = resp.confirmations {
                    Ok(confirmations)
                } else {
                    Err(resp
                        .error
                        .unwrap_or(Error::UnknownTradeStationAPIError(String::new())))
                }
            }
            ApiResponse::Error(resp) => Err(Error::from_api_error(resp)),
        }
    }

    /// Place the `OrderRequest` getting back the result of the Order Request.
    ///
    /// # Example
    /// ---
    /// Place an order to buy 100 shares of JP Morgan (`"JPM"`)
    /// using a limit order with the limit price of $`"220.50"`, with
    /// a order duration of Good Till Closed.
    ///
    ///```ignore
    /// let order_req = OrderRequestBuilder::new()
    ///     .account_id("YOUR_EQUITIES_ACCOUNT_ID")
    ///     .symbol("JPM")
    ///     .trade_action(TradeAction::Buy)
    ///     .quantity("100")
    ///     .order_type(OrderType::Limit)
    ///     .limit_price("220.50")
    ///     .time_in_force(OrderTimeInForce {
    ///         duration: Duration::GTC,
    ///         expiration: None,
    ///     })
    ///     .build()?;
    ///
    /// match Order::place(order_req, &client,).await {
    ///     Ok(resp) => println!("Order Response: {resp:?}"),
    ///     Err(e) => println!("Order Response: {e:?}"),
    /// }
    /// ```
    pub async fn place(order_request: &OrderRequest, client: &Client) -> Result<Vec<Order>, Error> {
        let endpoint = String::from("orderexecution/orders");

        match client
            .post(&endpoint, &order_request)
            .await?
            .json::<ApiResponse<OrderRespRaw>>()
            .await?
        {
            ApiResponse::Success(resp_raw) => {
                let resp: OrderResp = resp_raw.into();
                if let Some(orders) = resp.orders {
                    Ok(orders)
                } else {
                    Err(resp.error.unwrap_or(Error::UnknownTradeStationAPIError(
                        "Unknown TradeStation API Error While Placing Order.".into(),
                    )))
                }
            }
            ApiResponse::Error(resp) => Err(Error::from_api_error(resp)),
        }
    }

    /// Creates an Order Confirmation for a group order. Request valid for
    /// Order Cancels Order (OCO) and Bracket (BRK) order types as well as
    /// grouped orders of other types (NORMAL).
    ///
    /// # Order Cancels Order (OCO)
    ///
    /// An OCO order is a group of orders whereby if one of the orders is
    /// filled or partially-filled, then all of the other orders in the
    /// group are cancelled.
    ///
    /// # Bracket OCO Orders
    ///
    /// A bracket order is a special instance of an OCO (Order Cancel Order).
    /// Bracket orders are used to exit an existing position. They are designed
    /// to limit loss and lock in profit by “bracketing” an order with a simultaneous
    /// stop and limit order.
    ///
    /// Bracket orders are limited so that the orders are all for the same symbol
    /// and are on the same side of the market (either all to sell or all to cover),
    /// and they are restricted to closing transactions.
    ///
    /// The reason that they follow these rules is because the orders need to be
    /// able to auto decrement when a partial fill occurs with one of the orders.
    /// For example, if the customer has a sell limit order for 1000 shares and
    /// a sell stop order for 1000 shares, and the limit order is partially filled
    /// for 500 shares, then the customer would want the stop to remain open, but
    /// it should automatically decrement the order to 500 shares to match the
    /// remaining open position.
    ///
    /// NOTE: When a group order is submitted, the order execution system treats
    /// each sibling order as an individual order. Thus, the system does not validate
    /// that each order has the same Quantity, and currently it is not able to update
    /// a bracket order as one transaction, instead you must update each order within
    /// a bracket.
    ///
    /// # Example
    /// ---
    /// Confirm a trade involving a bracket group of orders with one order
    /// for opening the position, one order for closing the position at a
    /// take profit price, and one order for closing the position at a stop
    /// loss price. A total of 3 orders making up this position.
    /// ```ignore
    /// use tradestation::{
    ///     execution::{Duration, Order, OrderRequestBuilder},
    ///     ClientBuilder, Error, Token,
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     // Create client
    ///     let client = ClientBuilder::new()?
    ///         .credentials("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")?
    ///         .token(Token { /* YOUR BEARER AUTH TOKEN */ })?
    ///         .build()
    ///         .await?;
    ///
    ///     let entry_order_req = OrderRequestBuilder::new()
    ///         .account_id("YOUR_EQUITIES_ACCOUNT_ID")
    ///         .symbol("XLRE")
    ///         .trade_action(TradeAction::SellShort)
    ///         .quantity("1000")
    ///         .order_type(OrderType::Market)
    ///         .time_in_force(OrderTimeInForce {
    ///             duration: Duration::GTC,
    ///             expiration: None,
    ///         })
    ///         .build()?;
    ///
    ///     let take_profit_order_req = OrderRequestBuilder::new()
    ///         .account_id("YOUR_EQUITIES_ACCOUNT_ID")
    ///         .symbol("XLRE")
    ///         .trade_action(TradeAction::BuyToCover)
    ///         .quantity("1000")
    ///         .order_type(OrderType::Limit)
    ///         .limit_price("35.75")
    ///         .time_in_force(OrderTimeInForce {
    ///             duration: Duration::GTC,
    ///             expiration: None,
    ///         })
    ///         .build()?;
    ///
    ///     let stop_loss_order_req = OrderRequestBuilder::new()
    ///         .account_id("YOUR_EQUITIES_ACCOUNT_ID")
    ///         .symbol("XLRE")
    ///         .trade_action(TradeAction::BuyToCover)
    ///         .quantity("1000")
    ///         .order_type(OrderType::StopMarket)
    ///         .stop_price("46.50")
    ///         .time_in_force(OrderTimeInForce {
    ///             duration: Duration::GTC,
    ///             expiration: None,
    ///         })
    ///         .build()?;
    ///
    ///     let order_group = OrderRequestGroupBuilder::new()
    ///         .order_requests(Vec::from([
    ///             entry_order_req,
    ///             take_profit_order_req,
    ///             stop_loss_order_req,
    ///         ]))
    ///         .group_type(OrderGroupType::BRK)
    ///         .build()?;
    ///
    ///     let order_confirmations = Order::confirm(&order_group, &client).await?;
    ///     println!("Confirm Orders Result: {order_confirmations:?}");
    /// }
    /// ```
    pub async fn confirm_group(
        order_req_group: &OrderRequestGroup,
        client: &Client,
    ) -> Result<Vec<OrderConfirmation>, Error> {
        let endpoint = String::from("orderexecution/ordergroupconfirm");

        match client
            .post(&endpoint, order_req_group)
            .await?
            .json::<ApiResponse<ConfirmOrderRespRaw>>()
            .await?
        {
            ApiResponse::Success(resp_raw) => {
                let resp: ConfirmOrderResp = resp_raw.into();
                if let Some(confirmations) = resp.confirmations {
                    Ok(confirmations)
                } else {
                    Err(resp.error.unwrap_or(Error::UnknownTradeStationAPIError(
                        "Unknown TradeStation Error While Confirming Group Order.".into(),
                    )))
                }
            }
            ApiResponse::Error(resp) => Err(Error::from_api_error(resp)),
        }
    }

    /// Submits a group order. Request valid for Order Cancels Order (OCO)
    /// and Bracket (BRK) order types as well as grouped orders of other
    /// types (NORMAL).
    ///
    /// # Order Cancels Order (OCO)
    ///
    /// An OCO order is a group of orders whereby if one of the orders is
    /// filled or partially-filled, then all of the other orders in the
    /// group are cancellCreates an Order Confirmation for a group order.
    /// Request valid for all account types. Request valid for Order Cancels
    /// Order (OCO) and Bracket (BRK) order types as well as grouped orders of
    /// other types (NORMAL).ed.
    ///
    /// # Bracket OCO Orders
    ///
    /// A bracket order is a special instance of an OCO (Order Cancel Order).
    /// Bracket orders are used to exit an existing position. They are designed
    /// to limit loss and lock in profit by “bracketing” an order with a simultaneous
    /// stop and limit order.
    ///
    /// Bracket orders are limited so that the orders are all for the same symbol
    /// and are on the same side of the market (either all to sell or all to cover),
    /// and they are restricted to closing transactions.
    ///
    /// The reason that they follow these rules is because the orders need to be
    /// able to auto decrement when a partial fill occurs with one of the orders.
    /// For example, if the customer has a sell limit order for 1000 shares and
    /// a sell stop order for 1000 shares, and the limit order is partially filled
    /// for 500 shares, then the customer would want the stop to remain open, but
    /// it should automatically decrement the order to 500 shares to match the
    /// remaining open position.
    ///
    /// NOTE: When a group order is submitted, the order execution system treats
    /// each sibling order as an individual order. Thus, the system does not validate
    /// that each order has the same Quantity, and currently it is not able to update
    /// a bracket order as one transaction, instead you must update each order within
    /// a bracket.
    ///
    /// # Example
    /// ---
    /// Place a trade involving a bracket group of orders with one order
    /// for opening the position, one order for closing the position at a
    /// take profit price, and one order for closing the position at a stop
    /// loss price. A total of 3 orders making up this position.
    /// ```ignore
    /// use tradestation::{
    ///     execution::{Duration, Order, OrderRequestBuilder},
    ///     ClientBuilder, Error, Token,
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     // Create client
    ///     let client = ClientBuilder::new()?
    ///         .credentials("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")?
    ///         .token(Token { /* YOUR BEARER AUTH TOKEN */ })?
    ///         .build()
    ///         .await?;
    ///
    ///     let entry_order_req = OrderRequestBuilder::new()
    ///         .account_id("YOUR_EQUITIES_ACCOUNT_ID")
    ///         .symbol("XLRE")
    ///         .trade_action(TradeAction::SellShort)
    ///         .quantity("1000")
    ///         .order_type(OrderType::Market)
    ///         .time_in_force(OrderTimeInForce {
    ///             duration: Duration::GTC,
    ///             expiration: None,
    ///         })
    ///         .build()?;
    ///
    ///     let take_profit_order_req = OrderRequestBuilder::new()
    ///         .account_id("YOUR_EQUITIES_ACCOUNT_ID")
    ///         .symbol("XLRE")
    ///         .trade_action(TradeAction::BuyToCover)
    ///         .quantity("1000")
    ///         .order_type(OrderType::Limit)
    ///         .limit_price("35.75")
    ///         .time_in_force(OrderTimeInForce {
    ///             duration: Duration::GTC,
    ///             expiration: None,
    ///         })
    ///         .build()?;
    ///
    ///     let stop_loss_order_req = OrderRequestBuilder::new()
    ///         .account_id("YOUR_EQUITIES_ACCOUNT_ID")
    ///         .symbol("XLRE")
    ///         .trade_action(TradeAction::BuyToCover)
    ///         .quantity("1000")
    ///         .order_type(OrderType::StopMarket)
    ///         .stop_price("46.50")
    ///         .time_in_force(OrderTimeInForce {
    ///             duration: Duration::GTC,
    ///             expiration: None,
    ///         })
    ///         .build()?;
    ///
    ///     let order_group = OrderRequestGroupBuilder::new()
    ///         .order_requests(Vec::from([
    ///             entry_order_req,
    ///             take_profit_order_req,
    ///             stop_loss_order_req,
    ///         ]))
    ///         .group_type(OrderGroupType::BRK)
    ///         .build()?;
    ///
    ///     let orders = Order::place_group(&order_group, &client).await?;
    ///     println!("Place Orders Result: {orders:?}");
    /// }
    /// ```
    pub async fn place_group(
        order_req_group: &OrderRequestGroup,
        client: &Client,
    ) -> Result<Vec<Order>, Error> {
        let endpoint = String::from("orderexecution/ordergroups");

        match client
            .post(&endpoint, order_req_group)
            .await?
            .json::<ApiResponse<OrderRespRaw>>()
            .await?
        {
            ApiResponse::Success(resp_raw) => {
                let resp: OrderResp = resp_raw.into();
                if let Some(orders) = resp.orders {
                    Ok(orders)
                } else {
                    Err(resp.error.unwrap_or(Error::UnknownTradeStationAPIError(
                        "Unknown TradeStation Error While Placing Group Order.".into(),
                    )))
                }
            }
            ApiResponse::Error(resp) => Err(Error::from_api_error(resp)),
        }
    }

    /// Replace an `Order` with an Order Update.
    ///
    /// # Example
    /// ---
    /// Replace an order to buy 100 shares of Palantir `"PLTR"`
    /// at the limit price of $`"40.00"` to instead be 25 shares
    /// at the limit price of $`"42.50"`.
    ///
    /// ```ignore
    /// let order_req = OrderRequestBuilder::new()
    ///     .account_id("YOUR_EQUITIES_ACCOUNT_ID")
    ///     .symbol("PLTR")
    ///     .trade_action(TradeAction::Buy)
    ///     .quantity("100")
    ///     .order_type(OrderType::Limit)
    ///     .limit_price("40.00")
    ///     .time_in_force(OrderTimeInForce {
    ///         duration: Duration::GTC,
    ///         expiration: None,
    ///     })
    ///     .build()?;
    ///
    /// let order = Order::place(&order_req, &client)
    ///     .await?
    ///     .into_iter()
    ///     .next();
    ///
    /// if let Some(order) = order {
    ///     order
    ///         .clone()
    ///         .replace(
    ///             OrderUpdate::new().limit_price("42.50").quantity("25"),
    ///             &client,
    ///         )
    ///         .await?;
    /// }
    /// ```
    pub async fn replace(self, order_update: OrderUpdate, client: &Client) -> Result<Order, Error> {
        let endpoint = format!("orderexecution/orders/{}", self.order_id);

        match client
            .put(&endpoint, &order_update)
            .await?
            .json::<ApiResponse<ModifyOrderRespRaw>>()
            .await?
        {
            ApiResponse::Success(resp_raw) => {
                let resp: ModifyOrderResp = resp_raw.into();
                let order: Order = resp.into();

                Ok(order)
            }
            ApiResponse::Error(resp) => Err(Error::from_api_error(resp)),
        }
    }

    /// Cancel an active `Order`.
    ///
    /// # Example
    /// ---
    ///
    /// ```ignore
    /// let order_req = OrderRequestBuilder::new()
    ///     .account_id("YOUR_EQUITIES_ACCOUNT_ID")
    ///     .symbol("JPM")
    ///     .trade_action(TradeAction::Buy)
    ///     .quantity("100")
    ///     .order_type(OrderType::Limit)
    ///     .limit_price("220.50")
    ///     .time_in_force(OrderTimeInForce {
    ///         duration: Duration::GTC,
    ///         expiration: None,
    ///     })
    ///     .build()?;
    ///
    /// let order = Order::place(&order_req, &client)
    ///     .await?
    ///     .into_iter()
    ///     .next();
    ///
    /// if let Some(order) = order {
    ///     order.cancel(&client).await?;
    /// }
    /// ```
    pub async fn cancel(self, client: &Client) -> Result<Order, Error> {
        let endpoint = format!("orderexecution/orders/{}", self.order_id);

        match client
            .delete(&endpoint)
            .await?
            .json::<ApiResponse<ModifyOrderRespRaw>>()
            .await?
        {
            ApiResponse::Success(resp_raw) => {
                let resp: ModifyOrderResp = resp_raw.into();
                let order: Order = resp.into();

                Ok(order)
            }
            ApiResponse::Error(resp) => Err(Error::from_api_error(resp)),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
/// An collection of `OrderRequest`'s to be sent together.
pub struct OrderRequestGroup {
    pub order_requests: Vec<OrderRequest>,
    pub group_type: OrderGroupType,
}
impl OrderRequestGroup {
    /// Submits a group order. Request valid for Order Cancels Order (OCO)
    /// and Bracket (BRK) order types as well as grouped orders of other
    /// types (NORMAL).
    ///
    /// # Order Cancels Order (OCO)
    ///
    /// An OCO order is a group of orders whereby if one of the orders is
    /// filled or partially-filled, then all of the other orders in the
    /// group are cancellCreates an Order Confirmation for a group order. Request valid for all account types. Request valid for Order Cancels Order (OCO) and Bracket (BRK) order types as well as grouped orders of other types (NORMAL).ed.
    ///
    /// # Bracket OCO Orders
    ///
    /// A bracket order is a special instance of an OCO (Order Cancel Order).
    /// Bracket orders are used to exit an existing position. They are designed
    /// to limit loss and lock in profit by “bracketing” an order with a simultaneous
    /// stop and limit order.
    ///
    /// Bracket orders are limited so that the orders are all for the same symbol
    /// and are on the same side of the market (either all to sell or all to cover),
    /// and they are restricted to closing transactions.
    ///
    /// The reason that they follow these rules is because the orders need to be
    /// able to auto decrement when a partial fill occurs with one of the orders.
    /// For example, if the customer has a sell limit order for 1000 shares and
    /// a sell stop order for 1000 shares, and the limit order is partially filled
    /// for 500 shares, then the customer would want the stop to remain open, but
    /// it should automatically decrement the order to 500 shares to match the
    /// remaining open position.
    ///
    /// NOTE: When a group order is submitted, the order execution system treats
    /// each sibling order as an individual order. Thus, the system does not validate
    /// that each order has the same Quantity, and currently it is not able to update
    /// a bracket order as one transaction, instead you must update each order within
    /// a bracket.
    ///
    /// # Example
    /// ---
    /// Place a trade involving a bracket group of orders with one order
    /// for opening the position, one order for closing the position at a
    /// take profit price, and one order for closing the position at a stop
    /// loss price. A total of 3 orders making up this position.
    /// ```ignore
    /// use tradestation::{
    ///     execution::{Duration, Order, OrderRequestBuilder},
    ///     ClientBuilder, Error, Token,
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     // Create client
    ///     let client = ClientBuilder::new()?
    ///         .credentials("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")?
    ///         .token(Token { /* YOUR BEARER AUTH TOKEN */ })?
    ///         .build()
    ///         .await?;
    ///
    ///     let entry_order_req = OrderRequestBuilder::new()
    ///         .account_id("YOUR_EQUITIES_ACCOUNT_ID")
    ///         .symbol("XLRE")
    ///         .trade_action(TradeAction::SellShort)
    ///         .quantity("1000")
    ///         .order_type(OrderType::Market)
    ///         .time_in_force(OrderTimeInForce {
    ///             duration: Duration::GTC,
    ///             expiration: None,
    ///         })
    ///         .build()?;
    ///
    ///     let take_profit_order_req = OrderRequestBuilder::new()
    ///         .account_id("YOUR_EQUITIES_ACCOUNT_ID")
    ///         .symbol("XLRE")
    ///         .trade_action(TradeAction::BuyToCover)
    ///         .quantity("1000")
    ///         .order_type(OrderType::Limit)
    ///         .limit_price("35.75")
    ///         .time_in_force(OrderTimeInForce {
    ///             duration: Duration::GTC,
    ///             expiration: None,
    ///         })
    ///         .build()?;
    ///
    ///     let stop_loss_order_req = OrderRequestBuilder::new()
    ///         .account_id("YOUR_EQUITIES_ACCOUNT_ID")
    ///         .symbol("XLRE")
    ///         .trade_action(TradeAction::BuyToCover)
    ///         .quantity("1000")
    ///         .order_type(OrderType::StopMarket)
    ///         .stop_price("46.50")
    ///         .time_in_force(OrderTimeInForce {
    ///             duration: Duration::GTC,
    ///             expiration: None,
    ///         })
    ///         .build()?;
    ///
    ///     let order_group = OrderRequestGroupBuilder::new()
    ///         .order_requests(Vec::from([
    ///             entry_order_req,
    ///             take_profit_order_req,
    ///             stop_loss_order_req,
    ///         ]))
    ///         .group_type(OrderGroupType::BRK)
    ///         .build()?;
    ///
    ///     let orders = order_group.place(&client).await?;
    ///     println!("Place Orders Result: {orders:?}");
    /// }
    /// ```
    pub async fn place(&self, client: &Client) -> Result<Vec<Order>, Error> {
        Order::place_group(self, client).await
    }

    /// Creates an Order Confirmation for a group order. Request valid for
    /// Order Cancels Order (OCO) and Bracket (BRK) order types as well as
    /// grouped orders of other types (NORMAL).
    ///
    /// # Order Cancels Order (OCO)
    ///
    /// An OCO order is a group of orders whereby if one of the orders is
    /// filled or partially-filled, then all of the other orders in the
    /// group are cancelled.
    ///
    /// # Bracket OCO Orders
    ///
    /// A bracket order is a special instance of an OCO (Order Cancel Order).
    /// Bracket orders are used to exit an existing position. They are designed
    /// to limit loss and lock in profit by “bracketing” an order with a simultaneous
    /// stop and limit order.
    ///
    /// Bracket orders are limited so that the orders are all for the same symbol
    /// and are on the same side of the market (either all to sell or all to cover),
    /// and they are restricted to closing transactions.
    ///
    /// The reason that they follow these rules is because the orders need to be
    /// able to auto decrement when a partial fill occurs with one of the orders.
    /// For example, if the customer has a sell limit order for 1000 shares and
    /// a sell stop order for 1000 shares, and the limit order is partially filled
    /// for 500 shares, then the customer would want the stop to remain open, but
    /// it should automatically decrement the order to 500 shares to match the
    /// remaining open position.
    ///
    /// NOTE: When a group order is submitted, the order execution system treats
    /// each sibling order as an individual order. Thus, the system does not validate
    /// that each order has the same Quantity, and currently it is not able to update
    /// a bracket order as one transaction, instead you must update each order within
    /// a bracket.
    ///
    /// # Example
    /// ---
    /// Confirm a trade involving a bracket group of orders with one order
    /// for opening the position, one order for closing the position at a
    /// take profit price, and one order for closing the position at a stop
    /// loss price. A total of 3 orders making up this position.
    /// ```ignore
    /// use tradestation::{
    ///     execution::{Duration, Order, OrderRequestBuilder},
    ///     ClientBuilder, Error, Token,
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     // Create client
    ///     let client = ClientBuilder::new()?
    ///         .credentials("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")?
    ///         .token(Token { /* YOUR BEARER AUTH TOKEN */ })?
    ///         .build()
    ///         .await?;
    ///
    ///     let entry_order_req = OrderRequestBuilder::new()
    ///         .account_id("YOUR_EQUITIES_ACCOUNT_ID")
    ///         .symbol("XLRE")
    ///         .trade_action(TradeAction::SellShort)
    ///         .quantity("1000")
    ///         .order_type(OrderType::Market)
    ///         .time_in_force(OrderTimeInForce {
    ///             duration: Duration::GTC,
    ///             expiration: None,
    ///         })
    ///         .build()?;
    ///
    ///     let take_profit_order_req = OrderRequestBuilder::new()
    ///         .account_id("YOUR_EQUITIES_ACCOUNT_ID")
    ///         .symbol("XLRE")
    ///         .trade_action(TradeAction::BuyToCover)
    ///         .quantity("1000")
    ///         .order_type(OrderType::Limit)
    ///         .limit_price("35.75")
    ///         .time_in_force(OrderTimeInForce {
    ///             duration: Duration::GTC,
    ///             expiration: None,
    ///         })
    ///         .build()?;
    ///
    ///     let stop_loss_order_req = OrderRequestBuilder::new()
    ///         .account_id("YOUR_EQUITIES_ACCOUNT_ID")
    ///         .symbol("XLRE")
    ///         .trade_action(TradeAction::BuyToCover)
    ///         .quantity("1000")
    ///         .order_type(OrderType::StopMarket)
    ///         .stop_price("46.50")
    ///         .time_in_force(OrderTimeInForce {
    ///             duration: Duration::GTC,
    ///             expiration: None,
    ///         })
    ///         .build()?;
    ///
    ///     let order_group = OrderRequestGroupBuilder::new()
    ///         .order_requests(Vec::from([
    ///             entry_order_req,
    ///             take_profit_order_req,
    ///             stop_loss_order_req,
    ///         ]))
    ///         .group_type(OrderGroupType::BRK)
    ///         .build()?;
    ///
    ///     let order_confirmations = order_group.confirm(&client).await?;
    ///     println!("Confirm Orders Result: {order_confirmations:?}");
    /// }
    /// ```
    pub async fn confirm(self, client: &Client) -> Result<Vec<OrderConfirmation>, Error> {
        Order::confirm_group(&self, client).await
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
/// `OrderRequestGroup` builder
pub struct OrderRequestGroupBuilder {
    order_requests: Option<Vec<OrderRequest>>,
    group_type: Option<OrderGroupType>,
}
impl OrderRequestGroupBuilder {
    /// Initialize a default builder struct for an `OrderRequestGroup`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the Order Requests (`Vec<execution::OrderRequest>`) for the group.
    pub fn order_requests(mut self, order_reqs: Vec<OrderRequest>) -> Self {
        self.order_requests = Some(order_reqs);
        self
    }

    /// Set the Order Group Type (`execution::OrderGroupType`).
    pub fn group_type(mut self, group_type: OrderGroupType) -> Self {
        self.group_type = Some(group_type);
        self
    }

    /// Finish building the `OrderRequestGroup`.
    ///
    /// NOTE: Setting `order_requests` is required before building.
    ///
    /// NOTE: Setting `group_type` is required before building.
    pub fn build(self) -> Result<OrderRequestGroup, Error> {
        Ok(OrderRequestGroup {
            order_requests: self.order_requests.ok_or(Error::OrderRequestsNotSet)?,
            group_type: self.group_type.ok_or(Error::OrderGroupTypeNotSet)?,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
/// The different types of order groups
pub enum OrderGroupType {
    /// Bracket Order
    BRK,

    /// Order Cancels Order
    OCO,

    /// Normal Group of Orders
    NORMAL,
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "PascalCase")]
/// An update to an existing [`Order`] already placed and
/// still alive in the market.
pub struct OrderUpdate {
    /// The limit price for this updated `Order`.
    pub limit_price: Option<String>,

    /// The stop price for this updated `Order`.
    pub stop_price: Option<String>,

    /// The order type for this updated `Order`.
    pub order_type: Option<OrderType>,

    /// The quantity for this updated `Order`.
    pub quantity: Option<String>,

    /// The advanced options of this updated `Order`.
    pub advanced_options: Option<AdvancedOrderOptions>,
}
impl OrderUpdate {
    /// Create a new default `OrderUpdate`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the limit price of the updated `Order`.
    pub fn limit_price(mut self, price: impl Into<String>) -> Self {
        self.limit_price = Some(price.into());

        self
    }

    /// Set the stop price of the updated `Order`.
    pub fn stop_price(mut self, price: impl Into<String>) -> Self {
        self.stop_price = Some(price.into());

        self
    }

    /// Set the order type of the updated `Order`.
    pub fn order_type(mut self, order_type: OrderType) -> Self {
        self.order_type = Some(order_type);

        self
    }

    /// Set the quantity for the updated `Order`.
    pub fn quantity(mut self, qty: impl Into<String>) -> Self {
        self.quantity = Some(qty.into());

        self
    }

    /// Set the advanced options of the updated `Order`.
    pub fn advanced_options(mut self, opts: AdvancedOrderOptions) -> Self {
        self.advanced_options = Some(opts);

        self
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// The initial stage of an `Order`, this is what
/// is sent to the route for creating a `Order`.
pub struct OrderRequest {
    /// The TradeStation Account ID the order is for.
    pub account_id: String,

    /// Advanced Options for configuring an order.
    pub advanced_options: Option<AdvancedOrderOptions>,

    /// The different statuses for buing power warnings.
    pub buying_power_warning: Option<BPWarningStatus>,

    /// The additional legs to this order.
    pub legs: Option<Vec<OrderRequestLeg>>,

    /// The limit price for this order.
    pub limit_price: Option<String>,

    /// Order Sends Orders
    pub osos: Option<Vec<Oso>>,

    /// A unique identifier regarding an order used
    /// to prevent duplicates. Must be unique per API
    /// key, per order, per user.
    pub order_confirm_id: Option<String>,

    /// The order type of the order.
    pub order_type: OrderType,

    /// The quantity of shares, or contracts for the order.
    ///
    /// NOTE: Only required if not provided within order legs.
    pub quantity: Option<String>,

    /// The route of the order.
    ///
    /// NOTE: For Stocks and Options, Route value will
    /// default to Intelligent if no value is set.
    ///
    /// NOTE: Routes can be obtained from `Order::get_routes()`.
    pub route: Option<String>,

    /// The stop price for this order.
    ///
    /// NOTE: If a TrailingStop amount or percent is passed
    /// in with the request (in the `AdvancedOrderOptions`),
    /// and a Stop Price value is also passed in, the Stop
    /// Price value is ignored.
    pub stop_price: Option<String>,

    /// The symbol used for this order.
    ///
    /// NOTE: Only required if not provided within order legs.
    pub symbol: Option<String>,

    /// Defines the duration and expiration timestamp of an Order.
    pub time_in_force: OrderTimeInForce,

    /// The different trade actions that can be sent or
    /// received, and conveys the intent of the order.
    ///
    /// NOTE: Only required if not provided within order legs.
    pub trade_action: Option<TradeAction>,
}
impl OrderRequest {
    /// Confirm an order getting back an estimated cost
    /// and commission information for the order without
    /// actually placing the order.
    ///
    /// NOTE: Only valid for `Market Limit`, `Stop Market`,
    /// `Options`, and `Order Sends Order (OSO)` order types.
    ///
    /// # Example
    /// ---
    ///
    /// Confirm a limit buy order for 3 Month SOFR Futures at the
    /// March 2025 contract @ 96.0725 with a quantity of 50 contracts
    /// and a duration of Good Till Close (GTC).
    ///
    /// ```ignore
    /// use tradestation::{ClientBuilder, Error, Token, execution::Duration};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     // Create client
    ///     let client = ClientBuilder::new()?
    ///         .credentials("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")?
    ///         .token(Token { /* YOUR BEARER AUTH TOKEN */ })?
    ///         .build()
    ///         .await?;
    ///
    ///     let order_req = OrderRequestBuilder::new()
    ///         .account_id("YOUR_FUTURES_ACCOUNT_ID")
    ///         .symbol("SR3H25")
    ///         .trade_action(TradeAction::Buy)
    ///         .quantity("50")
    ///         .order_type(OrderType::Limit)
    ///         .limit_price("96.0725")
    ///         .time_in_force(OrderTimeInForce {
    ///             duration: Duration::GTC,
    ///             expiration: None,
    ///         })
    ///         .build()?;
    ///
    ///     match order_req.confirm(&client).await {
    ///         Ok(confirmation) => println!("Confirmed Order: {confirmation:?}"),
    ///         Err(e) => println!("Issue Confirming Order: {e:?}"),
    ///     };
    ///     Ok(())
    /// }
    ///```
    pub async fn confirm(self, client: &Client) -> Result<Vec<OrderConfirmation>, Error> {
        Order::confirm(&self, client).await
    }
}

#[derive(Debug, Default)]
/// The initial stage of an `Order`, this is what
/// is sent to the route for creating a `Order`.
pub struct OrderRequestBuilder {
    account_id: Option<String>,
    advanced_options: Option<AdvancedOrderOptions>,
    buying_power_warning: Option<BPWarningStatus>,
    legs: Option<Vec<OrderRequestLeg>>,
    limit_price: Option<String>,
    osos: Option<Vec<Oso>>,
    order_confirm_id: Option<String>,
    order_type: Option<OrderType>,
    quantity: Option<String>,
    route: Option<String>,
    stop_price: Option<String>,
    symbol: Option<String>,
    time_in_force: Option<OrderTimeInForce>,
    trade_action: Option<TradeAction>,
}
impl OrderRequestBuilder {
    /// Initialize a new builder for `OrderRequest`.
    pub fn new() -> Self {
        OrderRequestBuilder::default()
    }

    /// Set the Account ID the `OrderRequest` belongs to.
    ///
    /// NOTE: Required to be set to build an `OrderRequest`.
    pub fn account_id(mut self, id: impl Into<String>) -> Self {
        self.account_id = Some(id.into());
        self
    }

    /// Set the Order Type for the `OrderRequest`.
    ///
    /// NOTE: Required to be set to build an `OrderRequest`.
    pub fn order_type(mut self, order_type: OrderType) -> Self {
        self.order_type = Some(order_type);
        self
    }

    /// Set the Symbol the `OrderRequest` is for.
    ///
    /// NOTE: This is required if no order legs are provided.
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        self
    }

    /// Set the Time In Force (Duration or expiration timestamp)
    /// for the `OrderRequest`.
    ///
    /// NOTE: Required to be set to build an `OrderRequest`.
    pub fn time_in_force(mut self, time_in_force: OrderTimeInForce) -> Self {
        self.time_in_force = Some(time_in_force);
        self
    }

    /// Set the Quantity of shares or contracts for the `OrderRequest`.
    ///
    /// NOTE: This is required if no order legs are provided.
    pub fn quantity(mut self, quantity: impl Into<String>) -> Self {
        self.quantity = Some(quantity.into());
        self
    }

    /// Set the Trade Action for the `OrderRequest`.
    ///
    /// NOTE: This is required if no order legs are provided.
    pub fn trade_action(mut self, action: TradeAction) -> Self {
        self.trade_action = Some(action);
        self
    }

    /// Set the Execution Route for the `OrderRequest`.
    pub fn route(mut self, route: impl Into<String>) -> Self {
        self.route = Some(route.into());
        self
    }

    /// Set a Stop Price for the `OrderRequest`.
    pub fn stop_price(mut self, price: impl Into<String>) -> Self {
        self.stop_price = Some(price.into());
        self
    }

    /// Set an Order Confirm ID for the `OrderRequest`.
    pub fn order_confirm_id(mut self, id: impl Into<String>) -> Self {
        self.order_confirm_id = Some(id.into());
        self
    }

    /// Set the Order Sends Order for the `OrderRequest`.
    pub fn osos(mut self, osos: Vec<Oso>) -> Self {
        self.osos = Some(osos);
        self
    }

    /// Set a Limit Price for the `OrderRequest`.
    pub fn limit_price(mut self, price: impl Into<String>) -> Self {
        self.limit_price = Some(price.into());
        self
    }

    /// Set the Legs of the `OrderRequest`.
    pub fn legs(mut self, legs: Vec<OrderRequestLeg>) -> Self {
        self.legs = Some(legs);
        self
    }

    /// Set the Buying Power Warning Status for the `OrderRequest`.
    pub fn buying_power_warning(mut self, status: BPWarningStatus) -> Self {
        self.buying_power_warning = Some(status);
        self
    }

    /// Set the Advanced Options for the `OrderRequest`.
    pub fn advanced_options(mut self, options: AdvancedOrderOptions) -> Self {
        self.advanced_options = Some(options);
        self
    }

    /// Finish building the `OrderRequest`.
    ///
    /// NOTE: `account_id`, `order_type`, and `time_in_force` are all required.
    pub fn build(self) -> Result<OrderRequest, Error> {
        Ok(OrderRequest {
            account_id: self.account_id.ok_or(Error::AccountIdNotSet)?,
            advanced_options: self.advanced_options,
            buying_power_warning: self.buying_power_warning,
            legs: self.legs,
            osos: self.osos,
            order_confirm_id: self.order_confirm_id,
            route: self.route,
            trade_action: self.trade_action,
            time_in_force: self.time_in_force.ok_or(Error::TimeInForceNotSet)?,
            symbol: self.symbol,
            order_type: self.order_type.ok_or(Error::OrderTypeNotSet)?,
            quantity: self.quantity,
            stop_price: self.stop_price,
            limit_price: self.limit_price,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// Advanced options for configuring orders.
pub struct AdvancedOrderOptions {
    /// This option allows you to place orders that will
    /// only add liquidity on the route you selected. To
    /// place an Add Liquidity order, the user must also
    /// select Book Only order type.
    ///
    /// NOTE: Only calid for Equities.
    pub add_liquidity: bool,

    /// Use this advanced order feature when you do not
    /// want a partial fill. Your order will be filled
    /// in its entirety or not at all.
    ///
    /// NOTE: Valid for Equities and Options.
    pub all_or_none: bool,

    /// This option restricts the destination you choose
    /// in the direct routing from re-routing your order
    /// to another destination. This type of order is useful
    /// in controlling your execution costs by avoiding
    /// fees the Exchanges can charge for rerouting your
    /// order to another market center.
    ///
    /// NOTE: Only valid for Equities.
    pub book_only: bool,

    /// You can use this option to reflect a Bid/Ask
    /// at a lower/higher price than you are willing
    /// to pay using a specified price increment.
    ///
    /// NOTE: Only valid for `Limit` and `StopLimit` orders.
    ///
    /// NOTE: Only valid for Equities.
    pub discretionary_price: String,

    /// Allows you to specify when an order will be placed
    /// based on the price action of one or more symbols.
    pub market_activation_rules: Vec<MarketActivationRule>,

    /// When you send a non-display order, it will not be
    /// reflected in either the Market Depth display or
    /// ECN books.
    ///
    /// NOTE: Only valid for Equities.
    pub non_display: bool,

    /// This order type is useful to achieve a fair price in
    /// a fast or volatile market.
    ///
    /// NOTE: Only valid for Equities.
    pub peg_value: PegValue,

    /// Hides the true number of shares or contracts intended
    /// to be bought or sold.
    ///
    /// NOTE: Only valid for `Limit` and `StopLimit` order types.
    ///
    /// NOTE: Only valid for Equities and Futures.
    ///
    /// <div class="warning">NOTE: Not valid for all exchanges.</div>
    pub show_only_quantity: String,

    /// Allows you to specify a time that an order will be placed.
    pub time_activation_rules: Vec<TimeActivationRule>,

    /// Trailing Stop offeset, amount or percent.
    pub trailing_stop: TrailingStop,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
/// The different types of valid peg values.
pub enum PegValue {
    /// The best Bid for a buy order and
    /// the best Ask for a sell order.
    #[serde(rename = "BEST")]
    Best,

    /// The mid-point price between the
    /// best bid and the best ask.
    #[serde(rename = "MID")]
    Mid,
}

// TODO: There is a similar enum in `crate::account`
// it should instead just use this enum.
#[derive(Clone, Debug, Deserialize, Serialize)]
/// The different trade actions that can be sent or
/// received, and conveys the intent of the order.
pub enum TradeAction {
    #[serde(rename = "BUY")]
    /// NOTE: Only for Equities and Futures
    Buy,

    #[serde(rename = "SELL")]
    /// NOTE: Only for Equities and Futures
    Sell,

    #[serde(rename = "BUYTOCOVER")]
    /// NOTE: Only for Equities
    BuyToCover,

    #[serde(rename = "SELLSHORT")]
    /// NOTE: Only for Equities
    SellShort,

    #[serde(rename = "BUYTOOPEN")]
    /// NOTE: Only for Options
    BuyToOpen,

    #[serde(rename = "BUYTOCLOSE")]
    /// NOTE: Only for Options
    BuyToClose,

    #[serde(rename = "SELLTOOPEN")]
    /// NOTE: Only for Options
    SellToOpen,

    #[serde(rename = "SELLTOCLOSE")]
    /// NOTE: Only for Options
    SellToClose,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// Defines the duration and expiration of an Order.
pub struct OrderTimeInForce {
    /// The duration type for the order.
    pub duration: Duration,

    /// The expiration timestamp for the order.
    pub expiration: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// A sub component order apart of the overall
/// trade the Order is for.
pub struct OrderRequestLeg {
    /// The symbol used for this leg of the order.
    pub symbol: String,

    /// The quantity of the order.
    pub quantity: String,

    /// The intent of the order.
    pub trade_action: TradeAction,

    /// The strike price for this option.
    ///
    /// NOTE: Only valid for options.
    pub strike_price: Option<String>,

    /// The type of option.
    ///
    /// NOTE: Only valid for options.
    pub option_type: Option<OptionType>,

    /// Timestamp represented as an `RFC3339` formatted date,
    /// a profile of the ISO 8601 date standard.
    ///
    /// E.g: `"2021-12-17T00:00:00Z"`.
    pub expiration_date: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
/// The different statuses for buing power warnings.
pub enum BPWarningStatus {
    /// Enforce, this status indicates that a buying
    /// power warning should be enforced.
    Enforce,

    /// Preconfirmed, this status indicates that a buying
    /// power warning has been displayed but not yet confirmed.
    Preconfirmed,

    /// Confirmed, this status indicates that a buying power
    /// warning has been displayed and is confirmed.
    Confirmed,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
/// The length of time for which an order will
/// remain valid in the market.
pub enum Duration {
    /// Day, valid until the end of the
    /// regular trading session.
    DAY,

    /// Day Plus, valid until the end of the
    /// extended trading session.
    DYP,

    /// Good Till Canceled, valid until the
    /// order is canceled.
    ///
    /// NOTE: There is a maximum lifespan of
    /// 90 calendar days.
    GTC,

    /// Good till Canceled Plus, valid until
    /// the order is canceled.
    ///
    /// NOTE: There is a maximum lifespan of
    /// 90 calendar days.
    GCP,

    /// Good Through Date, valid until a
    /// specified date.
    ///
    /// NOTE: There is a maximum lifespan of
    /// 90 calendar days.
    GTD,

    /// Good thourgh Date Plus, valid until a
    /// specified date.
    ///
    /// NOTE: There is a maximum lifespan of
    /// 90 calendar days.
    GDP,

    /// Opening, only valid for listed
    /// stocks at the opening session price.
    OPG,

    /// Close, orders that target the closing
    /// session of an exchange.
    CLO,

    /// Immediate Or Cancel, filled immediatly
    /// or canceled.
    ///
    /// NOTE: Partial fills of an order are accepted.
    IOC,

    /// Fill Or Kill, filled entirely or canceled.
    ///
    /// NOTE: Does NOT accept partial fills.
    FOK,

    /// 1 Minute, expires after one minute of
    /// being placed.
    #[serde(rename = "1")]
    OneMinute,

    /// 3 Minute, expires after three minutes of
    /// being placed.
    #[serde(rename = "3")]
    ThreeMinute,

    /// 5 Minute, expires after five minutes of
    /// being placed.
    #[serde(rename = "5")]
    FiveMinute,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// Order Sends Orders
pub struct Oso {
    /// Other orders in the OSO
    pub orders: Vec<OrderRequest>,

    /// The type of OSO Order
    pub r#type: AdvancedOrderType,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
/// Different types of advanced order types
pub enum AdvancedOrderType {
    /// Normal Order
    Normal,

    /// Bracket Order, multiple orders ranged
    /// in price.
    BRK,

    /// Order Cancels Other, multiple orders
    /// but only one can be filled as the rest
    /// cancel when any of the orders is filled.
    OCO,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// A Route for Order Execution
pub struct Route {
    /// The ID that must be sent in the optional Route
    /// property of a POST order request, when specifying
    /// a route for an order.
    pub id: String,

    /// The name of the route.
    pub name: String,

    /// The asset type of the route
    pub asset_types: Vec<AssetType>,
}
impl Route {
    /// Fetch valid routes for sending an order for execution.
    ///
    /// # Example
    /// ---
    /// Example: Fetch a list of routes to send orders for execution.
    /// ```ignore
    /// let routes = client.get_execution_routes().await?;
    /// println!("Valid routes for order execution: {routes:?}");
    /// ```
    pub async fn fetch(client: &Client) -> Result<Vec<Route>, Error> {
        let endpoint = String::from("orderexecution/routes");

        match client
            .get(&endpoint)
            .await?
            .json::<ApiResponse<GetExecutionRoutesRespRaw>>()
            .await?
        {
            ApiResponse::Success(resp_raw) => {
                let resp: GetExecutionRoutesResp = resp_raw.into();

                if let Some(routes) = resp.routes {
                    Ok(routes)
                } else {
                    Err(resp.error.unwrap_or(Error::UnknownTradeStationAPIError(
                        "Unknown TradeStation Error While Fetching Execution Routes.".into(),
                    )))
                }
            }
            ApiResponse::Error(resp) => Err(Error::from_api_error(resp)),
        }
    }
}
impl Client {
    /// Fetch valid routes for sending an order for execution.
    ///
    /// # Example
    /// ---
    /// Example: Fetch a list of routes to send orders for execution.
    /// ```ignore
    /// use tradestation::{ClientBuilder, Error, Token};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     // Create client
    ///     let client = ClientBuilder::new()?
    ///         .credentials("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")?
    ///         .token(Token { /* YOUR BEARER AUTH TOKEN */ })?
    ///         .build()
    ///         .await?;
    ///
    ///     // Example: Fetch a list of routes to send orders for execution.
    ///     let routes = client.get_execution_routes().await?;
    ///     println!("Valid routes for order execution: {routes:?}");
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_execution_routes(&self) -> Result<Vec<Route>, Error> {
        Route::fetch(self).await
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// Valid Activation Triggers for an Order.
pub struct ActivationTrigger {
    /// The Activation Trigger Key
    ///
    /// NOTE: This is what you with your orders.
    pub key: ActivationTriggerKey,

    /// Name of the Activation Trigger.
    pub name: String,

    /// Description of the Activation Trigger.
    pub description: String,
}
impl ActivationTrigger {
    /// Fetch Activation Triggers for Order Execution.
    ///
    /// NOTE: This provides the `key` that must be sent with an
    /// order to utilize and be triggered by the activation function.
    ///
    /// # Example
    /// ---
    /// Fetch valid activation triggers to utilize with your orders.
    ///
    /// ```ignore
    /// let triggers = client.get_activation_triggers().await?;
    /// println!("Valid activation triggers for order execution: {triggers:?}");
    /// ```
    pub async fn fetch(client: &Client) -> Result<Vec<ActivationTrigger>, Error> {
        let endpoint = String::from("orderexecution/activationtriggers");

        match client
            .get(&endpoint)
            .await?
            .json::<ApiResponse<GetActivationTriggersRespRaw>>()
            .await?
        {
            ApiResponse::Success(resp_raw) => {
                let resp: GetActivationTriggersResp = resp_raw.into();

                if let Some(triggers) = resp.activation_triggers {
                    Ok(triggers)
                } else {
                    Err(resp.error.unwrap_or(Error::UnknownTradeStationAPIError(
                        "Unknown TradeStation Error While Fetching Activation Triggers.".into(),
                    )))
                }
            }
            ApiResponse::Error(resp) => Err(Error::from_api_error(resp)),
        }
    }
}
impl Client {
    /// Fetch Activation Triggers for Order Execution.
    ///
    /// NOTE: This provides the `key` that must be sent with an
    /// order to utilize and be triggered by the activation function.
    ///
    /// # Example
    /// ---
    /// Fetch valid activation triggers to utilize with your orders.
    ///
    /// ```ignore
    /// use tradestation::{ClientBuilder, Error, Token};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     // Initialize client
    ///     let client = ClientBuilder::new()?
    ///         .credentials("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")?
    ///         .token(Token {
    ///             access_token: String::from("YOUR_ACCESS_TOKEN"),
    ///             refresh_token: String::from("YOUR_REFRESH_TOKEN"),
    ///             id_token: String::from("YOUR_ID_TOKEN"),
    ///             token_type: String::from("Bearer"),
    ///             scope: String::from("YOUR_SCOPES SPACE_SEPERATED FOR_EACH_SCOPE"),
    ///             expires_in: 1200,
    ///         })?
    ///         .build()
    ///         .await?;
    ///
    ///     // Fetch a list of valid activation triggers for order execution.
    ///     let triggers = client.get_activation_triggers().await?;
    ///     println!("Valid activation triggers for order execution: {triggers:?}");
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_activation_triggers(&self) -> Result<Vec<ActivationTrigger>, Error> {
        ActivationTrigger::fetch(self).await
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
/// The different types of activation trigger keys.
pub enum ActivationTriggerKey {
    /// Single Trade Tick, one trade tick must print
    /// within your stop price to trigger your stop.
    STT,

    /// Single Trade Tick Within NBBO, one trade tick
    /// within the National Best Bid or Offer (NBBO)
    /// must print within your stop price to trigger
    /// your stop.
    STTN,

    /// Single Bid/Ask Tick
    /// ---
    /// * Buy/Cover Orders: One Ask tick must print within
    ///   your stop price to trigger your stop.
    ///
    /// * Sell/Short Orders: One Bid tick must print within
    ///   your stop price to trigger your stop.
    SBA,

    /// Single Ask/Bid Tick
    /// ---
    /// * Buy/Cover Orders: One Bid tick must print within
    ///   your stop price to trigger your stop.
    ///
    /// * Sell/Short Orders: One Ask tick must print within
    ///   your stop price to trigger your stop.
    SAB,

    /// Double Trade Tick, two consecutive trade ticks must
    /// print within your stop price to trigger your stop.
    DTT,

    /// Double Trade Tick Within NBBO, two consecutive trade
    /// ticks within the National Best Bid or Offer (NBBO) must
    /// print within your stop price to trigger your stop.
    DTTN,

    /// Double Bid/Ask Tick
    /// ---
    /// * Buy/Cover Orders: Two consecutive Ask ticks must print
    ///   within your stop price to trigger your stop.
    ///
    /// * Sell/Short Orders: Two consecutive Bid ticks must print
    ///   within your stop price to trigger your stop.
    DBA,

    /// Double Ask/Bid Tick
    /// ---
    /// * Buy/Cover Orders: Two consecutive Bid ticks must print
    ///   within your stop price to trigger your stop.
    ///
    /// * Sell/Short Orders: Two consecutive Ask ticks must print
    ///   within your stop price to trigger your stop.
    DAB,

    /// Twice Trade Tick, two trade ticks must print within your
    /// stop price to trigger your stop.
    TTT,

    /// Twice Trade Tick Within NBBO, two trade ticks within the
    /// National Best Bid or Offer (NBBO) must print within your
    /// stop price to trigger your stop.
    TTTN,

    /// Twice Bid/Ask Tick
    /// ---
    /// * Buy/Cover Orders: Two Ask ticks must print within your
    ///   stop price to trigger your stop.
    ///
    /// * Sell/Short Orders: Two Bid ticks must print within your
    ///   stop price to trigger your stop.
    TBA,

    /// Twice Ask/Bid Tick
    /// ---
    /// * Buy/Cover Orders: Two Bid ticks must print within your
    ///   stop price to trigger your stop.
    ///
    /// * Sell/Short Orders: Two Ask ticks must print within your
    ///   stop price to trigger your stop.
    TAB,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// A confirmed order.
///
/// NOTE: This is NOT a placed order, but similar to a mock order
/// confirming it's valid and ready to be sent to the intended route.
pub struct OrderConfirmation {
    /// The route of the order.
    ///
    /// NOTE: For Stocks and Options, Route value will
    /// default to Intelligent if no value is set.
    pub route: String,

    /// Defines the duration or expiration timestamp of an Order.
    pub time_in_force: OrderTimeInForce,

    #[serde(rename = "AccountID")]
    /// The ID of the Account the order belongs to.
    pub account_id: String,

    /// A short text summary / description of the order.
    pub summary_message: String,

    /// The asset category for the asset an order is for.
    pub order_asset_category: OrderAssetCategory,

    #[serde(rename = "OrderConfirmID")]
    /// The ID of the order confirm.
    pub order_confirm_id: String,

    /// The limit price of the order.
    ///
    /// NOTE: Only valid for orders with `OrderType::Limit`.
    pub limit_price: Option<String>,

    /// When you send a non-display order, it will not be
    /// reflected in either the market depth display or ECN books.
    ///
    /// NOTE: Only valid for equities.
    pub non_display: Option<bool>,

    /// This order type is useful to achieve a
    /// fair price in a fast or volatile market.
    ///
    /// NOTE: Only valid for equities.
    pub peg_value: Option<PegValue>,

    /// Hides the true number of shares intended to be bought or sold.
    ///
    /// NOTE: Only valid for orders with `OrderType::Limit` and
    /// `OrderType::StopLimit`.
    ///
    /// NOTE: Not valid for all exchanges.
    pub show_only_quantity: Option<i64>,

    /// The option spread type.
    ///
    /// NOTE: Only valid for options.
    pub spread: Option<OptionSpreadType>,

    /// The stop price for open orders.
    pub stop_price: Option<String>,

    /// The trailing stop offset for an order.
    pub trailing_stop: Option<TrailingStop>,

    /// The order legs related to the overall order.
    pub legs: Option<Vec<OrderRequestLeg>>,

    /// The underlying symbol name the order is for.
    pub underlying: Option<String>,

    /// The estimated price of the order.
    pub estimated_price: String,

    /// The estimated display price of the order.
    pub estimated_price_display: Option<String>,

    /// The estimated cost of the order.
    pub estimated_cost: String,

    /// The estimated display cost of the order.
    pub estimated_cost_display: Option<String>,

    /// The estimated commission cost for the order.
    pub estimated_commission: String,

    /// The estimated commission cost display for the order.
    pub estimated_commission_display: Option<String>,

    /// The estimated debit or credit cost of the the order.
    ///
    /// NOTE: Debit costs will have a positive cost, and credit
    /// costs will have a negative cost.
    pub debit_credit_estimated_cost: Option<String>,

    /// The estimated debit or credit display cost of the the order.
    ///
    /// NOTE: Debit costs will have a positive cost, and credit
    /// costs will have a negative cost.
    pub debit_credit_estimated_cost_display: Option<String>,

    /// The descretionary price of the order, which can be used to
    /// reflect a Bid/Ask at a lower/higher price than you are willing
    /// to pay using a specified price increment.
    ///
    /// NOTE: Only valid for `OrderType::Limit` & `OrderType::StopLimit`.
    ///
    /// NOTE: Only valid for equities.
    pub descretionary_price: Option<String>,

    /// Is the order using book only option, which restricts the destination
    /// you choose in the direct routing from re-routing your order to another
    /// destination.
    ///
    /// This type of order is useful in controlling your execution costs by
    /// avoiding fees the exchanges can charge for re-routing your order to
    /// another market center.
    ///
    /// NOTE: Only valid for equities.
    pub book_only: Option<bool>,

    /// Is the order using the all or none option, which avoids partial fills
    /// on your order. Your order will either be filled in full or not at all
    /// when using this option.
    ///
    /// NOTE: Only valid for equities and options.
    pub all_or_none: Option<bool>,

    /// Is the order using the add liquidity option, which allows you to place
    /// orders that will only add liquidity on the route you selected.
    ///
    /// NOTE: Only valid if you're also using the `book_only` option on the order.
    ///
    /// NOTE: Only valid for equities.
    pub add_liquidity: Option<bool>,

    /// The currency the product is based on.
    ///
    /// NOTE: Only valid for futures orders.
    pub product_currency: Option<String>,

    /// The currency the account is based on.
    ///
    /// NOTE: Only valid for futures orders.
    pub account_currency: Option<String>,

    /// The initial margin display cost of the order.
    ///
    /// NOTE: Only valid for futures orders.
    pub initial_margin_display: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// The different asset categories an `Order` can fall into.
pub enum OrderAssetCategory {
    #[serde(rename = "EQUITY")]
    /// Orders for Stocks, ETFs, ETNs, etc.
    Equity,

    #[serde(rename = "OPTION")]
    /// Orders for Options on Stocks, ETFs, ETNs, etc.
    Option,

    #[serde(rename = "FUTURE")]
    /// Orders for Future Contracts
    Future,
}
