//! Example file on basic usage for order execution endpoints

use tradestation::account::OrderType;
use tradestation::execution::{Duration, OrderRequestBuilder, OrderTimeInForce, TradeAction};
use tradestation::{ClientBuilder, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Create client
    let mut client = ClientBuilder::new()?
        .set_credentials("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")?
        .authorize("YOUR_AUTHORIZATION_CODE")
        .await?
        .build()
        .await?;
    println!("Your TradeStation API Bearer Token: {:?}", client.token);

    //--
    // Example: Fetch a list of routes to send orders for execution.
    let routes = client.get_execution_routes().await?;
    println!("Valid routes for order execution: {routes:?}");
    //--

    //--
    // Example: Fetch a list of valid activation triggers for order execution.
    let triggers = client.get_activation_triggers().await?;
    println!("Valid activation triggers for order execution: {triggers:?}");
    //---

    //--
    // Example: Place an order to buy 100 shares of JP Morgan (`"JPM"`)
    // using a limit order with the limit price of $`"220.50"`, with
    // a order duration of Good Till Closed.
    let order_req = OrderRequestBuilder::new()
        .account_id("YOUR_EQUITIES_ACCOUNT_ID")
        .symbol("JPM")
        .trade_action(TradeAction::Buy)
        .quantity("100")
        .order_type(OrderType::Limit)
        .limit_price("220.50")
        .time_in_force(OrderTimeInForce {
            duration: Duration::GTC,
            expiration: None,
        })
        .build()?;

    match order_req.place(&mut client).await {
        Ok(resp) => println!("Order Response: {resp:?}"),
        Err(e) => println!("Order Response: {e:?}"),
    }
    //--

    Ok(())
}
