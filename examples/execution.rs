//! Example file on basic usage for order execution endpoints

use tradestation::account::OrderType;
use tradestation::execution::{
    Duration, Order, OrderRequestBuilder, OrderTimeInForce, OrderUpdate, TradeAction,
};
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

    let order = Order::place(&mut client, &order_req)
        .await?
        .into_iter()
        .next();
    //--

    //--
    // Example: Replace the order above of 100 shares of JP Morgan
    // to instead be 25 shares at the limit price of $`"222.75"`
    if let Some(order) = order {
        let updated_order = order
            .replace(
                &mut client,
                OrderUpdate::new().limit_price("222.75").quantity("25"),
            )
            .await?
            .into_iter()
            .next();

        //--
        // Example: Cancel the updated order above
        if let Some(order) = updated_order {
            order.cancel(&mut client).await?;
        }
        //--
    }
    //--

    Ok(())
}
