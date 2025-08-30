//! Example file on how to handle the auth process.
//!
//! NOTE: If you already have performed this auth process,
//! AND have your token stored than you can skip this.
//! See this example of token auth instead:
//! (https://github.com/antonio-hickey/tradestation-rs/blob/master/examples/token_auth.rs)

use tradestation::{ClientBuilder, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // The TradeStation API Auth flow is a little annoying, but offers more security.
    //
    // NOTE: If you've already done this step before and have a token saved either in
    // environment or a protected file, then see the token_auth example.
    // (https://github.com/antonio-hickey/tradestation-rs/blob/master/examples/token_auth.rs)
    //
    // Read over the official docs here:
    // https://api.tradestation.com/docs/fundamentals/authentication/auth-code
    //
    // There are 3 stages from our perspective:
    // 1. Client Configuration: This is where we configure our client, for example
    //    set the desired scopes, redirect url, and our API credentials.
    //
    // 2. Client Authorization: This is where we sign into TradeStation using our
    //    actual account credentials (username and password) and auth our client.
    //    A successful authorization will result in a redirect to the `redirect_uri`
    //    set during the client configuration step with an `authorization_code` in
    //    url parameters.
    //
    // 3. Get Client Token: This is where we exhange the authorization code for a
    //    token which can then be used for all requests going forward until expiration.
    //    NOTE: Your token is automatically refreshed on expiration, so as long as your
    //    refresh token isn't expired then you can skip this flow by just using the token.

    // Stage 1: Client Configuration
    let client = ClientBuilder::new()
        .credentials("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")
        .redirect_uri("http://localhost:8080")
        .scopes(["openid", "offline_access", "profile", "MarketData"])
        .start_authorization();

    // Stage 2: Client Authorization: Use the URL which is printed
    // below using a browser to sign in and authorize your client.
    //
    // NOTE: You MUST have a server listening to the redirect URI
    // you configured, so you can use the `authorization_code` sent
    // to the redirect URI in the next stage.
    println!(
        "TradeStation Authorization URL:\n{}",
        client.authorization_url("SOME_STATE_ABC123")?
    );

    // Stage 3: Trade the authorization_code for a token.
    //
    // NOTE: Save your token as environment variables after this, so
    // you can skip this flow for all future uses until the refresh
    // token expires, then you will need to repeat.
    let client = client.exchange_code("YOUR_AUTHORIZATION_CODE").await?;
    let client = client.build().await?;
    println!(
        "Save your token in an environment variables to skip this in the future: {:?}",
        client.token
    );

    // .. now you can use your client to interact with the API

    // Example: Get a quote on Federal Funds Rate (December 2029).
    let fed_funds_quote = client.get_quotes(vec!["FFZ29"]).await?;
    println!("December 2029 Federal Funds Rate Quote: {fed_funds_quote:?}");

    Ok(())
}
