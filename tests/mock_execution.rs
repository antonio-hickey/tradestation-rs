use mockito::Server;
use tradestation::accounting::OrderType;
use tradestation::execution::{
    Duration, Order, OrderAssetCategory, OrderRequestBuilder, OrderRequestLeg, OrderTimeInForce,
    OrderUpdate, TradeAction,
};
use tradestation::ClientBuilder;
use tradestation::MarketData::OptionSpreadType;

#[test]
/// This test ensures that the parsing of
/// getting `OrderConfirmation` is correct.
///
/// NOTE: This test is specifically for the `EQUITY` asset category.
fn test_equity_confirm_order_mocked() {
    // Mock the `orderconfirm` endpoint with a raw JSON
    // string which was a real response from the API, but
    // was modified just to randomize personal information.
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/orderexecution/orderconfirm")
        .with_status(200)
        .with_body(
            "{\"Confirmations\":[{\"OrderAssetCategory\":\"EQUITY\",\"Currency\":\"USD\",\"Route\":\"Intelligent\",\"TimeInForce\":{\"Duration\":\"DAY\"},\"AccountID\":\"11111111\",\"OrderConfirmID\":\"i+wE/m+IrUX886i9/51TaH\",\"EstimatedPrice\":\"92.59\",\"EstimatedCost\":\"9259\",\"DebitCreditEstimatedCost\":\"-9259\",\"EstimatedCommission\":\"0\",\"SummaryMessage\":\"Sell Short 100 NVDA @ Market\"}]}"
        )
        .create();

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let client = ClientBuilder::new()
            .unwrap()
            .testing_url(&server.url())
            .build()
            .await
            .unwrap();

        // Build an order request to sell short 100
        // shares of NVDA @ market price.
        let order_req = OrderRequestBuilder::new()
            .account_id("11111111")
            .symbol("NVDA")
            .trade_action(TradeAction::SellShort)
            .quantity("100")
            .order_type(OrderType::Market)
            .time_in_force(OrderTimeInForce {
                duration: Duration::DAY,
                expiration: None,
            })
            .build()
            .unwrap();

        // Confirm the order request built above
        match order_req.confirm(&client).await {
            Ok(order_confirm) => {
                assert!(
                    order_confirm.len() == 1,
                    "Should have a length of 1 as there's only 1 order request sent to confirm"
                );
                assert!(
                    order_confirm[0].account_id == "11111111",
                    "Account ID should be `11111111`, but got {}",
                    order_confirm[0].account_id
                );
                assert!(
                    order_confirm[0].order_asset_category == OrderAssetCategory::Equity,
                    "The asset category for this order should be Equity, but got {:?}",
                    order_confirm[0].order_asset_category
                );
            }
            Err(e) => {
                panic!("Failed to parse `OrderConfirmation`: {e:?}")
            }
        }
    });

    // Ensure the mock was called
    mock.assert();
}

#[test]
/// This test ensures that the parsing of
/// getting `OrderConfirmation` is correct.
///
/// NOTE: This test is specifically for the `OPTION` asset category.
fn test_stock_option_confirm_order_mocked() {
    // Mock the `orderconfirm` endpoint with a raw JSON
    // string which was a real response from the API, but
    // was modified just to randomize personal information.
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/orderexecution/orderconfirm")
        .with_status(200)
        .with_body(
            "{\"Confirmations\":[{\"OrderAssetCategory\":\"OPTION\",\"Currency\":\"USD\",\"Route\":\"Intelligent\",\"TimeInForce\":{\"Duration\":\"GTC\"},\"AccountID\":\"11111111\",\"OrderConfirmID\":\"ctQteTXgHUuC6/p4iWq/FY\",\"EstimatedPrice\":\"-1.16\",\"EstimatedCost\":\"-580\",\"DebitCreditEstimatedCost\":\"-580\",\"EstimatedCommission\":\"12\",\"SummaryMessage\":\"5 INTC - Iron Condor @ -1.16 Limit\\r\\nBuy to Open 5 INTC Apr 11, 25 18 Put\\r\\nSell to Open 5 INTC Apr 11, 25 20.5 Put\\r\\nSell to Open 5 INTC Apr 11, 25 23 Call\\r\\nBuy to Open 5 INTC Apr 11, 25 25.5 Call\",\"Underlying\":\"INTC\",\"Legs\":[{\"ExpirationDate\":\"2025-04-11T00:00:00Z\",\"OptionType\":\"PUT\",\"Quantity\":\"5\",\"StrikePrice\":\"18\",\"Symbol\":\"INTC 250411P18\",\"TradeAction\":\"BUYTOOPEN\"},{\"ExpirationDate\":\"2025-04-11T00:00:00Z\",\"OptionType\":\"PUT\",\"Quantity\":\"5\",\"StrikePrice\":\"20.5\",\"Symbol\":\"INTC 250411P20.5\",\"TradeAction\":\"SELLTOOPEN\"},{\"ExpirationDate\":\"2025-04-11T00:00:00Z\",\"OptionType\":\"CALL\",\"Quantity\":\"5\",\"StrikePrice\":\"23\",\"Symbol\":\"INTC 250411C23\",\"TradeAction\":\"SELLTOOPEN\"},{\"ExpirationDate\":\"2025-04-11T00:00:00Z\",\"OptionType\":\"CALL\",\"Quantity\":\"5\",\"StrikePrice\":\"25.5\",\"Symbol\":\"INTC 250411C25.5\",\"TradeAction\":\"BUYTOOPEN\"}],\"LimitPrice\":\"-1.16\",\"Spread\":\"IronCondor\"}]}"
        )
        .create();

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let client = ClientBuilder::new()
            .unwrap()
            .testing_url(&server.url())
            .build()
            .await
            .unwrap();

        // Build an order request to sell 5 Iron Condors on Intel (INTC)
        //
        // NOTE: This order is made up of 4 separate order legs:
        // 1. Buying 5 Out Of The Money (OTM) Puts @ the $18 strike (exp: 04/11/25) (Wing)
        // 2. Selling 5 At The Money (ATM) Puts @ the $20.5 strike (exp: 04/11/25) (Range)
        // 3. Selling 5 Near The Money (NTM) Calls @ the $23 strike (exp: 04/11/25) (Range)
        // 4. Buying 5 Out Of The Money (OTM) Calls @ the $25 strike (exp: 04/11/25) (Wing)
        //
        // The total cost is a credit of 1.16 (* 100 = $116 (* 5 = $580))
        let order_req = OrderRequestBuilder::new()
            .account_id("11111111")
            .order_type(OrderType::Limit)
            .limit_price("-1.16")
            .time_in_force(OrderTimeInForce {
                duration: Duration::GTC,
                expiration: None,
            })
            .legs(vec![
                OrderRequestLeg {
                    symbol: "INTC 250411P18".into(),
                    trade_action: TradeAction::BuyToOpen,
                    quantity: "5".into(),
                    option_type: None,
                    expiration_date: None,
                    strike_price: None,
                },
                OrderRequestLeg {
                    symbol: "INTC 250411P20.5".into(),
                    trade_action: TradeAction::SellToOpen,
                    quantity: "5".into(),
                    option_type: None,
                    expiration_date: None,
                    strike_price: None,
                },
                OrderRequestLeg {
                    symbol: "INTC 250411C23".into(),
                    trade_action: TradeAction::SellToOpen,
                    quantity: "5".into(),
                    option_type: None,
                    expiration_date: None,
                    strike_price: None,
                },
                OrderRequestLeg {
                    symbol: "INTC 250411C25.5".into(),
                    trade_action: TradeAction::BuyToOpen,
                    quantity: "5".into(),
                    option_type: None,
                    expiration_date: None,
                    strike_price: None,
                },
            ])
            .build()
            .unwrap();

        // Confirm the order request built above
        match order_req.confirm(&client).await {
            Ok(order_confirm) => {
                assert!(
                    order_confirm.len() == 1,
                    "Should have a length of 1 as there's only 1 order request sent to confirm"
                );
                assert!(
                    order_confirm[0].account_id == "11111111",
                    "Account ID should be `11111111`, but got {}",
                    order_confirm[0].account_id
                );
                assert!(
                    order_confirm[0].order_asset_category == OrderAssetCategory::Option,
                    "The asset category for this order should be Option, but got {:?}",
                    order_confirm[0].order_asset_category
                );

                if let Some(spread) = order_confirm[0].spread.as_ref() {
                    assert!(
                        *spread == OptionSpreadType::IronCondor,
                        "The spread should be of type `OptionSpreadType::IronCondor`, but got {:?}",
                        spread
                    );
                } else {
                    panic!(
                        "The spread should be of type `OptionSpreadType::IronCondor`, but got None"
                    )
                }

                if let Some(order_legs) = order_confirm[0].legs.as_ref() {
                    assert!(
                        order_legs.len() == 4,
                        "The order should have 4 order legs, but got {:?}",
                        order_legs.len()
                    );
                } else {
                    panic!("The order should have 4 order legs, but got None")
                }
            }
            Err(e) => {
                panic!("Failed to parse `OrderConfirmation`: {e:?}")
            }
        }
    });

    // Ensure the mock was called
    mock.assert();
}

#[test]
/// This test ensures that the parsing of
/// getting `OrderConfirmation` is correct.
///
/// NOTE: This test is specifically for the `FUTURE` asset category.
fn test_future_confirm_order_mocked() {
    // Mock the `orderconfirm` endpoint with a raw JSON
    // string which was a real response from the API, but
    // was modified just to randomize personal information.
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/orderexecution/orderconfirm")
        .with_status(200)
        .with_body(
            "{\"Confirmations\":[{\"OrderAssetCategory\":\"FUTURE\",\"Currency\":\"USD\",\"Route\":\"Intelligent\",\"TimeInForce\":{\"Duration\":\"DAY\"},\"AccountID\":\"111ABC22\",\"OrderConfirmID\":\"LHkNu5iF/UqxTc9fMrO7Id\",\"EstimatedPrice\":\"113\",\"EstimatedCost\":\"10315\",\"EstimatedCommission\":\"0\",\"InitialMarginDisplay\":\"10,315.00 USD\",\"ProductCurrency\":\"USD\",\"AccountCurrency\":\"USD\",\"SummaryMessage\":\"Buy 5 TYM25 @ Market\"}]}"
        )
        .create();

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let client = ClientBuilder::new()
            .unwrap()
            .testing_url(&server.url())
            .build()
            .await
            .unwrap();

        // Build an order request to buy 5 contracts
        // on the 10 Year US Treasury @ market price.
        let order_req = OrderRequestBuilder::new()
            .account_id("111ABC22")
            .symbol("TYM25")
            .trade_action(TradeAction::Buy)
            .quantity("5")
            .order_type(OrderType::Market)
            .time_in_force(OrderTimeInForce {
                duration: Duration::DAY,
                expiration: None,
            })
            .build()
            .unwrap();

        // Confirm the order request built above
        match order_req.confirm(&client).await {
            Ok(order_confirm) => {
                assert!(
                    order_confirm.len() == 1,
                    "Should have a length of 1 as there's only 1 order request sent to confirm"
                );
                assert!(
                    order_confirm[0].account_id == "111ABC22",
                    "Account ID should be `111ABC22`, but got {}",
                    order_confirm[0].account_id
                );
                assert!(
                    order_confirm[0].order_asset_category == OrderAssetCategory::Future,
                    "The asset category for this order should be Future, but got {:?}",
                    order_confirm[0].order_asset_category
                );
            }
            Err(e) => {
                panic!("Failed to parse `OrderConfirmation`: {e:?}")
            }
        }
    });

    // Ensure the mock was called
    mock.assert();
}

#[test]
/// This test ensures that the parsing of placing an
/// `OrderRequest` and getting an `Order` back is correct.
fn test_place_order_mocked() {
    // Mock the `orderconfirm` endpoint with a raw JSON
    // string which was a real response from the API, but
    // was modified just to randomize personal information.
    let mut server = Server::new();
    let mock = server
        .mock("POST", "/orderexecution/orders")
        .with_status(200)
        .with_body(
            "{\"Orders\":[{\"Message\":\"Sent order: Buy 100 PLTR @ 75.00 Limit\",\"OrderID\":\"5555555555\"}]}"
        )
        .create();

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let client = ClientBuilder::new()
            .unwrap()
            .testing_url(&server.url())
            .build()
            .await
            .unwrap();

        // Build an order request to buy 100 shares
        // of Palantir at a limit price of $75.00
        let order_req = OrderRequestBuilder::new()
            .account_id("11111111")
            .symbol("PLTR")
            .trade_action(TradeAction::Buy)
            .quantity("100")
            .order_type(OrderType::Limit)
            .limit_price("75.00")
            .time_in_force(OrderTimeInForce {
                duration: Duration::DAY,
                expiration: None,
            })
            .build()
            .unwrap();

        // Place the order request built above
        match Order::place(&order_req, &client).await {
            Ok(orders) => {
                assert!(
                    orders.len() == 1,
                    "Should have a length of 1 as there's only 1 order request placed"
                );
            }
            Err(e) => {
                panic!("Failed to parse `Order`: {e:?}")
            }
        }
    });

    // Ensure the mock was called
    mock.assert();
}

#[test]
/// This test ensures replacing an `Order` works
/// and the response is parsed correctly.
fn test_replace_order_mocked() {
    // Mock the `orderconfirm` endpoint with a raw JSON
    // string which was a real response from the API, but
    // was modified just to randomize personal information.
    let mut server = Server::new();
    let mock = server
        .mock("PUT", "/orderexecution/orders/5555555555")
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body("{\"Message\":\"Cancel/Replace order sent.\",\"OrderID\":\"5555555555\"}")
        .create();

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let client = ClientBuilder::new()
            .unwrap()
            .testing_url(&server.url())
            .build()
            .await
            .unwrap();

        // Update some order to have a quantity of 25
        let order = Order::from_id("5555555555");
        let order_update = OrderUpdate::new().quantity("25");

        // Replace the order with the order update
        match order.replace(order_update, &client).await {
            Ok(order) => {
                assert!(
                    order.order_id == "5555555555",
                    "Should have an order id of `5555555555`, but got {}",
                    order.order_id
                );
            }
            Err(e) => {
                panic!("Failed to parse `Order`: {e:?}")
            }
        }
    });

    // Ensure the mock was called
    mock.assert();
}

#[test]
/// This test ensures canceling an `Order` works
/// and the response is parsed correctly.
fn test_cancel_order_mocked() {
    // Mock the `orderconfirm` endpoint with a raw JSON
    // string which was a real response from the API, but
    // was modified just to randomize personal information.
    let mut server = Server::new();
    let mock = server
        .mock("DELETE", "/orderexecution/orders/5555555555")
        .with_status(200)
        .with_body("{\"Message\":\"Cancel request sent\",\"OrderID\":\"5555555555\"}")
        .create();

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let client = ClientBuilder::new()
            .unwrap()
            .testing_url(&server.url())
            .build()
            .await
            .unwrap();

        // Some order to cancel
        let order = Order::from_id("5555555555");

        // Cancel the order
        match order.cancel(&client).await {
            Ok(order) => {
                assert!(
                    order.order_id == "5555555555",
                    "Should have an order id of `5555555555`, but got {}",
                    order.order_id
                );
            }
            Err(e) => {
                panic!("Failed to parse `Order`: {e:?}")
            }
        }
    });

    // Ensure the mock was called
    mock.assert();
}
