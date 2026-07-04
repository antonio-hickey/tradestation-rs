//! # Market Data Endpoints
//! - [Get Bars](#endpoint-get-bars): Fetches historical price bars for a symbol.
//! - [Stream Bars](#endpoint-stream-bars): Streams price bars for a symbol.
//! - [Get Option Expirations](#endpoint-get-option-expirations): Fetches available option expiration dates for an underlying symbol.
//! - [Get Option Risk/Reward](#endpoint-get-option-riskreward): Analyzes maximum gain, maximum loss, and breakeven points for an options spread.
//! - [Get Option Spread Strikes](#endpoint-get-option-spread-strikes): Fetches valid strike combinations for an option spread type.
//! - [Stream Option Chain](#endpoint-stream-option-chain): Streams option chain quotes for an underlying symbol.
//! - [Stream Option Quotes](#endpoint-stream-option-quotes): Streams calculated quotes and greeks for an options spread.
//! - [Get Quote Snapshots](#endpoint-get-quote-snapshots): Fetches latest quote snapshots for one or more symbols.
//! - [Stream Quotes](#endpoint-stream-quotes): Streams quote updates for one or more symbols.
//! - [Get Symbol Details](#endpoint-get-symbol-details): Fetches symbol metadata and formatting details.
//! - [Stream Market Depth Aggregates](#endpoint-stream-market-depth-aggregates): Streams aggregated market depth by price level.
//! - [Stream Market Depth Quotes](#endpoint-stream-market-depth-quotes): Streams participant-level market depth quotes.
//!
//! # Endpoint: Get Bars
//! - Fetches historical bars for a symbol using interval, unit, and date or bars-back filters.
//! - Bar Reference: [`crate::market_data::Bar`]
//! - Query Reference: [`crate::market_data::GetBarsQueryBuilder`]
//! - **Example**: Fetch recent five-minute bars for a futures contract.
//!
//! ```rust,no_run
//! # use tradestation::{Client, Error, market_data::{BarUnit, GetBarsQueryBuilder}};
//! # async fn get_bars_example(client: &Client) -> Result<(), Error> {
//! let query = GetBarsQueryBuilder::new()
//!     .symbol("CLX30")
//!     .unit(BarUnit::Minute)
//!     .interval(5)
//!     .bars_back(10)
//!     .build()?;
//!
//! let bars = client.get_bars(&query).await?;
//! println!("Loaded {} bars", bars.len());
//! # Ok(()) }
//! ```
//!
//! ---
//!
//! # Endpoint: Stream Bars
//! - Streams bars for a symbol as new market data is received.
//! - Stream Response Reference: [`crate::responses::market_data::StreamBarsResp`]
//! - Query Reference: [`crate::market_data::StreamBarsQueryBuilder`]
//! - **Example**: Stream hourly bars and stop after the first event.
//!
//! ```rust,no_run
//! # use tradestation::{Client, Error, market_data::{BarUnit, StreamBarsQueryBuilder}};
//! # async fn stream_bars_example(client: &Client) -> Result<(), Error> {
//! let query = StreamBarsQueryBuilder::new()
//!     .symbol("CLX30")
//!     .unit(BarUnit::Minute)
//!     .interval(60)
//!     .build()?;
//!
//! client.stream_bars_into(&query, |stream_event| {
//!     println!("Bar stream event: {stream_event:?}");
//!     Err(Error::StopStream)
//! }).await?;
//! # Ok(()) }
//! ```
//!
//! ---
//!
//! # Endpoint: Get Option Expirations
//! - Fetches available option contract expiration dates for an underlying symbol.
//! - Expiration Reference: [`crate::market_data::OptionExpiration`]
//! - **Example**: Load all known expirations for an equity option chain.
//!
//! ```rust,no_run
//! # use tradestation::{Client, Error};
//! # async fn get_option_expirations_example(client: &Client) -> Result<(), Error> {
//! let expirations = client.get_option_expirations("SPY", None).await?;
//! println!("Loaded {} expirations", expirations.len());
//! # Ok(()) }
//! ```
//!
//! ---
//!
//! # Endpoint: Get Option Risk/Reward
//! - Calculates maximum gain, maximum loss, and breakeven points for an options spread.
//! - Analysis Reference: [`crate::market_data::OptionRiskRewardAnalysis`]
//! - Leg Reference: [`crate::market_data::OptionsLeg`]
//! - **Example**: Analyze a long straddle.
//!
//! ```rust,no_run
//! # use tradestation::{Client, Error, market_data::{OptionTradeAction, OptionsLeg}};
//! # async fn analyze_options_risk_reward_example(client: &Client) -> Result<(), Error> {
//! let analysis = client
//!     .analyze_options_risk_reward(
//!         4.33,
//!         vec![
//!             OptionsLeg {
//!                 symbol: "TLT 260116C99".into(),
//!                 quantity: 1,
//!                 trade_action: OptionTradeAction::Buy,
//!             },
//!             OptionsLeg {
//!                 symbol: "TLT 260116P99".into(),
//!                 quantity: 1,
//!                 trade_action: OptionTradeAction::Buy,
//!             },
//!         ],
//!     )
//!     .await?;
//!
//! println!("Breakevens: {:?}", analysis.breakeven_points);
//! # Ok(()) }
//! ```
//!
//! ---
//!
//! # Endpoint: Get Option Spread Strikes
//! - Fetches valid strike combinations for a spread type and expiration date.
//! - Spread Strikes Reference: [`crate::market_data::OptionSpreadStrikes`]
//! - Query Reference: [`crate::market_data::OptionSpreadStrikesQueryBuilder`]
//! - **Example**: Load iron condor strike combinations.
//!
//! ```rust,no_run
//! # use tradestation::{Client, Error, market_data::{OptionSpreadStrikesQueryBuilder, OptionSpreadType}};
//! # async fn get_option_spread_strikes_example(client: &Client) -> Result<(), Error> {
//! let query = OptionSpreadStrikesQueryBuilder::new()
//!     .underlying("AMZN")
//!     .spread_type(OptionSpreadType::IronCondor)
//!     .expiration("2026-12-18")
//!     .build()?;
//!
//! let strikes = client.get_option_spread_strikes(query).await?;
//! println!("Loaded {} strike combinations", strikes.strikes.len());
//! # Ok(()) }
//! ```
//!
//! ---
//!
//! # Endpoint: Stream Option Chain
//! - Streams option chain quotes for an underlying symbol.
//! - Chain Reference: [`crate::market_data::OptionChain`]
//! - Query Reference: [`crate::market_data::OptionChainQueryBuilder`]
//! - **Example**: Stream an option chain and stop after the first event.
//!
//! ```rust,no_run
//! # use tradestation::{Client, Error, market_data::OptionChainQueryBuilder};
//! # async fn stream_option_chain_example(client: &Client) -> Result<(), Error> {
//! let query = OptionChainQueryBuilder::new()
//!     .underlying("AAPL")
//!     .build()?;
//!
//! client.stream_option_chain_into(&query, |stream_event| {
//!     println!("Option chain stream event: {stream_event:?}");
//!     Err(Error::StopStream)
//! }).await?;
//! # Ok(()) }
//! ```
//!
//! ---
//!
//! # Endpoint: Stream Option Quotes
//! - Streams calculated quotes and greeks for a single-leg or multi-leg options spread.
//! - Quote Reference: [`crate::market_data::OptionQuote`]
//! - Query Reference: [`crate::market_data::OptionQuoteQueryBuilder`]
//! - **Example**: Stream quotes for a vertical spread and stop after the first event.
//!
//! ```rust,no_run
//! # use tradestation::{Client, Error, market_data::{OptionQuoteLeg, OptionQuoteQueryBuilder}};
//! # async fn stream_option_quotes_example(client: &Client) -> Result<(), Error> {
//! let query = OptionQuoteQueryBuilder::new()
//!     .legs(vec![
//!         OptionQuoteLeg {
//!             symbol: "SPY 260116C500".into(),
//!             ratio: 1,
//!         },
//!         OptionQuoteLeg {
//!             symbol: "SPY 260116C510".into(),
//!             ratio: -1,
//!         },
//!     ])
//!     .risk_free_rate(0.04)
//!     .build()?;
//!
//! client.stream_option_quotes_into(&query, |stream_event| {
//!     println!("Option quote stream event: {stream_event:?}");
//!     Err(Error::StopStream)
//! }).await?;
//! # Ok(()) }
//! ```
//!
//! ---
//!
//! # Endpoint: Get Quote Snapshots
//! - Fetches full quote snapshots for one or more symbols.
//! - Quote Reference: [`crate::market_data::Quote`]
//! - **Example**: Fetch quotes for two equities.
//!
//! ```rust,no_run
//! # use tradestation::{Client, Error};
//! # async fn get_quote_snapshots_example(client: &Client) -> Result<(), Error> {
//! let quotes = client.get_quotes(vec!["NVDA", "AMD"]).await?;
//! println!("Loaded {} quotes", quotes.len());
//!
//! let spy = client.get_quote("SPY").await?;
//! println!("SPY last price: {}", spy.last);
//! # Ok(()) }
//! ```
//!
//! ---
//!
//! # Endpoint: Stream Quotes
//! - Streams real-time quote updates for one or more symbols.
//! - Stream Response Reference: [`crate::responses::market_data::StreamQuotesResp`]
//! - **Example**: Stream quote updates and stop after the first event.
//!
//! ```rust,no_run
//! # use tradestation::{Client, Error};
//! # async fn stream_quotes_example(client: &Client) -> Result<(), Error> {
//! client.stream_quotes_into(vec!["SPY", "QQQ"], |stream_event| {
//!     println!("Quote stream event: {stream_event:?}");
//!     Err(Error::StopStream)
//! }).await?;
//! # Ok(()) }
//! ```
//!
//! ---
//!
//! # Endpoint: Get Symbol Details
//! - Fetches symbol metadata and price/quantity formatting information.
//! - Symbol Reference: [`crate::market_data::SymbolDetails`]
//! - **Example**: Load symbol details for an equity and an option.
//!
//! ```rust,no_run
//! # use tradestation::{Client, Error};
//! # async fn get_symbol_details_example(client: &Client) -> Result<(), Error> {
//! let details = client
//!     .get_symbol_details(vec!["SPY", "SPY 260116C500"])
//!     .await?;
//!
//! println!("Loaded {} symbol records", details.len());
//! # Ok(()) }
//! ```
//!
//! ---
//!
//! # Endpoint: Stream Market Depth Aggregates
//! - Streams market depth aggregated by price level.
//! - Aggregate Reference: [`crate::market_data::MarketDepthAggregates`]
//! - Stream Response Reference: [`crate::responses::market_data::StreamMarketDepthAggregatesResp`]
//! - **Example**: Stream five levels of aggregated market depth.
//!
//! ```rust,no_run
//! # use tradestation::{Client, Error};
//! # async fn stream_market_depth_aggregates_example(client: &Client) -> Result<(), Error> {
//! client.stream_market_depth_aggregates_into("SPY", Some(5), |stream_event| {
//!     println!("Market depth aggregate event: {stream_event:?}");
//!     Err(Error::StopStream)
//! }).await?;
//! # Ok(()) }
//! ```
//!
//! ---
//!
//! # Endpoint: Stream Market Depth Quotes
//! - Streams participant-level market depth quotes.
//! - Quote Reference: [`crate::market_data::MarketDepthQuotes`]
//! - Stream Response Reference: [`crate::responses::market_data::StreamMarketDepthQuotesResp`]
//! - **Example**: Stream five levels of participant-level market depth.
//!
//! ```rust,no_run
//! # use tradestation::{Client, Error};
//! # async fn stream_market_depth_quotes_example(client: &Client) -> Result<(), Error> {
//! client.stream_market_depth_quotes_into("SPY", Some(5), |stream_event| {
//!     println!("Market depth quote event: {stream_event:?}");
//!     Err(Error::StopStream)
//! }).await?;
//! # Ok(()) }
//! ```
//!
/// Types and functionality for working with market data bars.
pub mod bar;

/// Types and functionality for working with market depth.
pub mod market_depth;

/// Types and functionality for working with options market data.
pub mod options;

/// Types and functionality for working with market data quotes.
pub mod quote;

/// Types and functionality for working with symbol details.
pub mod symbol;

pub use bar::{
    Bar, BarUnit, GetBarsQuery, GetBarsQueryBuilder, StreamBarsQuery, StreamBarsQueryBuilder,
};
pub use market_depth::{
    MarketDepthAggregate, MarketDepthAggregates, MarketDepthQuote, MarketDepthQuotes,
    MarketDepthSide,
};
pub use options::{
    OptionChain, OptionChainQuery, OptionChainQueryBuilder, OptionExpiration, OptionExpirationType,
    OptionQuote, OptionQuoteLeg, OptionQuoteQuery, OptionQuoteQueryBuilder,
    OptionRiskRewardAnalysis, OptionSpreadStrikes, OptionSpreadStrikesQuery,
    OptionSpreadStrikesQueryBuilder, OptionSpreadType, OptionTradeAction, OptionsLeg,
};
pub use quote::{MarketFlag, Quote, QuoteStreamUpdate};
pub use symbol::{
    Format, IncrementSchedule, IncrementStyle, PriceFormat, QuantityFormat, SymbolDetails,
};
