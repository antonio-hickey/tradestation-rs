use mockito::Server;
use tradestation::ClientBuilder;
use tradestation::MarketData::{BarUnit, GetBarsQueryBuilder, OptionTradeAction, OptionsLeg};

#[test]
/// This test ensures that the parsing of
/// getting `Bar`(s) is correct.
fn test_get_bars_mocked() {
    // Mock the `barcharts` endpoint with a raw JSON
    // string which was a real response.
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/marketdata/barcharts/MSFT?interval=1&unit=Daily&barsBack=5&sessionTemplate=Default")
        .with_status(200)
        .with_body(
            "{\"Bars\":[{\"High\":\"396.36\",\"Low\":\"392.64\",\"Open\":\"393.92\",\"Close\":\"395.16\",\"TimeStamp\":\"2025-03-25T20:00:00Z\",\"TotalVolume\":\"15774968\",\"DownTicks\":131319,\"DownVolume\":6708452,\"OpenInterest\":\"0\",\"IsRealtime\":false,\"IsEndOfHistory\":false,\"TotalTicks\":262246,\"UnchangedTicks\":0,\"UnchangedVolume\":0,\"UpTicks\":130927,\"UpVolume\":9066516,\"Epoch\":1742932800000,\"BarStatus\":\"Closed\"},{\"High\":\"395.31\",\"Low\":\"388.57\",\"Open\":\"395\",\"Close\":\"389.97\",\"TimeStamp\":\"2025-03-26T20:00:00Z\",\"TotalVolume\":\"16132906\",\"DownTicks\":141585,\"DownVolume\":6585638,\"OpenInterest\":\"0\",\"IsRealtime\":false,\"IsEndOfHistory\":false,\"TotalTicks\":283674,\"UnchangedTicks\":0,\"UnchangedVolume\":0,\"UpTicks\":142089,\"UpVolume\":9547268,\"Epoch\":1743019200000,\"BarStatus\":\"Closed\"},{\"High\":\"392.24\",\"Low\":\"387.4\",\"Open\":\"390.13\",\"Close\":\"390.58\",\"TimeStamp\":\"2025-03-27T20:00:00Z\",\"TotalVolume\":\"13766761\",\"DownTicks\":119983,\"DownVolume\":6022210,\"OpenInterest\":\"0\",\"IsRealtime\":false,\"IsEndOfHistory\":false,\"TotalTicks\":239997,\"UnchangedTicks\":0,\"UnchangedVolume\":0,\"UpTicks\":120014,\"UpVolume\":7744550,\"Epoch\":1743105600000,\"BarStatus\":\"Closed\"},{\"High\":\"389.13\",\"Low\":\"376.93\",\"Open\":\"388.08\",\"Close\":\"378.8\",\"TimeStamp\":\"2025-03-28T20:00:00Z\",\"TotalVolume\":\"21632016\",\"DownTicks\":210147,\"DownVolume\":8163859,\"OpenInterest\":\"0\",\"IsRealtime\":false,\"IsEndOfHistory\":false,\"TotalTicks\":420363,\"UnchangedTicks\":0,\"UnchangedVolume\":0,\"UpTicks\":210216,\"UpVolume\":13468156,\"Epoch\":1743192000000,\"BarStatus\":\"Closed\"},{\"High\":\"377.07\",\"Low\":\"367.24\",\"Open\":\"372.54\",\"Close\":\"375.39\",\"TimeStamp\":\"2025-03-31T20:00:00Z\",\"TotalVolume\":\"35184676\",\"DownTicks\":254838,\"DownVolume\":19387519,\"OpenInterest\":\"0\",\"IsRealtime\":false,\"IsEndOfHistory\":true,\"TotalTicks\":509282,\"UnchangedTicks\":0,\"UnchangedVolume\":0,\"UpTicks\":254444,\"UpVolume\":15797157,\"Epoch\":1743451200000,\"BarStatus\":\"Closed\"}]}"
        )
        .create();

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let mut client = ClientBuilder::new()
            .unwrap()
            .testing_url(&server.url())
            .build()
            .await
            .unwrap();

        // Build query to get a bar chart of Microsofts
        // last 5 days of trading activity
        let msft_last_5_days = GetBarsQueryBuilder::new()
            .symbol("MSFT")
            .unit(BarUnit::Daily)
            .bars_back(5)
            .build()
            .unwrap();

        // Get the bar chart for the query built above
        match client.get_bars(&msft_last_5_days).await {
            Ok(bar_chart) => {
                // Should have a length of 5
                assert_eq!(bar_chart.len(), 5);
            }
            Err(e) => {
                panic!("Failed to parse `Bar`: {e:?}")
            }
        }
    });

    // Ensure the mock was called
    mock.assert();
}

#[test]
/// This test ensures that the parsing of
/// getting `Quote`(s) is correct.
fn test_get_quotes_mocked() {
    // Mock the `quotes` endpoint with a raw JSON
    // string which was a real response.
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/marketdata/quotes/TLT")
        .with_status(200)
        .with_body(
            "{\"Quotes\":[{\"Symbol\":\"TLT\",\"Open\":\"92.37\",\"High\":\"92.37\",\"Low\":\"90.66\",\"PreviousClose\":\"91.49\",\"Last\":\"92.7001\",\"Ask\":\"92.73\",\"AskSize\":\"100\",\"Bid\":\"92.67\",\"BidSize\":\"500\",\"NetChange\":\"1.2101\",\"NetChangePct\":\"1.32265821401246\",\"High52Week\":\"101.64\",\"High52WeekTimestamp\":\"2024-09-17T00:00:00Z\",\"Low52Week\":\"84.89\",\"Low52WeekTimestamp\":\"2025-01-14T00:00:00Z\",\"Volume\":\"43488407\",\"PreviousVolume\":\"39800072\",\"Close\":\"91.43\",\"DailyOpenInterest\":\"0\",\"TradeTime\":\"2025-04-02T23:59:22Z\",\"TickSizeTier\":\"0\",\"MarketFlags\":{\"IsDelayed\":false,\"IsHardToBorrow\":false,\"IsBats\":false,\"IsHalted\":false},\"LastSize\":\"200\",\"LastVenue\":\"TRF\",\"VWAP\":\"91.3955921617407\"}]}"
        )
        .create();

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let mut client = ClientBuilder::new()
            .unwrap()
            .testing_url(&server.url())
            .build()
            .await
            .unwrap();

        // Get a quote for TLT
        match client.get_quotes(vec!["TLT"]).await {
            Ok(quotes) => {
                // Should have a length of 1
                assert_eq!(quotes.len(), 1);
                // Symbol should be TLT
                assert_eq!(quotes[0].symbol, "TLT");
            }
            Err(e) => {
                panic!("Failed to parse `Quote`: {e:?}")
            }
        }
    });

    // Ensure the mock was called
    mock.assert();
}

#[test]
/// This test ensures that the parsing of
/// getting `SymbolDetails` is correct.
///
/// NOTE: In specific this is for the FUTURES variant of `SymbolDetails`.
fn test_futures_get_symbol_details_mocked() {
    // Mock the `symbols` endpoint with a raw JSON
    // string which was a real response.
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/marketdata/symbols/@TY,@FF")
        .with_status(200)
        .with_body(
            "{\"Symbols\":[{\"AssetType\":\"FUTURE\",\"Country\":\"United States\",\"Currency\":\"USD\",\"Description\":\"30 Day Federal Funds Continuous Contract [Apr25]\",\"Exchange\":\"CBOT\",\"FutureType\":\"Electronic\",\"Symbol\":\"@FF\",\"Root\":\"FF\",\"Underlying\":\"FFJ25\",\"PriceFormat\":{\"Format\":\"Decimal\",\"Decimals\":\"4\",\"IncrementStyle\":\"Simple\",\"Increment\":\"0.0005\",\"PointValue\":\"4167\"},\"QuantityFormat\":{\"Format\":\"Decimal\",\"Decimals\":\"0\",\"IncrementStyle\":\"Simple\",\"Increment\":\"1\",\"MinimumTradeQuantity\":\"1\"}},{\"AssetType\":\"FUTURE\",\"Country\":\"United States\",\"Currency\":\"USD\",\"Description\":\"10 Yr U.S. Treasury Notes Continuous Contract [Jun25]\",\"Exchange\":\"CBOT\",\"FutureType\":\"Electronic\",\"Symbol\":\"@TY\",\"Root\":\"TY\",\"Underlying\":\"TYM25\",\"PriceFormat\":{\"Format\":\"SubFraction\",\"Fraction\":\"32\",\"SubFraction\":\"2\",\"IncrementStyle\":\"Simple\",\"Increment\":\"0.015625\",\"PointValue\":\"1000\"},\"QuantityFormat\":{\"Format\":\"Decimal\",\"Decimals\":\"0\",\"IncrementStyle\":\"Simple\",\"Increment\":\"1\",\"MinimumTradeQuantity\":\"1\"}}],\"Errors\":[]}"
        )
        .create();

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let mut client = ClientBuilder::new()
            .unwrap()
            .testing_url(&server.url())
            .build()
            .await
            .unwrap();

        // Get the symbol details for the 10 Year U.S Treasury Futures Contract,
        // and the 30 Day Federal Funds Rate Futures Contract.
        match client.get_symbol_details(vec!["@TY", "@FF"]).await {
            Ok(symbol_details) => {
                // Should have a length of 2
                assert_eq!(symbol_details.len(), 2);
                // Check the symbols
                symbol_details.iter().for_each(|details| {
                    assert!(
                        ["@TY", "@FF"].contains(&details.symbol.as_str()),
                        "Symbol should only be `@TY` or `@FF`, but got {}",
                        details.symbol
                    )
                })
            }
            Err(e) => {
                panic!("Failed to parse `SymbolDetails`: {e:?}")
            }
        }
    });

    // Ensure the mock was called
    mock.assert();
}

#[test]
/// This test ensures that the parsing of
/// getting `SymbolDetails` is correct.
///
/// NOTE: In specific this is for the STOCK variant of `SymbolDetails`.
fn test_stock_get_symbol_details_mocked() {
    // Mock the `symbols` endpoint with a raw JSON
    // string which was a real response.
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/marketdata/symbols/NET,META")
        .with_status(200)
        .with_body(
            "{\"Symbols\":[{\"AssetType\":\"STOCK\",\"Country\":\"United States\",\"Currency\":\"USD\",\"Description\":\"Meta Platforms Inc\",\"Exchange\":\"NASDAQ\",\"Symbol\":\"META\",\"Root\":\"META\",\"PriceFormat\":{\"Format\":\"Decimal\",\"Decimals\":\"2\",\"IncrementStyle\":\"Simple\",\"Increment\":\"0.01\",\"PointValue\":\"1\"},\"QuantityFormat\":{\"Format\":\"Decimal\",\"Decimals\":\"0\",\"IncrementStyle\":\"Simple\",\"Increment\":\"1\",\"MinimumTradeQuantity\":\"1\"}},{\"AssetType\":\"STOCK\",\"Country\":\"United States\",\"Currency\":\"USD\",\"Description\":\"CloudFlare Inc\",\"Exchange\":\"NYSE\",\"Symbol\":\"NET\",\"Root\":\"NET\",\"PriceFormat\":{\"Format\":\"Decimal\",\"Decimals\":\"2\",\"IncrementStyle\":\"Simple\",\"Increment\":\"0.01\",\"PointValue\":\"1\"},\"QuantityFormat\":{\"Format\":\"Decimal\",\"Decimals\":\"0\",\"IncrementStyle\":\"Simple\",\"Increment\":\"1\",\"MinimumTradeQuantity\":\"1\"}}],\"Errors\":[]}"
        )
        .create();

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let mut client = ClientBuilder::new()
            .unwrap()
            .testing_url(&server.url())
            .build()
            .await
            .unwrap();

        // Get the symbol details for CloudFlare Stock (NET), and Meta Stock (META).
        match client.get_symbol_details(vec!["NET", "META"]).await {
            Ok(symbol_details) => {
                // Should have a length of 2
                assert_eq!(symbol_details.len(), 2);
                // Check the symbols
                symbol_details.iter().for_each(|details| {
                    assert!(
                        ["NET", "META"].contains(&details.symbol.as_str()),
                        "Symbol should only be `NET` or `META`, but got {}",
                        details.symbol
                    )
                })
            }
            Err(e) => {
                panic!("Failed to parse `SymbolDetails`: {e:?}")
            }
        }
    });

    // Ensure the mock was called
    mock.assert();
}

#[test]
/// This test ensures that the parsing of
/// getting `SymbolDetails` is correct.
///
/// NOTE: In specific this is for the OPTIONS variant of `SymbolDetails`.
fn test_options_get_symbol_details_mocked() {
    // Mock the `symbols` endpoint with a raw JSON
    // string which was a real response.
    let mut server = Server::new();
    let mock = server
        // NOTE: The '%20' is a url encoded space character
        .mock("GET", "/marketdata/symbols/SPY%20250407C510,TLT%20250409P90")
        .with_status(200)
        .with_body(
            "{\"Symbols\":[{\"AssetType\":\"STOCKOPTION\",\"Country\":\"United States\",\"Currency\":\"USD\",\"Description\":\"SPDR S\\u0026P 500 ETF [SPY] Apr 2025 510.000 Call\",\"Exchange\":\"OPRA\",\"ExpirationDate\":\"2025-04-07T00:00:00Z\",\"Symbol\":\"SPY 250407C510\",\"OptionType\":\"CALL\",\"Root\":\"SPY\",\"StrikePrice\":\"510\",\"Underlying\":\"SPY\",\"PriceFormat\":{\"Format\":\"Decimal\",\"Decimals\":\"2\",\"IncrementStyle\":\"Simple\",\"Increment\":\"0.01\",\"PointValue\":\"100\"},\"QuantityFormat\":{\"Format\":\"Decimal\",\"Decimals\":\"0\",\"IncrementStyle\":\"Simple\",\"Increment\":\"1\",\"MinimumTradeQuantity\":\"1\"}},{\"AssetType\":\"STOCKOPTION\",\"Country\":\"United States\",\"Currency\":\"USD\",\"Description\":\"TLT [TLT] Apr 2025 90.000 Put\",\"Exchange\":\"OPRA\",\"ExpirationDate\":\"2025-04-09T00:00:00Z\",\"Symbol\":\"TLT 250409P90\",\"OptionType\":\"PUT\",\"Root\":\"TLT\",\"StrikePrice\":\"90\",\"Underlying\":\"TLT\",\"PriceFormat\":{\"Format\":\"Decimal\",\"Decimals\":\"2\",\"IncrementStyle\":\"Schedule\",\"PointValue\":\"100\",\"IncrementSchedule\":[{\"StartsAt\":\"0\",\"Increment\":\"0.01\"},{\"StartsAt\":\"3\",\"Increment\":\"0.05\"}]},\"QuantityFormat\":{\"Format\":\"Decimal\",\"Decimals\":\"0\",\"IncrementStyle\":\"Simple\",\"Increment\":\"1\",\"MinimumTradeQuantity\":\"1\"}}],\"Errors\":[]}"
        )
        .create();

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let mut client = ClientBuilder::new()
            .unwrap()
            .testing_url(&server.url())
            .build()
            .await
            .unwrap();

        // Get the symbol details for a call option @ the $510 strike on SPY
        // for 04/07/25 expiration and a put option @ the $90 strike on TLT
        // for 04/09/25 expiration
        match client
            .get_symbol_details(vec!["SPY 250407C510", "TLT 250409P90"])
            .await
        {
            Ok(symbol_details) => {
                // Should have a length of 2
                assert_eq!(symbol_details.len(), 2);
                // Check the symbols
                symbol_details.iter().for_each(|details| {
                    assert!(
                        ["SPY 250407C510", "TLT 250409P90"].contains(&details.symbol.as_str()),
                        "Symbol should only be `SPY 250407C510` or `TLT 250409P90`, but got {}",
                        details.symbol
                    )
                })
            }
            Err(e) => {
                panic!("Failed to parse `SymbolDetails`: {e:?}")
            }
        }
    });

    // Ensure the mock was called
    mock.assert();
}

#[test]
/// This test ensures that the parsing of
/// getting `OptionExpiration` is correct.
fn test_get_option_expirations_mocked() {
    // Mock the `symbols` endpoint with a raw JSON
    // string which was a real response.
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/marketdata/options/expirations/NEE")
        .with_status(200)
        .with_body(
            "{\"Expirations\":[{\"Date\":\"2025-04-11T00:00:00Z\",\"Type\":\"Weekly\"},{\"Date\":\"2025-04-17T00:00:00Z\",\"Type\":\"Monthly\"},{\"Date\":\"2025-04-25T00:00:00Z\",\"Type\":\"Weekly\"},{\"Date\":\"2025-05-02T00:00:00Z\",\"Type\":\"Weekly\"},{\"Date\":\"2025-05-09T00:00:00Z\",\"Type\":\"Weekly\"},{\"Date\":\"2025-05-16T00:00:00Z\",\"Type\":\"Monthly\"},{\"Date\":\"2025-05-23T00:00:00Z\",\"Type\":\"Weekly\"},{\"Date\":\"2025-06-20T00:00:00Z\",\"Type\":\"Monthly\"},{\"Date\":\"2025-07-18T00:00:00Z\",\"Type\":\"Monthly\"},{\"Date\":\"2025-08-15T00:00:00Z\",\"Type\":\"Monthly\"},{\"Date\":\"2025-09-19T00:00:00Z\",\"Type\":\"Monthly\"},{\"Date\":\"2025-10-17T00:00:00Z\",\"Type\":\"Monthly\"},{\"Date\":\"2025-12-19T00:00:00Z\",\"Type\":\"Monthly\"},{\"Date\":\"2026-01-16T00:00:00Z\",\"Type\":\"Monthly\"},{\"Date\":\"2026-03-20T00:00:00Z\",\"Type\":\"Monthly\"},{\"Date\":\"2026-06-18T00:00:00Z\",\"Type\":\"Monthly\"},{\"Date\":\"2027-01-15T00:00:00Z\",\"Type\":\"Monthly\"}]}"
        )
        .create();

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let mut client = ClientBuilder::new()
            .unwrap()
            .testing_url(&server.url())
            .build()
            .await
            .unwrap();

        // Try to get the current available option expirations for Next Era Energy (NEE)
        match client.get_option_expirations("NEE", None).await {
            Ok(expirations) => {
                assert!(
                    !expirations.is_empty(),
                    "The vector of expirations should not be empty"
                );
            }
            Err(e) => {
                panic!("Failed to parse `OptionExpiration`: {e:?}")
            }
        }
    });

    // Ensure the mock was called
    mock.assert();
}

#[test]
/// This test ensures that the parsing of getting
/// `OptionRiskRewardAnalysis` is correct.
fn test_analyze_option_risk_reward_mocked() {
    // Mock the `options/riskreward` endpoint with a raw JSON
    // string which was a real response from the API.
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/marketdata/options/riskreward")
        .with_status(200)
        .with_body(
            "{\"MaxGainIsInfinite\":true,\"AdjustedMaxGain\":\"0\",\"MaxLossIsInfinite\":false,\"AdjustedMaxLoss\":\"-4400\",\"BreakevenPoints\":[\"88.6\",\"97.4\"]}"
        )
        .create();

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let mut client = ClientBuilder::new()
            .unwrap()
            .testing_url(&server.url())
            .build()
            .await
            .unwrap();

        // Long volatility trade on TLT via buying both call and put
        // options at the same expiration date. Where the price for
        // each option contract pair (call + put) is 4.40 (* 100 = $440.00)
        let price = 4.40;
        let option_legs = vec![
            OptionsLeg {
                symbol: "TLT 250516C93".into(),
                quantity: 10,
                trade_action: OptionTradeAction::Buy,
            },
            OptionsLeg {
                symbol: "TLT 250516P93".into(),
                quantity: 10,
                trade_action: OptionTradeAction::Buy,
            },
        ];

        // Try to analyze the risk reward for the trade setup above
        match client.analyze_options_risk_reward(price, option_legs).await {
            Ok(analysis) => {
                assert!(
                    analysis.max_gain_is_infinite,
                    "The max gain of this trade should be infinite"
                );
                assert!(
                    !analysis.max_loss_is_infinite,
                    "The max loss of this trade should not be infinite"
                );
                assert!(
                    analysis.breakeven_points.len() == 2,
                    "There should be 2 breakeven points for this trade as it involes 2 legs"
                )
            }
            Err(e) => {
                panic!("Failed to parse `OptionRiskRewardAnalysis`: {e:?}")
            }
        }
    });

    // Ensure the mock was called
    mock.assert();
}
