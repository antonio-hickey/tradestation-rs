//! Example file on basic usage for market data endpoints

use tradestation_rs::MarketData::BarUnit;
use tradestation_rs::{ClientBuilder, Error, MarketData};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Example: initialize client
    // NOTE: With the `Client` you can interact with all of TradeStation's API endpoints,
    // but it's suggested to use the higher level abstractions provided in the examples below.
    let mut client = ClientBuilder::new()?
        .set_credentials("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")?
        .authorize("YOUR_AUTHORIZATION_CODE")
        .await?
        .build()
        .await?;
    println!("Your TradeStation API Bearer Token: {:?}", client.token);
    //---

    //--
    // Example: Get the 10 most recent 5 minute bars of trading
    // activity for November 2024 Crude Oil Futures.
    let fetch_bars_query = MarketData::GetBarsQueryBuilder::new()
        .set_symbol("CLX24")
        .set_unit(BarUnit::Minute)
        .set_interval("5")
        .set_bars_back("10")
        .build()?;

    let bars = client.fetch_bars(&fetch_bars_query).await?;

    // Do something with the bars, maybe make a chart?
    println!("{bars:?}");
    //--

    Ok(())
}
