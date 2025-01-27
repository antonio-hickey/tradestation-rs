// Example file on basic usage for account endoints

use tradestation::{
    account::{MultipleAccounts, OrderStatus, Position, PositionType},
    responses::account::{StreamOrdersResp, StreamPositionsResp},
    ClientBuilder, Error,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Example: initialize client
    // NOTE: With the `Client` you can interact with all of TradeStation's API endpoints,
    // but it's suggested to use the higher level abstractions provided in the examples below.
    let mut client = ClientBuilder::new()?
        .credentials("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")?
        .authorize("YOUR_AUTHORIZATION_CODE")
        .await?
        .build()
        .await?;
    println!("Your TradeStation API Bearer Token: {:?}", client.token);
    //---

    //---
    // Example: Get all of your registered `Account`(s)
    let accounts = client.get_accounts().await?;
    println!("Your TradeStation Accounts: {accounts:?}");
    //---

    //---
    // Example: Get the balances for all your `Account`(s)
    let balances = accounts.get_bod_balances(&mut client).await?;
    println!("Your Balances Per Account: {balances:?}");
    //---

    //---
    // Example: Get all historic orders (not including open orders) for your `Accounts`
    // since some date. NOTE: limited to 90 days prior to current date
    let order_history = accounts
        .get_historic_orders("2024-07-25", &mut client)
        .await?;
    println!("Your Order History Per Account: {order_history:?}");
    //---

    //---
    // Example: Get all the open positions for a specifc account
    if let Some(specific_account) = accounts.find_by_id("SPECIFIC_ACCOUNT_ID") {
        // Example: Get all the open positions for a specifc account
        let positions = specific_account.get_positions(&mut client).await?;
        println!("Open Positions for SPECIFIC_ACCOUNT_ID: {positions:?}");

        // Example: Get the amount of funds allocated to open orders
        let mut funds_allocated_to_open_orders = 0.00;
        specific_account
            .stream_orders(&mut client, |stream_data| {
                // The response type is `responses::account::StreamOrdersResp`
                // which has multiple variants the main one you care about is
                // `Order` which will contain order data sent from the stream.
                match stream_data {
                    StreamOrdersResp::Order(order) => {
                        // Response for an `Order` streamed in
                        println!("{order:?}");

                        // keep a live sum of all the funds allocated to open orders
                        let order_value = order.price_used_for_buying_power.parse::<f64>();
                        if let Ok(value) = order_value {
                            funds_allocated_to_open_orders += value;
                        }
                    }
                    StreamOrdersResp::Heartbeat(heartbeat) => {
                        // Response for periodic signals letting you know the connection is
                        // still alive. A heartbeat is sent every 5 seconds of inactivity.
                        println!("{heartbeat:?}");

                        // for the sake of this example after we recieve the
                        // tenth heartbeat, we will stop the stream session.
                        if heartbeat.heartbeat > 10 {
                            // Example: stopping a stream connection
                            return Err(Error::StopStream);
                        }
                    }
                    StreamOrdersResp::Status(status) => {
                        // Signal sent on state changes in the stream
                        // (closed, opened, paused, resumed)
                        println!("{status:?}");
                    }
                    StreamOrdersResp::Error(err) => {
                        // Response for when an error was encountered,
                        // with details on the error
                        println!("{err:?}");
                    }
                }

                Ok(())
            })
            .await?;

        // Example: collect losing trades into a vector
        let mut losing_positions: Vec<Position> = Vec::new();
        specific_account
            .stream_positions(&mut client, |stream_data| {
                // the response type is `responses::account::streampositionsresp`
                // which has multiple variants the main one you care about is
                // `order` which will contain order data sent from the stream.
                match stream_data {
                    StreamPositionsResp::Position(position) => {
                        // response for an `position` streamed in
                        println!("{position:?}");

                        if position.long_short == PositionType::Long {
                            if position.last < position.average_price {
                                losing_positions.push(*position)
                            }
                        } else if position.long_short == PositionType::Short {
                            if position.last > position.average_price {
                                losing_positions.push(*position)
                            }
                        }

                        // do something with the list of losing trades
                        // maybe send email or text of the positions
                        println!("{losing_positions:?}");
                    }
                    StreamPositionsResp::Heartbeat(heartbeat) => {
                        // response for periodic signals letting you know the connection is
                        // still alive. a heartbeat is sent every 5 seconds of inactivity.
                        println!("{heartbeat:?}");

                        // for the sake of this example after we recieve the
                        // tenth heartbeat, we will stop the stream session.
                        if heartbeat.heartbeat > 10 {
                            // example: stopping a stream connection
                            return Err(Error::StopStream);
                        }
                    }
                    StreamPositionsResp::Status(status) => {
                        // signal sent on state changes in the stream
                        // (closed, opened, paused, resumed)
                        println!("{status:?}");
                    }
                    StreamPositionsResp::Error(err) => {
                        // response for when an error was encountered,
                        // with details on the error
                        println!("{err:?}");
                    }
                }

                Ok(())
            })
            .await?;
        //--

        Ok(())
    } else {
        Err(Error::AccountNotFound)
    }
    //---
}
