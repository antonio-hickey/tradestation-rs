// Example file for how to use the client for basic account management.

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

    // Example: Get all of your registered `Account`(s)
    let accounts = client.get_accounts().await?;
    println!("Your TradeStation Accounts: {accounts:?}");

    // Example: Get the balances for all your `Account`(s)
    let balances = accounts.get_bod_balances(&mut client).await?;
    println!("Your Balances Per Account: {balances:?}");

    // Example: Get all historic orders (not including open orders) for your `Accounts`
    let order_history = accounts.get_historic_orders(&mut client).await?;
    println!("Your Order History Per Account: {order_history:?}");

    // Example: Find specific account and get only it's balance
    if let Some(futures_account) = accounts.find_by_id("YOUR FUTURES ACCOUNT ID") {
        println!("Your Futures Account: {futures_account:?}");

        // Example: Get the balance of an `Account`
        let balance = futures_account.get_balance(&mut client).await?;
        println!("Futures Account Balance: {balance:?}");

        Ok(())
    } else {
        Err(Error::AccountNotFound)
    }
}
