//! Example file on basic usage for market data endpoints

use tradestation::{
    responses::MarketData::StreamBarsResp,
    ClientBuilder, Error,
    MarketData::{
        self,
        options::{OptionTradeAction, OptionsLeg},
        BarUnit,
    },
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
    //--

    //--
    // Example: Get all the available option expirations for Cloudflare (NET).
    let cloudflare_option_expirations = client.fetch_option_expirations("NET", None).await?;
    println!("Cloudflare Option Expirations: {cloudflare_option_expirations:?}");

    // Example: Get all the available option expirations for Cloudflare (NET) at the $100 strike.
    let cloudflare_option_expirations_at_strike_100 =
        client.fetch_option_expirations("NET", Some(100.00)).await?;
    println!(
        "Cloudflare Option Expirations At The $100 strike price: {:?}",
        cloudflare_option_expirations_at_strike_100,
    );
    //--

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

    //--
    // Example: Analyze the risk vs reward of a long volatility
    // trade for TLT at the November 15th expiration via buying
    // a call **and** a put at the $99 strike.
    //
    // NOTE: The call will make money if TLT makes a big move up, and
    // the put will make money if TLT makes a big move down. The downside
    // of this trade comes from stable, slow, or small price movement.
    //
    // NOTE: This spread offers unlimited potential profit while defining
    // a max potential loss.
    let risk_reward_analysis = client
        .analyze_options_risk_reward(
            4.33,
            vec![
                OptionsLeg {
                    symbol: String::from("TLT 241115C99"),
                    quantity: 5,
                    trade_action: OptionTradeAction::Buy,
                },
                OptionsLeg {
                    symbol: String::from("TLT 241115P99"),
                    quantity: 5,
                    trade_action: OptionTradeAction::Buy,
                },
            ],
        )
        .await?;

    println!(
        "TLT November 15th Long Volatility Via ATM Straddle
         Risk vs Reward Analysis: {risk_reward_analysis:?}"
    );
    //--

    Ok(())
}
