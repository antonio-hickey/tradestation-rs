<h1 align="center">TradeStation Rust Client</h1>

<p align="center">
  <img src="https://img.shields.io/github/actions/workflow/status/antonio-hickey/tradestation-rs/pre-commit.yml" />
  <img alt="Crates.io Total Downloads" src="https://img.shields.io/crates/d/tradestation">
  <img src="https://img.shields.io/crates/l/tradestation" />
  <img alt="docs.rs" src="https://img.shields.io/docsrs/tradestation">
  <img src="https://img.shields.io/github/commit-activity/m/antonio-hickey/tradestation-rs" />

</p>

An ergonomic Rust client for the [TradeStation API](https://www.tradestation.com/platforms-and-tools/trading-api/) empowering you to build fast, scalable, and production ready trading systems and applications.

* [Crates.io Homepage](https://crates.io/crates/tradestation)
* [Documentation](https://docs.rs/tradestation/latest/tradestation)
* [GitHub Repository]()

Features
---
- 🧮 Accounting: Monitor your risk, positions, balances, order history, and more
across multiple accounts.
- 📈 Market Data: Easily fetch and stream real time and historic market data
on thousands of assets and derivatives.
- ⚡ Execution: Lightning fast trade execution allowing you to place, update,
and cancel orders with all kinds of custom configuration.
- 🧪 Testing: Supports mocking so you can seamlessly build out environments to test
your trading systems and applications.

Install
---
Use cargo CLI:
```
cargo install tradestation
```

Or manually add it into your `Cargo.toml`:
```toml
[dependencies]
tradestation = "0.0.5"
```

Usage
---

For more thorough information, read the [docs](https://docs.rs/tradestation/latest/tradestation/).

Simple example for streaming bars of trading activity:
```rust
use tradestation::{
    responses::MarketData::StreamBarsResp,
    ClientBuilder, Error,
    MarketData::{self, BarUnit},
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Build the TradeStation Client
    let mut client = ClientBuilder::new()?
        .credentials("YOUR_ACCESS_KEY", "YOUR_SECRET_KEY")?
        .token(Token {
            access_token: "YOUR_ACCESS_TOKEN".into(),
            refresh_token: "YOUR_REFRESH_TOKEN".into(),
            id_token: "YOUR_ID_TOKEN".into(),
            token_type: String::from("Bearer"),
            scope: vec![
                Scope::OpenId,
                Scope::Profile,
                Scope::ReadAccount,
                Scope::OfflineAccess,
                Scope::MarketData,
            ],
            expires_in: 1200,
        })?
        .build()
        .await?;

    // Build a query to stream Crude Oil Futures
    let stream_bars_query = MarketData::StreamBarsQueryBuilder::new()
        .set_symbol("CLX25")
        .set_unit(BarUnit::Minute)
        .set_interval("240")
        .build()?;

    // Stream bars of trading activity based on the query built above
    let streamed_bars = client
        .stream_bars(&stream_bars_query, |stream_data| {
            match stream_data {
                StreamBarsResp::Bar(bar) => {
                    // Do something with the bars like making a chart
                    println!("{bar:?}")
                }
                StreamBarsResp::Heartbeat(heartbeat) => {
                    if heartbeat.heartbeat > 10 {
                        return Err(Error::StopStream);
                    }
                }
                StreamBarsResp::Status(status) => {
                    println!("{status:?}");
                }
                StreamBarsResp::Error(err) => {
                    println!("{err:?}");
                }
            }

            Ok(())
        })
        .await?;

    // All the bars collected during the stream
    println!("{streamed_bars:?}");

    Ok(())
}
```

Contributing
---

There are many ways to contribute like reporting issues, writing documentation, building
out new features and abstractions, refactoring to improve on current abstractions, or
fixing bugs.

Keep an eye out on open issues :)
