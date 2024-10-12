use crate::{
    account::{AssetType, MarketActivationRule, OrderType, TimeActivationRule, TrailingStop},
    responses::execution::{
        GetActivationTriggersResp, GetActivationTriggersRespRaw, GetExecutionRoutesResp,
        GetExecutionRoutesRespRaw,
    },
    Client, Error,
};
use serde::{Deserialize, Serialize};

// TODO: Support builder pattern's for `OrderRequest`
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// The initial stage of an `Order`, this is what
/// is sent to the route for creating a `Order`.
pub struct OrderRequest {
    /// The TradeStation Account ID the order is for.
    pub account_id: String,

    /// Advanced Options for configuring an order.
    pub advanced_options: AdvancedOrderOptions,

    /// The different statuses for buing power warnings.
    pub buying_power_warning: Option<BPWarningStatus>,

    /// The additional legs to this order.
    pub legs: Vec<OrderRequestLeg>,

    /// The limit price for this order.
    pub limit_price: Option<String>,

    /// Order Sends Orders
    pub osos: Vec<Oso>,

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
    pub expiration: String,
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
    OneMinute,

    /// 3 Minute, expires after three minutes of
    /// being placed.
    ThreeMinute,

    /// 5 Minute, expires after five minutes of
    /// being placed.
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
