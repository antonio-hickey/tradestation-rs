use mockito::Server;
use tradestation::ClientBuilder;
use tradestation::MarketData::{BarUnit, GetBarsQueryBuilder};

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
