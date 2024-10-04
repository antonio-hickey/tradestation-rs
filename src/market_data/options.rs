use crate::{
    responses::{
        market_data::{OptionSpreadStrikesResp, OptionSpreadStrikesRespRaw},
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
        client: &mut Client,
        query: OptionSpreadStrikesQuery,
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
        OptionSpreadStrikes::fetch(self, query).await
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
