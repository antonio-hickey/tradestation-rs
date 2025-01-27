use crate::{
    responses::{
        market_data::{
            OptionSpreadStrikesResp, OptionSpreadStrikesRespRaw, StreamOptionChainResp,
            StreamOptionQuotesResp,
        },
        MarketData::{
            GetOptionExpirationsResp, GetOptionExpirationsRespRaw, GetOptionsRiskRewardResp,
            GetOptionsRiskRewardRespRaw,
        },
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
    pub date: String,
    /// The type of expiration for the options contract.
    /// E.g: `OptionExpirationType::Weekly`
    pub r#type: OptionExpirationType,
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
    /// let cloudflare_option_expirations = OptionExpiration::fetch("NET", None, &mut client).await?;
    /// println!("Cloudflare Option Expirations: {cloudflare_option_expirations:?}");
    /// ```
    pub async fn fetch(
        underlying_symbol: &str,
        strike_price: Option<f64>,
        client: &mut Client,
    ) -> Result<Vec<OptionExpiration>, Error> {
        let mut endpoint = format!("marketdata/options/expirations/{}", underlying_symbol);
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
    /// let cloudflare_option_expirations = client.get_option_expirations("NET", None).await?;
    /// println!("Cloudflare Option Expirations: {cloudflare_option_expirations:?}");
    /// ```
    pub async fn get_option_expirations(
        &mut self,
        underlying_symbol: &str,
        strike_price: Option<f64>,
    ) -> Result<Vec<OptionExpiration>, Error> {
        OptionExpiration::fetch(underlying_symbol, strike_price, self).await
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
///// Information on an options spread.
/////
///// e.g: a time spread, buying a long dated call
///// and hedging some of that duration premium
///// by selling a shorter dated call. This would
///// involve a spread with 2 different expirations.
pub enum OptionSpreadType {
    /// A single option position, either a call or put.
    ///
    /// <div class="warning">NOTE: Selling this spread type has the potential for infinite loss.</div>
    Single,
    /// A spread involving a call and a put with
    /// the same strike price and expiration date.
    ///
    /// <div class="warning">NOTE: Selling this spread type has the potential for infinite loss.</div>
    Straddle,
    /// A spread involving a call and a put at different
    /// strike prices, but the same expiration date.
    Strangle,
    /// A spread involving buying and selling two options
    /// of the same type (calls or puts) at different strike
    /// prices, but the same expiration date.
    Vertical,
    /// A spread involving buying one option and selling 2
    /// options of the same type as the one bought, but with
    /// different strike prices and the same expiration date.
    RatioBack1x2,
    /// A spread involving buying one option and selling 3
    /// options of the same type as the one bought, but with
    /// different strike prices and the same expiration date.
    ///
    /// NOTE: Similar to `OptionSpreadType::RatioBack1x2`, but
    /// `RatioBack1x3` offers more leverage.
    RatioBack1x3,
    /// A spread involving buying one option and selling 3
    /// options of the same type as the one bought, but with
    /// different strike prices and the same expiration date.
    ///
    /// NOTE: Similar to `OptionSpreadType::RatioBack1x2` and
    /// `OptionSpreadType::RatioBack1x3`, but a more balanced
    /// risk reward.
    RatioBack2x3,
    /// A spread involving buying an option at one strike price,
    /// and selling 2 options of that same option type bought (call or puts).
    ///
    /// NOTE: If using call options, then the 2 options to sell should be
    /// at a higher strike than the call bought. If using put options, then
    /// the 2 put options to sell should be at a lower strike price then the
    /// put bought.
    ///
    /// <div class="warning">NOTE: This spread type has the potential for infinite loss.</div>
    Butterfly,
    /// A spread involving selling an At-The-Money straddle `OptionSpreadType::Straddle`,
    /// and buying an Out-of-The-Money call and put to hedge risk. Where all options are
    /// of the same expiration date.
    IronButterfly,
    /// A spread involving buying an option at one strike price, selling 2 options
    /// at middle price, and buying one option at a higher strike price. Where all the
    /// options are of the same type (call or put), and the same expiration date.
    Condor,
    /// A spread involving selling a call spread and a put spread. Where all options
    /// are of the same expiration date.
    IronCondor,
    /// A spread involving the underlying asset and an option.
    ///
    /// E.g: Covered Call involves buying 100 shares (or one contract if futures options)
    /// of the underlying asset, and selling one call option.
    ///
    /// E.g: Covered Put involves short selling 100 shares (or one contract if futures options)
    /// of the underlying asset, and selling one put option.
    ///
    /// <div class="warning">NOTE: In a Covered Put, there is potential for infinite loss
    /// if the underlying asset's price rises indefinitely.</div>
    Covered,
    /// A spread involving holding a position in the underlying asset, buying a protective
    /// option (opposite of your underlying position, put if long or call if short the
    /// underlying), and selling an option the opposite of the option you bought to reduce
    /// the cost of the protection, but putting a cap on potential reward.
    Collar,
    /// A spread involving buying or selling a mix of option types at a mix of strike prices
    /// and expiration dates.
    ///
    /// <div class="warning">NOTE: Depending on the specific positions, this spread can have the
    /// potential for infinite loss, especially if it includes uncovered short options.</div>
    Combo,
    /// A spread involving buying and selling 2 options of the same type (calls or puts) with
    /// the same strike price, but different expiration dates.
    ///
    /// NOTE: Similar to diagonal spreads `OptionSpreadType::Diagonal`, but with the same
    /// strike prices allowing for a more neutral position.
    Calendar,
    /// A spread involving buying and selling 2 options of the same type (calls or puts), but
    /// with different strike prices, and expiration dates.
    ///
    /// NOTE: Similar to calendar spreads `OptionSpreadType::Calendar`, but with different
    /// strike prices allowing for more directional biases.
    ///
    /// <div class="warning">NOTE: Depending on the strike prices and the extent of coverage
    /// from long options, this spread type can have the potential for unlimited loss.</div>
    Diagonal,
}
impl OptionSpreadType {
    /// Get a vector of all the option spread types.
    ///
    /// # Example
    /// ---
    ///
    /// Get all the spread types and print information about them:
    ///
    /// ```rust
    /// use tradestation::MarketData::OptionSpreadType;
    ///
    /// let option_spread_types = OptionSpreadType::all();
    /// for spread_type in option_spread_types.iter() {
    ///     println!(
    ///         "{spread_type:?} | contains stike interval {} | contains expiration interval: {}",
    ///         spread_type.involves_strike_interval(),
    ///         spread_type.involves_expiration_interval()
    ///     );
    /// }
    /// ```
    pub fn all() -> Vec<OptionSpreadType> {
        vec![
            Self::Single,
            Self::Straddle,
            Self::Strangle,
            Self::Vertical,
            Self::RatioBack1x2,
            Self::RatioBack1x3,
            Self::RatioBack2x3,
            Self::Butterfly,
            Self::IronButterfly,
            Self::Condor,
            Self::IronCondor,
            Self::Covered,
            Self::Collar,
            Self::Combo,
            Self::Calendar,
            Self::Diagonal,
        ]
    }

    /// Does the `OptionSpreadType` involve an interval of strike prices?
    ///
    /// NOTE: This will return false for `OptionSpreadType::Combo` even though
    /// it may consist of multiple strikes, but TradeStations API returns false.
    pub fn involves_strike_interval(&self) -> bool {
        !matches!(
            self,
            Self::Calendar | Self::Combo | Self::Covered | Self::Single | Self::Straddle
        )
    }

    /// Does the `OptionSpreadType` involve an interval of expirations?
    ///
    /// NOTE: This will return false for `OptionSpreadType::Combo` even though
    /// it may consist of multiple expirations, but TradeStations API returns false.
    pub fn involves_expiration_interval(&self) -> bool {
        matches!(self, Self::Calendar | Self::Diagonal)
    }
}
impl Client {
    /// Get a vector of all the option spread types.
    ///
    /// # Example
    /// ---
    ///
    /// Get all the spread types and print information about them:
    ///
    /// ```rust
    /// use tradestation::MarketData::OptionSpreadType;
    ///
    /// let option_spread_types = OptionSpreadType::all();
    /// for spread_type in option_spread_types.iter() {
    ///     println!(
    ///         "{spread_type:?} | contains stike interval {} | contains expiration interval: {}",
    ///         spread_type.involves_strike_interval(),
    ///         spread_type.involves_expiration_interval()
    ///     );
    /// }
    /// ```
    pub fn get_option_spread_types(&mut self) -> Vec<OptionSpreadType> {
        OptionSpreadType::all()
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
    ///     .credentials("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")?
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
        price: f64,
        legs: Vec<OptionsLeg>,
        client: &mut Client,
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
    ///     .credentials("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")?
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
        OptionRiskRewardAnalysis::run(price, legs, self).await
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

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// The available strikes for an options spread.
pub struct OptionSpreadStrikes {
    /// Name of the spread type for these strikes.
    pub spread_type: OptionSpreadType,
    /// Vector of the strike prices for this spread type.
    ///
    /// NOTE: Each element in the Strikes vector is a vector
    /// of strike prices for a single spread.
    pub strikes: Vec<Vec<String>>,
}
impl OptionSpreadStrikes {
    /// Fetch the available strike prices for a spread type and expiration date.
    ///
    /// # Example
    /// ---
    ///
    /// Fetch all the availble strikes for an iron condor on Amazon
    /// `"AMZN"` for the Dec 20th 2024 expiration `"12-20-2024"`.
    ///
    /// ```ignore
    /// let query = OptionSpreadStrikesQueryBuilder::new()
    ///     .underlying("AMZN")
    ///     .spread_type(OptionSpreadType::IronCondor)
    ///     .expiration("12-20-2024")
    ///     .build()?;
    ///
    /// let availble_strikes = client.get_option_spread_strikes(query).await?;
    ///
    /// println!("Amazon Dec 20th Iron Condor Strikes Availble: {availble_strikes:?}");
    /// ```
    pub async fn fetch(
        query: OptionSpreadStrikesQuery,
        client: &mut Client,
    ) -> Result<OptionSpreadStrikes, Error> {
        let mut endpoint = format!(
            "marketdata/options/strikes/{}?spreadType={:?}&strikeInterval={}",
            query.underlying, query.spread_type, query.strike_interval,
        );

        if let Some(date) = query.expiration {
            let query_param = format!("&expiration={}", date);
            endpoint.push_str(&query_param);

            if let Some(date_2) = query.expiration2 {
                let query_param_2 = format!("&expiration2={}", date_2);
                endpoint.push_str(&query_param_2);
            }
        }

        let resp: OptionSpreadStrikesResp = client
            .get(&endpoint)
            .await?
            .json::<OptionSpreadStrikesRespRaw>()
            .await?
            .into();

        if let Some(spread_strikes) = resp.spread_strikes {
            Ok(spread_strikes)
        } else {
            eprintln!("{:?}", resp.error);
            Err(resp.error.unwrap_or(Error::UnknownTradeStationAPIError))
        }
    }
}
impl Client {
    /// Fetch the available strike prices for a spread type and expiration date.
    ///
    /// # Example
    /// ---
    ///
    /// Fetch all the availble strikes for an iron condor on Amazon
    /// `"AMZN"` for the Dec 20th 2024 expiration `"12-20-2024"`.
    ///
    /// ```ignore
    /// let query = OptionSpreadStrikesQueryBuilder::new()
    ///     .underlying("AMZN")
    ///     .spread_type(OptionSpreadType::IronCondor)
    ///     .expiration("12-20-2024")
    ///     .build()?;
    ///
    /// let availble_strikes = client.get_option_spread_strikes(query).await?;
    ///
    /// println!("Amazon Dec 20th Iron Condor Strikes Availble: {availble_strikes:?}");
    /// ```
    pub async fn get_option_spread_strikes(
        &mut self,
        query: OptionSpreadStrikesQuery,
    ) -> Result<OptionSpreadStrikes, Error> {
        OptionSpreadStrikes::fetch(query, self).await
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// The query required to fetch `OptionSpreadStrikes`.
pub struct OptionSpreadStrikesQuery {
    /// The symbol for the underlying security
    /// on which the option contracts are based.
    ///
    /// NOTE: The underlying symbol must be an equity or index.
    pub underlying: String,
    /// The type of spread `MarketData::OptionSpreadType`
    pub spread_type: OptionSpreadType,
    /// The desired interval between the strike prices
    /// in a spread. It must be greater than or equal to 1.
    /// A value of 1 uses consecutive strikes; a value of 2
    /// skips one between strikes; and so on.
    pub strike_interval: i32,
    /// The date on which the option contract expires;
    /// must be a valid expiration date.
    ///
    /// NOTE: Defaults to the next contract expiration date.
    pub expiration: Option<String>,
    /// The second contract expiration date required
    /// for Calendar and Diagonal spreads.
    pub expiration2: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "PascalCase")]
/// Builder for `OptionSpreadStrikesQuery`
pub struct OptionSpreadStrikesQueryBuilder {
    underlying: Option<String>,
    spread_type: Option<OptionSpreadType>,
    strike_interval: Option<i32>,
    expiration: Option<String>,
    expiration2: Option<String>,
}
impl OptionSpreadStrikesQueryBuilder {
    /// Create a new `OptionSpreadStrikesQueryBuilder`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the symbol for the underlying security
    /// on which the option contracts are based.
    ///
    /// NOTE: The underlying symbol must be an equity or index.
    pub fn underlying<S: Into<String>>(mut self, symbol: S) -> Self {
        self.underlying = Some(symbol.into());

        self
    }

    /// Set the type of spread `MarketData::OptionSpreadType`
    pub fn spread_type(mut self, spread: OptionSpreadType) -> Self {
        self.spread_type = Some(spread);

        self
    }

    /// Set the desired interval between the strike prices in a spread.
    /// It must be greater than or equal to 1. A value of 1 uses consecutive strikes;
    /// a value of 2 skips one between strikes; and so on.
    pub fn strike_interval(mut self, interval: i32) -> Self {
        self.strike_interval = Some(interval);

        self
    }

    /// Set the date on which the option contract expires; must be a valid expiration date.
    ///
    /// NOTE: Defaults to the next contract expiration date.
    pub fn expiration<S: Into<String>>(mut self, date: S) -> Self {
        self.expiration = Some(date.into());

        self
    }

    /// Set the second contract expiration date required
    /// for Calendar and Diagonal spreads.
    pub fn expiration2<S: Into<String>>(mut self, date: S) -> Self {
        self.expiration2 = Some(date.into());

        self
    }

    /// Finish building, returning a `OptionSpreadStrikesQuery`.
    ///
    /// NOTE: You must set `symbol` before calling `build`.
    pub fn build(self) -> Result<OptionSpreadStrikesQuery, Error> {
        Ok(OptionSpreadStrikesQuery {
            underlying: self.underlying.ok_or_else(|| Error::SymbolNotSet)?,
            spread_type: self.spread_type.unwrap_or(OptionSpreadType::Single),
            strike_interval: self.strike_interval.unwrap_or(1),
            expiration: self.expiration,
            expiration2: self.expiration2,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// A chain of option spreads for a given underlying symbol,
/// spread type, and expiration.
pub struct OptionChain {
    /// The expected change in an option position’s value resulting
    /// from a one point increase in the price of the underlying security.
    pub delta: Option<String>,
    /// The expected decline in an option position’s value resulting
    /// from the passage of one day’s time, holding all other variables
    /// (price of the underlying, volatility, etc.) constant.
    pub theta: Option<String>,
    /// The expected change in an option position’s delta resulting
    /// from a one point increase in the price of the underlying security.
    pub gamma: Option<String>,
    /// The expected change in an option position’s value resulting
    /// from an increase of one percentage point in the risk-free
    /// interest rate (e.g. an increase from 3% to 4%).
    pub rho: Option<String>,
    /// The expected change in an option position’s value resulting
    /// from an increase of one percentage point in the volatility of
    /// the underlying security (e.g. an increase from 26% to 27%).
    pub vega: Option<String>,
    /// The volatility of the underlying implied by an option
    /// position’s current price.
    pub implied_volatility: Option<String>,
    /// The value of an option position exclusive of the position’s
    /// time value. The value of the option position if it were to
    /// expire immediately.
    pub intrinsic_value: String,
    /// The time value of an option position.
    ///
    /// NOTE: The market value of an option position minus
    /// the position’s intrinsic value.
    pub extrinsic_value: String,
    /// The value of an option position based on a theoretical model
    /// of option prices (the Bjerksund-Stensland model).
    ///
    /// NOTE: Calculated using volatility of the underlying.
    pub theoretical_value: String,
    #[serde(rename = "ProbabilityITM")]
    /// The calculated probability that an option position will have
    /// intrinsic value at expiration.
    ///
    /// NOTE: Calculated using volatility of the underlying.
    pub probability_itm: Option<String>,
    #[serde(rename = "ProbabilityOTM")]
    /// The calculated probability that an option position will not have
    /// intrinsic value at expiration.
    ///
    /// NOTE: Calculated using volatility of the underlying.
    pub probability_otm: Option<String>,
    #[serde(rename = "ProbabilityBE")]
    /// The calculated probability that an option position will have
    /// a value at expiration that is equal to or greater than the
    /// position’s current cost.
    ///
    /// NOTE: Calculated using volatility of the underlying.
    pub probability_be: Option<String>,
    #[serde(rename = "ProbabilityITM_IV")]
    /// The calculated probability that an option position will have
    /// intrinsic value at expiration.
    ///
    /// NOTE: Calculated using implied volatility.
    pub probability_itm_iv: Option<String>,
    #[serde(rename = "ProbabilityOTM_IV")]
    /// The calculated probability that an option position will not
    /// have intrinsic value at expiration.
    ///
    /// NOTE: Calculated using implied volatility.
    pub probability_otm_iv: Option<String>,
    #[serde(rename = "ProbabilityBE_IV")]
    /// The calculated probability that an option position will have a
    /// value at expiration that is equal to or greater than the position’s
    /// current cost.
    ///
    /// NOTE: Calculated using implied volatility.
    pub probability_be_iv: Option<String>,
    #[serde(rename = "TheoreticalValueIV")]
    /// The value of an option position based on a theoretical model of
    /// option prices (the Bjerksund-Stensland model).
    ///
    /// NOTE: Calculated using implied volatility.
    pub theoretical_value_iv: Option<String>,
    /// Total number of open contracts for the option spread.
    ///
    /// NOTE: This value is updated daily.
    pub daily_open_interest: i32,
    /// Ask price. The price a seller is willing to accept for the option spread.
    pub ask: String,
    /// Bid price. The price a buyer is willing to pay for the option spread.
    pub bid: String,
    /// Average between `ask` and `bid`.
    pub mid: String,
    /// Amount of contracts at the given `ask` price.
    pub ask_size: i32,
    /// Amount of contracts at the given `bid` price.
    pub bid_size: i32,
    /// The last traded price for the option spread.
    ///
    /// NOTE: This value only updates during the official market session.
    pub close: String,
    /// Today's highest price for the option spread.
    pub high: String,
    /// The last traded price for the option spread.
    pub last: String,
    /// Today's lowest traded price for the option spread.
    pub low: String,
    /// Difference between prior Close price and current Close price for the
    /// option spread.
    pub net_change: String,
    /// Percentage changed between prior `close` price and current `close` price
    /// for the option spread.
    pub net_change_pct: String,
    /// The initial price for the option spread during the official market session.
    pub open: String,
    /// Prior day's Closing price.
    pub previous_close: String,
    /// The number of contracts traded today.
    pub volume: i32,
    /// The side of the option chain.
    pub side: OptionChainSide,
    /// The strike prices for the option contracts in the legs of this spread.
    pub strikes: Vec<String>,
    /// The legs of the option spread.
    pub legs: Vec<OptionSpreadLeg>,
}
impl OptionChain {
    /// Stream an options chain for a given query `OptionChainQuery`.
    ///
    /// NOTE: You need to provide a function to handle each stream chunk.
    ///
    /// # Example
    /// ---
    ///
    /// Example: Stream an option chain for Apple `"AAPL"`.
    ///
    /// ```ignore
    /// let stream_aapl_option_chain_query = MarketData::OptionChainQueryBuilder::new()
    ///     .underlying("AAPL")
    ///     .build()?;
    ///
    /// let streamed_chains = client
    ///     .stream_option_chain(&stream_aapl_option_chain_query, |stream_data| {
    ///         // The response type is `responses::market_data::StreamOptionChainResp`
    ///         // which has multiple variants the main one you care about is `OptionChain`
    ///         // which will contain option chain data sent from the stream.
    ///         match stream_data {
    ///             StreamOptionChainResp::OptionChain(chain) => {
    ///                 // Do something with the option chain like
    ///                 // display it with a table on a website.
    ///                 println!("{chain:?}")
    ///             }
    ///             StreamOptionChainResp::Heartbeat(heartbeat) => {
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
    ///             StreamOptionChainResp::Status(status) => {
    ///                 // Signal sent on state changes in the stream
    ///                 // (closed, opened, paused, resumed)
    ///                 println!("{status:?}");
    ///             }
    ///             StreamOptionChainResp::Error(err) => {
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
    /// // After the stream ends print all the collected option chains
    /// println!("{streamed_chains:?}");
    /// ```
    pub async fn stream<F>(
        client: &mut Client,
        query: &OptionChainQuery,
        mut on_chunk: F,
    ) -> Result<Vec<OptionChain>, Error>
    where
        F: FnMut(StreamOptionChainResp) -> Result<(), Error>,
    {
        let endpoint = format!(
            "marketdata/stream/options/chains/{}{}",
            query.underlying,
            query.as_query_string()
        );

        let mut collected_chains: Vec<OptionChain> = Vec::new();
        client
            .stream(&endpoint, |chunk| {
                let parsed_chunk = serde_json::from_value::<StreamOptionChainResp>(chunk)?;
                on_chunk(parsed_chunk.clone())?;

                // Only collect orders, so when the stream is done
                // all the orders that were streamed can be returned
                if let StreamOptionChainResp::OptionChain(option_chain) = parsed_chunk {
                    collected_chains.push(*option_chain);
                }

                Ok(())
            })
            .await?;

        Ok(collected_chains)
    }
}
impl Client {
    /// Stream an options chain for a given query `OptionChainQuery`.
    ///
    /// NOTE: You need to provide a function to handle each stream chunk.
    ///
    /// # Example
    /// ---
    ///
    /// Example: Stream an option chain for Apple `"AAPL"`.
    ///
    /// ```ignore
    /// let stream_aapl_option_chain_query = MarketData::OptionChainQueryBuilder::new()
    ///     .underlying("AAPL")
    ///     .build()?;
    ///
    /// let streamed_chains = client
    ///     .stream_option_chain(&stream_aapl_option_chain_query, |stream_data| {
    ///         // The response type is `responses::market_data::StreamOptionChainResp`
    ///         // which has multiple variants the main one you care about is `OptionChain`
    ///         // which will contain option chain data sent from the stream.
    ///         match stream_data {
    ///             StreamOptionChainResp::OptionChain(chain) => {
    ///                 // Do something with the option chain like
    ///                 // display it with a table on a website.
    ///                 println!("{chain:?}")
    ///             }
    ///             StreamOptionChainResp::Heartbeat(heartbeat) => {
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
    ///             StreamOptionChainResp::Status(status) => {
    ///                 // Signal sent on state changes in the stream
    ///                 // (closed, opened, paused, resumed)
    ///                 println!("{status:?}");
    ///             }
    ///             StreamOptionChainResp::Error(err) => {
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
    /// // After the stream ends print all the collected option chains
    /// println!("{streamed_chains:?}");
    /// ```
    pub async fn stream_option_chain<F>(
        &mut self,
        query: &OptionChainQuery,
        on_chunk: F,
    ) -> Result<Vec<OptionChain>, Error>
    where
        F: FnMut(StreamOptionChainResp) -> Result<(), Error>,
    {
        OptionChain::stream(self, query, on_chunk).await
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// The query used to sream an options chain.
pub struct OptionChainQuery {
    /// The symbol for the underlying security on which the option contracts are based.
    pub underlying: String,
    /// Date on which the option contract expires; must be a valid expiration date.
    ///
    /// NOTE: Defaults to the next contract expiration date.
    pub expiration: Option<String>,
    /// Second contract expiration date required
    /// for Calendar and Diagonal spreads.
    pub expiration2: Option<String>,
    /// Specifies the number of spreads to display above and below the price center.
    ///
    /// NOTE: Defaults to a proximity of `5` strikes above and below the price center.
    pub strike_proximity: i32,
    /// Specifies the name of the spread type to use.
    pub spread_type: OptionSpreadType,
    /// The theoretical rate of return of an investment with zero risk.
    /// NOTE: Defaults to the current quote for `$IRX.X`.
    ///
    /// NOTE: The percentage rate should be specified as a decimal value.
    /// E.g, to use 2% for the rate, pass in `0.02`.
    pub risk_free_rate: Option<f64>,
    /// Specifies the strike price center.
    ///
    /// NOTE: Defaults to the last quoted price for the underlying security.
    pub price_center: Option<f64>,
    /// Specifies the desired interval between the strike prices in a spread.
    ///
    /// NOTE: It must be greater than or equal to 1. A value of 1 uses consecutive strikes;
    /// a value of 2 skips one between strikes; and so on.
    ///
    /// NOTE: Defaults to `1`.
    pub strike_interval: i32,
    /// Specifies whether or not greeks properties are returned.
    ///
    /// NOTE: Defaults to `true`.
    pub enable_greeks: bool,
    /// Set the option chain filter for specific range of options.
    ///
    /// NOTE: Defaults to all `OptionStrikeRange::All`.
    ///
    /// E.g: Filter the chain for out of the money options:
    /// `OptionStrikeRange::OTM`.
    pub strike_range: OptionStrikeRange,
    /// Filters the spreads by a specific option type.
    pub option_type: OptionType,
}
impl OptionChainQuery {
    pub fn as_query_string(&self) -> String {
        let mut query_string = String::from("?");

        query_string.push_str(&format!("strikeInterval={}&", self.strike_interval));
        query_string.push_str(&format!("strikeProximity={}&", self.strike_proximity));
        query_string.push_str(&format!("spreadType={:?}&", self.spread_type));
        query_string.push_str(&format!("enableGreeks={}&", self.enable_greeks));
        query_string.push_str(&format!("strikeRange={:?}&", self.strike_range));
        query_string.push_str(&format!("optionType={:?}&", self.option_type));

        if let Some(expiration) = &self.expiration {
            query_string.push_str(&format!("expiration={}&", expiration));
        }
        if let Some(expiration) = &self.expiration2 {
            query_string.push_str(&format!("expiration2={}&", expiration));
        }
        if let Some(rate) = self.risk_free_rate {
            query_string.push_str(&format!("riskFreeRate={}&", rate));
        }
        if let Some(price) = self.price_center {
            query_string.push_str(&format!("priceCenter={}&", price));
        }

        if query_string.ends_with('&') {
            query_string.pop();
        }

        query_string
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "PascalCase")]
/// Builder for `OptionChainQuery`.
pub struct OptionChainQueryBuilder {
    /// The symbol for the underlying security on which the option contracts are based.
    underlying: Option<String>,
    /// Date on which the option contract expires; must be a valid expiration date.
    ///
    /// NOTE: Defaults to the next contract expiration date.
    expiration: Option<String>,
    /// Second contract expiration date required
    /// for Calendar and Diagonal spreads.
    expiration2: Option<String>,
    /// Specifies the number of spreads to display above and below the price center.
    ///
    /// NOTE: Defaults to a proximity of `5` strikes above and below the price center.
    strike_proximity: Option<i32>,
    /// Specifies the name of the spread type to use.
    spread_type: Option<OptionSpreadType>,
    /// The theoretical rate of return of an investment with zero risk.
    /// NOTE: Defaults to the current quote for `$IRX.X`.
    ///
    /// NOTE: The percentage rate should be specified as a decimal value.
    /// E.g, to use 2% for the rate, pass in `0.02`.
    risk_free_rate: Option<f64>,
    /// Specifies the strike price center.
    ///
    /// NOTE: Defaults to the last quoted price for the underlying security.
    price_center: Option<f64>,
    /// Specifies the desired interval between the strike prices in a spread.
    ///
    /// NOTE: It must be greater than or equal to 1. A value of 1 uses consecutive strikes;
    /// a value of 2 skips one between strikes; and so on.
    ///
    /// NOTE: Defaults to `1`.
    strike_interval: Option<i32>,
    /// Specifies whether or not greeks properties are returned.
    ///
    /// NOTE: Defaults to `true`.
    enable_greeks: Option<bool>,
    /// Set the option chain filter for specific range of options.
    ///
    /// NOTE: Defaults to all `OptionStrikeRange::All`.
    ///
    /// E.g: Filter the chain for out of the money options:
    /// `OptionStrikeRange::OTM`.
    strike_range: Option<OptionStrikeRange>,
    /// Filters the spreads by a specific option type.
    option_type: Option<OptionType>,
}
impl OptionChainQueryBuilder {
    /// Create a new builder for `OptionChainQuery`
    pub fn new() -> Self {
        OptionChainQueryBuilder::default()
    }

    /// Set the expiration date for an option chain.
    ///
    /// NOTE: This is required to be set before calling
    /// `OptionChainQueryBuilder::build()`.
    pub fn underlying<S: Into<String>>(mut self, symbol: S) -> Self {
        self.underlying = Some(symbol.into());

        self
    }

    /// Set the expiration date for an option chain.
    pub fn expiration<S: Into<String>>(mut self, date: S) -> Self {
        self.expiration = Some(date.into());

        self
    }

    /// Set the second expiration date for an option chain.
    ///
    /// NOTE: This is required for `OptionSpreadType::Calendar` and
    /// `OptionSpreadType::Diagonal` option spreads.
    pub fn expiration2<S: Into<String>>(mut self, date: S) -> Self {
        self.expiration2 = Some(date.into());

        self
    }

    /// Set the strike proximity (of the center price) for the option chain.
    pub fn strike_proximity(mut self, proximity: i32) -> Self {
        self.strike_proximity = Some(proximity);

        self
    }

    /// Set the spread type for the option chain
    pub fn spread_type(mut self, spread_type: OptionSpreadType) -> Self {
        self.spread_type = Some(spread_type);

        self
    }

    /// Set your risk free rate.
    pub fn risk_free_rate(mut self, rate: f64) -> Self {
        self.risk_free_rate = Some(rate);

        self
    }

    /// Set the center price point for the option chain.
    pub fn price_center(mut self, price: f64) -> Self {
        self.price_center = Some(price);

        self
    }

    /// Set the interval of strikes for the option chain.
    pub fn strike_interval(mut self, interval: i32) -> Self {
        self.strike_interval = Some(interval);

        self
    }

    /// Set if the option chain should contain greeks.
    pub fn enable_greeks(mut self, val: bool) -> Self {
        self.enable_greeks = Some(val);

        self
    }

    /// Set the option chain filter for specific range of options.
    pub fn strike_range(mut self, range: OptionStrikeRange) -> Self {
        self.strike_range = Some(range);

        self
    }

    /// Set the option chain filter for a specific option type.
    pub fn option_type(mut self, opt_type: OptionType) -> Self {
        self.option_type = Some(opt_type);

        self
    }

    /// Finish building `OptionChainQuery`.
    ///
    /// NOTE: Must set the `underlying` symbol before calling `build`.
    pub fn build(self) -> Result<OptionChainQuery, Error> {
        Ok(OptionChainQuery {
            underlying: self.underlying.ok_or_else(|| Error::SymbolNotSet)?,
            expiration: self.expiration,
            expiration2: self.expiration2,
            strike_range: self.strike_range.unwrap_or(OptionStrikeRange::All),
            option_type: self.option_type.unwrap_or(OptionType::All),
            enable_greeks: self.enable_greeks.unwrap_or(true),
            strike_interval: self.strike_interval.unwrap_or(1),
            spread_type: self.spread_type.unwrap_or(OptionSpreadType::Single),
            strike_proximity: self.strike_proximity.unwrap_or(5),
            price_center: self.price_center,
            risk_free_rate: self.risk_free_rate,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
/// The different types of option strike ranges.
pub enum OptionStrikeRange {
    /// A range containing all strikes
    All,
    /// A range containing In-The-Money strikes
    ITM,
    /// A range containing Out-of-The-Money strikes
    OTM,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// A component of a larger options trade.
pub struct OptionSpreadLeg {
    /// Option contract symbol or underlying symbol to be traded for this leg.
    pub symbol: String,
    /// The number of option contracts or underlying shares for this leg,
    /// relative to the other legs. A positive number represents a buy trade
    /// and a negative number represents a sell trade.
    ///
    /// E.g, a Butterfly spread can be represented using ratios of 1, -2, and 1:
    /// buy 1 contract of the first leg, sell 2 contracts of the second leg, and
    /// buy 1 contract of the third leg.
    pub ratio: i32,
    /// The strike price of the option contract for this leg.
    pub strike_price: String,
    /// Date on which the contract expires.
    /// E.g: `2021-12-17T00:00:00Z`.
    pub expiration: String,
    /// The option type `MarketData::OptionType`
    pub option_type: OptionType,
    /// The asset category for this leg.
    pub asset_type: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
/// The different sides of the option chain.
pub enum OptionChainSide {
    /// The side of the option chain with call options.
    Call,
    /// The side of the option chain with put options.
    Put,
    /// The side of the option chain with
    /// both call and put options.
    Both,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
/// The type of option.
pub enum OptionType {
    /// Call Option
    Call,
    /// Put Option
    Put,
    /// All Options (Calls, and Puts)
    All,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct OptionQuote {
    /// The expected change in an option position’s value resulting
    /// from a one point increase in the price of the underlying security.
    pub delta: Option<String>,
    /// The expected decline in an option position’s value resulting
    /// from the passage of one day’s time, holding all other variables
    /// (price of the underlying, volatility, etc.) constant.
    pub theta: Option<String>,
    /// The expected change in an option position’s delta resulting
    /// from a one point increase in the price of the underlying security.
    pub gamma: Option<String>,
    /// The expected change in an option position’s value resulting
    /// from an increase of one percentage point in the risk-free
    /// interest rate (e.g. an increase from 3% to 4%).
    pub rho: Option<String>,
    /// The expected change in an option position’s value resulting
    /// from an increase of one percentage point in the volatility of
    /// the underlying security (e.g. an increase from 26% to 27%).
    pub vega: Option<String>,
    /// The volatility of the underlying implied by an option
    /// position’s current price.
    pub implied_volatility: Option<String>,
    /// The value of an option position exclusive of the position’s
    /// time value. The value of the option position if it were to
    /// expire immediately.
    pub intrinsic_value: String,
    /// The time value of an option position.
    ///
    /// NOTE: The market value of an option position minus
    /// the position’s intrinsic value.
    pub extrinsic_value: String,
    /// The value of an option position based on a theoretical model
    /// of option prices (the Bjerksund-Stensland model).
    ///
    /// NOTE: Calculated using volatility of the underlying.
    pub theoretical_value: String,
    #[serde(rename = "ProbabilityITM")]
    /// The calculated probability that an option position will have
    /// intrinsic value at expiration.
    ///
    /// NOTE: Calculated using volatility of the underlying.
    pub probability_itm: Option<String>,
    #[serde(rename = "ProbabilityOTM")]
    /// The calculated probability that an option position will not have
    /// intrinsic value at expiration.
    ///
    /// NOTE: Calculated using volatility of the underlying.
    pub probability_otm: Option<String>,
    #[serde(rename = "ProbabilityBE")]
    /// The calculated probability that an option position will have
    /// a value at expiration that is equal to or greater than the
    /// position’s current cost.
    ///
    /// NOTE: Calculated using volatility of the underlying.
    pub probability_be: Option<String>,
    #[serde(rename = "ProbabilityITM_IV")]
    /// The calculated probability that an option position will have
    /// intrinsic value at expiration.
    ///
    /// NOTE: Calculated using implied volatility.
    pub probability_itm_iv: Option<String>,
    #[serde(rename = "ProbabilityOTM_IV")]
    /// The calculated probability that an option position will not
    /// have intrinsic value at expiration.
    ///
    /// NOTE: Calculated using implied volatility.
    pub probability_otm_iv: Option<String>,
    #[serde(rename = "ProbabilityBE_IV")]
    /// The calculated probability that an option position will have a
    /// value at expiration that is equal to or greater than the position’s
    /// current cost.
    ///
    /// NOTE: Calculated using implied volatility.
    pub probability_be_iv: Option<String>,
    #[serde(rename = "TheoreticalValueIV")]
    /// The value of an option position based on a theoretical model of
    /// option prices (the Bjerksund-Stensland model).
    ///
    /// NOTE: Calculated using implied volatility.
    pub theoretical_value_iv: Option<String>,
    /// Total number of open contracts for the option spread.
    ///
    /// NOTE: This value is updated daily.
    pub daily_open_interest: i32,
    /// Ask price. The price a seller is willing to accept for the option spread.
    pub ask: String,
    /// Bid price. The price a buyer is willing to pay for the option spread.
    pub bid: String,
    /// Average between `ask` and `bid`.
    pub mid: String,
    /// Amount of contracts at the given `ask` price.
    pub ask_size: i32,
    /// Amount of contracts at the given `bid` price.
    pub bid_size: i32,
    /// The last traded price for the option spread.
    ///
    /// NOTE: This value only updates during the official market session.
    pub close: String,
    /// Today's highest price for the option spread.
    pub high: String,
    /// The last traded price for the option spread.
    pub last: String,
    /// Today's lowest traded price for the option spread.
    pub low: String,
    /// Difference between prior Close price and current Close price for the
    /// option spread.
    pub net_change: String,
    /// Percentage changed between prior `close` price and current `close` price
    /// for the option spread.
    pub net_change_pct: String,
    /// The initial price for the option spread during the official market session.
    pub open: String,
    /// Prior day's Closing price.
    pub previous_close: String,
    /// The number of contracts traded today.
    pub volume: i32,
    /// The side of the option chain.
    pub side: OptionChainSide,
    /// The strike prices for the option contracts in the legs of this spread.
    pub strikes: Vec<String>,
    /// The legs of the option spread.
    pub legs: Vec<OptionSpreadLeg>,
}
impl OptionQuote {
    /// Stream quotes of an options spread for given a query `OptionQuoteQuery`.
    ///
    /// <div class="warning">WARNING: There's a max of 10 concurrent streams allowed.</div>
    ///
    /// NOTE: You need to provide a function to handle each stream chunk.
    ///
    /// # Example
    /// ---
    ///
    /// Example: Stream quotes on Iron Butterfly options trade on `"TLT"`
    /// expiring October 11th 2024. E.g: Say you just bought this iron buttefly
    /// now you can stream quotes on it to watch profit/loss, or take some kind
    /// of action based on market conditions.
    ///
    /// ```ignore
    /// let stream_tlt_iron_butterfly_query = MarketData::OptionQuoteQueryBuilder::new()
    ///     .legs(vec![
    ///         OptionQouteLeg {
    ///             symbol: "TLT 241011P93".into(),
    ///             ratio: -10,
    ///         },
    ///         OptionQouteLeg {
    ///             symbol: "TLT 241011P95.5".into(),
    ///             ratio: 10,
    ///         },
    ///         OptionQouteLeg {
    ///             symbol: "TLT 241011C95.5".into(),
    ///             ratio: 10,
    ///         },
    ///         OptionQouteLeg {
    ///             symbol: "TLT 241011C98".into(),
    ///             ratio: -10,
    ///         },
    ///     ])
    ///     // Using the 1 month us treasury
    ///     // to base the risk free rate off
    ///     // which is currently 4.85%
    ///     .risk_free_rate(0.0485)
    ///     .build()?;
    ///
    /// let streamed_quotes = client
    ///     .stream_option_quotes(&stream_tlt_iron_butterfly_query, |stream_data| {
    ///         // The response type is `responses::market_data::StreamOptionQuotesResp`
    ///         // which has multiple variants the main one you care about is `OptionQuotes`
    ///         // which will contain option chain data sent from the stream.
    ///         match stream_data {
    ///             StreamOptionQuotesResp::OptionQuotes(quote) => {
    ///                 // Do something with the option quote like
    ///                 // send a text / email alert based on some
    ///                 // data from the quote like a certain price,
    ///                 // market spread, volatility change, or something.
    ///                 println!("{quote:?}")
    ///             }
    ///             StreamOptionQuotesResp::Heartbeat(heartbeat) => {
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
    ///             StreamOptionQuotesResp::Status(status) => {
    ///                 // Signal sent on state changes in the stream
    ///                 // (closed, opened, paused, resumed)
    ///                 println!("{status:?}");
    ///             }
    ///             StreamOptionQuotesResp::Error(err) => {
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
    /// // After the stream ends print all the collected option quotes
    /// println!("{streamed_quotes:?}");
    /// ```
    pub async fn stream<F>(
        client: &mut Client,
        query: &OptionQuoteQuery,
        mut on_chunk: F,
    ) -> Result<Vec<OptionQuote>, Error>
    where
        F: FnMut(StreamOptionQuotesResp) -> Result<(), Error>,
    {
        let endpoint = format!(
            "marketdata/stream/options/quotes{}",
            query.as_query_string()
        );
        println!("endpoint: {endpoint}");

        let mut collected_quotes: Vec<OptionQuote> = Vec::new();
        client
            .stream(&endpoint, |chunk| {
                let parsed_chunk = serde_json::from_value::<StreamOptionQuotesResp>(chunk)?;
                on_chunk(parsed_chunk.clone())?;

                // Only collect orders, so when the stream is done
                // all the orders that were streamed can be returned
                if let StreamOptionQuotesResp::OptionQuotes(option_chain) = parsed_chunk {
                    collected_quotes.push(*option_chain);
                }

                Ok(())
            })
            .await?;

        Ok(collected_quotes)
    }
}
impl Client {
    /// Stream quotes of an options spread for given a query `OptionQuoteQuery`.
    ///
    /// <div class="warning">WARNING: There's a max of 10 concurrent streams allowed.</div>
    ///
    /// NOTE: You need to provide a function to handle each stream chunk.
    ///
    /// # Example
    /// ---
    ///
    /// Example: Stream quotes on Iron Butterfly options trade on `"TLT"`
    /// expiring October 11th 2024. E.g: Say you just bought this iron buttefly
    /// now you can stream quotes on it to watch profit/loss, or take some kind
    /// of action based on market conditions.
    ///
    /// ```ignore
    /// let stream_tlt_iron_butterfly_query = MarketData::OptionQuoteQueryBuilder::new()
    ///     .legs(vec![
    ///         OptionQouteLeg {
    ///             symbol: "TLT 241011P93".into(),
    ///             ratio: -10,
    ///         },
    ///         OptionQouteLeg {
    ///             symbol: "TLT 241011P95.5".into(),
    ///             ratio: 10,
    ///         },
    ///         OptionQouteLeg {
    ///             symbol: "TLT 241011C95.5".into(),
    ///             ratio: 10,
    ///         },
    ///         OptionQouteLeg {
    ///             symbol: "TLT 241011C98".into(),
    ///             ratio: -10,
    ///         },
    ///     ])
    ///     // Using the 1 month us treasury
    ///     // to base the risk free rate off
    ///     // which is currently 4.85%
    ///     .risk_free_rate(0.0485)
    ///     .build()?;
    ///
    /// let streamed_quotes = client
    ///     .stream_option_quotes(&stream_tlt_iron_butterfly_query, |stream_data| {
    ///         // The response type is `responses::market_data::StreamOptionQuotesResp`
    ///         // which has multiple variants the main one you care about is `OptionQuotes`
    ///         // which will contain option chain data sent from the stream.
    ///         match stream_data {
    ///             StreamOptionQuotesResp::OptionQuotes(quote) => {
    ///                 // Do something with the option quote like
    ///                 // send a text / email alert based on some
    ///                 // data from the quote like a certain price,
    ///                 // market spread, volatility change, or something.
    ///                 println!("{quote:?}")
    ///             }
    ///             StreamOptionQuotesResp::Heartbeat(heartbeat) => {
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
    ///             StreamOptionQuotesResp::Status(status) => {
    ///                 // Signal sent on state changes in the stream
    ///                 // (closed, opened, paused, resumed)
    ///                 println!("{status:?}");
    ///             }
    ///             StreamOptionQuotesResp::Error(err) => {
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
    /// // After the stream ends print all the collected option quotes
    /// println!("{streamed_quotes:?}");
    /// ```
    pub async fn stream_option_quotes<F>(
        &mut self,
        query: &OptionQuoteQuery,
        on_chunk: F,
    ) -> Result<Vec<OptionQuote>, Error>
    where
        F: FnMut(StreamOptionQuotesResp) -> Result<(), Error>,
    {
        OptionQuote::stream(self, query, on_chunk).await
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// The query to stream quotes on an options spread.
pub struct OptionQuoteQuery {
    /// The individual positions making up a larger trade.
    pub legs: Vec<OptionQouteLeg>,
    /// The theoretical rate of return of an
    /// investment with zero risk.
    ///
    /// NOTE: Defaults to the current quote for `"$IRX.X"`
    /// (The 13 Week US Treasury).
    ///
    /// NOTE: The percentage rate should be specified as a decimal value.
    /// E.g: 2% = `0.02`.
    pub risk_free_rate: Option<f64>,
    /// Specifies whether or not greeks properties are returned.
    ///
    /// NOTE: Defaults to `true`.
    pub enable_greeks: bool,
}
impl OptionQuoteQuery {
    /// Convert the query into a string
    pub fn as_query_string(&self) -> String {
        let legs: Vec<String> = self
            .legs
            .iter()
            .enumerate()
            .map(|(idx, leg)| {
                format!(
                    "legs[{idx}].Symbol={}&legs[{idx}].Ratio={}",
                    leg.symbol, leg.ratio
                )
            })
            .collect();

        let legs_string = legs.join("&");

        if let Some(rate) = self.risk_free_rate {
            format!(
                "?{legs_string}&riskFreeRate={rate}&enableGreeks={}",
                self.enable_greeks
            )
        } else {
            format!("?{legs_string}&enableGreeks={}", self.enable_greeks)
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// The leg for an `OptionQuoteQuery`
pub struct OptionQouteLeg {
    /// Option contract symbol or underlying
    /// symbol to be traded for this leg.
    pub symbol: String,
    /// The number of option contracts or underlying
    /// shares for this leg, relative to the other legs.
    ///
    /// NOTE: Use a positive number to represent a buy trade
    /// and a negative number to represent a sell trade.
    pub ratio: i32,
}

#[derive(Default)]
/// Builder for `OptionQuoteQuery`
pub struct OptionQuoteQueryBuilder {
    legs: Option<Vec<OptionQouteLeg>>,
    risk_free_rate: Option<f64>,
    enable_greeks: Option<bool>,
}
impl OptionQuoteQueryBuilder {
    /// Create a new builder for `OptionQuoteQuery`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the option legs (each option position in the overall trade).
    ///
    /// NOTE: For a trade with just a single option you still need to pass
    /// the option as a leg.
    pub fn legs(mut self, option_legs: Vec<OptionQouteLeg>) -> Self {
        self.legs = Some(option_legs);

        self
    }

    /// Set your risk free rate.
    pub fn risk_free_rate(mut self, rate: f64) -> Self {
        self.risk_free_rate = Some(rate);

        self
    }

    /// Set if the option chain should contain greeks.
    pub fn enable_greeks(mut self, enable: bool) -> Self {
        self.enable_greeks = Some(enable);

        self
    }

    /// Finish building `OptionQuoteQuery`
    pub fn build(self) -> Result<OptionQuoteQuery, Error> {
        Ok(OptionQuoteQuery {
            legs: self.legs.ok_or_else(|| Error::OptionLegsNotSet)?,
            enable_greeks: self.enable_greeks.unwrap_or(true),
            risk_free_rate: self.risk_free_rate,
        })
    }
}
