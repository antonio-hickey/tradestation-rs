use crate::account::{Account, Balance};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// The TradeStation API Response for getting accounts.
pub struct GetAccountsResp {
    pub accounts: Vec<Account>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// The TradeStation API Response for getting account's balance.
// TODO: This also gives a key for errors, look into using these.
pub struct GetBalanceResp {
    pub balances: Vec<Balance>,
}
