//! Example file on how to handle the auth process if you already
//! have a token stored either in .env or another safer file.
//!
//! NOTE: If you haven't already performed an initial auth,
//! so you don't have a token saved yet, then see the initial auth example.
//! (https://github.com/antonio-hickey/tradestation-rs/blob/master/examples/initial_auth.rs)

use tradestation::{ClientBuilder, Error, Scope, Token};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // TIP: Use environment variables instead of hardcoding.
    let client = ClientBuilder::new()
        .credentials("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")
        // NOTE: Once you already performed the initial auth, AND have your
        // token stored then you can skip the whole multi step auth flow.
        //
        // This is how most of your app's will work after performing the first
        // initial authorization. See more on the initial auth if not done yet:
        // https://github.com/antonio-hickey/tradestation-rs/blob/master/examples/initial_auth.rs
        .with_token(Token {
            access_token: String::from("YOUR_ACCESS_TOKEN"),
            refresh_token: String::from("YOUR_REFRESH_TOKEN"),
            id_token: String::from("YOUR_ID_TOKEN"),
            token_type: String::from("Bearer"),
            scope: vec![
                Scope::Trade,
                /* ... Your Other Desired Scopes */
            ],
            expires_in: 1200,
        })
        .build()
        .await?;

    // .. now you can use your client to interact with the API

    // Example: Get a quote on the 3 Month Secured Overnight Financing Rate (December 2032).
    let sofr_quote = client.get_quotes(vec!["SR3Z32"]).await?;
    println!("December 2032 3M SOFR Quote: {sofr_quote:?}");

    Ok(())
}
