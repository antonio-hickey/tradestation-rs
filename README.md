# TradeStation Rust Client

An ergonomic Rust client for the [TradeStation API](https://www.tradestation.com/platforms-and-tools/trading-api/).

* [crates.io homepage](https://crates.io/crates/tradestation)
* [documentation](https://docs.rs/tradestation/latest/tradestation)

Install
---
Use cargo CLI:
```
cargo install tradestation
```

Or manually add it into your `Cargo.toml`:
```toml
[dependencies]
tradestation = "0.0.3"
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
    let mut client = ClientBuilder::new()?
        .set_credentials("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")?
        .authorize("YOUR_AUTHORIZATION_CODE")
        .await?
        .build()
        .await?;
    println!("Your TradeStation API Bearer Token: {:?}", client.token);

    let stream_bars_query = MarketData::StreamBarsQueryBuilder::new()
        .set_symbol("CLX24")
        .set_unit(BarUnit::Minute)
        .set_interval("240")
        .build()?;

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
