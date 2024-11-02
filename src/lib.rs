//! # Tradestation Rust Client
//!
//! Fully featured and ergonomic rust client for the TradeStation API.
//!
//! ## Features
//!
//! - Accounting
//! - Market Data
//! - Execution
//!
//! ## Install
//!
//! Use Cargo CLI:
//! ```ignore
//! cargo install tradestation
//! ```
//! Or manually add it into your `Cargo.toml`:
//! ```ignore
//! [dependencies]
//! tradestation = "0.0.2"
//! ```
//!
//! ## Usage
//!
//! Simple example for streaming 4 hour aggregated
//! bars of trading activity for Crude Oil Futures:
//! ```ignore
//! use tradestation::{
//!     responses::MarketData::StreamBarsResp,
//!     ClientBuilder, Error,
//!     MarketData::{self, BarUnit},
//! };
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Error> {
//!     let mut client = ClientBuilder::new()?
//!         .set_credentials("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")?
//!         .authorize("YOUR_AUTHORIZATION_CODE")
//!         .await?
//!         .build()
//!         .await?;
//!     println!("Your TradeStation API Bearer Token: {:?}", client.token);
//!
//!     let stream_bars_query = MarketData::StreamBarsQueryBuilder::new()
//!         .set_symbol("CLX24")
//!         .set_unit(BarUnit::Minute)
//!         .set_interval("240")
//!         .build()?;
//!
//!     let streamed_bars = client
//!         .stream_bars(&stream_bars_query, |stream_data| {
//!             match stream_data {
//!                 StreamBarsResp::Bar(bar) => {
//!                     // Do something with the bars like making a chart
//!                     println!("{bar:?}")
//!                 }
//!                 StreamBarsResp::Heartbeat(heartbeat) => {
//!                     if heartbeat.heartbeat > 10 {
//!                         return Err(Error::StopStream);
//!                     }
//!                 }
//!                 StreamBarsResp::Status(status) => {
//!                     println!("{status:?}");
//!                 }
//!                 StreamBarsResp::Error(err) => {
//!                     println!("{err:?}");
//!                 }
//!             }
//!
//!             Ok(())
//!         })
//!         .await?;
//!
//!     // All the bars collected during the stream
//!     println!("{streamed_bars:?}");
//!
//!     Ok(())
//! }
//! ```

pub mod account;
pub mod responses;

pub mod client;
pub use client::{Client, ClientBuilder};

pub mod error;
pub use error::Error;

pub mod market_data;
pub use market_data as MarketData;

pub mod token;
pub use token::Token;

pub mod execution;
pub use execution::Route;
