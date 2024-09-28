//! Example file on basic usage for market data endpoints

use tradestation::{
    responses::MarketData::StreamBarsResp,
    ClientBuilder, Error,
    MarketData::{self, BarUnit},
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Example: initialize client
    // NOTE: With the `Client` you can directly interact with all of TradeStation's API endpoints,
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
    // Example: Get symbol details (`MarketData::SymbolDetails`) on symbols the nasdaq index `NQQ`,
    // and Feburary 21st 2025 $105 call option for 20+ Year Treasury fund `TLT 250221C105`.
    let symbols = vec!["NQQ", "TLT 250221C105"];
    let details = client.get_symbol_details(symbols).await?;
    println!("Symbol Details: {details:?}");
    //--

    //--
    // Example: Stream bars of November 2024 Crude Oil Futures trading activity in
    // 4 hour (240 minute) intervals.
    let stream_bars_query = MarketData::StreamBarsQueryBuilder::new()
        .set_symbol("CLX24")
        .set_unit(BarUnit::Minute)
        .set_interval("240")
        .build()?;

    let streamed_bars = client
        .stream_bars(&stream_bars_query, |stream_data| {
            // The response type is `responses::market_data::StreamBarsResp`
            // which has multiple variants the main one you care about is
            // `Bar` which will contain order data sent from the stream.
            match stream_data {
                StreamBarsResp::Bar(bar) => {
                    // Do something with the bars like making a chart
                    println!("{bar:?}")
                }
                StreamBarsResp::Heartbeat(heartbeat) => {
                    // Response for periodic signals letting you know the connection is
                    // still alive. A heartbeat is sent every 5 seconds of inactivity.
                    println!("{heartbeat:?}");

                    // for the sake of this example after we recieve the
                    // tenth heartbeat, we will stop the stream session.
                    if heartbeat.heartbeat > 10 {
                        // Example: stopping a stream connection
                        return Err(Error::StopStream);
                    }
                }
                StreamBarsResp::Status(status) => {
                    // Signal sent on state changes in the stream
                    // (closed, opened, paused, resumed)
                    println!("{status:?}");
                }
                StreamBarsResp::Error(err) => {
                    // Response for when an error was encountered,
                    // with details on the error
                    println!("{err:?}");
                }
            }

            Ok(())
        })
        .await?;

    // All the bars collected during the stream
    println!("{streamed_bars:?}");
    //--

    Ok(())
}
