use crate::{
    accounting::{
        orders::{MarketActivationRule, TimeActivationRule, TrailingStop},
        OptionType, Order,
    },
    execution::{OrderRequest, OrderRequestGroup, OrderTicket, OrderUpdate},
    responses::{
        execution::{ModifyOrderResp, ModifyOrderRespRaw, OrderResp, OrderRespRaw},
        ApiResponse,
    },
    Client, Error,
};
use serde::{Deserialize, Serialize};

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
    /// match Order::place(order_req, &client,).await {
    ///     Ok(resp) => println!("Order Response: {resp:?}"),
    ///     Err(e) => println!("Order Response: {e:?}"),
    /// }
    /// ```
    pub async fn place(
        order_request: &OrderRequest,
        client: &Client,
    ) -> Result<Vec<OrderTicket>, Error> {
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
    ) -> Result<Vec<OrderTicket>, Error> {
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
    pub async fn replace(
        self,
        order_update: OrderUpdate,
        client: &Client,
    ) -> Result<OrderTicket, Error> {
        let endpoint = format!("orderexecution/orders/{}", self.order_id);

        match client
            .put(&endpoint, &order_update)
            .await?
            .json::<ApiResponse<ModifyOrderRespRaw>>()
            .await?
        {
            ApiResponse::Success(resp_raw) => {
                let resp: ModifyOrderResp = resp_raw.into();
                let order: OrderTicket = resp.into();

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
    pub async fn cancel(self, client: &Client) -> Result<OrderTicket, Error> {
        let endpoint = format!("orderexecution/orders/{}", self.order_id);

        match client
            .delete(&endpoint)
            .await?
            .json::<ApiResponse<ModifyOrderRespRaw>>()
            .await?
        {
            ApiResponse::Success(resp_raw) => {
                let resp: ModifyOrderResp = resp_raw.into();
                let order: OrderTicket = resp.into();

                Ok(order)
            }
            ApiResponse::Error(resp) => Err(Error::from_api_error(resp)),
        }
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
/// The different types of order groups
pub enum OrderGroupType {
    /// Bracket Order
    BRK,

    /// Order Cancels Order
    OCO,

    /// Normal Group of Orders
    NORMAL,
}
