use crate::{
    accounting::orders::{Order, OrderRelationship, OrderType},
    execution::{
        orders::{
            AdvancedOrderOptions, BPWarningStatus, OrderRequestLeg, OrderTimeInForce, Oso,
            TradeAction,
        },
        ticket::OrderTicket,
    },
    Client, Error,
};
use serde::{Deserialize, Serialize};

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
/// An collection of `OrderRequest`'s to be sent together.
pub struct OrderRequestGroup {
    pub order_requests: Vec<OrderRequest>,
    pub group_type: OrderRelationship,
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
    ///     accounting::OrderRelationship,
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
    ///         .group_type(OrderRelationship::BRK)
    ///         .build()?;
    ///
    ///     let orders = order_group.place(&client).await?;
    ///     println!("Place Orders Result: {orders:?}");
    /// }
    /// ```
    pub async fn place(&self, client: &Client) -> Result<Vec<OrderTicket>, Error> {
        Order::place_group(self, client).await
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
/// `OrderRequestGroup` builder
pub struct OrderRequestGroupBuilder {
    order_requests: Option<Vec<OrderRequest>>,
    group_type: Option<OrderRelationship>,
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

    /// Set the Order Group Type ([`OrderRelationship`]).
    pub fn group_type(mut self, group_type: OrderRelationship) -> Self {
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
