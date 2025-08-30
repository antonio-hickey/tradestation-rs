use crate::{
    responses::{
        ApiResponse,
        MarketData::{GetBarsResp, GetBarsRespRaw, StreamBarsResp},
    },
    Client, Error,
};
use futures::{Stream, StreamExt};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// Market Data Bars ("candlesticks")
pub struct Bar {
    /// The closing price of the current bar.
    pub close: String,

    /// The number of times a trade was made at a price
    /// less than or equal to the previous trade price.
    pub down_ticks: u64,

    /// The number of shares or contracts traded on down ticks.
    pub down_volume: u64,

    /// The unix epoch time.
    pub epoch: i64,

    /// The highest price traded in the current bar.
    pub high: String,

    /// Conveys that all historical bars in the request have been delivered.
    pub is_end_of_history: bool,

    /// Set when there is data in the bar and the data
    /// is being built in "real time" from a trade.
    pub is_real_time: Option<bool>,

    /// The lowest price traded in the current bar.
    pub low: String,

    /// The opening price of the current bar.
    pub open: String,

    /// The number of open contracts.
    ///
    /// NOTE: Futures and Options ONLY.
    pub open_interest: Option<String>,

    /// Timestamp represented as an RFC3339 formatted date.
    /// E.g: `"2024-09-01T23:30:30Z"`
    pub time_stamp: String,

    /// The total number of ticks (upticks and downticks together).
    pub total_ticks: u64,

    /// The sum of up and down volume.
    pub total_volume: String,

    /// The number of times a trade was made at the same price
    /// of the previous trade price.
    ///
    /// DEPRECATED: it's value will always be 0
    pub unchanged_ticks: u8,

    /// The number of shares or contracts traded on unchanged ticks.
    ///
    /// DEPRECATED: it's value will always be 0
    pub unchanged_volume: u8,

    /// The number of times a trade was made at a price
    /// greater than or equal to the previous trade price.
    pub up_ticks: u64,

    /// The number of shares or contracts traded on up ticks.
    pub up_volume: u64,

    /// Indicates if the current `Bar` is Open or Closed.
    pub bar_status: BarStatus,
}
impl Bar {
    /// Fetch `Vec<Bar>` for a given query `GetBarsQuery`.
    ///
    /// # Example
    /// ---
    ///
    /// Get the 10 most recent 5 minute bars of trading
    /// activity for November 2024 Crude Oil Futures.
    /// ```ignore
    /// let get_bars_query = MarketData::GetBarsQueryBuilder::new()
    ///     .symbol("CLX24")
    ///     .unit(BarUnit::Minute)
    ///     .interval("5")
    ///     .bars_back("10")
    ///     .build()?;
    ///
    /// let bars = client.get_bars(&get_bars_query).await?;
    ///
    /// // Do something with the bars, maybe make a chart?
    /// println!("{bars:?}");
    /// ```
    pub async fn fetch(query: &GetBarsQuery, client: &Client) -> Result<Vec<Bar>, Error> {
        let endpoint = format!(
            "marketdata/barcharts/{}{}",
            query.symbol,
            query.as_query_string()
        );

        match client
            .get(&endpoint)
            .await?
            .json::<ApiResponse<GetBarsRespRaw>>()
            .await?
        {
            ApiResponse::Success(resp_raw) => {
                let resp: GetBarsResp = resp_raw.clone().into();

                if let Some(bars) = resp.bars {
                    Ok(bars)
                } else {
                    Err(resp.error.unwrap_or(Error::UnknownTradeStationAPIError(
                        resp_raw.message.unwrap_or_default(),
                    )))
                }
            }
            ApiResponse::Error(resp) => Err(Error::from_api_error(resp)),
        }
    }

    /// Stream bars of market activity for a given query `GetBarsQuery`.
    ///
    /// <div class="warning">WARNING: There's a max of 10 concurrent streams allowed.</div>
    ///
    /// NOTE: You must pin the stream before polling it.
    ///
    /// # Example
    /// ---
    /// Stream bars of November 2030 Crude Oil Futures trading activity
    /// in 4 hour (240 minute) intervals.
    /// ```ignore
    /// let stream_bars_query = MarketData::StreamBarsQueryBuilder::new()
    ///     .symbol("CLX30")
    ///     .unit(BarUnit::Minute)
    ///     .interval(240)
    ///     .build()?;
    ///
    /// // Start streaming bars of trading activity
    /// let bars_stream = Bar::stream(&client, &stream_bars_query);
    /// tokio::pin!(bars_stream); // NOTE: You must pin the stream to the stack
    /// while let Some(stream_resp) = bars_stream.next().await {
    ///     // The response type is `responses::market_data::StreamBarsResp`
    ///     // which has multiple variants the main one you care about is
    ///     // `Bar` which will contain order data sent from the stream.
    ///     match stream_resp {
    ///         Ok(StreamBarsResp::Bar(bar)) => {
    ///             // Do something with the bars like making a chart
    ///             println!("{bar:?}")
    ///         }
    ///         Ok(StreamBarsResp::Heartbeat(heartbeat)) => {
    ///             // Response for periodic signals letting you know the connection is
    ///             // still alive. A heartbeat is sent every 5 seconds of inactivity.
    ///             println!("{heartbeat:?}");
    ///
    ///             // for the sake of this example after we recieve the
    ///             // tenth heartbeat, we will stop the stream session.
    ///             if heartbeat.heartbeat > 10 {
    ///                 // Example: stopping a stream connection
    ///                 return Err(Error::StopStream);
    ///             }
    ///         }
    ///         Ok(StreamBarsResp::Status(status)) => {
    ///             // Signal sent on state changes in the stream
    ///             // (closed, opened, paused, resumed)
    ///             println!("{status:?}");
    ///         }
    ///         Ok(StreamBarsResp::Error(err)) => {
    ///             // Response for when an error was encountered,
    ///             // with details on the error
    ///             eprintln!("{err:?}");
    ///         }
    ///         Err(err) => {
    ///             // Stream / Network error
    ///             eprintln!("{err:?}");
    ///         }
    ///     }
    /// }
    /// ```
    pub fn stream<'a>(
        client: &'a Client,
        query: &'a StreamBarsQuery,
    ) -> impl Stream<Item = Result<StreamBarsResp, Error>> + 'a {
        let endpoint = format!(
            "marketdata/stream/barcharts/{}{}",
            query.symbol,
            query.as_query_string()
        );

        client.stream(endpoint).filter_map(|chunk| async {
            match chunk {
                Ok(value) => match serde_json::from_value::<StreamBarsResp>(value) {
                    Ok(resp) => Some(Ok(resp)),
                    Err(e) => Some(Err(Error::Json(e))),
                },
                Err(e) => Some(Err(e)),
            }
        })
    }

    /// Streams [`Bar`]'s for a given symbol.
    ///
    /// This method builds a stream connection from the provided [`GetBarsQuery`]
    /// and continuously passes incoming stream events ([`StreamBarsResp`]) to
    /// the provided `callback` closure for processing.
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
    /// - deserialization of a stream event into [`StreamBarsResp`] fails,
    /// - or the `callback` returns an error other than [`Error::StopStream`].
    ///
    /// # Examples
    ///
    /// Receive 5 bars from the stream and then stop streaming:
    /// ```rust,no_run
    /// # use tradestation::{Error, Client, market_data::{Bar, StreamBarsQuery}};
    /// # async fn example(client: &Client, query: &StreamBarsQuery) -> Result<(), Error> {
    /// let mut count = 0;
    ///
    /// Bar::stream_into(client, query, |bar| {
    ///     println!("bar {count}: {:?}", bar);
    ///     count += 1;
    ///
    ///     if count >= 5 {
    ///         // Gracefully stop the stream
    ///         return Err(Error::StopStream);
    ///     }
    ///
    ///     Ok(())
    /// }).await?;
    /// # Ok(()) }
    /// ```
    pub async fn stream_into(
        client: &Client,
        query: &StreamBarsQuery,
        mut callback: impl FnMut(StreamBarsResp) -> Result<(), Error>,
    ) -> Result<(), Error> {
        let endpoint = format!(
            "marketdata/stream/barcharts/{}{}",
            query.symbol,
            query.as_query_string()
        );

        client
            .stream_into(&endpoint, |stream_event| {
                let parsed_event: StreamBarsResp = serde_json::from_value(stream_event)?;
                callback(parsed_event)
            })
            .await
            .or_else(|e| match e {
                Error::StopStream => Ok(()),
                other => Err(other),
            })
    }
}
impl Client {
    /// Fetch `Vec<Bar>` for a given query `GetBarsQuery`.
    pub async fn get_bars(&self, query: &GetBarsQuery) -> Result<Vec<Bar>, Error> {
        Bar::fetch(query, self).await
    }

    /// Stream bars of market activity for a given query `StreamBarsQuery`.
    ///
    /// <div class="warning">WARNING: There's a max of 10 concurrent streams allowed.</div>
    ///
    /// NOTE: You must pin the stream before polling it.
    ///
    /// # Example
    /// ---
    /// Stream bars of November 2030 Crude Oil Futures trading activity
    /// in 4 hour (240 minute) intervals.
    /// ```ignore
    /// let stream_bars_query = MarketData::StreamBarsQueryBuilder::new()
    ///     .symbol("CLX30")
    ///     .unit(BarUnit::Minute)
    ///     .interval(240)
    ///     .build()?;
    ///
    /// // Start streaming bars of trading activity
    /// let bars_stream = client.stream_bars(&stream_bars_query);
    /// tokio::pin!(bars_stream); // NOTE: You must pin the stream to the stack
    /// while let Some(stream_resp) = bars_stream.next().await {
    ///     // The response type is `responses::market_data::StreamBarsResp`
    ///     // which has multiple variants the main one you care about is
    ///     // `Bar` which will contain order data sent from the stream.
    ///     match stream_resp {
    ///         Ok(StreamBarsResp::Bar(bar)) => {
    ///             // Do something with the bars like making a chart
    ///             println!("{bar:?}")
    ///         }
    ///         Ok(StreamBarsResp::Heartbeat(heartbeat)) => {
    ///             // Response for periodic signals letting you know the connection is
    ///             // still alive. A heartbeat is sent every 5 seconds of inactivity.
    ///             println!("{heartbeat:?}");
    ///
    ///             // for the sake of this example after we recieve the
    ///             // tenth heartbeat, we will stop the stream session.
    ///             if heartbeat.heartbeat > 10 {
    ///                 // Example: stopping a stream connection
    ///                 return Err(Error::StopStream);
    ///             }
    ///         }
    ///         Ok(StreamBarsResp::Status(status)) => {
    ///             // Signal sent on state changes in the stream
    ///             // (closed, opened, paused, resumed)
    ///             println!("{status:?}");
    ///         }
    ///         Ok(StreamBarsResp::Error(err)) => {
    ///             // Response for when an error was encountered,
    ///             // with details on the error
    ///             eprintln!("{err:?}");
    ///         }
    ///         Err(err) => {
    ///             // Stream / Network error
    ///             eprintln!("{err:?}");
    ///         }
    ///     }
    /// }
    /// ```
    pub fn stream_bars<'a>(
        &'a self,
        query: &'a StreamBarsQuery,
    ) -> impl Stream<Item = Result<StreamBarsResp, Error>> + 'a {
        Bar::stream(self, query)
    }

    /// Streams [`Bar`]'s for a given symbol.
    ///
    /// This method builds a stream connection from the provided [`GetBarsQuery`]
    /// and continuously passes incoming stream events ([`StreamBarsResp`]) to
    /// the provided `callback` closure for processing.
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
    /// - deserialization of a stream event into [`StreamBarsResp`] fails,
    /// - or the `callback` returns an error other than [`Error::StopStream`].
    ///
    /// # Examples
    ///
    /// Receive 5 bars from the stream and then stop streaming:
    /// ```rust,no_run
    /// # use tradestation::{Error, Client, market_data::{Bar, StreamBarsQuery}};
    /// # async fn example(client: &Client, query: &StreamBarsQuery) -> Result<(), Error> {
    /// let mut count = 0;
    ///
    /// client.stream_bars_into(query, |bar| {
    ///     println!("bar {count}: {:?}", bar);
    ///     count += 1;
    ///
    ///     if count >= 5 {
    ///         // Gracefully stop the stream
    ///         return Err(Error::StopStream);
    ///     }
    ///
    ///     Ok(())
    /// }).await?;
    /// # Ok(()) }
    /// ```
    pub async fn stream_bars_into(
        &self,
        query: &StreamBarsQuery,
        callback: impl FnMut(StreamBarsResp) -> Result<(), Error>,
    ) -> Result<(), Error> {
        Bar::stream_into(self, query, callback).await
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// A query to fetch [`Bar`]'s of market data.
pub struct GetBarsQuery {
    /// The symbol of the security you want bars for.
    ///
    /// E.g: `"SR3Z24"` for bars on Three Month SOFR Futures December 2024 Contract.
    /// or
    /// E.g: `"PLTR"` for bars on the stock Palantir.
    pub symbol: String,

    /// The interval (of time units) that each bar will consist of
    ///
    /// NOTE: Always defaults to 1, and if using the unit `BarUnit::Minute`
    /// then the max allowed interval is 1440.
    ///
    /// E.g: If unit is set to `BarUnit::Minute` than an interval of 5
    /// would mean each `Bar` is a 5 minute aggregation of market data.
    pub interval: i16,

    /// The unit of measurement for time in each bar interval.
    pub unit: BarUnit,

    /// Number of bars back to fetch.
    ///
    /// NOTE: Always defaults to 1, and the max number of intraday bars back
    /// is 57,600. There is no limit on `BarUnit::Daily`, `BarUnit::Weekly`,
    /// or `BarUnit::Monthly` unit.
    ///
    /// NOTE: This parameter is mutually exclusive with the `first_date` parameter.
    pub bars_back: Option<u32>,

    /// The first date formatted as `"YYYY-MM-DD"`, or `"2020-04-20T18:00:00Z"`.
    ///
    /// NOTE: This parameter is mutually exclusive with the `bars_back` parameter.
    pub first_date: Option<String>,

    /// The last date formatted as `"YYYY-MM-DD"`, or `"2020-04-20T18:00:00Z"`.
    ///
    /// NOTE: Defaults to the current timestamp.
    ///
    /// NOTE: This parameter is mutually exclusive with the `start_date` parameter
    /// and should be used instead of that parameter, since startdate is deprecated.
    pub last_date: Option<String>,

    /// The United States (US) stock market session template.
    ///
    /// NOTE: Ignored for non U.S equity symbols.
    pub session_template: SessionTemplate,

    /// DEPRECATED: Use `last_date` instead of `start_date` !
    ///
    /// The last date formatted as `"YYYY-MM-DD"`, or `"2020-04-20T18:00:00Z"`.
    pub start_date: Option<String>,
}
impl GetBarsQuery {
    /// Convert the `GetBarsQuery` as a query param string.
    pub fn as_query_string(&self) -> String {
        let mut query_string = String::from("?");

        query_string.push_str(&format!("interval={}&", self.interval));
        query_string.push_str(&format!("unit={:?}&", self.unit));
        if let Some(bars_back) = self.bars_back {
            query_string.push_str(&format!("barsBack={bars_back}&"));
        }
        if let Some(date) = &self.first_date {
            query_string.push_str(&format!("firstDate={date}&"));
        }
        if let Some(date) = &self.last_date {
            query_string.push_str(&format!("lastDate={date}&"));
        }
        query_string.push_str(&format!("sessionTemplate={:?}&", self.session_template));
        if let Some(date) = &self.start_date {
            query_string.push_str(&format!("startDate={date}&"));
        }

        if query_string.ends_with('&') {
            query_string.pop();
        }

        query_string
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// A query to stream [`Bar`]'s of market data.
pub struct StreamBarsQuery {
    /// The symbol of the security you want bars for.
    ///
    /// E.g: `"SR3Z24"` for bars on Three Month SOFR Futures December 2024 Contract.
    /// or
    /// E.g: `"PLTR"` for bars on the stock Palantir.
    pub symbol: String,

    /// The interval (of time units) that each bar will consist of
    ///
    /// NOTE: Always defaults to 1, and if using the unit `BarUnit::Minute`
    /// then the max allowed interval is 1440.
    ///
    /// E.g: If unit is set to `BarUnit::Minute` than an interval of 5
    /// would mean each `Bar` is a 5 minute aggregation of market data.
    pub interval: i16,

    /// The unit of measurement for time in each bar interval.
    pub unit: BarUnit,

    /// Number of bars back to fetch.
    ///
    /// NOTE: Always defaults to 1, and the max number of intraday bars back
    /// is 57,600. There is no limit on `BarUnit::Daily`, `BarUnit::Weekly`,
    /// or `BarUnit::Monthly` unit.
    ///
    /// NOTE: This parameter is mutually exclusive with the `first_date` parameter.
    pub bars_back: u32,

    /// The United States (US) stock market session template.
    ///
    /// NOTE: Ignored for non U.S equity symbols.
    pub session_template: SessionTemplate,
}
impl StreamBarsQuery {
    pub fn as_query_string(&self) -> String {
        let mut query_string = String::from("?");

        query_string.push_str(&format!("interval={}&", self.interval));
        query_string.push_str(&format!("unit={:?}&", self.unit));
        query_string.push_str(&format!("barsBack={}&", self.bars_back));
        query_string.push_str(&format!("sessionTemplate={:?}&", self.session_template));

        if query_string.ends_with('&') {
            query_string.pop();
        }

        query_string
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
/// The different types of Market Session Templates.
pub enum SessionTemplate {
    /// U.S Equities Pre Market Session Template
    USEQPre,

    /// U.S Equities Post Market Session Template
    USEQPost,

    /// U.S Equities Pre And Post Market Session Template
    USEQPreAndPost,

    /// U.S Equities 24 Hour Session Template
    USEQ24Hour,

    /// U.S Equities Normal Market Session Template
    Default,
}
impl FromStr for SessionTemplate {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "useqpre" => Ok(SessionTemplate::USEQPre),
            "useqpost" => Ok(SessionTemplate::USEQPost),
            "useqpreandpost" => Ok(SessionTemplate::USEQPreAndPost),
            "useq24hour" => Ok(SessionTemplate::USEQ24Hour),
            "default" => Ok(SessionTemplate::Default),

            _ => Err(String::from("Invalid value for SessionTemplate: {s}")),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
/// The types of unit of measurement for time in each bar interval.
pub enum BarUnit {
    /// Minute Bars
    Minute,

    /// Daily Bars
    Daily,

    /// Weekly Bars
    Weekly,

    /// Monthly Bars
    Monthly,
}
impl FromStr for BarUnit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "minute" => Ok(BarUnit::Minute),
            "daily" => Ok(BarUnit::Daily),
            "weekly" => Ok(BarUnit::Weekly),
            "monthly" => Ok(BarUnit::Monthly),

            _ => Err("Invalid value for BarUnit: {s}".into()),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
/// The 2 types of state a `Bar` can be in, open or closed.
pub enum BarStatus {
    /// Indicates the `Bar` is still trading.
    Open,

    /// Indicates the `Bar` is finished trading.
    Closed,
}

#[derive(Debug, Default)]
/// Builder pattern struct for [`GetBarsQuery`].
pub struct GetBarsQueryBuilder {
    /// The symbol of the security you want bars for.
    ///
    /// E.g: `"SR3Z24"` for bars on Three Month SOFR Futures December 2024 Contract.
    /// or
    /// E.g: `"PLTR"` for bars on the stock Palantir.
    symbol: Option<String>,

    /// The interval (of time units) that each bar will consist of
    ///
    /// NOTE: Always defaults to 1, and if using the unit `BarUnit::Minute`
    /// then the max allowed interval is 1440.
    ///
    /// E.g: If unit is set to `BarUnit::Minute` than an interval of 5
    /// would mean each `Bar` is a 5 minute aggregation of market data.
    interval: Option<i16>,

    /// The unit of measurement for time in each bar interval.
    unit: Option<BarUnit>,

    /// Number of bars back to fetch.
    ///
    /// NOTE: Always defaults to 1, and the max number of intraday bars back
    /// is 57,600. There is no limit on `BarUnit::Daily`, `BarUnit::Weekly`,
    /// or `BarUnit::Monthly` unit.
    ///
    /// NOTE: This parameter is mutually exclusive with the `first_date` parameter.
    bars_back: Option<u32>,

    /// The first date formatted as `"YYYY-MM-DD"`, or `"2020-04-20T18:00:00Z"`.
    ///
    /// NOTE: This parameter is mutually exclusive with the `bars_back` parameter.
    first_date: Option<String>,

    /// The last date formatted as `"YYYY-MM-DD"`, or `"2020-04-20T18:00:00Z"`.
    ///
    /// NOTE: Defaults to the current timestamp.
    ///
    /// NOTE: This parameter is mutually exclusive with the `start_date` parameter
    /// and should be used instead of that parameter, since startdate is deprecated.
    last_date: Option<String>,

    /// The United States (US) stock market session template.
    ///
    /// NOTE: Ignored for non U.S equity symbols.
    session_template: Option<SessionTemplate>,

    /// DEPRECATED: Use `last_date` instead of `start_date` !
    ///
    /// The last date formatted as `"YYYY-MM-DD"`, or `"2020-04-20T18:00:00Z"`.
    start_date: Option<String>,
}
impl GetBarsQueryBuilder {
    /// Initialize a new builder for `GetBarsQuery`.
    pub fn new() -> Self {
        GetBarsQueryBuilder {
            symbol: None,
            interval: None,
            unit: None,
            bars_back: None,
            first_date: None,
            last_date: None,
            session_template: None,
            start_date: None,
        }
    }

    /// Set the symbol of the security you want bars for.
    ///
    /// E.g: `"SR3Z24"` for bars on Three Month SOFR Futures December 2024 Contract.
    /// or
    /// E.g: `"PLTR"` for bars on the stock Palantir.
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        self
    }

    /// Set the interval (of time units) that each bar will consist of
    ///
    /// NOTE: Always defaults to 1, and if using the unit `BarUnit::Minute`
    /// then the max allowed interval is 1440.
    ///
    /// E.g: If unit is set to `BarUnit::Minute` than an interval of 5
    /// would mean each `Bar` is a 5 minute aggregation of market data.
    pub fn interval(mut self, interval: i16) -> Self {
        self.interval = Some(interval);
        self
    }

    /// Set the unit of measurement for time in each bar interval.
    pub fn unit(mut self, unit: BarUnit) -> Self {
        self.unit = Some(unit);
        self
    }

    /// Set the number of bars back to fetch.
    ///
    /// NOTE: Always defaults to 1, and the max number of intraday bars back
    /// is 57,600. There is no limit on `BarUnit::Daily`, `BarUnit::Weekly`,
    /// or `BarUnit::Monthly` unit.
    ///
    /// NOTE: This parameter is mutually exclusive with the `first_date` parameter.
    pub fn bars_back(mut self, bars_back: u32) -> Self {
        self.bars_back = Some(bars_back);
        self
    }

    /// Fetch the maximum (57,600) bars back to fetch.
    ///
    /// NOTE: This parameter is mutually exclusive with the `first_date` and `bars_back`
    /// parameters. If using this method `max_bars_back` then you should not use either
    /// `first_date` or `bars_back` methods as they can overwrite this parameter.
    pub fn max_bars_back(mut self) -> Self {
        self.bars_back = Some(57_600);
        self
    }

    /// Set the first date formatted as `"YYYY-MM-DD"`, or `"2020-04-20T18:00:00Z"`.
    ///
    /// NOTE: This parameter is mutually exclusive with the `bars_back` parameter.
    pub fn first_date(mut self, first_date: impl Into<String>) -> Self {
        self.first_date = Some(first_date.into());
        self
    }

    /// Set the last date formatted as `"YYYY-MM-DD"`, or `"2020-04-20T18:00:00Z"`.
    ///
    /// NOTE: Defaults to the current timestamp.
    ///
    /// NOTE: This parameter is mutually exclusive with the `start_date` parameter
    /// and should be used instead of that parameter, since startdate is deprecated.
    pub fn last_date(mut self, last_date: impl Into<String>) -> Self {
        self.last_date = Some(last_date.into());
        self
    }

    /// Set the United States (US) stock market session template.
    ///
    /// NOTE: Ignored for non U.S equity symbols.
    pub fn session_template(mut self, session_template: SessionTemplate) -> Self {
        self.session_template = Some(session_template);
        self
    }

    /// DEPRECATED: Use `last_date` instead of `start_date` !
    ///
    /// Set the last date formatted as `"YYYY-MM-DD"`, or `"2020-04-20T18:00:00Z"`.
    pub fn start_date(mut self, start_date: impl Into<String>) -> Self {
        self.start_date = Some(start_date.into());
        self
    }

    /// Finish building, returning a `GetBarsQuery`.
    ///
    /// NOTE: You must set `symbol` before calling `build`.
    pub fn build(self) -> Result<GetBarsQuery, Error> {
        Ok(GetBarsQuery {
            symbol: self.symbol.ok_or_else(|| Error::SymbolNotSet)?,
            interval: self.interval.unwrap_or(1),
            unit: self.unit.unwrap_or(BarUnit::Daily),
            bars_back: self.bars_back,
            first_date: self.first_date,
            last_date: self.last_date,
            session_template: self.session_template.unwrap_or(SessionTemplate::Default),
            start_date: self.start_date,
        })
    }
}

#[derive(Debug, Default)]
/// Builder pattern struct for [`StreamBarsQuery`].
pub struct StreamBarsQueryBuilder {
    /// The symbol of the security you want bars for.
    ///
    /// E.g: `"SR3Z24"` for bars on Three Month SOFR Futures December 2024 Contract.
    /// or
    /// E.g: `"PLTR"` for bars on the stock Palantir.
    symbol: Option<String>,

    /// The interval (of time units) that each bar will consist of
    ///
    /// NOTE: Always defaults to 1, and if using the unit `BarUnit::Minute`
    /// then the max allowed interval is 1440.
    ///
    /// E.g: If unit is set to `BarUnit::Minute` than an interval of 5
    /// would mean each `Bar` is a 5 minute aggregation of market data.
    interval: Option<i16>,

    /// The unit of measurement for time in each bar interval.
    unit: Option<BarUnit>,

    /// Number of bars back to fetch.
    ///
    /// NOTE: Always defaults to 1, and the max number of intraday bars back
    /// is 57,600. There is no limit on `BarUnit::Daily`, `BarUnit::Weekly`,
    /// or `BarUnit::Monthly` unit.
    ///
    /// NOTE: This parameter is mutually exclusive with the `first_date` parameter.
    bars_back: Option<u32>,

    /// The United States (US) stock market session template.
    ///
    /// NOTE: Ignored for non U.S equity symbols.
    session_template: Option<SessionTemplate>,
}
impl StreamBarsQueryBuilder {
    /// Initialize a new builder for `GetBarsQuery`.
    pub fn new() -> Self {
        StreamBarsQueryBuilder {
            symbol: None,
            interval: None,
            unit: None,
            bars_back: None,
            session_template: None,
        }
    }

    /// Set the symbol of the security you want bars for.
    ///
    /// E.g: `"SR3Z24"` for bars on Three Month SOFR Futures December 2024 Contract.
    /// or
    /// E.g: `"PLTR"` for bars on the stock Palantir.
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        self
    }

    /// Set the interval (of time units) that each bar will consist of
    ///
    /// NOTE: Always defaults to 1, and if using the unit `BarUnit::Minute`
    /// then the max allowed interval is 1440.
    ///
    /// E.g: If unit is set to `BarUnit::Minute` than an interval of 5
    /// would mean each `Bar` is a 5 minute aggregation of market data.
    pub fn interval(mut self, interval: i16) -> Self {
        self.interval = Some(interval);
        self
    }

    /// Set the unit of measurement for time in each bar interval.
    pub fn unit(mut self, unit: BarUnit) -> Self {
        self.unit = Some(unit);
        self
    }

    /// Set the number of bars back to fetch.
    ///
    /// NOTE: Always defaults to 1, and the max number of intraday bars back
    /// is 57,600. There is no limit on `BarUnit::Daily`, `BarUnit::Weekly`,
    /// or `BarUnit::Monthly` unit.
    ///
    /// NOTE: This parameter is mutually exclusive with the `max_bars_back` parameter.
    /// If using this method `bars_back` then you should not use the `max_bars_back`
    /// method as it can overwrite this parameter.
    pub fn bars_back(mut self, bars_back: u32) -> Self {
        self.bars_back = Some(bars_back);
        self
    }

    /// Fetch the maximum (57,600) bars back to fetch with the stream.
    ///
    /// NOTE: This parameter is mutually exclusive with the `bars_back` parameter.
    /// If using this method `max_bars_back` then you should not use the `bars_back`
    /// method as it can overwrite this parameter.
    pub fn max_bars_back(mut self) -> Self {
        self.bars_back = Some(57_600);
        self
    }

    /// Set the United States (US) stock market session template.
    ///
    /// NOTE: Ignored for non U.S equity symbols.
    pub fn session_template(mut self, session_template: SessionTemplate) -> Self {
        self.session_template = Some(session_template);
        self
    }

    /// Finish building, returning a `StreamBarsQuery`.
    ///
    /// NOTE: You must set `symbol` before calling `build`.
    pub fn build(self) -> Result<StreamBarsQuery, Error> {
        Ok(StreamBarsQuery {
            symbol: self.symbol.ok_or_else(|| Error::SymbolNotSet)?,
            interval: self.interval.unwrap_or(1),
            unit: self.unit.unwrap_or(BarUnit::Daily),
            bars_back: self.bars_back.unwrap_or(1),
            session_template: self.session_template.unwrap_or(SessionTemplate::Default),
        })
    }
}
