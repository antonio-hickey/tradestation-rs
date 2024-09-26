use crate::responses::market_data::GetBarsResp;
use crate::{responses::market_data as responses, Client, Error};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// Market Data Bars (candlestick bars)
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
    ///
    /// Example: 2024-09-01T23:30:30Z
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
    /// Fetch `Vec<Bar>` for a given query `GetBarsQuery`
    ///
    /// Example: Get the 10 most recent 5 minute bars of trading
    /// activity for November 2024 Crude Oil Futures.
    /// ```rust
    /// let fetch_bars_query = MarketData::GetBarsQueryBuilder::new()
    ///     .set_symbol("CLX24")
    ///     .set_unit(BarUnit::Minute)
    ///     .set_interval("5")
    ///     .set_bars_back("10")
    ///     .build()?;
    ///
    /// let bars = client.fetch_bars(&fetch_bars_query).await?;
    ///
    /// // Do something with the bars, maybe make a chart?
    /// println!("{bars:?}");
    /// ```
    pub async fn fetch(client: &mut Client, query: &GetBarsQuery) -> Result<Vec<Bar>, Error> {
        let endpoint = format!(
            "marketdata/barcharts/{}{}",
            query.symbol,
            query.as_query_string()
        );

        let resp_raw = client
            .get(&endpoint)
            .await?
            .json::<responses::GetBarsRespRaw>()
            .await?;

        let resp: GetBarsResp = resp_raw.into();

        if let Some(bars) = resp.bars {
            Ok(bars)
        } else {
            Err(resp.error.unwrap_or(Error::UnknownTradeStationAPIError))
        }
    }
}
impl Client {
    /// Fetch `Vec<Bar>` for a given query `GetBarsQuery`
    pub async fn fetch_bars(&mut self, query: &GetBarsQuery) -> Result<Vec<Bar>, Error> {
        Bar::fetch(self, query).await
    }
}

pub struct GetBarsQuery {
    /// The symbol of the security you want bars for.
    ///
    /// Example: `"SR3Z24"` for bars on Three Month SOFR Futures December 2024 Contract.
    /// or
    /// Example: `"PLTR"` for bars on the stock Palantir.
    pub symbol: String,
    /// The interval (of time units) that each bar will consist of
    ///
    /// NOTE: Always defaults to 1, and if using the unit `BarUnit::Minute`
    /// then the max allowed interval is 1440.
    ///
    /// Example: If unit is set to `BarUnit::Minute` than an interval of 5
    /// would mean each `Bar` is a 5 minute aggregation of market data.
    pub interval: String,
    /// The unit of measurement for time in each bar interval.
    pub unit: BarUnit,
    /// Number of bars back to fetch.
    ///
    /// NOTE: Always defaults to 1, and the max number of intraday bars back
    /// is 57,600. There is no limit on `BarUnit::Daily`, `BarUnit::Weekly`,
    /// or `BarUnit::Monthly` unit.
    ///
    /// NOTE: This parameter is mutually exclusive with the `first_date` parameter.
    pub bars_back: String,
    /// The first date formatted as `"YYYY-MM-DD"`, or `"2020-04-20T18:00:00Z"`.
    ///
    /// NOTE: This parameter is mutually exclusive with the `bars_back` parameter.
    pub first_date: String,
    /// The last date formatted as `"YYYY-MM-DD"`, or `"2020-04-20T18:00:00Z"`.
    ///
    /// NOTE: Defaults to the current timestamp.
    ///
    /// NOTE: This parameter is mutually exclusive with the `start_date` parameter
    /// and should be used instead of that parameter, since startdate is deprecated.
    pub last_date: String,
    /// The United States (US) stock market session template.
    ///
    /// NOTE: Ignored for non U.S equity symbols.
    pub session_template: SessionTemplate,
    /// DEPRECATED: Use `last_date` instead of `start_date` !
    ///
    /// The last date formatted as `"YYYY-MM-DD"`, or `"2020-04-20T18:00:00Z"`.
    pub start_date: String,
}
impl GetBarsQuery {
    pub fn as_query_string(&self) -> String {
        let mut query_string = String::from("?");

        query_string.push_str(&format!("interval={}&", self.interval));
        query_string.push_str(&format!("unit={:?}&", self.unit));
        if !self.bars_back.is_empty() {
            query_string.push_str(&format!("barsBack={}&", self.bars_back));
        }
        if !self.first_date.is_empty() {
            query_string.push_str(&format!("firstDate={}&", self.first_date));
        }
        if !self.last_date.is_empty() {
            query_string.push_str(&format!("lastDate={}&", self.last_date));
        }
        query_string.push_str(&format!("sessionTemplate={:?}&", self.session_template));
        if !self.start_date.is_empty() {
            query_string.push_str(&format!("startDate={}&", self.start_date));
        }

        if query_string.ends_with('&') {
            query_string.pop();
        }

        query_string
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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

#[derive(Clone, Debug, Deserialize, Serialize)]
/// The 2 types of state a `Bar` can be in, open or closed.
pub enum BarStatus {
    /// Indicates the `Bar` is still trading.
    Open,
    /// Indicates the `Bar` is finished trading.
    Closed,
}

#[derive(Debug, Default)]
/// Builder pattern struct for `GetBarsQuery`.
pub struct GetBarsQueryBuilder {
    /// The symbol of the security you want bars for.
    ///
    /// Example: `"SR3Z24"` for bars on Three Month SOFR Futures December 2024 Contract.
    /// or
    /// Example: `"PLTR"` for bars on the stock Palantir.
    symbol: Option<String>,
    /// The interval (of time units) that each bar will consist of
    ///
    /// NOTE: Always defaults to 1, and if using the unit `BarUnit::Minute`
    /// then the max allowed interval is 1440.
    ///
    /// Example: If unit is set to `BarUnit::Minute` than an interval of 5
    /// would mean each `Bar` is a 5 minute aggregation of market data.
    interval: Option<String>,
    /// The unit of measurement for time in each bar interval.
    unit: Option<BarUnit>,
    /// Number of bars back to fetch.
    ///
    /// NOTE: Always defaults to 1, and the max number of intraday bars back
    /// is 57,600. There is no limit on `BarUnit::Daily`, `BarUnit::Weekly`,
    /// or `BarUnit::Monthly` unit.
    ///
    /// NOTE: This parameter is mutually exclusive with the `first_date` parameter.
    bars_back: Option<String>,
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
    /// Example: `"SR3Z24"` for bars on Three Month SOFR Futures December 2024 Contract.
    /// or
    /// Example: `"PLTR"` for bars on the stock Palantir.
    pub fn set_symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        self
    }

    /// Set the interval (of time units) that each bar will consist of
    ///
    /// NOTE: Always defaults to 1, and if using the unit `BarUnit::Minute`
    /// then the max allowed interval is 1440.
    ///
    /// Example: If unit is set to `BarUnit::Minute` than an interval of 5
    /// would mean each `Bar` is a 5 minute aggregation of market data.
    pub fn set_interval(mut self, interval: impl Into<String>) -> Self {
        self.interval = Some(interval.into());
        self
    }

    /// Set the unit of measurement for time in each bar interval.
    pub fn set_unit(mut self, unit: BarUnit) -> Self {
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
    pub fn set_bars_back(mut self, bars_back: impl Into<String>) -> Self {
        self.bars_back = Some(bars_back.into());
        self
    }

    /// Set the first date formatted as `"YYYY-MM-DD"`, or `"2020-04-20T18:00:00Z"`.
    ///
    /// NOTE: This parameter is mutually exclusive with the `bars_back` parameter.
    pub fn set_first_date(mut self, first_date: impl Into<String>) -> Self {
        self.first_date = Some(first_date.into());
        self
    }

    /// Set the last date formatted as `"YYYY-MM-DD"`, or `"2020-04-20T18:00:00Z"`.
    ///
    /// NOTE: Defaults to the current timestamp.
    ///
    /// NOTE: This parameter is mutually exclusive with the `start_date` parameter
    /// and should be used instead of that parameter, since startdate is deprecated.
    pub fn set_last_date(mut self, last_date: impl Into<String>) -> Self {
        self.last_date = Some(last_date.into());
        self
    }

    /// Set the United States (US) stock market session template.
    ///
    /// NOTE: Ignored for non U.S equity symbols.
    pub fn set_session_template(mut self, session_template: SessionTemplate) -> Self {
        self.session_template = Some(session_template);
        self
    }

    /// DEPRECATED: Use `last_date` instead of `start_date` !
    ///
    /// Set the last date formatted as `"YYYY-MM-DD"`, or `"2020-04-20T18:00:00Z"`.
    pub fn set_start_date(mut self, start_date: impl Into<String>) -> Self {
        self.start_date = Some(start_date.into());
        self
    }

    /// Finish building, returning a `GetBarsQuery`.
    ///
    /// NOTE: You must call `set_symbol` before calling `build`.
    pub fn build(self) -> Result<GetBarsQuery, Error> {
        Ok(GetBarsQuery {
            symbol: self.symbol.ok_or_else(|| Error::SymbolNotSet)?,
            interval: self.interval.unwrap_or(String::from("1")),
            unit: self.unit.unwrap_or(BarUnit::Daily),
            bars_back: self.bars_back.unwrap_or(String::from("1")),
            first_date: self.first_date.unwrap_or_default(),
            last_date: self.last_date.unwrap_or_default(),
            session_template: self.session_template.unwrap_or(SessionTemplate::Default),
            start_date: self.start_date.unwrap_or_default(),
        })
    }
}
