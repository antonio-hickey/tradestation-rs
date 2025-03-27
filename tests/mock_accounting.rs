use mockito::Server;
use tradestation::{
    accounting::{accounts::AccountType, Account, AssetType},
    ClientBuilder,
};

/// Account is the core abstraction around this API
/// so it makes sense to just generate it as needed.
///
/// NOTE: We have mock tests for this so we know it
/// works as long as the tests are passing.
///
/// NOTE: For ALL mocks we will use the account id of `11111111`.
fn generate_mock_account() -> Account {
    Account {
        account_id: String::from("11111111"),
        currency: String::from("USD"),
        account_type: tradestation::accounting::accounts::AccountType::Cash,
        account_detail: None,
    }
}

#[test]
/// This test ensures that the parsing of
/// getting `Account`(s) is correct.
fn test_get_accounts_mocked() {
    // Mock the `accounts` endpoint with a raw JSON
    // string which was a real response, but was then
    // modified to randomize personal information.
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/brokerage/accounts")
        .with_status(200)
        .with_body(
            "{\"Accounts\":[{\"AccountID\":\"11111111\",\"Currency\":\"USD\",\"Status\":\"Active\",\"AccountType\":\"Cash\",\"AccountDetail\":{\"IsStockLocateEligible\":false,\"EnrolledInRegTProgram\":false,\"RequiresBuyingPowerWarning\":false,\"CryptoEnabled\":false,\"DayTradingQualified\":true,\"OptionApprovalLevel\":3,\"PatternDayTrader\":false}},{\"AccountID\":\"111ABC11\",\"Currency\":\"USD\",\"Status\":\"Active\",\"AccountType\":\"Futures\"},{\"AccountID\":\"22222222\",\"Currency\":\"USD\",\"Status\":\"Active\",\"AccountType\":\"Cash\",\"AccountDetail\":{\"IsStockLocateEligible\":false,\"EnrolledInRegTProgram\":false,\"RequiresBuyingPowerWarning\":false,\"CryptoEnabled\":false,\"DayTradingQualified\":false,\"OptionApprovalLevel\":2,\"PatternDayTrader\":false}},{\"AccountID\":\"33333333\",\"Currency\":\"USD\",\"Status\":\"Closing Transactions Only\",\"AccountType\":\"Margin\",\"AccountDetail\":{\"IsStockLocateEligible\":false,\"EnrolledInRegTProgram\":true,\"RequiresBuyingPowerWarning\":true,\"CryptoEnabled\":false,\"DayTradingQualified\":true,\"OptionApprovalLevel\":4,\"PatternDayTrader\":false}},{\"AccountID\":\"222ABC22\",\"Currency\":\"USD\",\"Status\":\"Active\",\"AccountType\":\"Futures\"}]}"
        )
        .create();

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let mut client = ClientBuilder::new()
            .unwrap()
            .testing_url(&server.url())
            .build()
            .await
            .unwrap();

        // Make sure we can parse the mocked response into `Vec<Account>`
        match client.get_accounts().await {
            Ok(accounts) => {
                assert_eq!(accounts.len(), 5);
            }
            Err(e) => {
                panic!("Failed to parse `Account`: {e:?}")
            }
        }
    });

    // Ensure the mock was called
    mock.assert();
}

#[test]
/// This test ensures that the parsing of
/// getting `Account`(s) is correct in specific
/// this makes sure a non existant account id
/// will return `Error::AccountNotFound`.
fn test_get_non_existant_account_mocked() {
    // Mock the `accounts` endpoint with a raw JSON
    // string which was a real response, but was then
    // modified to randomize personal information.
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/brokerage/accounts")
        .with_status(200)
        .with_body(
            "{\"Accounts\":[{\"AccountID\":\"11111111\",\"Currency\":\"USD\",\"Status\":\"Active\",\"AccountType\":\"Cash\",\"AccountDetail\":{\"IsStockLocateEligible\":false,\"EnrolledInRegTProgram\":false,\"RequiresBuyingPowerWarning\":false,\"CryptoEnabled\":false,\"DayTradingQualified\":true,\"OptionApprovalLevel\":3,\"PatternDayTrader\":false}}]}"
        )
        .create();

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let mut client = ClientBuilder::new()
            .unwrap()
            .testing_url(&server.url())
            .build()
            .await
            .unwrap();

        // Make sure we can parse the mocked response into `Vec<Account>`
        match client.get_account("11111112").await {
            Ok(_) => {
                panic!("Failed to send error for non existant account id");
            }
            Err(e) => match e {
                tradestation::Error::AccountNotFound => {}
                _ => panic!("Incorrect error, should be `AccountNotFound`"),
            },
        }
    });

    // Ensure the mock was called
    mock.assert();
}

#[test]
/// This test ensures that the parsing of getting an
/// `Account` (in specific the `AccountType::Cash`
/// type variant) is correct as well as finding
/// correct account.
fn test_get_cash_account_mocked() {
    // Mock the `accounts` endpoint with a raw JSON
    // string which was a real response, but was then
    // modified to randomize personal information.
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/brokerage/accounts")
        .with_status(200)
        .with_body(
            "{\"Accounts\":[{\"AccountID\":\"11111111\",\"Currency\":\"USD\",\"Status\":\"Active\",\"AccountType\":\"Cash\",\"AccountDetail\":{\"IsStockLocateEligible\":false,\"EnrolledInRegTProgram\":false,\"RequiresBuyingPowerWarning\":false,\"CryptoEnabled\":false,\"DayTradingQualified\":true,\"OptionApprovalLevel\":3,\"PatternDayTrader\":false}}]}"
        )
        .create();

    let account_id = "11111111";

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let mut client = ClientBuilder::new()
            .unwrap()
            .testing_url(&server.url())
            .build()
            .await
            .unwrap();

        // Make sure we can parse the mocked response into a `Account`
        match client.get_account(account_id).await {
            Ok(account) => {
                assert_eq!(account.account_id, account_id);
                assert_eq!(account.account_type, AccountType::Cash)
            }
            Err(e) => {
                panic!("Failed to parse `Account`: {e:?}")
            }
        }
    });

    // Ensure the mock was called
    mock.assert();
}

#[test]
/// This test ensures that the parsing of getting an
/// `Account` (in specific the `AccountType::Margin`
/// type variant) is correct as well as finding
/// correct account.
fn test_get_margin_account_mocked() {
    // Mock the `accounts` endpoint with a raw JSON
    // string which was a real response, but was then
    // modified to randomize personal information.
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/brokerage/accounts")
        .with_status(200)
        .with_body(
            "{\"Accounts\":[{\"AccountID\":\"11111111\",\"Currency\":\"USD\",\"Status\":\"Closing Transactions Only\",\"AccountType\":\"Margin\",\"AccountDetail\":{\"IsStockLocateEligible\":false,\"EnrolledInRegTProgram\":true,\"RequiresBuyingPowerWarning\":true,\"CryptoEnabled\":false,\"DayTradingQualified\":true,\"OptionApprovalLevel\":4,\"PatternDayTrader\":false}}]}"
        )
        .create();

    let account_id = "11111111";

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let mut client = ClientBuilder::new()
            .unwrap()
            .testing_url(&server.url())
            .build()
            .await
            .unwrap();

        // Make sure we can parse the mocked response into a `Account`
        match client.get_account(account_id).await {
            Ok(account) => {
                assert_eq!(account.account_id, account_id);
                assert_eq!(account.account_type, AccountType::Margin)
            }
            Err(e) => {
                panic!("Failed to parse `Account`: {e:?}")
            }
        }
    });

    // Ensure the mock was called
    mock.assert();
}

#[test]
/// This test ensures that the parsing of getting an
/// `Account` (in specific the `AccountType::Futures`
/// type variant) is correct as well as finding
/// correct account.
fn test_get_futures_account_mocked() {
    // Mock the `accounts` endpoint with a raw JSON
    // string which was a real response, but was then
    // modified to randomize personal information.
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/brokerage/accounts")
        .with_status(200)
        .with_body("{\"Accounts\":[{\"AccountID\":\"11111111\",\"Currency\":\"USD\",\"Status\":\"Active\",\"AccountType\":\"Futures\"}]}")
        .create();

    let account_id = "11111111";

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let mut client = ClientBuilder::new()
            .unwrap()
            .testing_url(&server.url())
            .build()
            .await
            .unwrap();

        // Make sure we can parse the mocked response into a `Account`
        match client.get_account(account_id).await {
            Ok(account) => {
                assert_eq!(account.account_id, account_id);
                assert_eq!(account.account_type, AccountType::Futures)
            }
            Err(e) => {
                panic!("Failed to parse `Account`: {e:?}")
            }
        }
    });

    // Ensure the mock was called
    mock.assert();
}

#[test]
/// This test ensures that the parsing of getting
/// an `Accounts` `Balance` is correct.
fn test_get_account_balance_mocked() {
    // NOTE: since we already have a mock test for get
    // accounts it's fine to use this for other tests
    // as long as the mock test for get accounts passes.
    let account = generate_mock_account();

    // Mock the `balances` endpoint with a raw JSON
    // string which was a real response, but was then
    // modified to randomize personal information.
    let mut server = Server::new();
    let mock = server
        .mock("GET", format!("/brokerage/accounts/{}/balances", account.account_id).as_str())
        .with_status(200)
        .with_body(
            "{\"Balances\":[{\"AccountID\":\"11111111\",\"AccountType\":\"Cash\",\"CashBalance\":\"3662.31\",\"BuyingPower\":\"3662.31\",\"Equity\":\"43689.0051\",\"MarketValue\":\"43685.6951\",\"TodaysProfitLoss\":\"-100.1949\",\"UnclearedDeposit\":\"0\",\"BalanceDetail\":{\"CostOfPositions\":\"40000.89\",\"DayTrades\":\"0\",\"MaintenanceRate\":\"0.47\",\"OptionBuyingPower\":\"3662.31\",\"OptionsMarketValue\":\"0\",\"OvernightBuyingPower\":\"3662.31\",\"RequiredMargin\":\"43662.56\",\"UnsettledFunds\":\"0\",\"DayTradeExcess\":\"3662.31\",\"RealizedProfitLoss\":\"0\",\"UnrealizedProfitLoss\":\"238.1350999997\"},\"Commission\":\"0\"}],\"Errors\":[]}",
        )
        .create();

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let mut client = ClientBuilder::new()
            .unwrap()
            .testing_url(&server.url())
            .build()
            .await
            .unwrap();

        // Make sure we can parse the mocked response into a `Balance`
        match account.get_balance(&mut client).await {
            Ok(balance) => {
                assert_eq!(account.account_id, balance.account_id);
            }
            Err(e) => {
                panic!("Failed to parse `Balance`: {e:?}")
            }
        }
    });

    // Ensure the mock was called
    mock.assert();
}

#[test]
/// This test ensures that the parsing of getting
/// an `Account`(s) `BODBalance` is correct.
fn test_get_account_bod_balance_mocked() {
    // NOTE: since we already have a mock test for get
    // accounts it's fine to use this for other tests
    // as long as the mock test for get accounts passes.
    let account = generate_mock_account();

    // Mock the `bodbalances` endpoint with a raw JSON
    // string which was a real response, but was then
    // modified to randomize personal information.
    let mut server = Server::new();
    let mock = server
        .mock("GET", format!("/brokerage/accounts/{}/bodbalances", account.account_id).as_str())
        .with_status(200)
        .with_body(
            "{\"BODBalances\":[{\"AccountID\":\"11111111\",\"AccountType\":\"Cash\",\"BalanceDetail\":{\"AccountBalance\":\"3662.31\",\"CashAvailableToWithdraw\":\"3662.31\",\"DayTradingMarginableBuyingPower\":\"3662.31\",\"DayTrades\":\"0\",\"Equity\":\"43689.22\",\"NetCash\":\"3662.31\",\"OptionBuyingPower\":\"3662.31\",\"OptionValue\":\"0\",\"OvernightBuyingPower\":\"3662.31\"}}],\"Errors\":[]}")
        .create();

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let mut client = ClientBuilder::new()
            .unwrap()
            .testing_url(&server.url())
            .build()
            .await
            .unwrap();

        // Make sure we can parse the mocked response into a `BODBalance`
        match account.get_bod_balance(&mut client).await {
            Ok(bod_balance) => {
                assert_eq!(account.account_id, bod_balance.account_id);
            }
            Err(e) => {
                panic!("Failed to parse `BODBalance`: {e:?}")
            }
        }
    });

    // Ensure the mock was called
    mock.assert();
}

#[test]
/// This test ensures that the parsing of getting
/// an `Account`(s) historic `Order`(s) is correct.
///
/// In specific the `AssetType::Future` `Order` variant.
fn test_get_historic_orders_futures_mocked() {
    // NOTE: since we already have a mock test for get
    // accounts it's fine to use this for other tests
    // as long as the mock test for get accounts passes.
    let account = generate_mock_account();

    // Mock the `accounts` endpoint with a raw JSON
    // string which was a real response, but was then
    // modified to randomize personal information.
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/brokerage/accounts/11111111/historicalorders?since=2025-01-23")
        .with_status(200)
        .with_body(
            "{\"Orders\":[{\"AccountID\":\"11111111\",\"CommissionFee\":\"1.5\",\"ClosedDateTime\":\"2025-01-24T18:59:18Z\",\"Currency\":\"USD\",\"Duration\":\"DAY\",\"FilledPrice\":\"108.4375\",\"Legs\":[{\"ExpirationDate\":\"2025-03-20T00:00:00Z\",\"QuantityOrdered\":\"1\",\"ExecQuantity\":\"1\",\"QuantityRemaining\":\"0\",\"BuyOrSell\":\"Buy\",\"Symbol\":\"TYH25\",\"Underlying\":\"TY\",\"AssetType\":\"FUTURE\",\"ExecutionPrice\":\"108.4375\"}],\"LimitPrice\":\"108.4375\",\"OrderID\":\"1124957022\",\"OpenedDateTime\":\"2025-01-24T18:05:03Z\",\"OrderType\":\"Limit\",\"PriceUsedForBuyingPower\":\"108.4375\",\"Status\":\"FLL\",\"StatusDescription\":\"Filled\",\"ConversionRate\":\"1\",\"UnbundledRouteFee\":\"0\"},{\"AccountID\":\"11111111\",\"CommissionFee\":\"3\",\"ClosedDateTime\":\"2025-01-24T18:05:02Z\",\"Currency\":\"USD\",\"Duration\":\"DAY\",\"FilledPrice\":\"108.5\",\"Legs\":[{\"ExpirationDate\":\"2025-03-20T00:00:00Z\",\"QuantityOrdered\":\"2\",\"ExecQuantity\":\"2\",\"QuantityRemaining\":\"0\",\"BuyOrSell\":\"Sell\",\"Symbol\":\"TYH25\",\"Underlying\":\"TY\",\"AssetType\":\"FUTURE\",\"ExecutionPrice\":\"108.5\"}],\"OrderID\":\"1124956996\",\"OpenedDateTime\":\"2025-01-24T18:05:02Z\",\"OrderType\":\"Market\",\"PriceUsedForBuyingPower\":\"108.515625\",\"Status\":\"FLL\",\"StatusDescription\":\"Filled\",\"ConversionRate\":\"1\",\"UnbundledRouteFee\":\"0\"},{\"AccountID\":\"11111111\",\"CommissionFee\":\"3\",\"ClosedDateTime\":\"2025-01-24T18:03:32Z\",\"Currency\":\"USD\",\"Duration\":\"DAY\",\"FilledPrice\":\"108.484375\",\"Legs\":[{\"ExpirationDate\":\"2025-03-20T00:00:00Z\",\"QuantityOrdered\":\"2\",\"ExecQuantity\":\"2\",\"QuantityRemaining\":\"0\",\"BuyOrSell\":\"Sell\",\"Symbol\":\"TYH25\",\"Underlying\":\"TY\",\"AssetType\":\"FUTURE\",\"ExecutionPrice\":\"108.484375\"}],\"LimitPrice\":\"108.484375\",\"OrderID\":\"1124956136\",\"OpenedDateTime\":\"2025-01-24T18:03:31Z\",\"OrderType\":\"Limit\",\"PriceUsedForBuyingPower\":\"108.484375\",\"Status\":\"FLL\",\"StatusDescription\":\"Filled\",\"ConversionRate\":\"1\",\"UnbundledRouteFee\":\"0\"}],\"Errors\":[]}"
        )
        .create();

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let mut client = ClientBuilder::new()
            .unwrap()
            .testing_url(&server.url())
            .build()
            .await
            .unwrap();

        // Make sure we can parse the mocked response into `Vec<Order>`
        match account.get_historic_orders("2025-01-23", &mut client).await {
            Ok(orders) => {
                // Should be 3 orders
                assert_eq!(orders.len(), 3);
                // Account id should match the orders
                assert_eq!(orders[0].account_id, account.account_id);
                // The asset type of the order should be a futures contract
                assert_eq!(orders[0].legs[0].asset_type, AssetType::Future);
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
/// This test ensures that the parsing of getting
/// an `Account`(s) historic `Order`(s) is correct.
///
/// In specific the `AssetType::STOCK` `Order` variant.
fn test_get_historic_orders_stocks_mocked() {
    // NOTE: since we already have a mock test for get
    // accounts it's fine to use this for other tests
    // as long as the mock test for get accounts passes.
    let account = generate_mock_account();

    // Mock the `accounts` endpoint with a raw JSON
    // string which was a real response, but was then
    // modified to randomize personal information.
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/brokerage/accounts/11111111/historicalorders?since=2025-01-23")
        .with_status(200)
        .with_body(
            "{\"Orders\":[{\"AccountID\":\"11111111\",\"CommissionFee\":\"0\",\"ClosedDateTime\":\"2025-03-21T19:49:58Z\",\"Currency\":\"USD\",\"Duration\":\"DAY\",\"FilledPrice\":\"2.59\",\"Legs\":[{\"OpenOrClose\":\"Open\",\"QuantityOrdered\":\"1000\",\"ExecQuantity\":\"1000\",\"QuantityRemaining\":\"0\",\"BuyOrSell\":\"Buy\",\"Symbol\":\"NAT\",\"AssetType\":\"STOCK\",\"ExecutionPrice\":\"2.59\"}],\"LimitPrice\":\"2.59\",\"OrderID\":\"1141645097\",\"OpenedDateTime\":\"2025-03-21T19:48:50Z\",\"OrderType\":\"Limit\",\"PriceUsedForBuyingPower\":\"2.59\",\"Routing\":\"Intelligent\",\"Status\":\"FLL\",\"StatusDescription\":\"Filled\",\"ConversionRate\":\"1\",\"UnbundledRouteFee\":\"0\"},{\"AccountID\":\"11111111\",\"CommissionFee\":\"0\",\"ClosedDateTime\":\"2025-03-14T20:25:00Z\",\"Currency\":\"USD\",\"Duration\":\"DAY\",\"FilledPrice\":\"0\",\"Legs\":[{\"OpenOrClose\":\"Open\",\"QuantityOrdered\":\"1000\",\"ExecQuantity\":\"1000\",\"QuantityRemaining\":\"0\",\"BuyOrSell\":\"Buy\",\"Symbol\":\"NAT\",\"AssetType\":\"STOCK\"}],\"LimitPrice\":\"2.45\",\"OrderID\":\"1139623577\",\"OpenedDateTime\":\"2025-03-14T16:09:54Z\",\"OrderType\":\"Limit\",\"PriceUsedForBuyingPower\":\"2.45\",\"Routing\":\"Intelligent\",\"Status\":\"EXP\",\"StatusDescription\":\"Expired\",\"ConversionRate\":\"1\",\"UnbundledRouteFee\":\"0\"},{\"AccountID\":\"11111111\",\"CommissionFee\":\"0\",\"ClosedDateTime\":\"2025-03-14T13:41:40Z\",\"Currency\":\"USD\",\"Duration\":\"DAY\",\"FilledPrice\":\"7.92\",\"Legs\":[{\"OpenOrClose\":\"Open\",\"QuantityOrdered\":\"250\",\"ExecQuantity\":\"250\",\"QuantityRemaining\":\"0\",\"BuyOrSell\":\"Buy\",\"Symbol\":\"AMDY\",\"AssetType\":\"STOCK\",\"ExecutionPrice\":\"7.92\"}],\"LimitPrice\":\"7.92\",\"OrderID\":\"1139457086\",\"OpenedDateTime\":\"2025-03-14T13:41:28Z\",\"OrderType\":\"Limit\",\"PriceUsedForBuyingPower\":\"7.92\",\"Routing\":\"Intelligent\",\"Status\":\"FLL\",\"StatusDescription\":\"Filled\",\"ConversionRate\":\"1\",\"UnbundledRouteFee\":\"0\"},{\"AccountID\":\"11111111\",\"CommissionFee\":\"0\",\"ClosedDateTime\":\"2025-03-13T18:02:18Z\",\"Currency\":\"USD\",\"Duration\":\"DAY\",\"FilledPrice\":\"18.49\",\"Legs\":[{\"OpenOrClose\":\"Close\",\"QuantityOrdered\":\"100\",\"ExecQuantity\":\"100\",\"QuantityRemaining\":\"0\",\"BuyOrSell\":\"Sell\",\"Symbol\":\"S\",\"AssetType\":\"STOCK\",\"ExecutionPrice\":\"18.49\"}],\"OrderID\":\"1139308736\",\"OpenedDateTime\":\"2025-03-13T18:02:18Z\",\"OrderType\":\"Market\",\"PriceUsedForBuyingPower\":\"18.51\",\"Routing\":\"Intelligent\",\"Status\":\"FLL\",\"StatusDescription\":\"Filled\",\"ConversionRate\":\"1\",\"UnbundledRouteFee\":\"0\"}],\"Errors\":[]}"
        )
        .create();

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let mut client = ClientBuilder::new()
            .unwrap()
            .testing_url(&server.url())
            .build()
            .await
            .unwrap();

        // Make sure we can parse the mocked response into `Vec<Order>`
        match account.get_historic_orders("2025-01-23", &mut client).await {
            Ok(orders) => {
                // Should be 4 orders
                assert_eq!(orders.len(), 4);
                // Account id should match the orders
                assert_eq!(orders[0].account_id, account.account_id);
                // The asset type of the order should be stock
                assert_eq!(orders[0].legs[0].asset_type, AssetType::Stock);
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
/// This test ensures that the parsing of getting
/// an `Account`(s) `Position`(s) is correct.
///
/// In specific the `AssetType::STOCK` `Position` variant.
fn test_get_positions_mocked() {
    // NOTE: since we already have a mock test for get
    // accounts it's fine to use this for other tests
    // as long as the mock test for get accounts passes.
    let account = generate_mock_account();

    // Mock the `positions` endpoint with a raw JSON
    // string which was a real response, but was then
    // modified to randomize into fake data to avoid
    // sensitive info leaking.
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/brokerage/accounts/11111111/positions")
        .with_status(200)
        .with_body(
            "{\"Positions\":[{\"AccountID\":\"11111111\",\"AveragePrice\":\"7.92\",\"AssetType\":\"STOCK\",\"Last\":\"8.48\",\"Bid\":\"8.27\",\"Ask\":\"8.52\",\"ConversionRate\":\"1\",\"DayTradeRequirement\":\"0\",\"InitialRequirement\":\"0\",\"MaintenanceMargin\":\"0\",\"PositionID\":\"222222222\",\"LongShort\":\"Long\",\"Quantity\":\"1050\",\"Symbol\":\"AMDY\",\"Timestamp\":\"2025-03-14T13:41:40Z\",\"TodaysProfitLoss\":\"-294.00\",\"TotalCost\":\"8316.00\",\"MarketValue\":\"8904.00\",\"MarkToMarketPrice\":\"8.76\",\"UnrealizedProfitLoss\":\"7.28\",\"UnrealizedProfitLossPercent\":\"7.071\",\"UnrealizedProfitLossQty\":\"0.56\"},{\"AccountID\":\"11111111\",\"AveragePrice\":\"2.4688888889\",\"AssetType\":\"STOCK\",\"Last\":\"2.56\",\"Bid\":\"2.55\",\"Ask\":\"2.57\",\"ConversionRate\":\"1\",\"DayTradeRequirement\":\"0\",\"InitialRequirement\":\"0\",\"MaintenanceMargin\":\"0\",\"PositionID\":\"222222223\",\"LongShort\":\"Long\",\"Quantity\":\"1800\",\"Symbol\":\"NAT\",\"Timestamp\":\"2025-03-04T14:49:49Z\",\"TodaysProfitLoss\":\"36.00\",\"TotalCost\":\"4446.4400000002\",\"MarketValue\":\"4608.08\",\"MarkToMarketPrice\":\"2.54\",\"UnrealizedProfitLoss\":\"161.64\",\"UnrealizedProfitLossPercent\":\"3.69\",\"UnrealizedProfitLossQty\":\"0.09\"},{\"AccountID\":\"11111111\",\"AveragePrice\":\"70.268\",\"AssetType\":\"STOCK\",\"Last\":\"69.77\",\"Bid\":\"69.48\",\"Ask\":\"69.94\",\"ConversionRate\":\"1\",\"DayTradeRequirement\":\"0\",\"InitialRequirement\":\"0\",\"MaintenanceMargin\":\"0\",\"PositionID\":\"222222224\",\"LongShort\":\"Long\",\"Quantity\":\"125\",\"Symbol\":\"NEE\",\"Timestamp\":\"2025-02-28T16:11:56Z\",\"TodaysProfitLoss\":\"147.5\",\"TotalCost\":\"8783.75\",\"MarketValue\":\"8721.25\",\"MarkToMarketPrice\":\"68.59\",\"UnrealizedProfitLoss\":\"-62.50\",\"UnrealizedProfitLossPercent\":\"-0.709\",\"UnrealizedProfitLossQty\":\"-0.5\"},{\"AccountID\":\"11111111\",\"AveragePrice\":\"83.205\",\"AssetType\":\"STOCK\",\"Last\":\"90.6\",\"Bid\":\"90.55\",\"Ask\":\"90.6\",\"ConversionRate\":\"1\",\"DayTradeRequirement\":\"0\",\"InitialRequirement\":\"0\",\"MaintenanceMargin\":\"0\",\"PositionID\":\"222222225\",\"LongShort\":\"Long\",\"Quantity\":\"2\",\"Symbol\":\"PLTR\",\"Timestamp\":\"2025-02-28T15:13:05Z\",\"TodaysProfitLoss\":\"-11.8\",\"TotalCost\":\"166.41\",\"MarketValue\":\"181.2\",\"MarkToMarketPrice\":\"96.5\",\"UnrealizedProfitLoss\":\"14.79\",\"UnrealizedProfitLossPercent\":\"8.888\",\"UnrealizedProfitLossQty\":\"7.4\"}],\"Errors\":[]}"
        )
        .create();

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let mut client = ClientBuilder::new()
            .unwrap()
            .testing_url(&server.url())
            .build()
            .await
            .unwrap();

        // Make sure we can parse the mocked response into `Vec<Position>`
        match account.get_positions(&mut client).await {
            Ok(positions) => {
                // Should be 4 positions in this test mock data
                assert_eq!(positions.len(), 4);
                // Account id should match the positions
                assert_eq!(positions[0].account_id, account.account_id);
                // The asset type of the position should be stock
                assert_eq!(positions[0].asset_type, AssetType::Stock);
            }
            Err(e) => {
                panic!("Failed to parse `Position`: {e:?}")
            }
        }
    });

    // Ensure the mock was called
    mock.assert();
}
