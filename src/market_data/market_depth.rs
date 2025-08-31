use crate::{
    responses::market_data::{StreamMarketDepthAggregatesResp, StreamMarketDepthQuotesResp},
    Client, Error,
};
use futures::{Stream, StreamExt};
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
    /// Stream realtime market depth quotes for the given Symbol.
    ///
    /// <div class="warning">WARNING: There's a max of 10 concurrent streams allowed.</div>
    ///
    /// NOTE: You must pin the stream before polling it.
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
    /// let mut market_depth_stream = MarketDepthQuotes::stream(&client, "SPY", Some(depth_levels));
    /// tokio::pin!(market_depth_stream); // You must pin the stream
    ///
    /// let mut streamed_quotes = Vec::new();
    ///
    /// while let Some(stream_resp) = market_depth_stream.next().await {
    ///     // The response type is `responses::market_data::StreamMarketDepthQuotesResp`
    ///     // which has multiple variants. The main one you care about is `Quote`,
    ///     // which will contain market depth quote data sent from the stream.
    ///     match stream_resp {
    ///         Ok(StreamMarketDepthQuotesResp::Quote(quote)) => {
    ///             // Do something with the market depth data
    ///             println!("{quote:?}");
    ///             streamed_quotes.push(quote);
    ///         }
    ///         Ok(StreamMarketDepthQuotesResp::Heartbeat(heartbeat)) => {
    ///             // Response for periodic signals letting you know the connection is
    ///             // still alive. A heartbeat is sent every 5 seconds of inactivity.
    ///             println!("{heartbeat:?}");
    ///
    ///             // For the sake of this example, after we receive the
    ///             // tenth heartbeat, we will stop the stream session.
    ///             if heartbeat.heartbeat > 10 {
    ///                 // Example: stopping a stream connection
    ///                 return Err(Error::StopStream);
    ///             }
    ///         }
    ///         Ok(StreamMarketDepthQuotesResp::Status(status)) => {
    ///             // Signal sent on state changes in the stream
    ///             // (closed, opened, paused, resumed)
    ///             println!("{status:?}");
    ///         }
    ///         Ok(StreamMarketDepthQuotesResp::Error(err)) => {
    ///             // Response for when an error was encountered,
    ///             // with details on the error
    ///             eprintln!("{err:?}");
    ///         }
    ///         Err(err) => {
    ///             // Stream / network error
    ///             eprintln!("{err:?}");
    ///         }
    ///     }
    /// }
    ///
    /// // After the stream ends, print all the collected market depth quotes
    /// println!("{streamed_quotes:?}");
    /// ```
    pub fn stream<S: Into<String>>(
        client: &Client,
        symbol: S,
        levels: Option<i32>,
    ) -> impl Stream<Item = Result<StreamMarketDepthQuotesResp, Error>> + '_ {
        let endpoint = format!(
            "marketdata/stream/marketdepth/quotes/{}?maxlevels={}",
            symbol.into(),
            levels.unwrap_or(20),
        );

        client.stream(endpoint).filter_map(|chunk| async {
            match chunk {
                Ok(value) => match serde_json::from_value::<StreamMarketDepthQuotesResp>(value) {
                    Ok(parsed) => Some(Ok(parsed)),
                    Err(e) => Some(Err(Error::Json(e))),
                },
                Err(e) => Some(Err(e)),
            }
        })
    }

    /// Streams [`MarketDepthQuotes`] for the provided symbol.
    ///
    /// This method builds a stream connection and continuously passes
    /// incoming stream events ([`StreamMarketDepthQuotesResp`]) to the
    /// provided `callback` closure for processing.
    ///
    /// # Depth Levels
    /// Pass the number of market depth levels you wish to stream.
    /// NOTE: Default's to a level of 20 when not provided.
    ///
    /// # Stopping the stream
    ///
    /// To stop the stream gracefully from within the callback, return
    /// `Err(Error::StopStream)`. This is treated as a control signal and will
    /// terminate the stream without propagating an error. Any other error
    /// returned from the callback will abort the stream and be returned to
    /// the caller.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if:
    /// - the underlying HTTP/WebSocket stream fails,
    /// - deserialization of a stream event into [`StreamMarketDepthQuotesResp`] fails,
    /// - or the `callback` returns an error other than [`Error::StopStream`].
    ///
    /// # Examples
    ///
    /// Stream events on the market depth for the S&P 500 ETF.
    /// ```rust,no_run
    /// # use tradestation::{Error, Client, market_data::MarketDepthQuotes};
    /// # async fn example(client: &Client) -> Result<(), Error> {
    /// MarketDepthQuotes::stream_into(client, "SPY", Some(5), |stream_event| {
    ///     println!("Market Depth Stream Event: {stream_event:?}");
    ///     Ok(())
    /// }).await?;
    /// # Ok(()) }
    pub async fn stream_into(
        client: &Client,
        symbol: impl Into<String>,
        levels: Option<u32>,
        mut callback: impl FnMut(StreamMarketDepthQuotesResp) -> Result<(), Error>,
    ) -> Result<(), Error> {
        let endpoint = format!(
            "marketdata/stream/marketdepth/quotes/{}?maxlevels={}",
            symbol.into(),
            levels.unwrap_or(20),
        );

        client
            .stream_into(&endpoint, |stream_event| {
                let event: StreamMarketDepthQuotesResp = serde_json::from_value(stream_event)?;
                callback(event)
            })
            .await
            .or_else(|e| match e {
                Error::StopStream => Ok(()),
                other => Err(other),
            })
    }
}
impl Client {
    /// Stream realtime market depth quotes for the given Symbol.
    ///
    /// <div class="warning">WARNING: There's a max of 10 concurrent streams allowed.</div>
    ///
    /// NOTE: You must pin the stream before polling it.
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
    /// let mut market_depth_stream = client.stream_market_depth_quotes("SPY", Some(depth_levels));
    /// tokio::pin!(market_depth_stream); // You must pin the stream
    ///
    /// let mut streamed_quotes = Vec::new();
    ///
    /// while let Some(stream_resp) = market_depth_stream.next().await {
    ///     // The response type is `responses::market_data::StreamMarketDepthQuotesResp`
    ///     // which has multiple variants. The main one you care about is `Quote`,
    ///     // which will contain market depth quote data sent from the stream.
    ///     match stream_resp {
    ///         Ok(StreamMarketDepthQuotesResp::Quote(quote)) => {
    ///             // Do something with the market depth data
    ///             println!("{quote:?}");
    ///             streamed_quotes.push(quote);
    ///         }
    ///         Ok(StreamMarketDepthQuotesResp::Heartbeat(heartbeat)) => {
    ///             // Response for periodic signals letting you know the connection is
    ///             // still alive. A heartbeat is sent every 5 seconds of inactivity.
    ///             println!("{heartbeat:?}");
    ///
    ///             // For the sake of this example, after we receive the
    ///             // tenth heartbeat, we will stop the stream session.
    ///             if heartbeat.heartbeat > 10 {
    ///                 // Example: stopping a stream connection
    ///                 return Err(Error::StopStream);
    ///             }
    ///         }
    ///         Ok(StreamMarketDepthQuotesResp::Status(status)) => {
    ///             // Signal sent on state changes in the stream
    ///             // (closed, opened, paused, resumed)
    ///             println!("{status:?}");
    ///         }
    ///         Ok(StreamMarketDepthQuotesResp::Error(err)) => {
    ///             // Response for when an error was encountered,
    ///             // with details on the error
    ///             eprintln!("{err:?}");
    ///         }
    ///         Err(err) => {
    ///             // Stream / network error
    ///             eprintln!("{err:?}");
    ///         }
    ///     }
    /// }
    ///
    /// // After the stream ends, print all the collected market depth quotes
    /// println!("{streamed_quotes:?}");
    /// ```
    pub fn stream_market_depth_quotes<S: Into<String>>(
        &self,
        symbol: S,
        levels: Option<i32>,
    ) -> impl Stream<Item = Result<StreamMarketDepthQuotesResp, Error>> + '_ {
        MarketDepthQuotes::stream(self, symbol, levels)
    }

    /// Streams [`MarketDepthQuotes`] for the provided symbol.
    ///
    /// This method builds a stream connection and continuously passes
    /// incoming stream events ([`StreamMarketDepthQuotesResp`]) to the
    /// provided `callback` closure for processing.
    ///
    /// # Depth Levels
    /// Pass the number of market depth levels you wish to stream.
    /// NOTE: Default's to a level of 20 when not provided.
    ///
    /// # Stopping the stream
    ///
    /// To stop the stream gracefully from within the callback, return
    /// `Err(Error::StopStream)`. This is treated as a control signal and will
    /// terminate the stream without propagating an error. Any other error
    /// returned from the callback will abort the stream and be returned to
    /// the caller.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if:
    /// - the underlying HTTP/WebSocket stream fails,
    /// - deserialization of a stream event into [`StreamMarketDepthQuotesResp`] fails,
    /// - or the `callback` returns an error other than [`Error::StopStream`].
    ///
    /// # Examples
    ///
    /// Stream events on the market depth for the S&P 500 ETF.
    /// ```rust,no_run
    /// # use tradestation::{Error, Client, market_data::MarketDepthQuotes};
    /// # async fn example(client: &Client) -> Result<(), Error> {
    /// client.stream_market_depth_quotes_into("SPY", Some(5), |stream_event| {
    ///     println!("Market Depth Stream Event: {stream_event:?}");
    ///     Ok(())
    /// }).await?;
    /// # Ok(()) }
    pub async fn stream_market_depth_quotes_into(
        &self,
        symbol: impl Into<String>,
        levels: Option<u32>,
        callback: impl FnMut(StreamMarketDepthQuotesResp) -> Result<(), Error>,
    ) -> Result<(), Error> {
        MarketDepthQuotes::stream_into(self, symbol, levels, callback).await
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
/// A collection of [`MarketDepthAggregate`]'s split into `bids` and `asks`.
pub struct MarketDepthAggregates {
    /// Aggregated bid quotes, ordered from high to low price.
    bids: Vec<MarketDepthAggregate>,

    /// Aggregated ask quotes, ordered from low to high price.
    asks: Vec<MarketDepthAggregate>,
}
impl MarketDepthAggregates {
    /// Stream realtime aggregates of market depth for the given Symbol.
    ///
    /// <div class="warning">WARNING: There's a max of 10 concurrent streams allowed.</div>
    ///
    /// NOTE: You must pin the stream before polling it.
    ///
    /// NOTE: `symbol` must be a string of a valid symbol.
    /// E.g: `"AMD"`.
    ///
    /// NOTE: `levels` must be `None` (defaults to 20 levels) or `Some(i32)`.
    /// This is for specifying how many levels of price to stream quotes for.
    ///
    /// # Example
    /// ---
    /// Stream market depth aggregates on December 2030 Natural Gas Futures `"NGZ30"`
    /// to watch order flow, maybe detecting iceberg orders, or whatever else.
    /// ```ignore
    /// let depth_levels: i32 = 25;
    /// let mut aggregate_stream = MarketDepthAggregates::stream(&client, "NGZ30", Some(depth_levels));
    /// tokio::pin!(aggregate_stream); // You must pin the stream
    ///
    /// let mut streamed_aggregates = Vec::new();
    ///
    /// while let Some(stream_resp) = aggregate_stream.next().await {
    ///     // The response type is `responses::market_data::StreamMarketDepthAggregatesResp`
    ///     // which has multiple variants. The main one you care about is `Aggregate`
    ///     // which will contain market depth aggregate data sent from the stream.
    ///     match stream_resp {
    ///         Ok(StreamMarketDepthAggregatesResp::Aggregate(quote)) => {
    ///             // Do something with the quote — for example, calculate
    ///             // the implied quote for a synthetic position or spread.
    ///             println!("{quote:?}");
    ///             streamed_aggregates.push(quote);
    ///         }
    ///         Ok(StreamMarketDepthAggregatesResp::Heartbeat(heartbeat)) => {
    ///             // Response for periodic signals letting you know the connection is
    ///             // still alive. A heartbeat is sent every 5 seconds of inactivity.
    ///             println!("{heartbeat:?}");
    ///
    ///             // For the sake of this example, after we receive the
    ///             // tenth heartbeat, we will stop the stream session.
    ///             if heartbeat.heartbeat > 10 {
    ///                 // Example: stopping a stream connection
    ///                 return Err(Error::StopStream);
    ///             }
    ///         }
    ///         Ok(StreamMarketDepthAggregatesResp::Status(status)) => {
    ///             // Signal sent on state changes in the stream
    ///             // (closed, opened, paused, resumed)
    ///             println!("{status:?}");
    ///         }
    ///         Ok(StreamMarketDepthAggregatesResp::Error(err)) => {
    ///             // Response for when an error was encountered,
    ///             // with details on the error
    ///             eprintln!("{err:?}");
    ///         }
    ///         Err(err) => {
    ///             // Stream / network error
    ///             eprintln!("{err:?}");
    ///         }
    ///     }
    /// }
    ///
    /// // After the stream ends, print all the collected market depth aggregates
    /// println!("{streamed_aggregates:?}");
    /// ```
    pub fn stream<S: Into<String>>(
        client: &Client,
        symbol: S,
        levels: Option<i32>,
    ) -> impl Stream<Item = Result<StreamMarketDepthAggregatesResp, Error>> + '_ {
        let endpoint = format!(
            "marketdata/stream/marketdepth/aggregates/{}?maxlevels={}",
            symbol.into(),
            levels.unwrap_or(20),
        );

        client.stream(endpoint).filter_map(|chunk| async {
            match chunk {
                Ok(value) => match serde_json::from_value::<StreamMarketDepthAggregatesResp>(value)
                {
                    Ok(parsed) => Some(Ok(parsed)),
                    Err(e) => Some(Err(Error::Json(e))),
                },
                Err(e) => Some(Err(e)),
            }
        })
    }

    /// Streams [`MarketDepthAggregates`] for the provided symbol.
    ///
    /// This method builds a stream connection and continuously passes
    /// incoming stream events ([`StreamMarketDepthAggregatesResp`]) to
    /// the provided `callback` closure for processing.
    ///
    /// # Depth Levels
    /// Pass the number of market depth levels you wish to stream.
    /// NOTE: Default's to a level of 20 when not provided.
    ///
    /// # Stopping the stream
    ///
    /// To stop the stream gracefully from within the callback, return
    /// `Err(Error::StopStream)`. This is treated as a control signal and will
    /// terminate the stream without propagating an error. Any other error
    /// returned from the callback will abort the stream and be returned to
    /// the caller.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if:
    /// - the underlying HTTP/WebSocket stream fails,
    /// - deserialization of a stream event into [`StreamMarketDepthAggregatesResp`] fails,
    /// - or the `callback` returns an error other than [`Error::StopStream`].
    ///
    /// # Examples
    ///
    /// Stream events on the market depth aggregates for the S&P 500 ETF.
    /// ```rust,no_run
    /// # use tradestation::{Error, Client, market_data::MarketDepthAggregates};
    /// # async fn example(client: &Client) -> Result<(), Error> {
    /// MarketDepthAggregates::stream_into(client, "SPY", Some(5), |stream_event| {
    ///     println!("Market Depth Aggregates Stream Event: {stream_event:?}");
    ///     Ok(())
    /// }).await?;
    /// # Ok(()) }
    pub async fn stream_into(
        client: &Client,
        symbol: impl Into<String>,
        levels: Option<u32>,
        mut callback: impl FnMut(StreamMarketDepthAggregatesResp) -> Result<(), Error>,
    ) -> Result<(), Error> {
        let endpoint = format!(
            "marketdata/stream/marketdepth/aggregates/{}?maxlevels={}",
            symbol.into(),
            levels.unwrap_or(20),
        );

        client
            .stream_into(&endpoint, |stream_event| {
                let event: StreamMarketDepthAggregatesResp = serde_json::from_value(stream_event)?;
                callback(event)
            })
            .await
            .or_else(|e| match e {
                Error::StopStream => Ok(()),
                other => Err(other),
            })
    }
}
impl Client {
    /// Stream realtime aggregates of market depth for the given Symbol.
    ///
    /// <div class="warning">WARNING: There's a max of 10 concurrent streams allowed.</div>
    ///
    /// NOTE: You must pin the stream before polling it.
    ///
    /// NOTE: `symbol` must be a string of a valid symbol.
    /// E.g: `"AMD"`.
    ///
    /// NOTE: `levels` must be `None` (defaults to 20 levels) or `Some(i32)`.
    /// This is for specifying how many levels of price to stream quotes for.
    ///
    /// # Example
    /// ---
    /// Stream market depth aggregates on December 2030 Natural Gas Futures `"NGZ30"`
    /// to watch order flow, maybe detecting iceberg orders, or whatever else.
    /// ```ignore
    /// let depth_levels: i32 = 25;
    /// let mut aggregate_stream = client.stream_market_depth_aggregates("NGZ30", Some(depth_levels));
    /// tokio::pin!(aggregate_stream); // You must pin the stream
    ///
    /// let mut streamed_aggregates = Vec::new();
    ///
    /// while let Some(stream_resp) = aggregate_stream.next().await {
    ///     // The response type is `responses::market_data::StreamMarketDepthAggregatesResp`
    ///     // which has multiple variants. The main one you care about is `Aggregate`
    ///     // which will contain market depth aggregate data sent from the stream.
    ///     match stream_resp {
    ///         Ok(StreamMarketDepthAggregatesResp::Aggregate(quote)) => {
    ///             // Do something with the quote — for example, calculate
    ///             // the implied quote for a synthetic position or spread.
    ///             println!("{quote:?}");
    ///             streamed_aggregates.push(quote);
    ///         }
    ///         Ok(StreamMarketDepthAggregatesResp::Heartbeat(heartbeat)) => {
    ///             // Response for periodic signals letting you know the connection is
    ///             // still alive. A heartbeat is sent every 5 seconds of inactivity.
    ///             println!("{heartbeat:?}");
    ///
    ///             // For the sake of this example, after we receive the
    ///             // tenth heartbeat, we will stop the stream session.
    ///             if heartbeat.heartbeat > 10 {
    ///                 // Example: stopping a stream connection
    ///                 return Err(Error::StopStream);
    ///             }
    ///         }
    ///         Ok(StreamMarketDepthAggregatesResp::Status(status)) => {
    ///             // Signal sent on state changes in the stream
    ///             // (closed, opened, paused, resumed)
    ///             println!("{status:?}");
    ///         }
    ///         Ok(StreamMarketDepthAggregatesResp::Error(err)) => {
    ///             // Response for when an error was encountered,
    ///             // with details on the error
    ///             eprintln!("{err:?}");
    ///         }
    ///         Err(err) => {
    ///             // Stream / network error
    ///             eprintln!("{err:?}");
    ///         }
    ///     }
    /// }
    ///
    /// // After the stream ends, print all the collected market depth aggregates
    /// println!("{streamed_aggregates:?}");
    /// ```
    pub fn stream_market_depth_aggregates<S: Into<String>>(
        &self,
        symbol: S,
        levels: Option<i32>,
    ) -> impl Stream<Item = Result<StreamMarketDepthAggregatesResp, Error>> + '_ {
        MarketDepthAggregates::stream(self, symbol, levels)
    }

    /// Streams [`MarketDepthAggregates`] for the provided symbol.
    ///
    /// This method builds a stream connection and continuously passes
    /// incoming stream events ([`StreamMarketDepthAggregatesResp`]) to
    /// the provided `callback` closure for processing.
    ///
    /// # Depth Levels
    /// Pass the number of market depth levels you wish to stream.
    /// NOTE: Default's to a level of 20 when not provided.
    ///
    /// # Stopping the stream
    ///
    /// To stop the stream gracefully from within the callback, return
    /// `Err(Error::StopStream)`. This is treated as a control signal and will
    /// terminate the stream without propagating an error. Any other error
    /// returned from the callback will abort the stream and be returned to
    /// the caller.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if:
    /// - the underlying HTTP/WebSocket stream fails,
    /// - deserialization of a stream event into [`StreamMarketDepthAggregatesResp`] fails,
    /// - or the `callback` returns an error other than [`Error::StopStream`].
    ///
    /// # Examples
    ///
    /// Stream events on the market depth aggregates for the S&P 500 ETF.
    /// ```rust,no_run
    /// # use tradestation::{Error, Client, market_data::MarketDepthAggregates};
    /// # async fn example(client: &Client) -> Result<(), Error> {
    /// client.stream_market_depth_aggregates_into("SPY", Some(5), |stream_event| {
    ///     println!("Market Depth Aggregates Stream Event: {stream_event:?}");
    ///     Ok(())
    /// }).await?;
    /// # Ok(()) }
    pub async fn stream_market_depth_aggregates_into(
        &self,
        symbol: impl Into<String>,
        levels: Option<u32>,
        callback: impl FnMut(StreamMarketDepthAggregatesResp) -> Result<(), Error>,
    ) -> Result<(), Error> {
        MarketDepthAggregates::stream_into(self, symbol, levels, callback).await
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
