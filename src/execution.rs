use crate::{
    account::{AssetType, MarketActivationRule, OrderType, TimeActivationRule, TrailingStop},
    responses::execution::{
        ConfirmOrderResp, ConfirmOrderRespRaw, GetActivationTriggersResp,
        GetActivationTriggersRespRaw, GetExecutionRoutesResp, GetExecutionRoutesRespRaw, OrderResp,
        OrderRespRaw,
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
    /// match Order::place(&mut client, order_req).await {
    ///     Ok(resp) => println!("Order Response: {resp:?}"),
    ///     Err(e) => println!("Order Response: {e:?}"),
    /// }
    /// ```
    pub async fn place(
        client: &mut Client,
        order_request: &OrderRequest,
    ) -> Result<Vec<Order>, Error> {
        let endpoint = String::from("orderexecution/orders");

        let resp: OrderResp = client
            .post(&endpoint, &order_request)
            .await?
            .json::<OrderRespRaw>()
            .await?
            .into();

        if let Some(orders) = resp.orders {
            Ok(orders)
        } else {
            Err(resp.error.unwrap_or(Error::UnknownTradeStationAPIError))
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
    /// let order = Order::place(&mut client, &order_req).await?;
    ///
    /// if let Some(order) = order.first() {
    ///     order
    ///         .clone()
    ///         .replace(
    ///             &mut client,
    ///             OrderUpdate::new().limit_price("42.50").quantity("25"),
    ///         )
    ///         .await?;
    /// }
    /// ```
    pub async fn replace(
        self,
        client: &mut Client,
        order_update: OrderUpdate,
    ) -> Result<Vec<Order>, Error> {
        let endpoint = format!("orderexecution/orders/{}", self.order_id);

        let resp: OrderResp = client
            .put(&endpoint, &order_update)
            .await?
            .json::<OrderRespRaw>()
            .await?
            .into();

        if let Some(orders) = resp.orders {
            Ok(orders)
        } else {
            Err(resp.error.unwrap_or(Error::UnknownTradeStationAPIError))
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "PascalCase")]
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

// TODO: Support builder pattern's for `OrderRequest`
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
    pub quantity: String,

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
    pub symbol: String,

    /// Defines the duration and expiration timestamp of an Order.
    pub time_in_force: OrderTimeInForce,

    /// The different trade actions that can be sent or
    /// received, and conveys the intent of the order.
    pub trade_action: TradeAction,
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
    ///     let mut client = ClientBuilder::new()?
    ///         .set_credentials("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")?
    ///         .set_token(Token { /* YOUR BEARER AUTH TOKEN */ })?
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
    ///     match order_req.confirm(&mut client).await {
    ///         Ok(confirmation) => println!("Confirmed Order: {confirmation:?}"),
    ///         Err(e) => println!("Issue Confirming Order: {e:?}"),
    ///     };
    ///     Ok(())
    /// }
    ///```
    pub async fn confirm(self, client: &mut Client) -> Result<Vec<OrderConfirmation>, Error> {
        let endpoint = String::from("orderexecution/orderconfirm");
        let resp: ConfirmOrderResp = client
            .post(&endpoint, &self)
            .await?
            .json::<ConfirmOrderRespRaw>()
            .await?
            .into();

        if let Some(confirmations) = resp.confirmations {
            Ok(confirmations)
        } else {
            Err(resp.error.unwrap_or(Error::UnknownTradeStationAPIError))
        }
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
    /// NOTE: Required to be set to build an `OrderRequest`.
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
    pub fn quantity(mut self, quantity: impl Into<String>) -> Self {
        self.quantity = Some(quantity.into());
        self
    }

    /// Set the Trade Action for the `OrderRequest`.
    ///
    /// NOTE: Required to be set to build an `OrderRequest`.
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
    /// NOTE: `account_id`, `order_type`, `quantity`, `symbol`,
    /// `time_in_force`, and `trade_action` are all required.
    pub fn build(self) -> Result<OrderRequest, Error> {
        Ok(OrderRequest {
            account_id: self.account_id.ok_or(Error::AccountIdNotSet)?,
            advanced_options: self.advanced_options,
            buying_power_warning: self.buying_power_warning,
            legs: self.legs,
            osos: self.osos,
            order_confirm_id: self.order_confirm_id,
            route: self.route,
            trade_action: self.trade_action.ok_or(Error::TradeActionNotSet)?,
            time_in_force: self.time_in_force.ok_or(Error::TimeInForceNotSet)?,
            symbol: self.symbol.ok_or(Error::SymbolNotSet)?,
            order_type: self.order_type.ok_or(Error::OrderTypeNotSet)?,
            quantity: self.quantity.ok_or(Error::QuantityNotSet)?,
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

    // TODO: I think I can enum this
    /// This order type is useful to achieve a fair price in
    /// a fast or volatile market.
    ///
    /// NOTE: Only valid for Equities.
    pub peg_value: String,

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
    /// The quantity of the order.
    pub quantity: String,
    /// The symbol used for this leg of the order.
    pub symbol: String,
    /// The intent of the order.
    pub trade_action: TradeAction,
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
    /// use tradestation::{ClientBuilder, Error, Token};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     // Create client
    ///     let mut client = ClientBuilder::new()?
    ///         .set_credentials("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")?
    ///         .set_token(Token { /* YOUR BEARER AUTH TOKEN */ })?
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
    pub async fn fetch(client: &mut Client) -> Result<Vec<Route>, Error> {
        let endpoint = String::from("orderexecution/routes");
        let resp: GetExecutionRoutesResp = client
            .get(&endpoint)
            .await?
            .json::<GetExecutionRoutesRespRaw>()
            .await?
            .into();

        if let Some(routes) = resp.routes {
            Ok(routes)
        } else {
            Err(resp.error.unwrap_or(Error::UnknownTradeStationAPIError))
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
    ///     let mut client = ClientBuilder::new()?
    ///         .set_credentials("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")?
    ///         .set_token(Token { /* YOUR BEARER AUTH TOKEN */ })?
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
    pub async fn get_execution_routes(&mut self) -> Result<Vec<Route>, Error> {
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
    /// use tradestation::{ClientBuilder, Error, Token};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     // Initialize client
    ///     let mut client = ClientBuilder::new()?
    ///         .set_credentials("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")?
    ///         .set_token(Token {
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
    pub async fn fetch(client: &mut Client) -> Result<Vec<ActivationTrigger>, Error> {
        let endpoint = String::from("orderexecution/activationtriggers");
        let resp: GetActivationTriggersResp = client
            .get(&endpoint)
            .await?
            .json::<GetActivationTriggersRespRaw>()
            .await?
            .into();

        if let Some(triggers) = resp.activation_triggers {
            Ok(triggers)
        } else {
            Err(resp.error.unwrap_or(Error::UnknownTradeStationAPIError))
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
    ///     let mut client = ClientBuilder::new()?
    ///         .set_credentials("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")?
    ///         .set_token(Token {
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
    pub async fn get_activation_triggers(&mut self) -> Result<Vec<ActivationTrigger>, Error> {
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

    #[serde(rename = "OrderConfirmID")]
    /// The ID of the order confirm.
    pub order_confirm_id: String,

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
