//! <h1 align="center">TradeStation Rust Client</h1>
//!
//! <p align="center">
//!   <img src="https://img.shields.io/github/actions/workflow/status/antonio-hickey/tradestation-rs/pre-commit.yml" />
//!   <img alt="Crates.io Total Downloads" src="https://img.shields.io/crates/d/tradestation">
//!   <img src="https://img.shields.io/crates/l/tradestation" />
//!   <img alt="docs.rs" src="https://img.shields.io/docsrs/tradestation">
//!   <img src="https://img.shields.io/github/commit-activity/m/antonio-hickey/tradestation-rs" />
//! </p>
//!
//! An ergonomic Rust client for the [TradeStation API](https://www.tradestation.com/platforms-and-tools/trading-api/) empowering you to build fast, scalable, and production ready trading systems and applications.
//!
//! * [Crates.io Homepage](https://crates.io/crates/tradestation)
//! * [Documentation](https://docs.rs/tradestation/latest/tradestation)
//! * [GitHub Repository](https://github.com/antonio-hickey/tradestation-rs)
//! * [Examples](https://github.com/antonio-hickey/tradestation-rs/tree/v0.0.8/examples)
//!
//! Features
//! ---
//! - 🧮 [Accounting](https://docs.rs/tradestation/latest/tradestation/accounting/index.html): Monitor your risk, positions, balances, order history, and more across multiple accounts.
//! - 📈 [Market Data](https://docs.rs/tradestation/latest/tradestation/market_data/index.html): Easily fetch and stream real time and historic market data on thousands of assets and derivatives.
//! - ⚡ [Execution](https://docs.rs/tradestation/latest/tradestation/execution/index.html): Lightning fast trade execution allowing you to place, update, and cancel orders with all kinds of custom configuration.
//! - 🧪 [Testing](https://github.com/antonio-hickey/tradestation-rs/blob/master/examples/mocking.rs): Supports mocking so you can seamlessly build out environments to test your trading systems and applications.
//!
//! Install
//! ---
//! Use cargo CLI:
//! ```sh
//! cargo add tradestation
//! ```
//!
//! Or manually add it into your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! tradestation = "0.0.8"
//! ```
//!
//! Usage
//! ---
//!
//! For more thorough information, read the [docs](https://docs.rs/tradestation/latest/tradestation/).
//!
//! NOTE: See initial auth example on how to get your token (if not already done): https://github.com/antonio-hickey/tradestation-rs/blob/master/examples/initial_auth.rs
//!
//! Simple example for streaming bars of trading activity:
//! ```rust,no_run
//! use tradestation::{
//!     responses::market_data::StreamBarsResp,
//!     ClientBuilder, Error,
//!     market_data::{BarUnit, StreamBarsQueryBuilder},
//! };
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Error> {
//!     // Build the TradeStation Client
//!     let mut client = ClientBuilder::new()
//!         .credentials("YOUR_ACCESS_KEY", "YOUR_SECRET_KEY")
//!         .with_token(Token {
//!             access_token: "YOUR_ACCESS_TOKEN".into(),
//!             refresh_token: "YOUR_REFRESH_TOKEN".into(),
//!             id_token: "YOUR_ID_TOKEN".into(),
//!             token_type: String::from("Bearer"),
//!             scope: vec![Scope::MarketData],
//!             expires_in: 1200,
//!         })
//!         .build()
//!         .await?;
//!
//!     // Build a query to stream Crude Oil Futures
//!     let stream_bars_query = StreamBarsQueryBuilder::new()
//!         .symbol("CLX30")
//!         .unit(BarUnit::Minute)
//!         .interval("240")
//!         .build()?;
//!
//!     // Stream the bars based on the query built above into
//!     // a custom function to process each bar streamed in.
//!     client
//!         .stream_bars_into(&stream_bars_query, |stream_event| {
//!             println!("Stream Bar Event: {stream_event:?}");
//!             Ok(())
//!         })
//!         .await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! Contributing
//! ---
//!
//! There are many ways to contribute like reporting issues, writing documentation, building
//! out new features and abstractions, refactoring to improve on current abstractions, or
//! fixing bugs.
//!
//! Keep an eye out on open issues :)

/// Functions, structs, and primitives related to accounting.
pub mod accounting;

/// Structs for all TradeStation API responses.
pub mod responses;

/// Functions, structs, and primitives related to the tradestation-rs `Client`.
pub mod client;
pub use client::{Client, ClientBuilder};

/// The tradestation-rs error definitions.
mod error;
/// The tradestation-rs error type.
pub use error::Error;

/// Functions, Structs, and primitives related to market data.
pub mod market_data;

/// Functions, structs, and primitives related to auth tokens.
pub mod token;
pub use token::{Scope, Token};

/// Functions, Structs, and primitives related to market data.
pub mod execution;

/// Abstractions, functions, and primitives related to orders.
pub mod orders {
    pub use crate::{
        accounting::orders::{
            ConditionalOrder, LogicOp, OptionType, Order, OrderAction, OrderLeg, OrderRelationship,
            OrderStage, OrderStatus, OrderType,
        },
        execution::{
            confirm::OrderConfirmation,
            orders::{
                AdvancedOrderOptions, BPWarningStatus, Duration, OrderRequestLeg, OrderTimeInForce,
                Oso, PegValue, TradeAction,
            },
            request::{
                OrderRequest, OrderRequestBuilder, OrderRequestGroup, OrderRequestGroupBuilder,
            },
            route::Route,
            ticket::OrderTicket,
            trigger::{ActivationTrigger, ActivationTriggerKey},
            update::OrderUpdate,
        },
    };
}
