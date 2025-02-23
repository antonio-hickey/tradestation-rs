use crate::{
    responses::market_data::{StreamMarketDepthAggregatesResp, StreamMarketDepthQuotesResp},
    Client, Error,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// Market Depth Quotes
pub struct MarketDepthQuotes {
    /// Contains bid quotes, ordered from high to low price.
    bids: Vec<MarketDepthQuote>,

    /// Contains ask quotes, ordered from high to low price.
    asks: Vec<MarketDepthQuote>,
}
impl MarketDepthQuotes {
    fn new() -> Self {
        Self {
            bids: Vec::new(),
            asks: Vec::new(),
        }
    }

    /// Stream realtime market depth quotes for the given Symbol.
    ///
    /// NOTE: `symbol` must be a string of a valid symbol.
    /// E.g: `"NVDA"`.
    ///
    /// NOTE: `levels` must be `None` (defaults to 20 levels) or `Some(i32)`.
    /// This is for specifying how many levels of price to stream quotes for.
    ///
    /// # Example
    /// ---
    /// Stream market depth quotes on S&P 500 ETF Trust `"SPY"` to watch order
    /// flow, maybe to use for detecting iceberg orders, or watching for large
    /// block trades, etc.
    /// ```ignore
    /// let depth_levels: i32 = 10;
    /// let streamed_quotes = client
    ///     .stream_market_depth_quotes("SPY", Some(depth_levels), |stream_data| {
    ///         // The response type is `responses::MarketData::StreamMarketDepthQuotesResp`
    ///         // which has multiple variants the main one you care about is `Quote`
    ///         // which will contain market depth quote data sent from the stream.
    ///         match stream_data {
    ///             StreamMarketDepthQuotesResp::Quote(quote) => {
    ///                 // Do something with the market depth data.
    ///                 println!("{quote:?}");
    ///             }
    ///             StreamMarketDepthQuotesResp::Heartbeat(heartbeat) => {
    ///                 // Response for periodic signals letting you know the connection is
    ///                 // still alive. A heartbeat is sent every 5 seconds of inactivity.
    ///                 println!("{heartbeat:?}");
    ///
    ///                 // for the sake of this example after we recieve the
    ///                 // tenth heartbeat, we will stop the stream session.
    ///                 if heartbeat.heartbeat > 10 {
    ///                     // Example: stopping a stream connection
    ///                     return Err(Error::StopStream);
    ///                 }
    ///             }
    ///             StreamMarketDepthQuotesResp::Status(status) => {
    ///                 // Signal sent on state changes in the stream
    ///                 // (closed, opened, paused, resumed)
    ///                 println!("{status:?}");
    ///             }
    ///             StreamMarketDepthQuotesResp::Error(err) => {
    ///                 // Response for when an error was encountered,
    ///                 // with details on the error
    ///                 println!("{err:?}");
    ///             }
    ///         }
    ///
    ///         Ok(())
    ///     })
    ///     .await?;
    ///
    /// // After the stream ends print all the collected market depth quotes
    /// println!("{streamed_quotes:?}");
    /// ```
    pub async fn stream<F, S: Into<String>>(
        client: &mut Client,
        symbol: S,
        levels: Option<i32>,
        mut on_chunk: F,
    ) -> Result<MarketDepthQuotes, Error>
    where
        F: FnMut(StreamMarketDepthQuotesResp) -> Result<(), Error>,
    {
        let endpoint = format!(
            "marketdata/stream/marketdepth/quotes/{}?maxlevels={}",
            symbol.into(),
            levels.unwrap_or(20),
        );

        let mut collected_market_depth_quotes = MarketDepthQuotes::new();
        client
            .stream(&endpoint, |chunk| {
                let parsed_chunk = serde_json::from_value::<StreamMarketDepthQuotesResp>(chunk)?;
                on_chunk(parsed_chunk.clone())?;

                // Only collect quotes, so when the stream is done
                // all the quotes that were streamed can be returned.
                if let StreamMarketDepthQuotesResp::Quote(quote) = parsed_chunk {
                    if let Some(bid) = quote.bids.first() {
                        collected_market_depth_quotes.bids.push(bid.clone());
                    }

                    if let Some(ask) = quote.asks.first() {
                        collected_market_depth_quotes.asks.push(ask.clone());
                    }
                }

                Ok(())
            })
            .await?;

        Ok(collected_market_depth_quotes)
    }
}
impl Client {
    /// Stream realtime market depth quotes for the given Symbol.
    ///
    /// NOTE: `symbol` must be a string of a valid symbol.
    /// E.g: `"NVDA"`.
    ///
    /// NOTE: `levels` must be `None` (defaults to 20 levels) or `Some(i32)`.
    /// This is for specifying how many levels of price to stream quotes for.
    ///
    /// # Example
    /// ---
    /// Stream market depth quotes on S&P 500 ETF Trust `"SPY"` to watch order
    /// flow, maybe to use for detecting iceberg orders, or watching for large
    /// block trades, etc.
    /// ```ignore
    /// let depth_levels: i32 = 10;
    /// let streamed_quotes = client
    ///     .stream_market_depth_quotes("SPY", Some(depth_levels), |stream_data| {
    ///         // The response type is `responses::MarketData::StreamMarketDepthQuotesResp`
    ///         // which has multiple variants the main one you care about is `Quote`
    ///         // which will contain market depth quote data sent from the stream.
    ///         match stream_data {
    ///             StreamMarketDepthQuotesResp::Quote(quote) => {
    ///                 // Do something with the market depth data.
    ///                 println!("{quote:?}");
    ///             }
    ///             StreamMarketDepthQuotesResp::Heartbeat(heartbeat) => {
    ///                 // Response for periodic signals letting you know the connection is
    ///                 // still alive. A heartbeat is sent every 5 seconds of inactivity.
    ///                 println!("{heartbeat:?}");
    ///
    ///                 // for the sake of this example after we recieve the
    ///                 // tenth heartbeat, we will stop the stream session.
    ///                 if heartbeat.heartbeat > 10 {
    ///                     // Example: stopping a stream connection
    ///                     return Err(Error::StopStream);
    ///                 }
    ///             }
    ///             StreamMarketDepthQuotesResp::Status(status) => {
    ///                 // Signal sent on state changes in the stream
    ///                 // (closed, opened, paused, resumed)
    ///                 println!("{status:?}");
    ///             }
    ///             StreamMarketDepthQuotesResp::Error(err) => {
    ///                 // Response for when an error was encountered,
    ///                 // with details on the error
    ///                 println!("{err:?}");
    ///             }
    ///         }
    ///
    ///         Ok(())
    ///     })
    ///     .await?;
    ///
    /// // After the stream ends print all the collected market depth quotes
    /// println!("{streamed_quotes:?}");
    /// ```
    pub async fn stream_market_depth_quotes<S: Into<String>, F>(
        &mut self,
        symbol: S,
        levels: Option<i32>,
        on_chunk: F,
    ) -> Result<MarketDepthQuotes, Error>
    where
        F: FnMut(StreamMarketDepthQuotesResp) -> Result<(), Error>,
    {
        MarketDepthQuotes::stream(self, symbol, levels, on_chunk).await
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// Order book / market depth quote.
pub struct MarketDepthQuote {
    /// The timestamp for this quote, represented as an RFC3339 formatted
    /// date, a profile of the ISO 8601 date standard.
    /// E.g. `"2026-06-28T12:34:01Z"`.
    pub time_stamp: String,

    /// The side of the quote in the order book (Bid or Ask).
    pub side: MarketDepthSide,

    /// The price of the quote.
    pub price: String,

    /// The total number of shares offered/requested by participants.
    pub size: String,

    /// The number of orders aggregated together for this quote by the
    /// participant (market maker or ECN).
    pub order_count: i32,

    /// The name of the participant associated with this quote.
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct MarketDepthAggregates {
    /// Aggregated bid quotes, ordered from high to low price.
    bids: Vec<MarketDepthAggregate>,

    /// Aggregated ask quotes, ordered from low to high price.
    asks: Vec<MarketDepthAggregate>,
}
impl MarketDepthAggregates {
    fn new() -> Self {
        Self {
            bids: Vec::new(),
            asks: Vec::new(),
        }
    }

    /// Stream realtime aggregates of market depth for the given Symbol.
    ///
    /// NOTE: `symbol` must be a string of a valid symbol.
    /// E.g: `"AMD"`.
    ///
    /// NOTE: `levels` must be `None` (defaults to 20 levels) or `Some(i32)`.
    /// This is for specifying how many levels of price to stream quotes for.
    ///
    /// # Example
    /// ---
    /// Stream market depth aggregates on December 2024 Natural Gas Futures `"NGZ24"`
    /// to watch order flow, maybe detecting iceberg orders, or whatever else.
    /// ```ignore
    /// let depth_levels: i32 = 25;
    /// let streamed_aggregates = client
    ///     .stream_market_depth_aggregates("NGZ24", Some(depth_levels), |stream_data| {
    ///         // The response type is `responses::MarketData::StreamAggregatesResp`
    ///         // which has multiple variants the main one you care about is `Aggregate`
    ///         // which will contain market depth aggregate data sent from the stream.
    ///         match stream_data {
    ///             StreamMarketDepthAggregatesResp::Aggregate(quote) => {
    ///                 // Do something with the quote for example derive
    ///                 // a quote for a long amd short nvidia trade.
    ///                 println!("{quote:?}");
    ///             }
    ///             StreamMarketDepthAggregatesResp::Heartbeat(heartbeat) => {
    ///                 // Response for periodic signals letting you know the connection is
    ///                 // still alive. A heartbeat is sent every 5 seconds of inactivity.
    ///                 println!("{heartbeat:?}");
    ///
    ///                 // for the sake of this example after we recieve the
    ///                 // tenth heartbeat, we will stop the stream session.
    ///                 if heartbeat.heartbeat > 10 {
    ///                     // Example: stopping a stream connection
    ///                     return Err(Error::StopStream);
    ///                 }
    ///             }
    ///             StreamMarketDepthAggregatesResp::Status(status) => {
    ///                 // Signal sent on state changes in the stream
    ///                 // (closed, opened, paused, resumed)
    ///                 println!("{status:?}");
    ///             }
    ///             StreamMarketDepthAggregatesResp::Error(err) => {
    ///                 // Response for when an error was encountered,
    ///                 // with details on the error
    ///                 println!("{err:?}");
    ///             }
    ///         }
    ///
    ///         Ok(())
    ///     })
    ///     .await?;
    ///
    /// // After the stream ends print all the collected
    /// // market depth aggregates.
    /// println!("{streamed_aggregates:?}");
    /// ```
    pub async fn stream<F, S: Into<String>>(
        client: &mut Client,
        symbol: S,
        levels: Option<i32>,
        mut on_chunk: F,
    ) -> Result<MarketDepthAggregates, Error>
    where
        F: FnMut(StreamMarketDepthAggregatesResp) -> Result<(), Error>,
    {
        let endpoint = format!(
            "marketdata/stream/marketdepth/aggregates/{}?maxlevels={}",
            symbol.into(),
            levels.unwrap_or(20),
        );

        let mut collected_market_depth_quotes = MarketDepthAggregates::new();
        client
            .stream(&endpoint, |chunk| {
                let parsed_chunk =
                    serde_json::from_value::<StreamMarketDepthAggregatesResp>(chunk)?;
                on_chunk(parsed_chunk.clone())?;

                // Only collect quotes, so when the stream is done
                // all the quotes that were streamed can be returned.
                if let StreamMarketDepthAggregatesResp::Aggregate(quote) = parsed_chunk {
                    if let Some(bid) = quote.bids.first() {
                        collected_market_depth_quotes.bids.push(bid.clone());
                    }

                    if let Some(ask) = quote.asks.first() {
                        collected_market_depth_quotes.asks.push(ask.clone());
                    }
                }

                Ok(())
            })
            .await?;

        Ok(collected_market_depth_quotes)
    }
}
impl Client {
    /// Stream realtime aggregates of market depth for the given Symbol.
    ///
    /// NOTE: `symbol` must be a string of a valid symbol.
    /// E.g: `"AMD"`.
    ///
    /// NOTE: `levels` must be `None` (defaults to 20 levels) or `Some(i32)`.
    /// This is for specifying how many levels of price to stream quotes for.
    ///
    /// # Example
    /// ---
    /// Stream market depth aggregates on December 2024 Natural Gas Futures `"NGZ24"`
    /// to watch order flow, maybe detecting iceberg orders, or whatever else.
    /// ```ignore
    /// let depth_levels: i32 = 25;
    /// let streamed_aggregates = client
    ///     .stream_market_depth_aggregates("NGZ24", Some(depth_levels), |stream_data| {
    ///         // The response type is `responses::MarketData::StreamAggregatesResp`
    ///         // which has multiple variants the main one you care about is `Aggregate`
    ///         // which will contain market depth aggregate data sent from the stream.
    ///         match stream_data {
    ///             StreamMarketDepthAggregatesResp::Aggregate(quote) => {
    ///                 // Do something with the quote for example derive
    ///                 // a quote for a long amd short nvidia trade.
    ///                 println!("{quote:?}");
    ///             }
    ///             StreamMarketDepthAggregatesResp::Heartbeat(heartbeat) => {
    ///                 // Response for periodic signals letting you know the connection is
    ///                 // still alive. A heartbeat is sent every 5 seconds of inactivity.
    ///                 println!("{heartbeat:?}");
    ///
    ///                 // for the sake of this example after we recieve the
    ///                 // tenth heartbeat, we will stop the stream session.
    ///                 if heartbeat.heartbeat > 10 {
    ///                     // Example: stopping a stream connection
    ///                     return Err(Error::StopStream);
    ///                 }
    ///             }
    ///             StreamMarketDepthAggregatesResp::Status(status) => {
    ///                 // Signal sent on state changes in the stream
    ///                 // (closed, opened, paused, resumed)
    ///                 println!("{status:?}");
    ///             }
    ///             StreamMarketDepthAggregatesResp::Error(err) => {
    ///                 // Response for when an error was encountered,
    ///                 // with details on the error
    ///                 println!("{err:?}");
    ///             }
    ///         }
    ///
    ///         Ok(())
    ///     })
    ///     .await?;
    ///
    /// // After the stream ends print all the collected
    /// // market depth aggregates.
    /// println!("{streamed_aggregates:?}");
    /// ```
    pub async fn stream_market_depth_aggregates<S: Into<String>, F>(
        &mut self,
        symbol: S,
        levels: Option<i32>,
        on_chunk: F,
    ) -> Result<MarketDepthAggregates, Error>
    where
        F: FnMut(StreamMarketDepthAggregatesResp) -> Result<(), Error>,
    {
        MarketDepthAggregates::stream(self, symbol, levels, on_chunk).await
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// Aggregated quotes, ordered from high to low price.
pub struct MarketDepthAggregate {
    /// The earliest participant timestamp for this quote, represented as an
    /// RFC3339 formatted date, a profile of the ISO 8601 date standard.
    /// E.g. `"2022-06-28T12:34:01Z"`.
    pub earliest_time: String,

    /// The latest participant timestamp for this quote, represented as an
    /// RFC3339 formatted date, a profile of the ISO 8601 date standard.
    /// E.g. `"2022-06-28T12:34:56Z"`.
    pub latest_time: String,

    /// The side of the quote on the order book (bid or ask).
    pub side: MarketDepthSide,

    /// The price of the quote.
    pub price: String,

    /// The total number of shares, or contracts offered/requested by all participants.
    pub total_size: String,

    /// The largest number of shares, or contracts offered/requested by any participant.
    pub biggest_size: String,

    /// The smallest number of shares, or contracts offered/requested by any participant.
    pub smallest_size: String,

    /// The number of participants offering/requesting this price.
    pub num_participants: i32,

    /// The sum of the order counts for all participants offering/requesting
    /// this price.
    pub total_order_count: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize, Copy)]
/// The different sides of the market depth order book.
pub enum MarketDepthSide {
    /// Represents the bid side of the order book.
    Bid,

    /// Represents the ask side of the order book.
    Ask,
}
