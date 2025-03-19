use mockito::Server;
use tradestation::{accounting::Account, ClientBuilder};

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
