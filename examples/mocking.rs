// Example on how to set up the `Client` to use a mocking/testing server
// to send requests to instead of the default TradeStation API.

use tradestation::{ClientBuilder, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // The address to your test/mock server
    let mock_server_url = "127.0.0.1:3865";

    // Example: Initialize a `Client` for a test/mock server
    let client = ClientBuilder::new()?
        // This will tell the `Client` to send requests
        // to your test/mock server address.
        .testing_url(mock_server_url)
        .build()
        .await?;

    println!("Client is using URL: {}", client.base_url);

    Ok(())
}
