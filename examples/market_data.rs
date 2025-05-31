//! Example file on basic usage for market data endpoints

use futures::StreamExt;
use tradestation::{
    responses::MarketData::{
        StreamBarsResp, StreamMarketDepthQuotesResp, StreamOptionChainResp, StreamQuotesResp,
    },
    ClientBuilder, Error,
    MarketData::{
        self,
        options::{OptionSpreadType, OptionTradeAction, OptionsLeg},
        BarUnit,
    },
    Scope, Token,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Example: initialize client
    // NOTE: With the `Client` you can directly interact with all of TradeStation's API endpoints,
    // but it's suggested to use the higher level abstractions provided in the examples below.
    let client = ClientBuilder::new()?
        .credentials("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")?
        .token(Token {
            access_token: String::from("YOUR_ACCESS_TOKEN"),
            refresh_token: String::from("YOUR_REFRESH_TOKEN"),
            id_token: String::from("YOUR_ID_TOKEN"),
            token_type: String::from("Bearer"),
            scope: vec![
                Scope::OpenId,
                Scope::OfflineAccess,
                /* ... Your Other Desired Scopes */
            ],
            expires_in: 1200,
        })?
        .build()
        .await?;
    println!("Your TradeStation API Bearer Token: {:?}", client.token);
    //--

    //--
    // Example: Get all the available option expirations for Cloudflare (NET).
    let cloudflare_option_expirations = client.get_option_expirations("NET", None).await?;
    println!("Cloudflare Option Expirations: {cloudflare_option_expirations:?}");

    // Example: Get all the available option expirations for Cloudflare (NET) at the $100 strike.
    let cloudflare_option_expirations_at_strike_100 =
        client.get_option_expirations("NET", Some(100.00)).await?;
    println!(
        "Cloudflare Option Expirations At The $100 strike price: {cloudflare_option_expirations_at_strike_100:?}",
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
    // Example: Stream bars of November 2030 Crude Oil Futures trading activity in
    // 4 hour (240 minute) intervals.
    let stream_bars_query = MarketData::StreamBarsQueryBuilder::new()
        .symbol("CLX30")
        .unit(BarUnit::Minute)
        .interval(240)
        .build()?;

    // Start streaming bars of trading activity
    let bars_stream = client.stream_bars(&stream_bars_query);
    tokio::pin!(bars_stream); // NOTE: You must pin the stream to the stack
    while let Some(stream_resp) = bars_stream.next().await {
        // The response type is `responses::market_data::StreamBarsResp`
        // which has multiple variants the main one you care about is
        // `Bar` which will contain order data sent from the stream.
        match stream_resp {
            Ok(StreamBarsResp::Bar(bar)) => {
                // Do something with the bars like making a chart
                println!("{bar:?}")
            }
            Ok(StreamBarsResp::Heartbeat(heartbeat)) => {
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
            Ok(StreamBarsResp::Status(status)) => {
                // Signal sent on state changes in the stream
                // (closed, opened, paused, resumed)
                println!("{status:?}");
            }
            Ok(StreamBarsResp::Error(err)) => {
                // Response for when an error was encountered,
                // with details on the error
                eprintln!("{err:?}");
            }
            Err(err) => {
                // Stream / Network error
                eprintln!("{err:?}");
            }
        }
    }
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

    //--
    // Example: Stream an option chain of iron butterfly's for Apple `"AAPL"`
    // expiring December 20th 2024.
    let stream_aapl_option_chain_query = MarketData::OptionChainQueryBuilder::new()
        .underlying("AAPL")
        .spread_type(OptionSpreadType::IronButterfly)
        .expiration("12-20-2024")
        // Using the 1 month us treasury
        // to base the risk free rate off
        // which is currently 4.85%
        .risk_free_rate(0.04)
        .build()?;

    let option_chain_stream = client.stream_option_chain(&stream_aapl_option_chain_query);
    tokio::pin!(option_chain_stream);

    while let Some(stream_resp) = option_chain_stream.next().await {
        // The response type is `responses::market_data::StreamOptionChainResp`
        // which has multiple variants the main one you care about is `OptionChain`
        // which will contain option chain data sent from the stream.
        match stream_resp {
            Ok(StreamOptionChainResp::OptionChain(chain)) => {
                // Do something with the option chain like display it
                // with a table on a website.
                println!("{chain:?}")
            }
            Ok(StreamOptionChainResp::Heartbeat(heartbeat)) => {
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
            Ok(StreamOptionChainResp::Status(status)) => {
                // Signal sent on state changes in the stream
                // (closed, opened, paused, resumed)
                println!("{status:?}");
            }
            Ok(StreamOptionChainResp::Error(err)) => {
                // Response for when an error was encountered,
                // with details on the error
                eprintln!("{err:?}");
            }
            Err(err) => {
                // Stream / Network error
                eprintln!("{err:?}");
            }
        }
    }
    //--

    //--
    // Example: Get a quote on Palantir.
    let palantir_quote = client.get_quotes(vec!["PLTR"]).await?;
    println!("Palantir Quote: {palantir_quote:?}");
    //--

    //--
    // Example: Stream quotes on Nvidia (`"NVDA"`) and AMD (`"AMD"`).
    let quotes_stream = client.stream_quotes(vec!["NVDA", "AMD"]);
    tokio::pin!(quotes_stream);

    while let Some(stream_resp) = quotes_stream.next().await {
        // The response type is `responses::MarketData::StreamQuotesResp`
        // which has multiple variants the main one you care about is `Quote`
        // which will contain option chain data sent from the stream.
        match stream_resp {
            Ok(StreamQuotesResp::Quote(quote)) => {
                // Do something with the quote for example derive
                // a quote for a long amd short nvidia trade.
                println!("{quote:?}");
            }
            Ok(StreamQuotesResp::Heartbeat(heartbeat)) => {
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
            Ok(StreamQuotesResp::Status(status)) => {
                // Signal sent on state changes in the stream
                // (closed, opened, paused, resumed)
                println!("{status:?}");
            }
            Ok(StreamQuotesResp::Error(err)) => {
                // Response for when an error was encountered,
                // with details on the error
                eprintln!("{err:?}");
            }
            Err(err) => {
                // Stream / Network error
                eprintln!("{err:?}");
            }
        }
    }
    //--

    //--
    // Example: Stream market depth aggregates on December 2030 Natural Gas Futures
    // (`"NGZ30"`).
    let depth_levels: i32 = 10;
    let market_depth_stream = client.stream_market_depth_quotes("NGZ30", Some(depth_levels));
    tokio::pin!(market_depth_stream); // Pin the stream to the stack

    while let Some(stream_resp) = market_depth_stream.next().await {
        match stream_resp {
            Ok(StreamMarketDepthQuotesResp::Quote(quote)) => {
                // Do something with the quote for example derive
                // a quote for a long amd short nvidia trade.
                println!("{quote:?}");
            }
            Ok(StreamMarketDepthQuotesResp::Heartbeat(heartbeat)) => {
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
            Ok(StreamMarketDepthQuotesResp::Status(status)) => {
                // Signal sent on state changes in the stream
                // (closed, opened, paused, resumed)
                println!("{status:?}");
            }
            Ok(StreamMarketDepthQuotesResp::Error(err)) => {
                // Response for when an error was encountered,
                // with details on the error
                eprintln!("{err:?}");
            }
            Err(err) => {
                // Stream / Network error
                eprintln!("{err:?}");
            }
        }
    }
    //--

    Ok(())
}
