//! Example file on basic usage for order execution endpoints

use tradestation::{ClientBuilder, Error, Token};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Create client
    let mut client = ClientBuilder::new()?
        .set_credentials("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")?
        .set_token(Token {
            access_token: String::from("YOUR_ACCESS_TOKEN"),
            refresh_token: String::from("YOUR_REFRESH_TOKEN"),
            id_token: String::from("YOUR_ID_TOKEN"),
            token_type: String::from("Bearer"),
            scope: String::from("YOUR_SCOPES SPACE_SEPERATED FOR_EACH_SCOPE"),
            expires_in: 1200,
        })?
        .build()
        .await?;

    //--
    // Example: Fetch a list of routes to send orders for execution.
    let routes = client.get_execution_routes().await?;
    println!("Valid routes for order execution: {routes:?}");
    //--

    Ok(())
}
