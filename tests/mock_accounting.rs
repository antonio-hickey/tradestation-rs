use mockito::Server;
use tradestation::{
    accounting::{accounts::AccountType, Account},
    ClientBuilder,
};

/// Account is the core abstraction around this API
/// since we already have a mock test for get accounts
/// it's fine to use this for other tests as long as
/// the mock test for get accounts passes.
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
/// will return `None`.
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
/// an `Accounts` `BODBalance` is correct.
fn test_get_account_bod_balance_mocked() {
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
