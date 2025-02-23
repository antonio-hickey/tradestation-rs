use crate::responses::market_data::{
    GetQuoteSnapshotsResp, GetQuoteSnapshotsRespRaw, StreamQuotesResp,
};
use crate::responses::ApiResponse;
use crate::{Client, Error};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
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
    pub async fn fetch(symbols: Vec<&str>, client: &mut Client) -> Result<Vec<Quote>, Error> {
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
    /// let streamed_quotes = client
    ///     .stream_quotes(vec!["NVDA", "AMD"], |stream_data| {
    ///         // The response type is `responses::MarketData::StreamQuotesResp`
    ///         // which has multiple variants the main one you care about is `Quote`
    ///         // which will contain option chain data sent from the stream.
    ///         match stream_data {
    ///             StreamQuotesResp::Quote(quote) => {
    ///                 // Do something with the quote for example derive
    ///                 // a quote for a long amd short nvidia trade.
    ///                 println!("{quote:?}");
    ///             }
    ///             StreamQuotesResp::Heartbeat(heartbeat) => {
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
    ///             StreamQuotesResp::Status(status) => {
    ///                 // Signal sent on state changes in the stream
    ///                 // (closed, opened, paused, resumed)
    ///                 println!("{status:?}");
    ///             }
    ///             StreamQuotesResp::Error(err) => {
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
    /// // After the stream ends print all the collected quotes
    /// println!("{streamed_quotes:?}");
    /// ```
    pub async fn stream<F>(
        client: &mut Client,
        symbols: Vec<&str>,
        mut on_chunk: F,
    ) -> Result<Vec<Quote>, Error>
    where
        F: FnMut(StreamQuotesResp) -> Result<(), Error>,
    {
        let endpoint = format!("marketdata/stream/quotes/{}", symbols.join(","));

        let mut collected_quotes: Vec<Quote> = Vec::new();
        client
            .stream(&endpoint, |chunk| {
                let parsed_chunk = serde_json::from_value::<StreamQuotesResp>(chunk)?;
                on_chunk(parsed_chunk.clone())?;

                // Only collect orders, so when the stream is done
                // all the orders that were streamed can be returned
                if let StreamQuotesResp::Quote(quote) = parsed_chunk {
                    collected_quotes.push(*quote);
                }

                Ok(())
            })
            .await?;

        Ok(collected_quotes)
    }
}
impl Client {
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
    pub async fn get_quotes(&mut self, symbols: Vec<&str>) -> Result<Vec<Quote>, Error> {
        Quote::fetch(symbols, self).await
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
    /// let streamed_quotes = client
    ///     .stream_quotes(vec!["NVDA", "AMD"], |stream_data| {
    ///         // The response type is `responses::MarketData::StreamQuotesResp`
    ///         // which has multiple variants the main one you care about is `Quote`
    ///         // which will contain option chain data sent from the stream.
    ///         match stream_data {
    ///             StreamQuotesResp::Quote(quote) => {
    ///                 // Do something with the quote for example derive
    ///                 // a quote for a long amd short nvidia trade.
    ///                 println!("{quote:?}");
    ///             }
    ///             StreamQuotesResp::Heartbeat(heartbeat) => {
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
    ///             StreamQuotesResp::Status(status) => {
    ///                 // Signal sent on state changes in the stream
    ///                 // (closed, opened, paused, resumed)
    ///                 println!("{status:?}");
    ///             }
    ///             StreamQuotesResp::Error(err) => {
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
    /// // After the stream ends print all the collected quotes
    /// println!("{streamed_quotes:?}");
    /// ```
    pub async fn stream_quotes<F>(
        &mut self,
        symbols: Vec<&str>,
        on_chunk: F,
    ) -> Result<Vec<Quote>, Error>
    where
        F: FnMut(StreamQuotesResp) -> Result<(), Error>,
    {
        Quote::stream(self, symbols, on_chunk).await
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
