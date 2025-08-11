use crate::{
    accounting::orders::{AssetType, Order, TrailingStop},
    execution::{
        orders::{OrderRequestLeg, OrderTimeInForce, PegValue},
        request::{OrderRequest, OrderRequestGroup},
    },
    market_data::OptionSpreadType,
    responses::{
        execution::{ConfirmOrderResp, ConfirmOrderRespRaw},
        ApiResponse,
    },
    Client, Error,
};
use serde::{Deserialize, Serialize};

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
    pub order_asset_category: AssetType,

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

impl Order {
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

impl OrderRequestGroup {
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
    ///     let order_confirmations = order_group.confirm(&client).await?;
    ///     println!("Confirm Orders Result: {order_confirmations:?}");
    /// }
    /// ```
    pub async fn confirm(self, client: &Client) -> Result<Vec<OrderConfirmation>, Error> {
        Order::confirm_group(&self, client).await
    }
}
