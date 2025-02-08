use super::accounts::AccountType;
use crate::{
    responses::account::{GetBODBalanceResp, GetBalanceResp},
    Client, Error,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// The real time balance of an `Account`.
pub struct Balance {
    #[serde(rename = "AccountID")]
    /// The main identifier for a TradeStation account
    pub account_id: String,

    /// The type of account, examples: "Cash" or "Margin"
    pub account_type: AccountType,

    /// The real time Cash Balance value for the `Account`
    pub cash_balance: String,

    /// The real time Buying Power value for the `Account`
    pub buying_power: String,

    /// The real time Equity value for the `Account`
    pub equity: String,

    /// The real time Market Value for the `Account`
    pub market_value: String,

    #[serde(rename = "TodaysProfitLoss")]
    /// The real time (profit - loss) value for the `Account` over a 24 hour period
    pub todays_pnl: String,

    /// The value of uncleared funds for the `Account`
    pub uncleared_deposit: String,

    /// Deeper details on the `Balance` of an `Account`
    pub balance_detail: BalanceDetail,

    /// The amount paid in brokerage commissions.
    ///
    /// NOTE: This value does not include slippage.
    pub commission: String,
}
impl Balance {
    /// Get the current balance of an `Account`.
    pub(super) async fn get<S: Into<String>>(
        account_id: S,
        client: &mut Client,
    ) -> Result<Balance, Error> {
        let endpoint = format!("brokerage/accounts/{}/balances", account_id.into());

        if let Some(balance) = client
            .get(&endpoint)
            .await?
            .json::<GetBalanceResp>()
            .await?
            .balances
            .pop()
        {
            Ok(balance)
        } else {
            Err(Error::AccountNotFound)
        }
    }

    /// Get the current balance of all `Account`(s) by account ids.
    pub(super) async fn get_multiple(
        account_ids: Vec<&str>,
        client: &mut Client,
    ) -> Result<Vec<Balance>, Error> {
        let endpoint = format!("brokerage/accounts/{}/balances", account_ids.join(","));

        let resp = client
            .get(&endpoint)
            .await?
            .json::<GetBalanceResp>()
            .await?;

        Ok(resp.balances)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// Real time balance information for an `Account`.
pub struct BalanceDetail {
    /// The real time cost for all positions open in the `Account`
    ///
    /// NOTE: Positions are based on the actual entry price
    pub cost_of_positions: Option<String>,

    /// The number of day trades the `Account` has taken over the previous 4 days
    ///
    /// NOTE: This updates daily
    ///
    /// NOTE: This is always None for futures `Account`.
    pub day_trades: Option<String>,

    /// The real time dollar amount of required funds for `Account` margin maintenance
    ///
    /// NOTE: SUM(maintenance margin of all open positions in the account).
    ///
    /// NOTE: This is always None for futures `Account`.
    pub maintenance_rate: Option<String>,

    /// The real time value of intraday buying power for options
    ///
    /// NOTE: This is always None for futures `Account`.
    pub option_buying_power: Option<String>,

    /// The real time Market Value of current open option positions in an `Account`.
    pub options_market_value: Option<String>,

    /// The real time Buying Power value that can be held overnight w/o triggering a margin call.
    ///
    /// NOTE: (Equity - Overnight Requirement %) / 50 %.
    pub overnight_buying_power: Option<String>,

    /// The real time dollar value of open order Day Trade Margins for an `Account`.
    ///
    /// NOTE: SUM(Day Trade Margin of all open orders in the account).
    ///
    /// NOTE: Always `None` for cash & margin accounts
    pub day_trade_open_order_margin: Option<String>,

    /// The real time dollar value of open order Initial Margin for an `Account`.
    ///
    /// NOTE: SUM(Initial Margin of all open orders in the account).
    ///
    /// NOTE: Always `None` for cash & margin accounts.
    pub open_order_margin: Option<String>,

    /// The real time dollar value of Initial Margin for an `Account`.
    ///
    /// NOTE: SUM(Initial Margin of all open positions in the account).
    pub initial_margin: Option<String>,

    /// The real time dollar value of Maintenance Margin for an `Account`.
    ///
    /// NOTE: SUM(Maintenance Margins of all open positions in the account).
    ///
    /// NOTE: Always `None` for cash & margin accounts.
    pub maintenance_margin: Option<String>,

    /// The real time dollar amount of Trade Equity for an `Account`.
    ///
    /// NOTE: Always `None` for cash & margin accounts.
    pub trade_equity: Option<String>,

    /// The value of special securities deposited with the clearing firm
    /// for the sole purpose of increasing purchasing power in `Account`
    ///
    /// NOTE: This number will be reset daily by the account balances clearing file.
    ///
    /// NOTE: The entire value of this field will increase purchasing power.
    ///
    /// NOTE: Always `None` for cash & margin accounts.
    pub security_on_deposit: Option<String>,

    /// The real time dollar value of Today's Trade Equity for an `Account`.
    ///
    /// NOTE: (Beginning Day Trade Equity - Real Time Trade Equity).
    pub today_real_time_trade_equity: Option<String>,

    /// Deeper details on base currency.
    ///
    /// NOTE: Always `None` for cash & margin accounts.
    pub currency_details: Option<CurrencyDetails>,

    /// The real time amount of required funds for `Account` margin maintenance.
    ///
    /// NOTE: The currency denomination is dependant on `Account::currency`.
    ///
    /// NOTE: SUM(maintenance margin of all open positions in the account).
    ///
    /// NOTE: Always `None` for futures accounts.
    pub required_margin: Option<String>,

    /// Funds received by TradeStation that are not settled from a transaction in the `Account`.
    ///
    /// NOTE: Always `None` for futures accounts.
    pub unsettled_funds: Option<String>,

    /// Maintenance Excess.
    ///
    /// NOTE: (Cash Balance + Long Market Value + Short Credit - Maintenance Requirement - Margin Debt - Short Market Value).
    pub day_trade_excess: String,

    #[serde(rename = "RealizedProfitLoss")]
    /// The net Realized Profit or Loss of an `Account` for the current trading day.
    ///
    /// NOTE: This includes all commissions and routing fees.
    pub realized_pnl: String,

    #[serde(rename = "UnrealizedProfitLoss")]
    /// The net Unrealized Profit or Loss of an `Account` for all currently open positions.
    ///
    /// NOTE: This does not include commissions or routing fees.
    pub unrealized_pnl: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// The beginning of day balance of an `Account`.
pub struct BODBalance {
    #[serde(rename = "AccountID")]
    /// The main identifier for a TradeStation account.
    pub account_id: String,

    /// The type of account, examples: "Cash" or "Margin".
    pub account_type: AccountType,

    /// Deeper details on the `Balance` of an `Account`.
    pub balance_detail: BODBalanceDetail,

    /// Deeper details on the `Currency` local of an `Account`.
    ///
    /// NOTE: Only applies to futures.
    pub currency_details: Option<Vec<BODCurrencyDetails>>,
}
impl BODBalance {
    /// Get the beginning of day balance of an `Account`.
    pub(super) async fn get<S: Into<String>>(
        account_id: S,
        client: &mut Client,
    ) -> Result<BODBalance, Error> {
        let endpoint = format!("brokerage/accounts/{}/bodbalances", account_id.into());

        if let Some(balance) = client
            .get(&endpoint)
            .await?
            .json::<GetBODBalanceResp>()
            .await?
            .bod_balances
            .pop()
        {
            Ok(balance)
        } else {
            Err(Error::AccountNotFound)
        }
    }

    /// Get the beginning of day balances for multiple `Account`(s) by account id.
    ///
    /// NOTE: If you have `Vec<Account>` you should instead use `Vec<Account>::get_bod_balances()`
    /// this method should only be used if you ONLY have account id's.
    pub(super) async fn get_multiple(
        account_ids: Vec<&str>,
        client: &mut Client,
    ) -> Result<Vec<BODBalance>, Error> {
        let endpoint = format!("brokerage/accounts/{}/bodbalances", account_ids.join(","));

        let resp = client
            .get(&endpoint)
            .await?
            .json::<GetBODBalanceResp>()
            .await?;

        Ok(resp.bod_balances)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// The beginning of day balance information of an `Account`.
pub struct BODBalanceDetail {
    /// The amount of cash in the account at the beginning of the day.
    ///
    /// NOTE: Only applies to equities.
    pub account_balance: Option<String>,

    /// Beginning of day value for cash available to withdraw
    pub cash_available_to_withdraw: Option<String>,

    /// The number of day trades placed in the account within the previous
    /// 4 trading days.
    ///
    /// NOTE: Only applies to equities.
    pub day_trades: Option<String>,

    /// The Intraday Buying Power with which the account started the trading day.
    ///
    /// NOTE: Only applies to equities.
    pub day_trading_marginable_buying_power: Option<String>,

    /// The total amount of equity with which you started the current trading day.
    pub equity: String,

    /// The amount of cash in the account at the beginning of the day.
    pub net_cash: String,

    /// Unrealized profit and loss at the beginning of the day.
    ///
    /// NOTE: Only applies to futures.
    pub open_trade_equity: Option<String>,

    /// Option buying power at the start of the trading day.
    ///
    /// NOTE: Only applies to equities.
    pub option_buying_power: Option<String>,

    /// Intraday liquidation value of option positions.
    ///
    /// NOTE: Only applies to equities.
    pub option_value: Option<String>,

    /// Overnight Buying Power (Regulation T) at the start of the trading day.
    ///
    /// NOTE: Only applies to equities.
    pub overnight_buying_power: Option<String>,

    /// The value of special securities that are deposited by the customer with
    /// the clearing firm for the sole purpose of increasing purchasing power in
    /// their trading account.
    ///
    /// NOTE: Only applies to futures.
    pub security_on_deposit: Option<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// The beginning of day currency information.
///
/// NOTE: Only applies to futures.
pub struct BODCurrencyDetails {
    /// The dollar amount of Beginning Day Margin for the given forex account
    pub account_margin_requirement: Option<String>,

    /// The dollar amount of Beginning Day Trade Equity for the given account
    pub account_open_trade_equity: String,

    /// The value of special securities that are deposited by the customer with
    /// the clearing firm for the sole purpose of increasing purchasing power in
    /// their trading account.
    ///
    /// NOTE: This number will be reset daily by the account balances
    /// clearing file.
    ///
    /// NOTE: The entire value of this field will increase purchasing power
    pub account_securities: String,

    /// The dollar amount of the Beginning Day Cash Balance for the given account
    pub cash_balance: String,

    /// The currency of the entity
    pub currency: String,

    /// The dollar amount of Beginning Day Margin for the given forex account
    pub margin_requirement: Option<String>,

    /// The dollar amount of Beginning Day Trade Equity for the given account
    pub open_trade_equity: String,

    /// Indicates the dollar amount of Beginning Day Securities
    pub securities: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// The properties that describe balance characteristics in different currencies.
///
/// NOTE: Only applies to futures.
pub struct CurrencyDetails {
    /// Base currency.
    currency: String,

    /// The net Unrealized Profit or Loss for all currently open positions.
    ///
    /// NOTE: This does not include commissions or routing fees.
    commission: String,

    /// The real time value of an `Account`(s) Cash Balance.
    cash_balance: String,

    #[serde(rename = "RealizedProfitLoss")]
    /// The net Realized Profit or Loss of an `Account` for the current trading day.
    ///
    /// NOTE: This includes all commissions and routing fees.
    realized_pnl: String,

    #[serde(rename = "UnrealizedProfitLoss")]
    /// The net Unrealized Profit or Loss of an `Account` for all currently open positions.
    ///
    /// NOTE: This does not include commissions or routing fees.
    unrealized_pnl: String,

    /// The real time dollar value of Initial Margin for an `Account`.
    ///
    /// NOTE: SUM(Initial Margin of all open positions in the account).
    initial_margin: String,

    /// The real time dollar value of Maintenance Margin for an `Account`.
    ///
    /// NOTE: SUM(Maintenance Margins of all open positions in the account).
    maintenance_margin: String,

    /// The real time conversion rate used to translate value from symbol currency to `Account` currency.
    account_conversion_rate: String,
}
