//! Example file on basic usage for order execution endpoints

use tradestation::account::OrderType;
use tradestation::execution::{
    Duration, Order, OrderGroupType, OrderRequestBuilder, OrderRequestGroupBuilder,
    OrderTimeInForce, OrderUpdate, TradeAction,
};
use tradestation::{ClientBuilder, Error, Token};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Create client
    let mut client = ClientBuilder::new()?
        .credentials("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")?
        .token(Token {
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
        .group_type(OrderGroupType::BRK)
        .build()?;

    let orders = order_group.place(&mut client).await?;
    println!("Place Orders Result: {orders:?}");
    //--

    Ok(())
}
