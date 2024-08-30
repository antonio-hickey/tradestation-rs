use crate::{responses::account as responses, Client, Error};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// TradeStation Account
pub struct Account {
    #[serde(rename = "AccountID")]
    /// The main identifier for a TradeStation account
    account_id: String,
    /// The currency the account is based on.
    currency: String,
    /// The type of account, examples: "Cash" or "Margin"
    // TODO: Make enum for this
    account_type: String,
    /// The account details, stuff like options level and day trading approval
    ///
    /// NOTE: This will always be `None` if it's a Futures `Account`
    account_detail: Option<AccountDetail>,
}
impl Account {
    /// Get a specific TradeStation `Account` by it's account id
    pub async fn get(client: &mut Client, account_id: &str) -> Result<Account, Error> {
        if let Some(account) = Account::get_all(client)
            .await?
            .iter()
            .find(|account| account.account_id == account_id)
        {
            Ok(account.clone())
        } else {
            Err(Error::AccountNotFound)
        }
    }

    /// Get all of your registered TradeStation `Account`(s)
    pub async fn get_all(client: &mut Client) -> Result<Vec<Account>, Error> {
        let endpoint = "brokerage/accounts";

        let resp = client
            .get(endpoint)
            .await?
            .json::<responses::GetAccountsResp>()
            .await?;

        Ok(resp.accounts)
    }

    /// Get the current balance of an `Account`
    pub async fn get_balance(&self, client: &mut Client) -> Result<Balance, Error> {
        let endpoint = format!("brokerage/accounts/{}/balances", self.account_id);

        if let Some(balance) = client
            .get(&endpoint)
            .await?
            .json::<responses::GetBalanceResp>()
            .await?
            .balances
            .pop()
        {
            Ok(balance)
        } else {
            Err(Error::AccountNotFound)
        }
    }

    /// Get the current balance of all `Account`(s)
    pub async fn get_balances(
        client: &mut Client,
        account_ids: Vec<&str>,
    ) -> Result<Vec<Balance>, Error> {
        let endpoint = format!("brokerage/accounts/{}/balances", account_ids.join(","));

        let resp = client
            .get(&endpoint)
            .await?
            .json::<responses::GetBalanceResp>()
            .await?;

        Ok(resp.balances)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// Deeper Details for an `Account`
pub struct AccountDetail {
    /// Can account locate securities for borrowing?
    ///
    /// For example if you want to short a stock, you need
    /// to "locate" shares to borrow and sell.
    is_stock_locate_eligible: bool,
    /// Is account enrolled with Regulation T ?
    ///
    /// Regulation T governs cash accounts and the amount of credit that
    /// broker-dealers can extend to investors for the purchase of securities.
    enrolled_in_reg_t_program: bool,
    /// Does the account require a buying power warning before order execution?
    ///
    /// TradeStation uses the greater of Overnight Buying Power or Day Trade
    /// Buying Power to determine if an order should be accepted or rejected.
    ///
    /// In cases where an order exceeds both values, the order will be rejected.
    /// If the order exceeds only one of the values, a Buying Power Warning will
    /// appear to notify you that the order could result in a margin call.
    requires_buying_power_warning: bool,
    /// Is the `Account` qualified for day trading?
    ///
    /// An `Account` MUST maintain a minimum equity balance of $25,000
    /// to be qualified for day trades. *(As per TradeStation compliance rules)*
    day_trading_qualified: bool,
    /// What options level is the `Account` approved for?
    ///
    /// The option approval level will determine what options strategies you will
    /// be able to employ on `Account`. In general terms, the levels are defined as:
    /// Level 0: No options trading allowed.
    /// Level 1: Writing of Covered Calls, Buying Protective Puts.
    /// Level 2: Level 1 + Buying Calls, Buying Puts, Writing Covered Puts.
    /// Level 3: Level 2 + Stock Option Spreads, Index Option Spreads, Butterfly Spreads, Condor Spreads, Iron Butterfly Spreads, Iron Condor Spreads.
    /// Level 4: Level 3 + Writing of Naked Puts (Stock Options).
    /// Level 5: Level 4 + Writing of Naked Puts (Index Options), Writing of Naked Calls (Stock Options), Writing of Naked Calls (Index Options).
    ///
    /// These levels vary depending on the funding and type of account.
    // TODO: Make enum for this
    option_approval_level: u8,
    /// Is the `Account` a Pattern Day Trader?
    ///
    /// As per FINRA rules, an `Account` will be considered a pattern day trader
    /// if it day-trades 4 or more times in 5 business days and it's day-trading
    /// activities are greater than 6 percent of it's total trading activity for
    /// that same five-day period.
    ///
    /// A pattern day trader must maintain minimum equity of $25,000 on any day
    /// that the customer day trades. If the account falls below the $25,000
    /// requirement, the pattern day trader will not be permitted to day trade
    /// until the account is restored to the $25,000 minimum equity level.
    pattern_day_trader: bool,
    /// Is the `Account` enabled to trade crypto?
    ///
    /// NOTE: As of 2024 TradeStation no longer offer's crypto trading.
    crypto_enabled: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Balance {
    #[serde(rename = "AccountID")]
    /// The main identifier for a TradeStation account
    pub account_id: String,
    /// The type of account, examples: "Cash" or "Margin"
    pub account_type: String,
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
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BalanceDetail {
    /// The real time cost for all positions open in the `Account`
    ///
    /// NOTE: Positions are based on the actual entry price
    pub cost_of_positions: Option<String>,
    /// The number of day trades the `Account` has taken over the previous 4 days
    ///
    /// NOTE: This updates daily
    ///
    /// NOTE: This is always None for futures `Account`
    pub day_trades: Option<String>,
    /// The real time dollar amount of required funds for `Account` margin maintenance
    ///
    /// NOTE: SUM(maintenance margin of all open positions in the account)
    ///
    /// NOTE: This is always None for futures `Account`
    pub maintenance_rate: Option<String>,
    /// The real time value of intraday buying power for options
    ///
    /// NOTE: This is always None for futures `Account`
    pub option_buying_power: Option<String>,
    /// The real time Market Value of current open option positions in an `Account`
    pub options_market_value: Option<String>,
    /// The real time Buying Power value that can be held overnight w/o triggering a margin call
    ///
    /// NOTE: (Equity - Overnight Requirement %) / 50 %
    pub overnight_buying_power: Option<String>,
    /// The real time dollar value of open order Day Trade Margins for an `Account`
    ///
    /// NOTE: SUM(Day Trade Margin of all open orders in the account)
    ///
    /// NOTE: Always `None` for cash & margin accounts
    pub day_trade_open_order_margin: Option<String>,
    /// The real time dollar value of open order Initial Margin for an `Account`
    ///
    /// NOTE: SUM(Initial Margin of all open orders in the account)
    ///
    /// NOTE: Always `None` for cash & margin accounts
    pub open_order_margin: Option<String>,
    /// The real time dollar value of Initial Margin for an `Account`
    ///
    /// NOTE: SUM(Initial Margin of all open positions in the account)
    pub initial_margin: Option<String>,
    /// The real time dollar value of Maintenance Margin for an `Account`
    ///
    /// NOTE: SUM(Maintenance Margins of all open positions in the account)
    ///
    /// NOTE: Always `None` for cash & margin accounts
    pub maintenance_margin: Option<String>,
    /// The real time dollar amount of Trade Equity for an `Account`
    ///
    /// NOTE: Always `None` for cash & margin accounts
    pub trade_equity: Option<String>,
    /// The value of special securities deposited with the clearing firm
    /// for the sole purpose of increasing purchasing power in `Account`
    ///
    /// NOTE: This number will be reset daily by the account balances clearing file
    ///
    /// NOTE: The entire value of this field will increase purchasing power
    ///
    /// NOTE: Always `None` for cash & margin accounts
    pub security_on_deposit: Option<String>,
    /// The real time dollar value of Today's Trade Equity for an `Account`
    ///
    /// NOTE: (Beginning Day Trade Equity - Real Time Trade Equity)
    pub today_real_time_trade_equity: Option<String>,
    /// Deeper details on base currency
    ///
    /// NOTE: Always `None` for cash & margin accounts
    pub currency_details: Option<CurrencyDetails>,
    /// The real time amount of required funds for `Account` margin maintenance
    ///
    /// NOTE: The currency denomination is dependant on `Account::currency`
    ///
    /// NOTE: SUM(maintenance margin of all open positions in the account)
    /// NOTE: Always `None` for futures accounts
    pub required_margin: Option<String>,
    /// Funds received by TradeStation that are not settled from a transaction in the `Account`
    /// NOTE: Always `None` for futures accounts
    pub unsettled_funds: Option<String>,
    /// Maintenance Excess
    ///
    /// NOTE: (Cash Balance + Long Market Value + Short Credit - Maintenance Requirement - Margin Debt - Short Market Value)
    pub day_trade_excess: String,
    #[serde(rename = "RealizedProfitLoss")]
    /// The net Realized Profit or Loss of an `Account` for the current trading day
    ///
    /// NOTE: This includes all commissions and routing fees
    pub realized_pnl: String,
    #[serde(rename = "UnrealizedProfitLoss")]
    /// The net Unrealized Profit or Loss of an `Account` for all currently open positions
    ///
    /// NOTE: This does not include commissions or routing fees
    pub unrealized_pnl: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CurrencyDetails {
    /// Base currency
    currency: String,
    /// The net Unrealized Profit or Loss for all currently open positions
    ///
    /// NOTE: This does not include commissions or routing fees
    commission: String,
    /// The real time value of an `Account`(s) Cash Balance
    cash_balance: String,
    #[serde(rename = "RealizedProfitLoss")]
    /// The net Realized Profit or Loss of an `Account` for the current trading day
    ///
    /// NOTE: This includes all commissions and routing fees
    realized_pnl: String,
    #[serde(rename = "UnrealizedProfitLoss")]
    /// The net Unrealized Profit or Loss of an `Account` for all currently open positions
    ///
    /// NOTE: This does not include commissions or routing fees
    unrealized_pnl: String,
    /// The real time dollar value of Initial Margin for an `Account`
    ///
    /// NOTE: SUM(Initial Margin of all open positions in the account)
    initial_margin: String,
    /// The real time dollar value of Maintenance Margin for an `Account`
    ///
    /// NOTE: SUM(Maintenance Margins of all open positions in the account)
    maintenance_margin: String,
    /// The real time conversion rate used to translate value from symbol currency to `Account` currency
    account_conversion_rate: String,
}

impl Client {
    /// Get all of your registered TradeStation `Accounts`
    pub async fn get_accounts(&mut self) -> Result<Vec<Account>, Error> {
        Account::get_all(self).await
    }

    /// Get a specific TradeStation `Account` by it's account id
    pub async fn get_account(&mut self, account_id: &str) -> Result<Account, Error> {
        Account::get(self, account_id).await
    }

    /// Get the current balance of a specific `Account` by it's account id
    pub async fn get_account_balance(&mut self, account_id: &str) -> Result<Balance, Error> {
        let mut balances = Account::get_balances(self, vec![account_id]).await?;
        if balances.len() == 1 {
            // NOTE: This unwrap is panic safe due to invariant above
            let balance = balances.pop().unwrap();
            Ok(balance)
        } else {
            Err(Error::AccountNotFound)
        }
    }

    /// Get the current balance of all `Account`(s)
    pub async fn get_account_balances(
        &mut self,
        account_ids: Vec<&str>,
    ) -> Result<Vec<Balance>, Error> {
        Account::get_balances(self, account_ids).await
    }
}
