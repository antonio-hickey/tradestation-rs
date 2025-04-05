use crate::{
    accounting::{AssetType, OptionType},
    responses::{
        market_data::{GetSymbolDetailsResp, GetSymbolDetailsRespRaw},
        ApiResponse,
    },
    Client, Error,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// Detailed Information on a sepecifc symbol
pub struct SymbolDetails {
    /// The type of financial instrument that a symbol represents.
    pub asset_type: AssetType,

    /// The country of the exchange where the symbol is listed.
    pub country: String,

    /// Displays the type of base currency for the selected symbol.
    pub currency: String,

    /// Displays the full name of the symbol.
    ///
    /// NOTE: Special characters may be formatted in unicode.
    pub description: String,

    /// Name of exchange where this symbol is traded.
    pub exchange: String,

    /// The UTC formatted expiration date of a future or option symbol,
    /// in the country the contract is traded in. The time portion of
    /// the value should be ignored.
    ///
    /// NOTE: Only for options and futures symbols.
    pub expiration_date: Option<String>,

    /// Displays the type of future contract the symbol represents.
    ///
    /// NOTE: Only for futures symbols.
    pub future_type: Option<String>,

    /// Defines whether an option is a call or a put.
    pub option_type: Option<OptionType>,

    /// Conveys number formatting information for symbol price fields.
    pub price_format: PriceFormat,

    /// Conveys number formatting information for symbol quantity fields.
    pub quantity_format: QuantityFormat,

    /// Displays the symbol root, e.g. `ES` for Futures symbol `@ESH25`,
    /// `OEX` for option `OEX 210129C1750`.
    pub root: String,

    /// The Strike Price for the Put or Call option.
    ///
    /// NOTE: Only for options symbols.
    pub strike_price: Option<String>,

    /// The Symbol name or abbreviation.
    pub symbol: String,

    /// The financial instrument on which an Options contract is
    /// based or derived. Can also apply to some Futures symbols,
    /// like continuous Futures contracts, e.g. `TYZ24` for `@TY`.
    pub underlying: String,
}
impl SymbolDetails {
    /// Fetches symbol details and formatting information for one or more symbols.
    ///
    /// NOTE: Symbols should be a vector of valid symbol string slices.
    /// e.g: `vec!["TLT", "SPY", "ESH25", "@SR3"]`
    ///
    /// # Example
    /// ---
    ///
    /// Get symbol details `MarketData::SymbolDetails` on symbols the nasdaq index `NQQ`,
    /// and Feburary 21st 2025 $105 call option for 20+ Year Treasury fund `TLT 250221C105`.
    ///
    /// ```ignore
    /// let symbols = vec!["NQQ", "TLT 250221C105"];
    /// let details = client.get_symbol_details(symbols).await?;
    /// println!("Symbol Details: {details:?}");
    /// ```
    pub async fn fetch(
        symbols: Vec<&str>,
        client: &mut Client,
    ) -> Result<Vec<SymbolDetails>, Error> {
        let endpoint = format!("marketdata/symbols/{}", symbols.join(","));

        match client
            .get(&endpoint)
            .await?
            .json::<ApiResponse<GetSymbolDetailsRespRaw>>()
            .await?
        {
            ApiResponse::Success(resp_raw) => {
                let err_msg = resp_raw.message.clone().unwrap_or_default();

                let resp: GetSymbolDetailsResp = resp_raw.into();
                if let Some(symbol_details) = resp.symbols {
                    Ok(symbol_details)
                } else {
                    Err(resp
                        .error
                        .unwrap_or(Error::UnknownTradeStationAPIError(err_msg)))
                }
            }
            ApiResponse::Error(resp) => Err(Error::from_api_error(resp)),
        }
    }
}
impl Client {
    /// Fetches symbol details and formatting information for one or more symbols.
    ///
    /// NOTE: Symbols should be a vector of valid symbol string slices.
    /// e.g: `vec!["TLT", "SPY", "ESH25", "@SR3"]`
    ///
    /// # Example
    /// ---
    ///
    /// Get symbol details (`MarketData::SymbolDetails`) on symbols the nasdaq index `NQQ`,
    /// and Feburary 21st 2025 $105 call option for 20+ Year Treasury fund `TLT 250221C105`.
    ///
    /// ```ignore
    /// let symbols = vec!["NQQ", "TLT 250221C105"];
    /// let details = client.get_symbol_details(symbols).await?;
    /// println!("Symbol Details: {details:?}");
    /// ```
    pub async fn get_symbol_details(
        &mut self,
        symbols: Vec<&str>,
    ) -> Result<Vec<SymbolDetails>, Error> {
        SymbolDetails::fetch(symbols, self).await
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// The pricing formatting information for symbol price fields
pub struct PriceFormat {
    /// The format of the price values.
    pub format: Format,

    /// The number of decimals precision.
    ///
    /// NOTE: Only applies to the format `Format::Decimals`.
    pub decimals: Option<String>,

    /// The denominator of the single fraction.
    ///
    /// NOTE: Only applies to the format `Format::Fraction`.
    pub fraction: Option<String>,

    /// The additional fraction of a fraction denominator.
    ///
    /// NOTE: Only applies to the format `Format::Fraction`.
    pub sub_fraction: Option<String>,

    /// The style of increment for price movements.
    pub increment_style: IncrementStyle,

    /// The decimal increment for all price movements.
    ///
    /// NOTE: Only applies to the simple increment style `IncrementStyle::Simple`.
    pub increment: Option<String>,

    /// The scheduling of increments.
    pub increment_schedule: Option<Vec<IncrementSchedule>>,

    /// The symbol's point value.
    pub point_value: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
/// The different types of formats for
/// price and quantity.
pub enum Format {
    /// Decimal values.
    ///
    /// E.g: `123.20`
    Decimal,

    /// Fraction values.
    ///
    /// E.g: `534 4/8`
    ///
    /// NOTE: Common in interest rate derivatives.
    Fraction,

    /// Sub Fractional values.
    ///
    /// E.g: `125'29.7`
    ///
    /// NOTE: Common in interest rate derivatives.
    SubFraction,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
/// The different types of increments
pub enum IncrementStyle {
    /// Simple Increments
    Simple,

    /// Scheduled Increments
    Schedule,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// The schedule of increments
pub struct IncrementSchedule {
    /// The incremental value.
    pub increment: String,

    /// The initial value to start incrementing from.
    pub starts_at: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// The quantity formatting information for symbol quantity fields.
pub struct QuantityFormat {
    /// The format of the quantity.
    ///
    /// NOTE: `QuantityFormat` is always decimal format `Format::Decimal`
    pub format: Format,

    /// The number of decimals precision.
    pub decimals: String,

    /// The incremental style.
    pub increment_style: IncrementStyle,

    /// The decimal increment for all quantity movements.
    ///
    /// NOTE: Only applies to the simple increment style `IncrementStyle::Simple`.
    pub increment: Option<String>,

    /// The scheduling of increments.
    pub increment_schedule: Option<Vec<IncrementSchedule>>,

    /// The minimum quantity of an asset that can be traded.
    pub minimum_trade_quantity: String,
}
