// Example on how to set up the `Client` to use a mocking/testing server
// to send requests to instead of the default TradeStation API.
use tradestation::{ClientBuilder, ClientEnvironment, Error, Token};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // The address to your test/mock server
    let mock_server_url = "http://127.0.0.1:3865".to_owned();

    // Example: Initialize a `Client` for a test/mock server
    let client = ClientBuilder::new()
        // This will tell the `Client` to send requests
        // to your test/mock server address.
        .environment(ClientEnvironment::Mock(mock_server_url))
        .with_token(Token::dummy())
        .build()
        .await?;

    if let ClientEnvironment::Mock(url) = client.environment {
        println!("Client is using URL: {url}");
    }

    // ...
    // You can now mock whatever endpoints you want and create a
    // custom test environment for you applications, and trading systems.
    // ..

    Ok(())
}
