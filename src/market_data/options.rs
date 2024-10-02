use crate::{
    responses::MarketData::{
        GetOptionExpirationsResp, GetOptionExpirationsRespRaw, GetOptionsRiskRewardResp,
        GetOptionsRiskRewardRespRaw, GetOptionsSpreadTypesResp, GetOptionsSpreadTypesRespRaw,
    },
    Client, Error,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// An option contract's expiration date and type.
pub struct OptionExpiration {
    /// Timestamp represented as an `RFC3339` formatted date, a profile of the ISO 8601 date standard.
    /// E.g: `"2021-12-17T00:00:00Z"`.
    date: String,
    /// The type of expiration for the options contract.
    /// E.g: `OptionExpirationType::Weekly`
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
    /// # Example
    /// ---
    ///
    /// Fetch all expirations for Cloudflare (NET) options.
    ///
    /// ```ignore
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
    /// # Example:
    /// ---
    ///
    /// Fetch all expirations for Cloudflare (NET) options.
    ///
    /// ```ignore
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

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// Information on an options spread.
///
/// e.g: a time spread, buying a long dated call
/// and hedging some of that duration premium
/// by selling a shorter dated call. This would
/// involve a spread with 2 different expirations.
pub struct OptionsSpreadType {
    /// Name of the spread type
    pub name: String,
    /// Conveys if the spread is composed of
    /// multiple strike prices.
    pub strike_interval: bool,
    /// Conveys if the spread is composed of
    /// multiple expirations.
    pub expiration_interval: bool,
}
impl OptionsSpreadType {
    /// Fetch the available spread types for option chains.
    ///
    // TODO: I feel like the HTTP call to the API can be skipped here, it always returns
    // the same Option Spread Types, so can just directly respond with them.
    pub async fn fetch_available(client: &mut Client) -> Result<Vec<OptionsSpreadType>, Error> {
        let resp: GetOptionsSpreadTypesResp = client
            .get("marketdata/options/spreadtypes")
            .await?
            .json::<GetOptionsSpreadTypesRespRaw>()
            .await?
            .into();

        if let Some(spread_types) = resp.spread_types {
            Ok(spread_types)
        } else {
            println!("{resp:?}");
            Err(resp.error.unwrap_or(Error::UnknownTradeStationAPIError))
        }
    }
}
impl Client {
    /// Fetch the available spread types for option chains.
    pub async fn get_option_spread_types(&mut self) -> Result<Vec<OptionsSpreadType>, Error> {
        OptionsSpreadType::fetch_available(self).await
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// An anlaysis on the risk vs reward of an options trade.
///
/// NOTE: Analysis is NOT available for trades with spread types
/// involving multiple expirations, such as Calandar or Diagonal.
pub struct OptionRiskRewardAnalysis {
    /// Indicates whether the maximum gain can be infinite.
    pub max_gain_is_infinite: bool,
    /// The adjusted maximum gain (if it is not infinite).
    pub adjusted_max_gain: String,
    /// Indicates whether the maximum loss can be infinite.
    pub max_loss_is_infinite: bool,
    /// The adjusted maximum loss (if it is not infinite).
    pub adjusted_max_loss: String,
    /// Market price that the underlying security must reach
    /// for the trade to avoid a loss.
    pub breakeven_points: Vec<String>,
}
impl OptionRiskRewardAnalysis {
    /// Run analysis on an options trade given a price and option legs.
    ///
    /// NOTE: All the option legs must be the same symbol, and expiration.
    ///
    /// # Example
    /// ---
    ///
    /// Analyze the risk vs reward of a long volatility
    /// trade for TLT at the November 15th expiration via buying
    /// a call **and** a put at the $99 strike.
    ///
    /// NOTE: The call will make money if TLT makes a big move up, and
    /// the put will make money if TLT makes a big move down. The downside
    /// of this trade comes from stable, slow, or small price movement.
    ///
    /// NOTE: This spread offers unlimited potential profit while defining
    /// a max potential loss.
    ///
    /// ```ignore
    /// use tradestation::{ClientBuilder, Error, MarketData::options::{OptionsLeg, OptionTradeAction}};
    ///
    /// let mut client = ClientBuilder::new()?
    ///     .set_credentials("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")?
    ///     .authorize("YOUR_AUTHORIZATION_CODE")
    ///     .await?
    ///     .build()
    ///     .await?;
    ///
    /// let risk_reward_analysis = client
    ///     .analyze_options_risk_reward(
    ///         4.33,
    ///         vec![
    ///             OptionsLeg {
    ///                 symbol: String::from("TLT 241115C99"),
    ///                 quantity: 5,
    ///                 trade_action: OptionTradeAction::Buy,
    ///             },
    ///             OptionsLeg {
    ///                 symbol: String::from("TLT 241115P99"),
    ///                 quantity: 5,
    ///                 trade_action: OptionTradeAction::Buy,
    ///             },
    ///         ],
    ///     )
    ///     .await?;
    ///
    /// println!(
    ///     "TLT November 15th Long Volatility Via ATM Straddle
    ///      Risk vs Reward Analysis: {risk_reward_analysis:?}"
    /// );
    /// ```
    pub async fn run(
        client: &mut Client,
        price: f64,
        legs: Vec<OptionsLeg>,
    ) -> Result<Self, Error> {
        let payload = json!({"SpreadPrice": price, "Legs": legs});

        let resp: GetOptionsRiskRewardResp = client
            .post("marketdata/options/riskreward", &payload)
            .await?
            .json::<GetOptionsRiskRewardRespRaw>()
            .await?
            .into();

        if let Some(analysis) = resp.analysis {
            Ok(analysis)
        } else {
            Err(resp.error.unwrap_or(Error::UnknownTradeStationAPIError))
        }
    }
}
impl Client {
    /// Run analysis on an options trade given a price and option legs.
    ///
    /// NOTE: All the option legs must be the same symbol, and expiration.
    ///
    /// # Example
    /// ---
    ///
    /// Analyze the risk vs reward of a long volatility
    /// trade for TLT at the November 15th expiration via buying
    /// a call **and** a put at the $99 strike.
    ///
    /// NOTE: The call will make money if TLT makes a big move up, and
    /// the put will make money if TLT makes a big move down. The downside
    /// of this trade comes from stable, slow, or small price movement.
    ///
    /// NOTE: This spread offers unlimited potential profit while defining
    /// a max potential loss.
    ///
    /// ```ignore
    /// use tradestation::{ClientBuilder, Error, MarketData::options::{OptionsLeg, OptionTradeAction}};
    ///
    /// let mut client = ClientBuilder::new()?
    ///     .set_credentials("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")?
    ///     .authorize("YOUR_AUTHORIZATION_CODE")
    ///     .await?
    ///     .build()
    ///     .await?;
    ///
    /// let risk_reward_analysis = client
    ///     .analyze_options_risk_reward(
    ///         4.33,
    ///         vec![
    ///             OptionsLeg {
    ///                 symbol: String::from("TLT 241115C99"),
    ///                 quantity: 5,
    ///                 trade_action: OptionTradeAction::Buy,
    ///             },
    ///             OptionsLeg {
    ///                 symbol: String::from("TLT 241115P99"),
    ///                 quantity: 5,
    ///                 trade_action: OptionTradeAction::Buy,
    ///             },
    ///         ],
    ///     )
    ///     .await?;
    ///
    /// println!(
    ///     "TLT November 15th Long Volatility Via ATM Straddle
    ///      Risk vs Reward Analysis: {risk_reward_analysis:?}"
    /// );
    /// ```
    pub async fn analyze_options_risk_reward(
        &mut self,
        price: f64,
        legs: Vec<OptionsLeg>,
    ) -> Result<OptionRiskRewardAnalysis, Error> {
        OptionRiskRewardAnalysis::run(self, price, legs).await
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// An options position that is apart of a bigger
/// overall options trade.
pub struct OptionsLeg {
    /// Option contract symbol or underlying symbol
    /// to be traded for this leg.
    pub symbol: String,
    /// The number of option contracts to buy or sell for this leg.
    ///
    /// NOTE: The value cannot be zero.
    pub quantity: i32,
    /// The kind of options trade (buying or selling).
    pub trade_action: OptionTradeAction,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
/// The different types of option trade actions.
pub enum OptionTradeAction {
    #[serde(rename = "BUY")]
    /// Buying an option contract
    Buy,
    #[serde(rename = "SELL")]
    /// Selling an option contract
    Sell,
}
