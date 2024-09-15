// Example file on basic usage for account endoints

use tradestation_rs::account::MultipleAccounts;
use tradestation_rs::{ClientBuilder, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Example: initialize client
    // NOTE: With the `Client` you can interact with all of TradeStation's API endpoints,
    // but it's suggested to use the higher level abstractions provided in the examples below.
    let mut client = ClientBuilder::new()?
        .set_credentials("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")?
        .authorize("YOUR_AUTHORIZATION_CODE")
        .await?
        .build()
        .await?;
    println!("Your TradeStation API Bearer Token: {:?}", client.token);

    //---
    // Example: Get all of your registered `Account`(s)
    let accounts = client.get_accounts().await?;
    println!("Your TradeStation Accounts: {accounts:?}");
    //---

    //---
    // Example: Get the balances for all your `Account`(s)
    let balances = accounts.get_bod_balances(&mut client).await?;
    println!("Your Balances Per Account: {balances:?}");
    //---

    //---
    // Example: Get all historic orders (not including open orders) for your `Accounts`
    // since some date. NOTE: limited to 90 days prior to current date
    let order_history = accounts
        .get_historic_orders(&mut client, "2024-07-25")
        .await?;
    println!("Your Order History Per Account: {order_history:?}");
    //---

    //---
    // Example: Get all the open positions for a specifc account
    if let Some(specific_account) = accounts.find_by_id("SPECIFIC_ACCOUNT_ID") {
        let positions = specific_account.get_positions(&mut client).await?;

        println!("Open Positions for SPECIFIC_ACCOUNT_ID: {positions:?}");

        Ok(())
    } else {
        Err(Error::AccountNotFound)
    }
    //---
}
