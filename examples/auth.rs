use tradestation_rs::{ClientBuilder, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = ClientBuilder::new()?
        .set_credentials("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")?
        .authorize("YOUR_AUTHORIZATION_CODE")
        .await?
        .build()
        .await?;

    println!("Your TradeStation API Bearer Token: {:?}", client.token);

    Ok(())
}
