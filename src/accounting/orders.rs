use crate::{
    responses::{
        account::{GetOrdersResp, StreamOrdersResp},
        ApiResponse,
    },
    Client, Error,
};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// An order to open, close, add, or trim positions.
pub struct Order {
    #[serde(rename = "AccountID")]
    /// The `Account` id to the this `Order` belongs to.
    pub account_id: String,

    /// The `Order rules` or brackets.
    pub advanced_options: Option<String>,

    /// The Closed Date Time of this `Order`.
    pub closed_date_time: Option<String>,

    /// The actual brokerage commission cost and routing fees
    /// for a trade based on the number of shares or contracts.
    pub commission_fee: String,

    /// The relationship between linked `Order`(s) in a group
    /// and this `Order` in specific.
    pub conditional_orders: Option<ConditionalOrder>,

    /// The rate used to convert from the currency of
    /// the symbol to the currency of the account.
    pub conversion_rate: Option<String>,

    /// The currency used to complete the `Order`.
    pub currency: String,

    /// The amount of time for which the `Order` is valid.
    pub duration: String,

    /// At the top level, this is the average fill price.
    ///
    /// At the expanded levels, this is the actual execution price.
    pub filled_price: Option<String>,

    /// The expiration date-time for the `Order`
    ///
    /// NOTE: The time portion, if `"T:00:00:00Z"`, should be ignored.
    pub good_till_date: Option<String>,

    /// An identifier for `Order`(s) that are part of the same bracket.
    pub group_name: Option<String>,

    /// Legs (multi step/part trade) associated with this `Order`
    pub legs: Vec<OrderLeg>,

    /// Allows you to specify when an order will be placed based on
    /// the price action of one or more symbols.
    #[serde(default, deserialize_with = "empty_vec_from_null")]
    pub market_activation_rules: Vec<MarketActivationRule>,

    /// Allows you to specify a time that an `Order` will be placed.
    #[serde(default, deserialize_with = "empty_vec_from_null")]
    pub time_activation_rules: Vec<TimeActivationRule>,

    /// The limit price for Limit and Stop Limit `Order`(s).
    pub limit_price: Option<String>,

    /// Time the `Order` was placed.
    pub opened_date_time: String,

    #[serde(rename = "OrderID")]
    /// The `Order` id.
    pub order_id: String,

    /// The type of `Order` this is.
    pub order_type: OrderType,

    /// Price used for the buying power calculation of the `Order`.
    pub price_used_for_buying_power: String,

    /// Identifies the routing selection made by the customer when
    /// placing the `Order`.
    ///
    /// NOTE: ONLY valid for Equities.
    pub routing: Option<String>,

    /// Hides the true number of shares intended to be bought or sold.
    ///
    /// NOTE: ONLY valid for `OrderType::Limit` or `Order::Type::StopLimit`
    /// `Order`(s).
    ///
    /// NOTE: Not valid for all exchanges.
    pub show_only_quantity: Option<String>,

    /// The spread type for an option `Order`
    pub spread: Option<String>,

    /// The status of an `Order`
    pub status: OrderStatus,

    /// Description of the `Order` status
    pub status_description: String,

    /// The stop price for `OrderType::StopLimit` and
    /// `OrderType::StopMarket` orders.
    pub stop_price: Option<String>,

    /// TrailingStop offset.
    ///
    /// NOTE: amount or percent.
    pub trailing_stop: Option<TrailingStop>,

    /// Only applies to equities.
    ///
    /// NOTE: Will contain a value if the order has received a routing fee.
    pub unbundled_route_fee: Option<String>,
}
impl Order {
    /// Fetches orders for the given `Account`.
    pub(super) async fn get_all_by_account<S: Into<String>>(
        account_id: S,
        client: &mut Client,
    ) -> Result<Vec<Order>, Error> {
        let endpoint = format!("brokerage/accounts/{}/orders", account_id.into());

        match client
            .get(&endpoint)
            .await?
            .json::<ApiResponse<GetOrdersResp>>()
            .await?
        {
            ApiResponse::Success(resp) => Ok(resp.orders),
            ApiResponse::Error(resp) => Err(Error::from_api_error(resp)),
        }
    }

    /// Fetches orders by order id for the given `Account`.
    pub(super) async fn find<S: Into<String>>(
        order_ids: Vec<S>,
        account_id: String,
        client: &mut Client,
    ) -> Result<Vec<Order>, Error> {
        let order_ids: Vec<String> = order_ids.into_iter().map(|id| id.into()).collect();

        let endpoint = format!(
            "brokerage/accounts/{}/orders/{}",
            account_id,
            &order_ids.join(",")
        );

        match client
            .get(&endpoint)
            .await?
            .json::<ApiResponse<GetOrdersResp>>()
            .await?
        {
            ApiResponse::Success(resp) => Ok(resp.orders),
            ApiResponse::Error(resp) => Err(Error::from_api_error(resp)),
        }
    }

    /// Fetches Historical `Order`(s) since a specific date for the given `Account`.
    pub(super) async fn get_historic<S: Into<String>>(
        account_id: S,
        since_date: &str,
        client: &mut Client,
    ) -> Result<Vec<Order>, Error> {
        let endpoint = format!(
            "brokerage/accounts/{}/historicalorders?since={}",
            account_id.into(),
            since_date
        );

        match client
            .get(&endpoint)
            .await?
            .json::<ApiResponse<GetOrdersResp>>()
            .await?
        {
            ApiResponse::Success(resp) => Ok(resp.orders),
            ApiResponse::Error(resp) => Err(Error::from_api_error(resp)),
        }
    }

    /// Fetches Historical `Order`(s) for the given `Account`(s) by id.
    pub(super) async fn get_historic_by_accounts(
        account_ids: Vec<&str>,
        since_date: &str,
        client: &mut Client,
    ) -> Result<Vec<Order>, Error> {
        let endpoint = format!(
            "brokerage/accounts/{}/historicalorders?since={}",
            account_ids.join(","),
            since_date,
        );

        match client
            .get(&endpoint)
            .await?
            .json::<ApiResponse<GetOrdersResp>>()
            .await?
        {
            ApiResponse::Success(resp) => Ok(resp.orders),
            ApiResponse::Error(resp) => Err(Error::from_api_error(resp)),
        }
    }

    /// Stream `Order`(s) for the given `Account`.
    pub(super) async fn stream<F, S: Into<String>>(
        account_id: S,
        client: &mut Client,
        mut on_chunk: F,
    ) -> Result<Vec<Order>, Error>
    where
        F: FnMut(StreamOrdersResp) -> Result<(), Error>,
    {
        let endpoint = format!("brokerage/stream/accounts/{}/orders", account_id.into());

        let mut collected_orders: Vec<Order> = Vec::new();
        client
            .stream(&endpoint, |chunk| {
                let parsed_chunk = serde_json::from_value::<StreamOrdersResp>(chunk)?;
                on_chunk(parsed_chunk.clone())?;

                // Only collect orders, so when the stream is done
                // all the orders that were streamed can be returned
                if let StreamOrdersResp::Order(order) = parsed_chunk {
                    collected_orders.push(*order);
                }

                Ok(())
            })
            .await?;

        Ok(collected_orders)
    }

    /// Stream `Order`(s) by order id's for the given `Account`.
    pub(super) async fn stream_by_ids<F>(
        order_ids: Vec<&str>,
        account_id: &str,
        client: &mut Client,
        mut on_chunk: F,
    ) -> Result<Vec<Order>, Error>
    where
        F: FnMut(StreamOrdersResp) -> Result<(), Error>,
    {
        let endpoint = format!(
            "brokerage/stream/accounts/{}/orders/{}",
            account_id,
            order_ids.join(",")
        );

        let mut collected_orders: Vec<Order> = Vec::new();
        client
            .stream(&endpoint, |chunk| {
                let parsed_chunk = serde_json::from_value::<StreamOrdersResp>(chunk)?;
                on_chunk(parsed_chunk.clone())?;

                // Only collect orders, so when the stream is done
                // all the orders that were streamed can be returned
                if let StreamOrdersResp::Order(order) = parsed_chunk {
                    collected_orders.push(*order);
                }

                Ok(())
            })
            .await?;

        Ok(collected_orders)
    }

    /// Stream `Order`(s) for the given `Account`.
    pub(super) async fn stream_by_accounts<F>(
        account_ids: Vec<&str>,
        client: &mut Client,
        mut on_chunk: F,
    ) -> Result<Vec<Order>, Error>
    where
        F: FnMut(StreamOrdersResp) -> Result<(), Error>,
    {
        let endpoint = format!("brokerage/stream/accounts/{}/orders", account_ids.join(","));

        let mut collected_orders: Vec<Order> = Vec::new();
        client
            .stream(&endpoint, |chunk| {
                let parsed_chunk = serde_json::from_value::<StreamOrdersResp>(chunk)?;
                on_chunk(parsed_chunk.clone())?;

                // Only collect orders so when the stream is done
                // all the orders that were streamed can be returned
                if let StreamOrdersResp::Order(order) = parsed_chunk {
                    collected_orders.push(*order);
                }

                Ok(())
            })
            .await?;

        Ok(collected_orders)
    }

    /// Stream `Order`s by order id's for the given `Account`(s).
    pub(super) async fn stream_by_ids_and_accounts<F>(
        client: &mut Client,
        order_ids: Vec<&str>,
        account_ids: Vec<&str>,
        mut on_chunk: F,
    ) -> Result<Vec<Order>, Error>
    where
        F: FnMut(StreamOrdersResp) -> Result<(), Error>,
    {
        let endpoint = format!(
            "brokerage/stream/accounts/{}/orders/{}",
            account_ids.join(","),
            order_ids.join(","),
        );

        let mut collected_orders: Vec<Order> = Vec::new();
        client
            .stream(&endpoint, |chunk| {
                let parsed_chunk = serde_json::from_value::<StreamOrdersResp>(chunk)?;
                on_chunk(parsed_chunk.clone())?;

                // Only collect orders so when the stream is done
                // all the orders that were streamed can be returned
                if let StreamOrdersResp::Order(order) = parsed_chunk {
                    collected_orders.push(*order);
                }

                Ok(())
            })
            .await?;

        Ok(collected_orders)
    }
}

/// Serialize null values into empty vectors.
fn empty_vec_from_null<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    let opt = Option::<Vec<T>>::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum OrderStatus {
    /// Acknowledged (Received)
    ACK,

    /// Option Assignment
    ASS,

    /// Bracket Canceled
    BRC,

    /// Bracket Filled
    BRF,

    /// Broken
    BRO,

    /// Change
    CHG,

    /// Condition Met
    CND,

    /// Fill Corrected
    COR,

    /// Cancel Sent
    CSN,

    /// Dispatched
    DIS,

    /// Dead
    DOA,

    /// Queued
    DON,

    /// Expiration Cancel Request
    ECN,

    /// Option Excercise
    EXE,

    /// Partial Fill (Alive)
    FPR,

    /// Too Late to Cancel
    LAT,

    /// Sent
    OPN,

    /// Order Sends Order
    OSO,

    /// Sending
    PLA,

    /// Big Brother Recall Request
    REC,

    /// Cancel Request Rejected
    RJC,

    /// Replace Pending
    RPD,

    /// Replace Sent
    RSN,

    /// Stop Hit
    STP,

    /// OrderStatus Message
    STT,

    /// Suspended
    SUS,

    /// Cancel Sent
    UCN,

    /// Canceled
    CAN,

    /// Expired
    EXP,

    /// UROut
    OUT,

    /// Change Request Rejected
    RJR,

    /// Big Brother Recall
    SCN,

    /// Trade Server Canceled
    TSC,

    /// Replaced
    UCH,

    /// Rejected
    REJ,

    /// Filled
    FLL,

    /// Partial Fill (UROut)
    FLP,

    /// Unmapped OrderStatus
    OTHER,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct TrailingStop {
    /// Currency Offset from current price.
    ///
    /// NOTE: Mutually exclusive with Percent.
    pub amount: Option<String>,

    /// Percentage offset from current price.
    ///
    /// NOTE: Mutually exclusive with Amount.
    pub percent: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
/// Types of `Order`(s).
pub enum OrderType {
    /// Limit Order
    Limit,

    /// Market Order
    Market,

    /// Stop Loss At Market Order
    StopMarket,

    /// Stop Loss At Limit Order
    StopLimit,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// Allows you to specify a time that an `Order` will be placed.
pub struct TimeActivationRule {
    /// Timestamp represented as an RFC3339 formatted date.
    time_utc: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// Allows you to specify when an order will be placed
/// based on the price action of one or more symbols.
pub struct MarketActivationRule {
    /// The type of activation rule.
    ///
    /// NOTE: Currently only supports `"Price"` for now.
    pub rule_type: String,

    /// The symbol that the rule is based on.
    pub symbol: String,

    /// The type of comparison predicate the rule is based on.
    pub predicate: Predicate,

    /// The ticks behavior for the activation rule.
    pub tigger_key: TickTrigger,

    /// The price at which the rule will trigger.
    pub price: Option<String>,

    /// Relation with the previous activation rule.
    ///
    /// NOTE: The first rule will never have a logic operator.
    pub logic_operator: Option<LogicOp>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
/// Logic Operators
pub enum LogicOp {
    And,

    Or,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
/// Types of tick triggers
pub enum TickTrigger {
    /// Single Trade Tick
    STT,

    /// Single Trade Tick Within NBBO
    STTN,

    /// Single Bid/Ask Tick
    SBA,

    /// Single Ask/Bid Tick
    SAB,

    /// Double Trade Tick
    DTT,

    /// Double Trade Tick Within NBBO
    DTTN,

    /// Double Bid/Ask Tick
    DBA,

    /// Double Ask/Bid Tick
    DAB,

    /// Triple Trade Tick
    TTT,

    /// Triple Trade Tick Within NBBO
    TTTN,

    /// Triple Bid/Ask Tick
    TBA,

    /// Triple Ask/Bid Tick
    TAB,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
/// Types of comparison predicates
pub enum Predicate {
    /// Less than
    Lt,

    /// Less than or Equal
    Lte,

    /// Greater than
    Gt,

    /// Greater than or Equal
    Gte,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// Order Leg associated with this `Order`
pub struct OrderLeg {
    /// Indicates the asset type of the `Order`.
    pub asset_type: AssetType,

    /// identifier for the `Order` action (buying or selling)
    pub buy_or_sell: OrderAction,

    /// Number of shares or contracts that have been executed.
    pub exec_quantity: String,

    /// The price at which `Order` execution occurred.
    pub execution_price: Option<String>,

    /// The expiration date of the future or option contract.
    pub expiration_date: Option<String>,

    /// The stage of the `Order` , is it opening or closing?
    pub open_or_close: Option<OrderStage>,

    /// The type of option
    pub option_type: Option<OptionType>,

    /// Number of shares or contracts being purchased or sold.
    pub quantity_ordered: String,

    /// In a partially filled `Order` , this is the number of shares
    /// or contracts the have NOT yet been filled.
    pub quantity_remaining: String,

    /// The price at which the holder of an options contract can buy
    /// or sell the underlying asset.
    ///
    /// NOTE: ONLY for option `Order`(s).
    pub strike_price: Option<String>,

    /// The securities symbol the `Order` is for.
    pub symbol: String,

    /// The underlying securities symbol the `Order` is for.
    ///
    /// NOTE: ONLY for futures and options.
    pub underlying: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
/// The type of option
pub enum OptionType {
    #[serde(rename = "CALL")]
    /// Call Option
    Call,

    #[serde(rename = "PUT")]
    /// Put Option
    Put,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
/// The stage of the `Order` , is it opening or closing?
pub enum OrderStage {
    /// Order to open position.
    Open,

    /// Order to close position.
    Close,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
/// The different types of order actions.
pub enum OrderAction {
    /// Buying to open.
    Buy,

    /// Selling to close.
    Sell,

    /// Open a short position.
    SellShort,

    /// Closing a short position.
    BuyToCover,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
/// The different types of asset's.
pub enum AssetType {
    #[serde(rename = "UNKNOWN")]
    /// An unknown asset type.
    Unknown,

    #[serde(rename = "STOCK")]
    /// An asset in the form of a stock.
    Stock,

    #[serde(rename = "STOCKOPTION")]
    /// An asset in the form of an option on a stock.
    StockOption,

    #[serde(rename = "FUTURE")]
    /// An asset in the form of a futures contract.
    Future,

    #[serde(rename = "FUTUREOPTION")]
    /// An asset in the form of an option on a futures contract.
    FutureOption,

    #[serde(rename = "FOREX")]
    /// An asset in the form of foriegn currency.
    Forex,

    #[serde(rename = "CURRENCYOPTION")]
    /// An asset in the form of an option on foriegn currency.
    CurrencyOption,

    #[serde(rename = "INDEX")]
    /// An asset in the form of an index.
    Index,

    #[serde(rename = "INDEXOPTION")]
    /// An asset in the form of an option on a index.
    IndexOption,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// Describes the relationship between linked
/// orders in a group and this order.
pub struct ConditionalOrder {
    #[serde(rename = "OrderID")]
    /// The id of the linked `Order`.
    pub order_id: String,

    /// The relationship of a linked order within a group order
    /// to the current returned `Order`.
    pub relationship: OrderRelationship,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
/// Types of `Order` relationships
pub enum OrderRelationship {
    /// Bracket Order
    BRK,

    /// One Sends Parent (linked parent)
    OSP,

    /// One Sends Other (linked child)
    OSO,

    /// One Cancels Other
    OCO,
}
