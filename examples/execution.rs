//! Example file on basic usage for order execution endpoints

use tradestation::{
    accounting::orders::{Order, OrderRelationship, OrderType},
    execution::{
        Duration, OrderRequestBuilder, OrderRequestGroupBuilder, OrderTimeInForce, OrderUpdate,
        TradeAction,
    },
    token::{Scope, Token},
    ClientBuilder, Error,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Create client
    //
    // TIP: Use environment variables instead of hardcoding.
    let client = ClientBuilder::new()
        .credentials("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")
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
    //--

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

    let order = Order::place(&order_req, &client).await?.into_iter().next();
    //--

    //--
    // Example: Replace the order above of 100 shares of JP Morgan
    // to instead be 25 shares at the limit price of $`"222.75"`
    if let Some(order) = order {
        let updated_order = order
            .replace(
                OrderUpdate::new().limit_price("222.75").quantity("25"),
                &client,
            )
            .await?;

        //--
        // Example: Cancel the updated order above
        updated_order.cancel(&client).await?;
    }
    //--

    //--
    // Example: Place a trade involving a bracket group of orders
    // with one order for opening the position, one order for closing
    // the position at a take profit price, and one order for closing
    // the position at a stop loss price. A total of 3 orders making
    // up this position.
    let entry_order_req = OrderRequestBuilder::new()
        .account_id("YOUR_EQUITIES_ACCOUNT_ID")
        .symbol("XLRE")
        .trade_action(TradeAction::SellShort)
        .quantity("1000")
        .order_type(OrderType::Market)
        .time_in_force(OrderTimeInForce {
            duration: Duration::GTC,
            expiration: None,
        })
        .build()?;

    let take_profit_order_req = OrderRequestBuilder::new()
        .account_id("YOUR_EQUITIES_ACCOUNT_ID")
        .symbol("XLRE")
        .trade_action(TradeAction::BuyToCover)
        .quantity("1000")
        .order_type(OrderType::Limit)
        .limit_price("35.75")
        .time_in_force(OrderTimeInForce {
            duration: Duration::GTC,
            expiration: None,
        })
        .build()?;

    let stop_loss_order_req = OrderRequestBuilder::new()
        .account_id("YOUR_EQUITIES_ACCOUNT_ID")
        .symbol("XLRE")
        .trade_action(TradeAction::BuyToCover)
        .quantity("1000")
        .order_type(OrderType::StopMarket)
        .stop_price("46.50")
        .time_in_force(OrderTimeInForce {
            duration: Duration::GTC,
            expiration: None,
        })
        .build()?;

    let order_group = OrderRequestGroupBuilder::new()
        .order_requests(Vec::from([
            entry_order_req,
            take_profit_order_req,
            stop_loss_order_req,
        ]))
        .group_type(OrderRelationship::BRK)
        .build()?;

    let orders = order_group.place(&client).await?;
    println!("Place Orders Result: {orders:?}");
    //--

    Ok(())
}
