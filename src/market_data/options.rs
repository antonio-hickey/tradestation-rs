use crate::{
    responses::MarketData::GetOptionExpirationsResp,
    responses::MarketData::GetOptionExpirationsRespRaw, Client, Error,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// An option contract's expiration date and type.
pub struct OptionExpiration {
    /// Timestamp represented as an `RFC3339` formatted date, a profile of the ISO 8601 date standard.
    ///
    /// Example: `"2021-12-17T00:00:00Z"`.
    date: String,
    /// The type of expiration for the options contract.
    ///
    /// Example:
    /// ```no_run
    /// OptionExpirationType::Weekly
    /// ```
    r#type: OptionExpirationType,
}
impl OptionExpiration {
    /// Fetch available option contract expiration dates for an underlying symbol.
    ///
    /// NOTE: `underlying_symbol` must be a valid symbol of the underlying asset
    /// a option is derived off of. e.g: "SPY" would be the underlying symbol
    /// for Feburary 28th 2025 $580 strike SPY call options "SPY 250228C580".
    ///
    /// NOTE: `strike_price` is optional, and if provided this will only return
    /// expirations for that strike price.
    ///
    /// Example: Fetch all expirations for Cloudflare (NET) options.
    /// ```no_run
    /// let cloudflare_option_expirations = client.fetch_option_expirations("NET", None).await?;
    /// println!("Cloudflare Option Expirations: {cloudflare_option_expirations:?}");
    /// ```
    pub async fn fetch(
        client: &mut Client,
        underlying_symbol: &str,
        strike_price: Option<f64>,
    ) -> Result<Vec<OptionExpiration>, Error> {
        let mut endpoint = format!("marketdata/symbols/{}", underlying_symbol);
        if let Some(strike) = strike_price {
            let query_param = format!("?strikePrice={}", strike);
            endpoint.push_str(&query_param);
        }

        let resp: GetOptionExpirationsResp = client
            .get(&endpoint)
            .await?
            .json::<GetOptionExpirationsRespRaw>()
            .await?
            .into();

        if let Some(expirations) = resp.expirations {
            Ok(expirations)
        } else {
            Err(resp.error.unwrap_or(Error::UnknownTradeStationAPIError))
        }
    }
}
impl Client {
    /// Fetch available option contract expiration dates for an underlying symbol.
    ///
    /// NOTE: `underlying_symbol` must be a valid symbol of the underlying asset
    /// a option is derived off of. e.g: "SPY" would be the underlying symbol
    /// for Feburary 28th 2025 $580 strike SPY call options "SPY 250228C580".
    ///
    /// NOTE: `strike_price` is optional, and if provided this will only return
    /// expirations for that strike price.
    ///
    /// Example: Fetch all expirations for Cloudflare (NET) options.
    /// ```no_run
    /// let cloudflare_option_expirations = client.fetch_option_expirations("NET", None).await?;
    /// println!("Cloudflare Option Expirations: {cloudflare_option_expirations:?}");
    /// ```
    pub async fn fetch_option_expirations(
        &mut self,
        underlying_symbol: &str,
        strike_price: Option<f64>,
    ) -> Result<Vec<OptionExpiration>, Error> {
        OptionExpiration::fetch(self, underlying_symbol, strike_price).await
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
/// The different types of option expirations.
pub enum OptionExpirationType {
    /// An options contract with weekly based expirations.
    Weekly,
    /// An options contract with monthly based expirations.
    Monthly,
    /// An options contract with quarterly based expirations.
    Quarterly,
    /// An options contract with end of month based expirations.
    EOM,
    /// An options contract with expirations based on other conditions.
    Other,
}
