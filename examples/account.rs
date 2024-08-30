use tradestation_rs::{ClientBuilder, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Example client and authorization
    let mut client = ClientBuilder::new()?
        .set_credentials("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")?
        .authorize("YOUR_AUTHORIZATION_CODE")
        .await?
        .build()
        .await?;

    println!("Your TradeStation API Bearer Token: {:?}", client.token);

    // Example: Get all of your registered `Account`(s)
    let accounts = client.get_accounts().await?;
    for account in accounts.iter() {
        println!("TradeStation Account: {account:?}");

        // Example: Get the balance of an `Account`
        let balance = account.get_balance(&mut client).await?;
        println!("Account Balance: {balance:?}");
    }

    Ok(())
}
