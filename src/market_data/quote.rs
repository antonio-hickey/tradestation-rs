use crate::responses::account::StreamPositionsResp;
use crate::responses::market_data::{
    GetQuoteSnapshotsResp, GetQuoteSnapshotsRespRaw, StreamQuotesResp,
};
use crate::responses::ApiResponse;
use crate::{Client, Error};
use futures::{Stream, StreamExt};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// A quote on a specific asset symbol.
pub struct Quote {
    /// The price at which a security, futures contract, or other
    /// financial instrument is offered for sale.
    pub ask: String,

    /// The number of trading units that prospective sellers are
    /// prepared to sell.
    pub ask_size: String,

    /// The highest price a prospective buyer is prepared to pay
    /// at a particular time for a trading unit of a given symbol.
    pub bid: String,

    /// The number of trading units that prospective buyers are
    /// prepared to purchase for a symbol.
    pub bid_size: String,

    /// The closing price of the day.
    pub close: String,

    /// The total number of open or outstanding (not closed or delivered)
    /// options and/or futures contracts that exist on a given day, delivered
    /// on a particular day.
    pub daily_open_interest: String,

    /// The highest price of the day.
    pub high: String,

    /// The lowest price of the day.
    pub low: String,

    /// The highest price of the past 52 weeks.
    pub high_52_week: String,

    /// Date of the highest price in the past 52 week.
    pub high_52_week_timestamp: String,

    /// The last price at which the symbol traded.
    pub last: String,

    /// The minimum price a commodity futures contract may be traded for
    /// the current session.
    ///
    /// NOTE: Only `Some(price)` for commodity futures symbols.
    pub min_price: Option<String>,

    /// The maximum price a commodity futures contract may be traded for
    /// the current session.
    ///
    /// NOTE: Only `Some(price)` for commodity futures symbols.
    pub max_price: Option<String>,

    /// The day after which an investor who has purchased a futures contract
    /// may be required to take physical delivery of the contracts underlying
    /// commodity.
    ///
    /// NOTE: Only `Some(date)` for futures symbols.
    pub first_notice_date: Option<String>,

    /// The final day that a futures contract may trade or be closed out before
    /// the delivery of the underlying asset or cash settlement must occur.
    ///
    /// NOTE: Only `Some(date)` for futures symbols.
    pub last_trading_date: Option<String>,

    /// The lowest price of the past 52 weeks.
    pub low_52_week: String,

    /// Date of the lowest price of the past 52 weeks.
    pub low_52_week_timestamp: String,

    /// Market specific information for a symbol.
    pub market_flags: MarketFlag,

    /// The difference between the last displayed price and the previous day's
    /// close.
    pub net_change: String,

    /// The percentage difference between the current price and previous day's
    /// close, expressed as a percentage. E.g: a price change from 100 to 103.5
    /// would be expressed as `"3.5"`.
    pub net_change_pct: String,

    /// The opening price of the day.
    pub open: String,

    /// The closing price of the previous day.
    pub previous_volume: String,

    /// Restriction if any returns array.
    pub restrictions: Option<Vec<String>>,

    /// The name identifying the financial instrument for which the data is displayed.
    pub symbol: String,

    /// Trading increment based on a level group.
    pub tick_size_tier: String,

    /// Time of the last trade.
    pub trade_time: String,

    /// Daily volume in shares/contracts.
    pub volume: String,

    /// Number of contracts/shares last traded.
    pub last_size: String,

    /// Exchange name of last trade.
    pub last_venue: String,

    #[serde(rename = "VWAP")]
    /// VWAP (Volume Weighted Average Price) is a measure of the price at which the
    /// majority of a given day's trading in a given security took place. It is
    /// calculated by adding the dollars traded for the average price of the bar
    /// throughout the day ("avgprice" x "number of shares traded" per bar) and
    /// dividing by the total shares traded for the day.
    pub vwap: String,
}
impl Quote {
    /// Fetches a full snapshot of the latest Quote for the given Symbols.
    ///
    /// NOTE: For realtime `Quote` updates, users should use the `Quote::stream()` endpoint.
    ///
    /// NOTE: `symbols` should be a vector of valid symbols, and no more than 100 symbols
    /// per request. E.g: `vec!["NVDA", "AMD"]`.
    ///
    /// # Example
    /// ---
    /// Get a quote on Palantir.
    /// ```ignore
    /// let palantir_quote = client.get_quotes(vec!["PLTR"]).await?;
    /// println!("Palantir Quote: {palantir_quote:?}");
    /// ```
    pub async fn fetch(symbols: Vec<&str>, client: &Client) -> Result<Vec<Quote>, Error> {
        let endpoint = format!("marketdata/quotes/{}", symbols.join(","));

        match client
            .get(&endpoint)
            .await?
            .json::<ApiResponse<GetQuoteSnapshotsRespRaw>>()
            .await?
        {
            ApiResponse::Success(resp_raw) => {
                let err_msg = resp_raw.message.clone().unwrap_or_default();

                let resp: GetQuoteSnapshotsResp = resp_raw.into();
                if let Some(quotes) = resp.quotes {
                    Ok(quotes)
                } else {
                    Err(resp
                        .error
                        .unwrap_or(Error::UnknownTradeStationAPIError(err_msg)))
                }
            }
            ApiResponse::Error(resp) => Err(Error::from_api_error(resp)),
        }
    }

    /// Stream realtime quotes for the given Symbols.
    ///
    /// NOTE: `symbols` should be a vector of valid symbols, and no more than 100 symbols
    /// per request. E.g: `vec!["NVDA", "AMD"]`.
    ///
    /// # Example
    /// ---
    ///
    /// Stream quotes on Nvidia (`"NVDA"`) and AMD (`"AMD"`).
    /// ```ignore
    /// // Start the stream and pin it to the stack
    /// let mut quote_stream = Quote::stream(&client, vec!["NVDA", "AMD"]);
    /// tokio::pin!(quote_stream); // You must pin the stream
    ///
    /// let mut streamed_quotes = Vec::new();
    ///
    /// while let Some(stream_resp) = quote_stream.next().await {
    ///     // The response type is `responses::market_data::StreamQuotesResp`
    ///     // which has multiple variants. The main one you care about is `Quote`
    ///     // which will contain quote data sent from the stream.
    ///     match stream_resp {
    ///         Ok(StreamQuotesResp::Quote(quote)) => {
    ///             // Do something with the quote — for example, derive
    ///             // a quote for a long AMD / short NVDA trade.
    ///             println!("{quote:?}");
    ///             streamed_quotes.push(quote);
    ///         }
    ///         Ok(StreamQuotesResp::Heartbeat(heartbeat)) => {
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
    ///         Ok(StreamQuotesResp::Status(status)) => {
    ///             // Signal sent on state changes in the stream
    ///             // (closed, opened, paused, resumed)
    ///             println!("{status:?}");
    ///         }
    ///         Ok(StreamQuotesResp::Error(err)) => {
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
    /// // After the stream ends, print all the collected quotes
    /// println!("{streamed_quotes:?}");
    /// ```
    pub fn stream<'a>(
        client: &'a Client,
        symbols: Vec<&'a str>,
    ) -> impl Stream<Item = Result<StreamQuotesResp, Error>> + 'a {
        let endpoint = format!("marketdata/stream/quotes/{}", symbols.join(","));

        let stream = client.stream(endpoint);

        stream.filter_map(|chunk| async {
            match chunk {
                Ok(value) => match serde_json::from_value::<StreamQuotesResp>(value) {
                    Ok(parsed) => Some(Ok(parsed)),
                    Err(e) => Some(Err(Error::Json(e))),
                },
                Err(e) => Some(Err(e)),
            }
        })
    }

    /// Streams [`Quote`]'s for the provided symbol's.
    ///
    /// This method builds a stream connection and continuously passes
    /// incoming stream events ([`StreamQuotesResp`]) to the provided
    /// `callback` closure for processing.
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
    /// - deserialization of a stream event into [`StreamQuotesResp`] fails,
    /// - or the `callback` returns an error other than [`Error::StopStream`].
    ///
    /// # Examples
    ///
    ///
    /// ```rust,no_run
    /// # use tradestation::{Error, Client, market_data::Quote};
    ///
    /// # async fn example(client: &Client) -> Result<(), Error> {
    /// Quote::stream_into(client, vec!["SPY", "NEE", "PLTR"], |stream_event| {
    ///     println!("Quote Stream Event: {stream_event:?}");
    ///     Ok(())
    /// }).await?;
    ///
    /// # Ok(()) }
    /// ```
    pub async fn stream_into(
        client: &Client,
        symbols: Vec<&str>,
        mut callback: impl FnMut(StreamPositionsResp) -> Result<(), Error>,
    ) -> Result<(), Error> {
        let endpoint = format!("marketdata/stream/quotes/{}", symbols.join(","));

        client
            .stream_into(&endpoint, |stream_event| {
                let parsed_event: StreamPositionsResp = serde_json::from_value(stream_event)?;
                callback(parsed_event)
            })
            .await
            .or_else(|e| {
                if matches!(e, Error::StopStream) {
                    Ok(())
                } else {
                    Err(e)
                }
            })
    }
}
impl Client {
    /// Fetches a full snapshot of the latest Quote for the given Symbol.
    ///
    /// NOTE: For realtime `Quote` updates, use the `Quote::stream()` endpoint.
    ///
    /// NOTE: `symbol` should be a valid symbol string, E.g: `"NVDA"`.
    ///
    /// # Example
    /// ---
    /// Get a quote on Palantir.
    /// ```ignore
    /// let palantir_quote = client.get_quote("PLTR").await?;
    /// println!("Palantir Quote: {palantir_quote:?}");
    /// ```
    pub async fn get_quote(&self, symbol: &str) -> Result<Quote, Error> {
        let mut quotes = Quote::fetch(vec![symbol], self).await?;

        // TODO: This error is not as accurate as it can be.
        // If this errors out here, it would not be that the
        // symbol is not set, but that the symbol is incorrect.
        quotes.pop().ok_or_else(|| Error::SymbolNotSet)
    }

    /// Fetches a full snapshot of the latest Quote for the given Symbols.
    ///
    /// NOTE: For realtime `Quote` updates, users should use the `Quote::stream()` endpoint.
    ///
    /// NOTE: `symbols` should be a vector of valid symbols, and no more than 100 symbols
    /// per request. E.g: `vec!["NVDA", "AMD"]`.
    ///
    /// # Example
    /// ---
    /// Get a quote on Palantir.
    /// ```ignore
    /// let palantir_quote = client.get_quotes(vec!["PLTR"]).await?;
    /// println!("Palantir Quote: {palantir_quote:?}");
    /// ```
    pub async fn get_quotes(&self, symbols: Vec<&str>) -> Result<Vec<Quote>, Error> {
        Quote::fetch(symbols, self).await
    }

    /// Stream realtime quotes for the given Symbols.
    ///
    /// <div class="warning">WARNING: There's a max of 10 concurrent streams allowed.</div>
    ///
    /// NOTE: You must pin the stream before polling it.
    ///
    /// NOTE: `symbols` should be a vector of valid symbols, and no more than 100 symbols
    /// per request. E.g: `vec!["NVDA", "AMD"]`.
    ///
    /// # Example
    /// ---
    ///
    /// Stream quotes on Nvidia (`"NVDA"`) and AMD (`"AMD"`).
    /// ```ignore
    /// // Start the stream and pin it to the stack
    /// let mut quote_stream = client.stream_quotes(vec!["NVDA", "AMD"]);
    /// tokio::pin!(quote_stream); // You must pin the stream
    ///
    /// let mut streamed_quotes = Vec::new();
    ///
    /// while let Some(stream_resp) = quote_stream.next().await {
    ///     // The response type is `responses::market_data::StreamQuotesResp`
    ///     // which has multiple variants. The main one you care about is `Quote`
    ///     // which will contain quote data sent from the stream.
    ///     match stream_resp {
    ///         Ok(StreamQuotesResp::Quote(quote)) => {
    ///             // Do something with the quote — for example, derive
    ///             // a quote for a long AMD / short NVDA trade.
    ///             println!("{quote:?}");
    ///             streamed_quotes.push(quote);
    ///         }
    ///         Ok(StreamQuotesResp::Heartbeat(heartbeat)) => {
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
    ///         Ok(StreamQuotesResp::Status(status)) => {
    ///             // Signal sent on state changes in the stream
    ///             // (closed, opened, paused, resumed)
    ///             println!("{status:?}");
    ///         }
    ///         Ok(StreamQuotesResp::Error(err)) => {
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
    /// // After the stream ends, print all the collected quotes
    /// println!("{streamed_quotes:?}");
    /// ```
    pub fn stream_quotes<'a>(
        &'a self,
        symbols: Vec<&'a str>,
    ) -> impl Stream<Item = Result<StreamQuotesResp, Error>> + 'a {
        Quote::stream(self, symbols)
    }

    /// Streams [`Quote`]'s for the provided symbol's.
    ///
    /// This method builds a stream connection and continuously passes
    /// incoming stream events ([`StreamQuotesResp`]) to the provided
    /// `callback` closure for processing.
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
    /// - deserialization of a stream event into [`StreamQuotesResp`] fails,
    /// - or the `callback` returns an error other than [`Error::StopStream`].
    ///
    /// # Examples
    ///
    ///
    /// ```rust,no_run
    /// # use tradestation::{Error, Client};
    ///
    /// # async fn example(client: &Client) -> Result<(), Error> {
    /// client.stream_quotes_into(vec!["SPY", "NEE", "PLTR"], |stream_event| {
    ///     println!("Quote Stream Event: {stream_event:?}");
    ///     Ok(())
    /// }).await?;
    ///
    /// # Ok(()) }
    /// ```
    pub async fn stream_quotes_into(
        &self,
        symbols: Vec<&str>,
        callback: impl FnMut(StreamPositionsResp) -> Result<(), Error>,
    ) -> Result<(), Error> {
        Quote::stream_into(self, symbols, callback).await
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// The different kinds of market flags.
pub struct MarketFlag {
    /// Is using BATS
    pub is_bats: bool,

    /// Is market data delayed
    pub is_delayed: bool,

    /// Is security halted
    pub is_halted: bool,

    /// Is security hard to borrow
    pub is_hard_to_borrow: bool,
}
