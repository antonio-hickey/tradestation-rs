use crate::{
    responses::{
        account::{GetAccountsResp, GetOrdersResp, StreamOrdersResp, StreamPositionsResp},
        ApiResponse,
    },
    Client, Error,
};
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, error::Error as StdErrorTrait, future::Future, pin::Pin};

use super::{BODBalance, Balance, Order, Position};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// TradeStation Account
pub struct Account {
    #[serde(rename = "AccountID")]
    /// The main identifier for a TradeStation account.
    pub account_id: String,

    /// The currency the account is based on.
    pub currency: String,

    /// The type of account, examples: "Cash" or "Margin"
    pub account_type: AccountType,

    /// The account details, stuff like options level and day trading approval
    ///
    /// NOTE: This will always be `None` if it's a Futures `Account`
    pub account_detail: Option<AccountDetail>,
}
impl Account {
    /// Get a specific TradeStation `Account` by it's account id.
    pub async fn get(account_id: &str, client: &Client) -> Result<Account, Error> {
        if let Some(account) = Account::get_all(client)
            .await?
            .iter()
            .find(|account| account.account_id == account_id)
        {
            Ok(account.clone())
        } else {
            Err(Error::AccountNotFound)
        }
    }

    /// Get all of your registered TradeStation `Account`(s).
    pub async fn get_all(client: &Client) -> Result<Vec<Account>, Error> {
        let endpoint = "brokerage/accounts";

        match client
            .get(endpoint)
            .await?
            .json::<ApiResponse<GetAccountsResp>>()
            .await?
        {
            ApiResponse::Success(resp) => Ok(resp.accounts),
            ApiResponse::Error(resp) => Err(Error::from_api_error(resp)),
        }
    }

    /// Get the current balance of an `Account`.
    pub async fn get_balance(&self, client: &Client) -> Result<Balance, Error> {
        Balance::get(&self.account_id, client).await
    }

    /// Get the current balance of all `Account`(s) by account ids.
    ///
    /// NOTE: If you have `Vec<Account>` you should instead use `Vec<Account>::get_balances()`
    /// this method should only be used in cases where you ONLY have account id's.
    pub async fn get_balances_by_accounts(
        account_ids: Vec<&str>,
        client: &Client,
    ) -> Result<Vec<Balance>, Error> {
        Balance::get_multiple(account_ids, client).await
    }

    /// Get the beginning of day balance of an `Account`.
    pub async fn get_bod_balance(&self, client: &Client) -> Result<BODBalance, Error> {
        BODBalance::get(&self.account_id, client).await
    }

    /// Get the beginning of day balances for multiple `Account`(s) by account id.
    ///
    /// NOTE: If you have `Vec<Account>` you should instead use `Vec<Account>::get_bod_balances()`
    /// this method should only be used if you ONLY have account id's.
    pub async fn get_bod_balances_by_accounts(
        account_ids: Vec<&str>,
        client: &Client,
    ) -> Result<Vec<BODBalance>, Error> {
        BODBalance::get_multiple(account_ids, client).await
    }

    /// Fetches Historical `Order`(s) since a specific date for the given `Account`.
    ///
    /// NOTE: Date format is {YEAR-MONTH-DAY} ex: `"2024-07-09"`, and is limited to 90
    /// days prior to the current date.
    ///
    /// NOTE: Excludes open `Order`(s) and is sorted in descending order of time closed.
    pub async fn get_historic_orders(
        &self,
        since_date: &str,
        client: &Client,
    ) -> Result<Vec<Order>, Error> {
        Order::get_historic(&self.account_id, since_date, client).await
    }

    /// Fetches Historical `Order`(s) for the given `Account`(s) by id.
    ///
    /// NOTE: Date format is {YEAR-MONTH-DAY} ex: `"2024-07-09"`, and is limited to 90
    /// days prior to the current date.
    ///
    /// NOTE: Excludes open `Order`(s) and is sorted in descending order of time closed.
    pub async fn get_historic_orders_by_accounts(
        account_ids: Vec<&str>,
        since_date: &str,
        client: &Client,
    ) -> Result<Vec<Order>, Error> {
        Order::get_historic_by_accounts(account_ids, since_date, client).await
    }

    /// Fetches orders for the given `Account`.
    ///
    /// # Example
    /// ---
    ///
    /// Grab all the orders for a specific account. Say you need to go
    /// through all the orders your algorithm placed today and filter out
    /// only the orders that were filled for data storage purposes.
    ///
    /// ```ignore
    /// // Initialize the client
    /// let client = ClientBuilder::new()?
    ///     .credentials("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")?
    ///     .token(Token {
    ///         access_token: String::from("YOUR_ACCESS_TOKEN"),
    ///         refresh_token: String::from("YOUR_REFRESH_TOKEN"),
    ///         id_token: String::from("YOUR_ID_TOKEN"),
    ///         token_type: String::from("Bearer"),
    ///         scope: String::from("YOUR_SCOPES SPACE_SEPERATED FOR_EACH_SCOPE"),
    ///         expires_in: 1200,
    ///     })?
    ///     .build()
    ///     .await?;
    ///
    /// // Grab your accounts and specify an account the orders were placed in
    /// let accounts = client.get_accounts().await?;
    /// if let Some(specific_account) = accounts.find_by_id("YOUR_ACCOUNT_ID") {
    ///     // Get all the orders from today for a specific account
    ///     let orders = specific_account.get_orders(&client).await?;
    ///
    ///     // Filter out only filled orders
    ///     let filled_orders: Vec<Order> = orders
    ///         .into_iter()
    ///         .filter(|order| order.status == OrderStatus::FLL)
    ///         .collect();
    ///
    ///     // Do something with your filled orders
    ///     for order in filled_orders {
    ///         println!("Filled Order: {order:?}");
    ///     }
    /// }
    /// ```
    pub async fn get_orders(&self, client: &Client) -> Result<Vec<Order>, Error> {
        Order::get_all_by_account(&self.account_id, client).await
    }

    /// NOTE: Same as `get_orders` but for multiple accounts
    /// NOTE: For internal use only. Use `Account::get_orders_by_id()`
    /// to access this functionality.
    async fn get_orders_for_accounts<S: Into<String>>(
        account_ids: Vec<S>,
        client: &Client,
    ) -> Result<Vec<Order>, Error> {
        let account_ids: Vec<String> = account_ids
            .into_iter()
            .map(|account_id| account_id.into())
            .collect();

        let endpoint = format!("brokerage/accounts/{}/orders", account_ids.join(","));

        match client
            .get(&endpoint)
            .await?
            .json::<ApiResponse<GetOrdersResp>>()
            .await?
        {
            ApiResponse::Success(resp) => Ok(resp.orders),
            ApiResponse::Error(resp) => Err(Error::from_api_error(resp)),
        }
    }

    /// Fetches orders by order id for the given `Account`.
    ///
    /// # Example
    /// ---
    ///
    /// Grab 2 specific orders by their id's, say you have a stop loss order
    /// and a take profit order you want to check the status on, this is how.
    ///
    /// ```ignore
    /// // Initialize the client
    /// let client = ClientBuilder::new()?
    ///     .credentials("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")?
    ///     .token(Token {
    ///         access_token: String::from("YOUR_ACCESS_TOKEN"),
    ///         refresh_token: String::from("YOUR_REFRESH_TOKEN"),
    ///         id_token: String::from("YOUR_ID_TOKEN"),
    ///         token_type: String::from("Bearer"),
    ///         scope: String::from("YOUR_SCOPES SPACE_SEPERATED FOR_EACH_SCOPE"),
    ///         expires_in: 1200,
    ///     })?
    ///     .build()
    ///     .await?;
    ///
    /// // Grab your accounts and specify an account the orders were placed in
    /// let accounts = client.get_accounts().await?;
    /// if let Some(specific_account) = accounts.find_by_id("YOUR_ACCOUNT_ID") {
    ///     // Get some specific orders by their order id's
    ///     let orders = specific_account.
    ///         get_orders_by_id(vec!["1115661503", "1115332365"], &client)
    ///         .await?;
    ///
    ///     // Log the status of the order's
    ///     for order in orders {
    ///         println!("Order ID ({}) status: {}", order.order_id, order.status);
    ///     }
    /// }
    /// ```
    pub async fn get_orders_by_id<S: Into<String>>(
        &self,
        order_ids: Vec<S>,
        client: &Client,
    ) -> Result<Vec<Order>, Error> {
        Order::find(order_ids, self.account_id.clone(), client).await
    }

    /// NOTE: Same as `get_orders_by_id` but for multiple accounts
    /// NOTE: For internal use only. Use `Account::get_orders_by_id()`
    /// to access this functionality.
    async fn get_orders_by_id_for_accounts<S: Into<String>>(
        account_ids: Vec<S>,
        order_ids: Vec<S>,
        client: &Client,
    ) -> Result<Vec<Order>, Error> {
        let account_ids: Vec<String> = account_ids
            .into_iter()
            .map(|account_id| account_id.into())
            .collect();

        let order_ids: Vec<String> = order_ids
            .into_iter()
            .map(|order_id| order_id.into())
            .collect();

        let endpoint = format!(
            "brokerage/accounts/{}/orders/{}",
            account_ids.join(","),
            &order_ids.join(",")
        );

        match client
            .get(&endpoint)
            .await?
            .json::<ApiResponse<GetOrdersResp>>()
            .await?
        {
            ApiResponse::Success(resp) => Ok(resp.orders),
            ApiResponse::Error(resp) => Err(Error::from_api_error(resp)),
        }
    }

    /// Fetches positions for the given `Account`.
    pub async fn get_positions(&self, client: &Client) -> Result<Vec<Position>, Error> {
        Position::get_by_account(&self.account_id, client).await
    }

    /// NOTE: Same as `Account::get_position` but for multiple accounts
    /// NOTE: For internal use only. Use `MultipleAccounts::get_position()`
    /// instead to access this functionality.
    async fn get_position_for_accounts(
        account_ids: String,
        position_id: String,
        client: &Client,
    ) -> Result<Position, Error> {
        let positions = Position::find(vec![position_id], account_ids, client).await?;

        let position = positions[0].clone();
        Ok(position)
    }

    /// Fetches specific `Position`(s) by their id for the `Account`.
    ///
    /// # Example
    /// ---
    ///
    /// Grab specific positions, say you need to check for updates on 2 specific
    /// positions and you already know their position ids, here's how you would do it.
    ///
    /// ```ignore
    /// // Initialize the client
    /// let client = ClientBuilder::new()?
    ///     .credentials("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")?
    ///     .token(Token {
    ///         access_token: String::from("YOUR_ACCESS_TOKEN"),
    ///         refresh_token: String::from("YOUR_REFRESH_TOKEN"),
    ///         id_token: String::from("YOUR_ID_TOKEN"),
    ///         token_type: String::from("Bearer"),
    ///         scope: String::from("YOUR_SCOPES SPACE_SEPERATED FOR_EACH_SCOPE"),
    ///         expires_in: 1200,
    ///     })?
    ///     .build()
    ///     .await?;
    ///
    /// // Grab the account where the position exists
    /// let account = client
    ///     .get_accounts()
    ///     .await?
    ///     .find_by_id("YOUR_ACCOUNT_ID")
    ///     .unwrap();
    ///
    /// let positions = account
    ///     .get_positions_by_ids(
    ///         vec!["YOUR_POSITION_ID_1", "YOUR_POSITION_ID_2"]
    ///     )
    ///     .await?;
    /// println!("Positions: {positions:?}");
    /// ```
    pub async fn get_positions_by_id<S: Into<String>>(
        &self,
        position_ids: Vec<S>,
        client: &Client,
    ) -> Result<Vec<Position>, Error> {
        Position::find(position_ids, self.account_id.clone(), client).await
    }

    /// NOTE: Same as `get_positions_by_id` but for multiple accounts
    /// NOTE: For internal use only. Use `Account::get_positions_by_id()`
    /// instead to access this functionality.
    async fn get_positions_by_id_for_accounts<S: Into<String>>(
        account_ids: String,
        position_ids: Vec<S>,
        client: &Client,
    ) -> Result<Vec<Position>, Error> {
        Position::find(position_ids, account_ids, client).await
    }

    /// Fetches positions for the given `Account`.
    ///
    /// NOTE: symbol should be a str of valid symbols in comma separated format;
    /// for example: `"MSFT,MSFT *,AAPL"`.
    ///
    /// NOTE: You can use an * as wildcard to make more complex filters.
    pub async fn get_positions_in_symbols(
        &self,
        symbols: &str,
        client: &Client,
    ) -> Result<Vec<Position>, Error> {
        Position::get_by_symbols(symbols, &self.account_id, client).await
    }

    /// Fetches positions for the given `Account`(s).
    ///
    /// NOTE: If you have `Vec<Account>` you should instead use `Vec<Account>::get_positions()`
    /// this method should only be used if you ONLY have account id's.
    pub async fn get_positions_by_accounts(
        account_ids: Vec<&str>,
        client: &Client,
    ) -> Result<Vec<Position>, Error> {
        Position::get_by_accounts(account_ids, client).await
    }

    /// Fetches positions for the given `Account`(s).
    ///
    /// NOTE: If you have `Vec<Account>` you should instead use `Vec<Account>::get_positions_in_symbols()`
    /// this method should only be used if you ONLY have account id's.
    ///
    /// NOTE: symbol should be a str of valid symbols in comma separated format;
    /// for example: `"MSFT,MSFT *,AAPL"`.
    ///
    /// NOTE: You can use an * as wildcard to make more complex filters.
    pub async fn get_positions_in_symbols_by_accounts(
        symbols: &str,
        account_ids: Vec<&str>,
        client: &Client,
    ) -> Result<Vec<Position>, Error> {
        Position::get_by_symbols_and_accounts(symbols, account_ids, client).await
    }

    /// Stream `Order`(s) for the given `Account`.
    ///
    /// NOTE: You need to pass a closure function that will handle
    /// each chunk of data (`StreamOrdersResp`) as it's streamed in.
    ///
    /// # Example
    /// ---
    ///
    /// Get the amount of funds allocated to open orders.
    /// ```ignore
    /// let mut funds_allocated_to_open_orders = 0.00;
    /// specific_account
    ///     .stream_orders(&client, |stream_data| {
    ///         // The response type is `responses::account::StreamOrdersResp`
    ///         // which has multiple variants the main one you care about is
    ///         // `Order` which will contain order data sent from the stream.
    ///         match stream_data {
    ///             StreamOrdersResp::Order(order) => {
    ///                 // Response for an `Order` streamed in
    ///                 println!("{order:?}");
    ///
    ///                 // keep a live sum of all the funds allocated to open orders
    ///                 let order_value = order.price_used_for_buying_power.parse::<f64>();
    ///                 if let Ok(value) = order_value {
    ///                     funds_allocated_to_open_orders += value;
    ///                 }
    ///             }
    ///             StreamOrdersResp::Heartbeat(heartbeat) => {
    ///                 // Response for periodic signals letting you know the connection is
    ///                 // still alive. A heartbeat is sent every 5 seconds of inactivity.
    ///                 println!("{heartbeat:?}");
    ///
    ///                 // for the sake of this example after we recieve the
    ///                 // tenth heartbeat, we will stop the stream session.
    ///                 if heartbeat.heartbeat > 10 {
    ///                     // Example: stopping a stream connection
    ///                     return Err(Error::StopStream);
    ///                 }
    ///             }
    ///             StreamOrdersResp::Status(status) => {
    ///                 // Signal sent on state changes in the stream
    ///                 // (closed, opened, paused, resumed)
    ///                 println!("{status:?}");
    ///             }
    ///             StreamOrdersResp::Error(err) => {
    ///                 // Response for when an error was encountered,
    ///                 // with details on the error
    ///                 println!("{err:?}");
    ///             }
    ///         }
    ///
    ///         Ok(())
    ///     })
    ///     .await?;
    /// ```
    pub async fn stream_orders<F>(&self, client: &Client, on_chunk: F) -> Result<Vec<Order>, Error>
    where
        F: FnMut(StreamOrdersResp) -> Result<(), Error>,
    {
        Order::stream(&self.account_id, client, on_chunk).await
    }

    /// Stream `Order`(s) by order id's for the given `Account`.
    ///
    /// NOTE: order ids should be a comma delimited string slice `"xxxxx,xxxxx,xxxxx"`
    ///
    /// NOTE: You need to pass a closure function that will handle
    /// each chunk of data `StreamOrdersResp` as it's streamed in.
    ///
    /// # Example
    /// ---
    ///
    /// Do something until all order's in a trade are filled.
    ///
    /// ```ignore
    /// let mut some_trades_order_statuses: HashMap<String, OrderStatus> = HashMap::new();
    ///
    /// specific_account
    ///     .stream_orders_by_id(
    ///         vec!["SOME_ORDER_ID_1", "SOME_ORDER_ID_2"],
    ///         "SOME_ACCOUNT_ID",
    ///         |stream_data| {
    ///             // The response type is `responses::account::StreamOrdersResp`
    ///             // which has multiple variants the main one you care about is
    ///             // `Order` which will contain order data sent from the stream.
    ///             match stream_data {
    ///                 StreamOrdersResp::Order(order) => {
    ///                     // Response for an `Order` streamed in
    ///                     println!("{order:?}");
    ///
    ///                     some_trades_order_statuses.insert(order.order_id, order.status);
    ///                     if some_trades_order_statuses
    ///                         .values()
    ///                         .all(|order_status| order_status == OrderStatus::FLL)
    ///                     {
    ///                         // When all order's are filled stop the stream
    ///                         return Err(Error::StopStream);
    ///                     } else {
    ///                         // Do something until all order's for a specific trade are filled
    ///                         // maybe update the limit price of the unfilled order's by 1 tick?
    ///                         //
    ///                         // NOTE: you can also "do nothing" essentially just waiting for some
    ///                         // scenario, maybe waiting for all order's to be filled to send an
    ///                         // email or text alerting that the trade is fully filled.
    ///                     }
    ///                 }
    ///                 StreamOrdersResp::Heartbeat(heartbeat) => {
    ///                     // Response for periodic signals letting you know the connection is
    ///                     // still alive. A heartbeat is sent every 5 seconds of inactivity.
    ///                     println!("{heartbeat:?}");
    ///
    ///                     // for the sake of this example after we recieve the
    ///                     // tenth heartbeat, we will stop the stream session.
    ///                     if heartbeat.heartbeat > 10 {
    ///                         // Example: stopping a stream connection
    ///                         return Err(Error::StopStream);
    ///                     }
    ///                 }
    ///                 StreamOrdersResp::Status(status) => {
    ///                     // Signal sent on state changes in the stream
    ///                     // (closed, opened, paused, resumed)
    ///                     println!("{status:?}");
    ///                 }
    ///                 StreamOrdersResp::Error(err) => {
    ///                     // Response for when an error was encountered,
    ///                     // with details on the error
    ///                     println!("{err:?}");
    ///                 }
    ///             }
    ///
    ///             Ok(())
    ///         })
    ///         .await?;
    /// ```
    pub async fn stream_orders_by_id<F>(
        &self,
        client: &Client,
        order_ids: Vec<&str>,
        on_chunk: F,
    ) -> Result<Vec<Order>, Error>
    where
        F: FnMut(StreamOrdersResp) -> Result<(), Error>,
    {
        Order::stream_by_ids(order_ids, &self.account_id, client, on_chunk).await
    }

    /// Stream `Order`(s) for the given `Account`.
    ///
    /// NOTE: You need to pass a closure function that will handle
    /// each chunk of data (`StreamOrdersResp`) as it's streamed in.
    ///
    /// # Example
    /// ---
    ///
    /// Get the amount of funds allocated to open orders.
    /// ```ignore
    /// let mut funds_allocated_to_open_orders = 0.00;
    /// specific_account
    ///     .stream_orders(&client, |stream_data| {
    ///         // The response type is `responses::account::StreamOrdersResp`
    ///         // which has multiple variants the main one you care about is
    ///         // `Order` which will contain order data sent from the stream.
    ///         match stream_data {
    ///             StreamOrdersResp::Order(order) => {
    ///                 // Response for an `Order` streamed in
    ///                 println!("{order:?}");
    ///
    ///                 // keep a live sum of all the funds allocated to open orders
    ///                 let order_value = order.price_used_for_buying_power.parse::<f64>();
    ///                 if let Ok(value) = order_value {
    ///                     funds_allocated_to_open_orders += value;
    ///                 }
    ///             }
    ///             StreamOrdersResp::Heartbeat(heartbeat) => {
    ///                 // Response for periodic signals letting you know the connection is
    ///                 // still alive. A heartbeat is sent every 5 seconds of inactivity.
    ///                 println!("{heartbeat:?}");
    ///
    ///                 // for the sake of this example after we recieve the
    ///                 // tenth heartbeat, we will stop the stream session.
    ///                 if heartbeat.heartbeat > 10 {
    ///                     // Example: stopping a stream connection
    ///                     return Err(Error::StopStream);
    ///                 }
    ///             }
    ///             StreamOrdersResp::Status(status) => {
    ///                 // Signal sent on state changes in the stream
    ///                 // (closed, opened, paused, resumed)
    ///                 println!("{status:?}");
    ///             }
    ///             StreamOrdersResp::Error(err) => {
    ///                 // Response for when an error was encountered,
    ///                 // with details on the error
    ///                 println!("{err:?}");
    ///             }
    ///         }
    ///
    ///         Ok(())
    ///     })
    ///     .await?;
    /// ```
    async fn stream_orders_for_accounts<F>(
        account_ids: Vec<&str>,
        client: &Client,
        on_chunk: F,
    ) -> Result<Vec<Order>, Error>
    where
        F: FnMut(StreamOrdersResp) -> Result<(), Error>,
    {
        Order::stream_by_accounts(account_ids, client, on_chunk).await
    }

    /// Stream `Order`s by order id's for the given `Account`(s).
    ///
    /// NOTE: order ids should be a comma delimited string slice `"xxxxx,xxxxx,xxxxx"`
    ///
    /// NOTE: You need to pass a closure function that will handle
    /// each chunk of data (`StreamOrdersResp`) as it's streamed in.
    ///
    /// # Example
    /// ---
    ///
    /// Do something until all order's in a trade are filled.
    /// ```ignore
    /// let mut some_trades_order_statuses: HashMap<String, OrderStatus> = HashMap::new();
    ///
    /// specific_account.stream_orders_by_id_for_accounts(
    ///     vec!["SOME_ORDER_ID_1", "SOME_ORDER_ID_2"],
    ///     vec!["SOME_ACCOUNT_ID_1", "SOME_ORDER_ID_2"],
    ///     &client,
    ///     |stream_data| {
    ///         // The response type is `responses::account::StreamOrdersResp`
    ///         // which has multiple variants the main one you care about is
    ///         // `Order` which will contain order data sent from the stream.
    ///         match stream_data {
    ///             StreamOrdersResp::Order(order) => {
    ///                 // Response for an `Order` streamed in
    ///                 println!("{order:?}");
    ///
    ///                 some_trades_order_statuses.insert(order.order_id, order.status);
    ///                 if some_trades_order_statuses
    ///                     .values()
    ///                     .all(|order_status| order_status == OrderStatus::FLL)
    ///                 {
    ///                     // When all order's are filled stop the stream
    ///                     return Err(Error::StopStream);
    ///                 } else {
    ///                     // Do something until all order's for a specific trade are filled
    ///                     // maybe update the limit price of the unfilled order's by 1 tick?
    ///                     //
    ///                     // NOTE: you can also "do nothing" essentially just waiting for some
    ///                     // scenario, maybe waiting for all order's to be filled to send an
    ///                     // email or text alerting that the trade is fully filled.
    ///                 }
    ///             }
    ///             StreamOrdersResp::Heartbeat(heartbeat) => {
    ///                 // Response for periodic signals letting you know the connection is
    ///                 // still alive. A heartbeat is sent every 5 seconds of inactivity.
    ///                 println!("{heartbeat:?}");
    ///
    ///                 // for the sake of this example after we recieve the
    ///                 // tenth heartbeat, we will stop the stream session.
    ///                 if heartbeat.heartbeat > 10 {
    ///                     // Example: stopping a stream connection
    ///                     return Err(Error::StopStream);
    ///                 }
    ///             }
    ///             StreamOrdersResp::Status(status) => {
    ///                 // Signal sent on state changes in the stream
    ///                 // (closed, opened, paused, resumed)
    ///                 println!("{status:?}");
    ///             }
    ///             StreamOrdersResp::Error(err) => {
    ///                 // Response for when an error was encountered,
    ///                 // with details on the error
    ///                 println!("{err:?}");
    ///             }
    ///         }
    ///
    ///         Ok(())
    ///     })
    ///     .await?;
    /// ```
    async fn stream_orders_by_id_for_accounts<F>(
        order_ids: Vec<&str>,
        account_ids: Vec<&str>,
        client: &Client,
        on_chunk: F,
    ) -> Result<Vec<Order>, Error>
    where
        F: FnMut(StreamOrdersResp) -> Result<(), Error>,
    {
        Order::stream_by_ids_and_accounts(client, order_ids, account_ids, on_chunk).await
    }

    /// Stream `Position`s for the given `Account`.
    ///
    /// NOTE: TODO: Currently does NOT support streaming `Position` changes.
    ///
    /// # Example
    /// ---
    ///
    /// Collect losing trades into a vector and do something with them.
    /// ```ignore
    /// let mut losing_positions: Vec<Position> = Vec::new();
    /// specific_account
    ///     .stream_positions(&client, |stream_data| {
    ///         // the response type is `responses::account::StreamPositionsResp`
    ///         // which has multiple variants the main one you care about is
    ///         // `order` which will contain order data sent from the stream.
    ///         match stream_data {
    ///             StreamPositionsResp::Position(position) => {
    ///                 // response for an `position` streamed in
    ///                 println!("{position:?}");
    ///
    ///                 if position.long_short == PositionType::Long {
    ///                     if position.last < position.average_price {
    ///                         losing_positions.push(*position)
    ///                     }
    ///                 } else if position.long_short == PositionType::Short {
    ///                     if position.last > position.average_price {
    ///                         losing_positions.push(*position)
    ///                     }
    ///                 }
    ///
    ///                 // do something with the list of losing trades
    ///                 // maybe send email or text of the positions
    ///                 println!("{losing_positions:?}");
    ///             }
    ///             StreamPositionsResp::Heartbeat(heartbeat) => {
    ///                 // response for periodic signals letting you know the connection is
    ///                 // still alive. a heartbeat is sent every 5 seconds of inactivity.
    ///                 println!("{heartbeat:?}");
    ///
    ///                 // for the sake of this example after we recieve the
    ///                 // tenth heartbeat, we will stop the stream session.
    ///                 if heartbeat.heartbeat > 10 {
    ///                     // example: stopping a stream connection
    ///                     return Err(Error::StopStream);
    ///                 }
    ///             }
    ///             StreamPositionsResp::Status(status) => {
    ///                 // signal sent on state changes in the stream
    ///                 // (closed, opened, paused, resumed)
    ///                 println!("{status:?}");
    ///             }
    ///             StreamPositionsResp::Error(err) => {
    ///                 // response for when an error was encountered,
    ///                 // with details on the error
    ///                 println!("{err:?}");
    ///             }
    ///         }
    ///
    ///         Ok(())
    ///     })
    ///     .await?;
    /// ```
    pub async fn stream_positions<F>(
        &self,
        client: &Client,
        on_chunk: F,
    ) -> Result<Vec<Position>, Error>
    where
        F: FnMut(StreamPositionsResp) -> Result<(), Error>,
    {
        Position::stream(&self.account_id, client, on_chunk).await
    }

    /// Stream `Position`s for the given `Account`(s).
    ///
    /// NOTE: TODO: Currently does NOT support streaming `Position` changes.
    ///
    /// # Example
    /// ---
    ///
    /// Collect losing trades into a vector and do something with them.
    /// ```ignore
    /// let mut losing_positions: Vec<Position> = Vec::new();
    /// specific_account
    ///     .stream_positions(&client, |stream_data| {
    ///         // the response type is `responses::account::StreamPositionsResp`
    ///         // which has multiple variants the main one you care about is
    ///         // `order` which will contain order data sent from the stream.
    ///         match stream_data {
    ///             StreamPositionsResp::Position(position) => {
    ///                 // response for an `position` streamed in
    ///                 println!("{position:?}");
    ///
    ///                 if position.long_short == PositionType::Long {
    ///                     if position.last < position.average_price {
    ///                         losing_positions.push(*position)
    ///                     }
    ///                 } else if position.long_short == PositionType::Short {
    ///                     if position.last > position.average_price {
    ///                         losing_positions.push(*position)
    ///                     }
    ///                 }
    ///
    ///                 // do something with the list of losing trades
    ///                 // maybe send email or text of the positions
    ///                 println!("{losing_positions:?}");
    ///             }
    ///             StreamPositionsResp::Heartbeat(heartbeat) => {
    ///                 // response for periodic signals letting you know the connection is
    ///                 // still alive. a heartbeat is sent every 5 seconds of inactivity.
    ///                 println!("{heartbeat:?}");
    ///
    ///                 // for the sake of this example after we recieve the
    ///                 // tenth heartbeat, we will stop the stream session.
    ///                 if heartbeat.heartbeat > 10 {
    ///                     // example: stopping a stream connection
    ///                     return Err(Error::StopStream);
    ///                 }
    ///             }
    ///             StreamPositionsResp::Status(status) => {
    ///                 // signal sent on state changes in the stream
    ///                 // (closed, opened, paused, resumed)
    ///                 println!("{status:?}");
    ///             }
    ///             StreamPositionsResp::Error(err) => {
    ///                 // response for when an error was encountered,
    ///                 // with details on the error
    ///                 println!("{err:?}");
    ///             }
    ///         }
    ///
    ///         Ok(())
    ///     })
    ///     .await?;
    /// ```
    pub async fn stream_positions_for_accounts<F, S: Into<String>>(
        account_ids: Vec<S>,
        client: &Client,
        on_chunk: F,
    ) -> Result<Vec<Position>, Error>
    where
        F: FnMut(StreamPositionsResp) -> Result<(), Error>,
    {
        Position::stream_for_accounts(account_ids, client, on_chunk).await
    }
}

/// Trait to allow calling methods on multiple accounts `Vec<Account>`.
pub trait MultipleAccounts {
    /// Find an `Account` by it's id.
    fn find_by_id(&self, id: &str) -> Option<Account>;

    type GetOrdersFuture<'a>: Future<Output = Result<Vec<Order>, Box<dyn StdErrorTrait + Send + Sync>>>
        + Send
        + 'a
    where
        Self: 'a;
    /// Get `Order`(s) for multiple `Account`(s).
    ///
    /// # Example
    /// ---
    ///
    /// Grab all the orders for a specific account. Say you need to go
    /// through all the orders your algorithm placed today and filter out
    /// only the orders that were filled for data storage purposes.
    ///
    /// ```ignore
    /// // Initialize the client
    /// let client = ClientBuilder::new()?
    ///     .credentials("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")?
    ///     .token(Token {
    ///         access_token: String::from("YOUR_ACCESS_TOKEN"),
    ///         refresh_token: String::from("YOUR_REFRESH_TOKEN"),
    ///         id_token: String::from("YOUR_ID_TOKEN"),
    ///         token_type: String::from("Bearer"),
    ///         scope: String::from("YOUR_SCOPES SPACE_SEPERATED FOR_EACH_SCOPE"),
    ///         expires_in: 1200,
    ///     })?
    ///     .build()
    ///     .await?;
    ///
    /// // Grab your accounts and specify an account the orders were placed in
    /// let accounts = client.get_accounts().await?;
    /// if let Some(specific_account) = accounts.find_by_id("YOUR_ACCOUNT_ID") {
    ///     // Get all the orders from today for a specific account
    ///     let orders = specific_account.get_orders(&client).await?;
    ///
    ///     // Filter out only filled orders
    ///     let filled_orders: Vec<Order> = orders
    ///         .into_iter()
    ///         .filter(|order| order.status == "FLL")
    ///         .collect();
    ///
    ///     // Do something with your filled orders
    ///     for order in filled_orders {
    ///         println!("Filled Order: {order:?}");
    ///     }
    /// }
    /// ```
    fn get_orders<'a>(&'a self, client: &'a Client) -> Self::GetOrdersFuture<'a>;

    /// Get specific `Order`(s) by their id's for multiple `Account`(s).
    ///
    /// # Example
    /// ---
    ///
    /// Grab 2 specific orders by their id's, say you have a stop loss order
    /// and a take profit order you want to check the status on, this is how.
    ///
    /// ```ignore
    /// // Initialize the client
    /// let client = ClientBuilder::new()?
    ///     .credentials("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")?
    ///     .token(Token {
    ///         access_token: String::from("YOUR_ACCESS_TOKEN"),
    ///         refresh_token: String::from("YOUR_REFRESH_TOKEN"),
    ///         id_token: String::from("YOUR_ID_TOKEN"),
    ///         token_type: String::from("Bearer"),
    ///         scope: String::from("YOUR_SCOPES SPACE_SEPERATED FOR_EACH_SCOPE"),
    ///         expires_in: 1200,
    ///     })?
    ///     .build()
    ///     .await?;
    ///
    /// // Grab your accounts and specify an account the orders were placed in
    /// let accounts = client.get_accounts().await?;
    /// if let Some(specific_account) = accounts.find_by_id("YOUR_ACCOUNT_ID") {
    ///     // Get some specific orders by their order id's
    ///     let orders = specific_account.
    ///         get_orders_by_id(vec!["1115661503", "1115332365"], &client)
    ///         .await?;
    ///
    ///     // Log the status of the order's
    ///     for order in orders {
    ///         println!("Order ID ({}) status: {}", order.order_id, order.status);
    ///     }
    /// }
    /// ```
    fn get_orders_by_id<'a>(
        &'a self,
        order_ids: &'a [&str],
        client: &'a Client,
    ) -> Self::GetOrdersFuture<'a>;

    type GetBalanceFuture<'a>: Future<Output = Result<Vec<Balance>, Box<dyn StdErrorTrait + Send + Sync>>>
        + Send
        + 'a
    where
        Self: 'a;
    /// Get the current balance of multiple `Account`(s).
    fn get_balances<'a>(&'a self, client: &'a Client) -> Self::GetBalanceFuture<'a>;

    type GetBODBalanceFuture<'a>: Future<Output = Result<Vec<BODBalance>, Box<dyn StdErrorTrait + Send + Sync>>>
        + Send
        + 'a
    where
        Self: 'a;
    /// Get the beginning of day balances for multiple `Account`(s) by account id.
    fn get_bod_balances<'a>(&'a self, client: &'a Client) -> Self::GetBODBalanceFuture<'a>;

    type GetHistoricOrdersFuture<'a>: Future<Output = Result<Vec<Order>, Box<dyn StdErrorTrait + Send + Sync>>>
        + Send
        + 'a
    where
        Self: 'a;
    /// Get the historical `Order`(s) for multiple `Account`(s).
    ///
    /// NOTE: Date format is {YEAR-MONTH-DAY} ex: `"2024-07-09"`, and is limited to 90
    /// days prior to the current date.
    ///
    /// NOTE: Excludes open `Order`(s) and is sorted in descending order of time closed.
    fn get_historic_orders<'a>(
        &'a self,
        since_date: &'a str,
        client: &'a Client,
    ) -> Self::GetHistoricOrdersFuture<'a>;

    type GetPositionFuture<'a>: Future<Output = Result<Position, Box<dyn StdErrorTrait + Send + Sync>>>
        + Send
        + 'a
    where
        Self: 'a;
    /// Fetches a specific `Position` by it's id for multiple `Account`(s).
    ///
    /// # Example
    /// ---
    ///
    /// Grab a specific position, say you need to check for updates on some
    /// position and you already know it's position id but maybe not which account
    /// so you want to search for the position through all accounts, here's how you
    /// would do it.
    ///
    /// ```ignore
    /// // Initialize the client
    /// let client = ClientBuilder::new()?
    ///     .credentials("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")?
    ///     .token(Token {
    ///         access_token: String::from("YOUR_ACCESS_TOKEN"),
    ///         refresh_token: String::from("YOUR_REFRESH_TOKEN"),
    ///         id_token: String::from("YOUR_ID_TOKEN"),
    ///         token_type: String::from("Bearer"),
    ///         scope: String::from("YOUR_SCOPES SPACE_SEPERATED FOR_EACH_SCOPE"),
    ///         expires_in: 1200,
    ///     })?
    ///     .build()
    ///     .await?;
    ///
    /// // Grab the account where the position exists
    /// let accounts = client
    ///     .get_accounts()
    ///     .await?;
    ///
    /// let position = accounts.get_position("YOUR_POSITION_ID").await?;
    /// println!("Position: {position:?}");
    /// ```
    fn get_position<'a, S: Into<String>>(
        &'a self,
        position_id: S,
        client: &'a Client,
    ) -> Self::GetPositionFuture<'a>;

    type GetPositionsFuture<'a>: Future<Output = Result<Vec<Position>, Box<dyn StdErrorTrait + Send + Sync>>>
        + Send
        + 'a
    where
        Self: 'a;
    /// Get the `Position`(s) for multiple `Account`(s).
    fn get_positions<'a>(&'a self, client: &'a Client) -> Self::GetPositionsFuture<'a>;

    /// Fetches specific `Position`(s) by their id for multiple `Account`(s).
    ///
    /// # Example
    /// ---
    ///
    /// Grab specific positions, say you need to check for updates on 2 specific
    /// positions within different accounts and you already know their position ids,
    /// but maybe not the account ids, here's how you would do it.
    ///
    /// ```ignore
    /// // Initialize the client
    /// let client = ClientBuilder::new()?
    ///     .credentials("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")?
    ///     .token(Token {
    ///         access_token: String::from("YOUR_ACCESS_TOKEN"),
    ///         refresh_token: String::from("YOUR_REFRESH_TOKEN"),
    ///         id_token: String::from("YOUR_ID_TOKEN"),
    ///         token_type: String::from("Bearer"),
    ///         scope: String::from("YOUR_SCOPES SPACE_SEPERATED FOR_EACH_SCOPE"),
    ///         expires_in: 1200,
    ///     })?
    ///     .build()
    ///     .await?;
    ///
    /// // Grab all accounts
    /// let accounts = client
    ///     .get_accounts()
    ///     .await?;
    ///
    /// let positions = accounts
    ///     .get_positions_by_ids(
    ///         vec!["YOUR_POSITION_ID_1", "YOUR_POSITION_ID_2"]
    ///     )
    ///     .await?;
    /// println!("Positions: {positions:?}");
    /// ```
    fn get_positions_by_ids<'a>(
        &'a self,
        position_ids: Vec<String>,
        client: &'a Client,
    ) -> Self::GetPositionsFuture<'a>;

    type GetPositionsInSymbolsFuture<'a>: Future<Output = Result<Vec<Position>, Box<dyn StdErrorTrait + Send + Sync>>>
        + Send
        + 'a
    where
        Self: 'a;
    /// Get the `Position`(s) in specific symbols for multiple `Account`(s).
    fn get_positions_in_symbols<'a>(
        &'a self,
        symbols: &'a str,
        client: &'a Client,
    ) -> Self::GetPositionsFuture<'a>;

    type StreamOrdersFuture<'a>: Future<Output = Result<Vec<Order>, Box<dyn StdErrorTrait + Send + Sync>>>
        + Send
        + 'a
    where
        Self: 'a;
    /// Stream `Order`(s) for the given `Account`.
    ///
    /// NOTE: You need to pass a closure function that will handle
    /// each chunk of data (`StreamOrdersResp`) as it's streamed in.
    ///
    /// # Example
    /// ---
    ///
    /// Get the amount of funds allocated to open orders.
    /// ```ignore
    /// let mut funds_allocated_to_open_orders = 0.00;
    /// specific_account
    ///     .stream_orders(&client, |stream_data| {
    ///         // The response type is `responses::account::StreamOrdersResp`
    ///         // which has multiple variants the main one you care about is
    ///         // `Order` which will contain order data sent from the stream.
    ///         match stream_data {
    ///             StreamOrdersResp::Order(order) => {
    ///                 // Response for an `Order` streamed in
    ///                 println!("{order:?}");
    ///
    ///                 // keep a live sum of all the funds allocated to open orders
    ///                 let order_value = order.price_used_for_buying_power.parse::<f64>();
    ///                 if let Ok(value) = order_value {
    ///                     funds_allocated_to_open_orders += value;
    ///                 }
    ///             }
    ///             StreamOrdersResp::Heartbeat(heartbeat) => {
    ///                 // Response for periodic signals letting you know the connection is
    ///                 // still alive. A heartbeat is sent every 5 seconds of inactivity.
    ///                 println!("{heartbeat:?}");
    ///
    ///                 // for the sake of this example after we recieve the
    ///                 // tenth heartbeat, we will stop the stream session.
    ///                 if heartbeat.heartbeat > 10 {
    ///                     // Example: stopping a stream connection
    ///                     return Err(Error::StopStream);
    ///                 }
    ///             }
    ///             StreamOrdersResp::Status(status) => {
    ///                 // Signal sent on state changes in the stream
    ///                 // (closed, opened, paused, resumed)
    ///                 println!("{status:?}");
    ///             }
    ///             StreamOrdersResp::Error(err) => {
    ///                 // Response for when an error was encountered,
    ///                 // with details on the error
    ///                 println!("{err:?}");
    ///             }
    ///         }
    ///
    ///         Ok(())
    ///     })
    ///     .await?;
    /// ```
    fn stream_orders<'a, F>(
        &'a self,
        on_chunk: &'a mut F,
        client: &'a Client,
    ) -> Self::StreamOrdersFuture<'a>
    where
        F: FnMut(StreamOrdersResp) -> Result<(), Error> + Send + 'a;

    type StreamOrdersByIdFuture<'a>: Future<Output = Result<Vec<Order>, Box<dyn StdErrorTrait + Send + Sync>>>
        + Send
        + 'a
    where
        Self: 'a;
    /// Stream `Order`s by order id's for the given `Account`(s).
    ///
    /// NOTE: order ids should be a comma delimited string slice `"xxxxx,xxxxx,xxxxx"`.
    ///
    /// NOTE: You need to pass a closure function that will handle
    /// each chunk of data (`StreamOrdersResp`) as it's streamed in.
    ///
    /// # Example
    /// ---
    ///
    /// Do something until all order's in a trade are filled.
    /// ```ignore
    /// let mut some_trades_order_statuses: HashMap<String, OrderStatus> = HashMap::new();
    /// specific_account
    ///     // NOTE: The order ids "1111,1112,1113,1114" are fake and not to be used.
    ///     .stream_orders_by_id(&client, "1111,1112,1113,1114", |stream_data| {
    ///         // The response type is `responses::account::StreamOrdersResp`
    ///         // which has multiple variants the main one you care about is
    ///         // `Order` which will contain order data sent from the stream.
    ///         match stream_data {
    ///             StreamOrdersResp::Order(order) => {
    ///                 // Response for an `Order` streamed in
    ///                 println!("{order:?}");
    ///
    ///                 some_trades_order_statuses.insert(order.order_id, order.status);
    ///                 if some_trades_order_statuses
    ///                     .values()
    ///                     .all(|order_status| order_status == OrderStatus::FLL)
    ///                 {
    ///                     // When all order's are filled stop the stream
    ///                     return Err(Error::StopStream);
    ///                 } else {
    ///                     // Do something until all order's for a specific trade are filled
    ///                     // maybe update the limit price of the unfilled order's by 1 tick?
    ///                     //
    ///                     // NOTE: you can also "do nothing" essentially just waiting for some
    ///                     // scenario, maybe waiting for all order's to be filled to send an
    ///                     // email or text alerting that the trade is fully filled.
    ///                 }
    ///             }
    ///             StreamOrdersResp::Heartbeat(heartbeat) => {
    ///                 // Response for periodic signals letting you know the connection is
    ///                 // still alive. A heartbeat is sent every 5 seconds of inactivity.
    ///                 println!("{heartbeat:?}");
    ///
    ///                 // for the sake of this example after we recieve the
    ///                 // tenth heartbeat, we will stop the stream session.
    ///                 if heartbeat.heartbeat > 10 {
    ///                     // Example: stopping a stream connection
    ///                     return Err(Error::StopStream);
    ///                 }
    ///             }
    ///             StreamOrdersResp::Status(status) => {
    ///                 // Signal sent on state changes in the stream
    ///                 // (closed, opened, paused, resumed)
    ///                 println!("{status:?}");
    ///             }
    ///             StreamOrdersResp::Error(err) => {
    ///                 // Response for when an error was encountered,
    ///                 // with details on the error
    ///                 println!("{err:?}");
    ///             }
    ///         }
    ///
    ///         Ok(())
    ///     })
    ///     .await?;
    /// ```
    fn stream_orders_by_id<'a, F>(
        &'a self,
        order_ids: Vec<&'a str>,
        on_chunk: &'a mut F,
        client: &'a Client,
    ) -> Self::StreamOrdersByIdFuture<'a>
    where
        F: FnMut(StreamOrdersResp) -> Result<(), Error> + Send + 'a;

    type StreamPositionsFuture<'a>: Future<Output = Result<Vec<Position>, Box<dyn StdErrorTrait + Send + Sync>>>
        + Send
        + 'a
    where
        Self: 'a;
    /// Stream `Position`s for the given `Account`(s).
    ///
    /// NOTE: TODO: Currently does NOT support streaming `Position` changes.
    ///
    /// # Example
    /// ---
    ///
    /// Collect losing trades into a vector and do something with them.
    /// ```ignore
    /// let mut losing_positions: Vec<Position> = Vec::new();
    /// specific_account
    ///     .stream_positions(&client, |stream_data| {
    ///         // the response type is `responses::account::StreamPositionsResp`
    ///         // which has multiple variants the main one you care about is
    ///         // `order` which will contain order data sent from the stream.
    ///         match stream_data {
    ///             StreamPositionsResp::Position(position) => {
    ///                 // response for an `position` streamed in
    ///                 println!("{position:?}");
    ///
    ///                 if position.long_short == PositionType::Long {
    ///                     if position.last < position.average_price {
    ///                         losing_positions.push(*position)
    ///                     }
    ///                 } else if position.long_short == PositionType::Short {
    ///                     if position.last > position.average_price {
    ///                         losing_positions.push(*position)
    ///                     }
    ///                 }
    ///
    ///                 // do something with the list of losing trades
    ///                 // maybe send email or text of the positions
    ///                 println!("{losing_positions:?}");
    ///             }
    ///             StreamPositionsResp::Heartbeat(heartbeat) => {
    ///                 // response for periodic signals letting you know the connection is
    ///                 // still alive. a heartbeat is sent every 5 seconds of inactivity.
    ///                 println!("{heartbeat:?}");
    ///
    ///                 // for the sake of this example after we recieve the
    ///                 // tenth heartbeat, we will stop the stream session.
    ///                 if heartbeat.heartbeat > 10 {
    ///                     // example: stopping a stream connection
    ///                     return Err(Error::StopStream);
    ///                 }
    ///             }
    ///             StreamPositionsResp::Status(status) => {
    ///                 // signal sent on state changes in the stream
    ///                 // (closed, opened, paused, resumed)
    ///                 println!("{status:?}");
    ///             }
    ///             StreamPositionsResp::Error(err) => {
    ///                 // response for when an error was encountered,
    ///                 // with details on the error
    ///                 println!("{err:?}");
    ///             }
    ///         }
    ///
    ///         Ok(())
    ///     })
    ///     .await?;
    /// ```
    fn stream_positions<'a, F>(
        &'a self,
        on_chunk: &'a mut F,
        client: &'a Client,
    ) -> Self::StreamPositionsFuture<'a>
    where
        F: FnMut(StreamPositionsResp) -> Result<(), Error> + Send + 'a;
}
impl MultipleAccounts for Vec<Account> {
    /// Find a specific account by a given account id from
    /// a `Vec<Account>`.
    fn find_by_id(&self, id: &str) -> Option<Account> {
        self.iter()
            .filter(|account| account.account_id == id)
            .collect::<Vec<&Account>>()
            .pop()
            .cloned()
    }

    type GetOrdersFuture<'a> = Pin<
        Box<
            dyn Future<Output = Result<Vec<Order>, Box<dyn StdErrorTrait + Send + Sync>>>
                + Send
                + 'a,
        >,
    >;
    fn get_orders<'a>(&'a self, client: &'a Client) -> Self::GetOrdersFuture<'a> {
        let account_ids: Vec<&str> = self
            .iter()
            .map(|account| account.account_id.as_str())
            .collect();

        Box::pin(async move {
            let orders = Account::get_orders_for_accounts(account_ids, client).await?;
            Ok(orders)
        })
    }

    fn get_orders_by_id<'a>(
        &'a self,
        order_ids: &'a [&str],
        client: &'a Client,
    ) -> Self::GetOrdersFuture<'a> {
        let account_ids: Vec<&str> = self
            .iter()
            .map(|account| account.account_id.as_str())
            .collect();

        Box::pin(async move {
            let orders =
                Account::get_orders_by_id_for_accounts(account_ids, order_ids.to_vec(), client)
                    .await?;

            Ok(orders)
        })
    }

    type GetBalanceFuture<'a> = Pin<
        Box<
            dyn Future<Output = Result<Vec<Balance>, Box<dyn StdErrorTrait + Send + Sync>>>
                + Send
                + 'a,
        >,
    >;
    /// Get the beginning of day balances for multiple `Account`(s).
    fn get_balances<'a>(&'a self, client: &'a Client) -> Self::GetBalanceFuture<'a> {
        let account_ids: Vec<&str> = self
            .iter()
            .map(|account| account.account_id.as_str())
            .collect();

        Box::pin(async move {
            let balances = Account::get_balances_by_accounts(account_ids, client).await?;
            Ok(balances)
        })
    }

    type GetBODBalanceFuture<'a> = Pin<
        Box<
            dyn Future<Output = Result<Vec<BODBalance>, Box<dyn StdErrorTrait + Send + Sync>>>
                + Send
                + 'a,
        >,
    >;
    /// Get the beginning of day balances for multiple `Account`(s)
    fn get_bod_balances<'a>(&'a self, client: &'a Client) -> Self::GetBODBalanceFuture<'a> {
        let account_ids: Vec<&str> = self
            .iter()
            .map(|account| account.account_id.as_str())
            .collect();

        Box::pin(async move {
            let balances = Account::get_bod_balances_by_accounts(account_ids, client).await?;
            Ok(balances)
        })
    }

    type GetHistoricOrdersFuture<'a> = Pin<
        Box<
            dyn Future<Output = Result<Vec<Order>, Box<dyn StdErrorTrait + Send + Sync>>>
                + Send
                + 'a,
        >,
    >;
    /// Get the historical `Order`(s) for multiple `Account`(s).
    fn get_historic_orders<'a>(
        &'a self,
        since_date: &'a str,
        client: &'a Client,
    ) -> Self::GetHistoricOrdersFuture<'a> {
        let account_ids: Vec<&str> = self
            .iter()
            .map(|account| account.account_id.as_str())
            .collect();

        Box::pin(async move {
            let balances =
                Account::get_historic_orders_by_accounts(account_ids, since_date, client).await?;
            Ok(balances)
        })
    }

    type GetPositionFuture<'a> = Pin<
        Box<
            dyn Future<Output = Result<Position, Box<dyn StdErrorTrait + Send + Sync>>> + Send + 'a,
        >,
    >;
    /// Fetches a specific `Position` by it's id for multiple `Account`(s).
    ///
    /// # Example
    /// ---
    ///
    /// Grab a specific position, say you need to check for updates on some
    /// position and you already know it's position id but maybe not which account
    /// so you want to search for the position through all accounts, here's how you
    /// would do it.
    ///
    /// ```ignore
    /// // Initialize the client
    /// let client = ClientBuilder::new()?
    ///     .credentials("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")?
    ///     .token(Token {
    ///         access_token: String::from("YOUR_ACCESS_TOKEN"),
    ///         refresh_token: String::from("YOUR_REFRESH_TOKEN"),
    ///         id_token: String::from("YOUR_ID_TOKEN"),
    ///         token_type: String::from("Bearer"),
    ///         scope: String::from("YOUR_SCOPES SPACE_SEPERATED FOR_EACH_SCOPE"),
    ///         expires_in: 1200,
    ///     })?
    ///     .build()
    ///     .await?;
    ///
    /// // Grab the account where the position exists
    /// let accounts = client
    ///     .get_accounts()
    ///     .await?;
    ///
    /// let position = accounts.get_position("YOUR_POSITION_ID").await?;
    /// println!("Position: {position:?}");
    /// ```
    fn get_position<'a, S: Into<String>>(
        &'a self,
        position_id: S,
        client: &'a Client,
    ) -> Self::GetPositionFuture<'a> {
        let account_ids = self
            .iter()
            .map(|account| account.account_id.clone())
            .collect::<Vec<String>>()
            .join(",");

        let position_id = position_id.into();

        Box::pin(async move {
            let positions =
                Account::get_position_for_accounts(account_ids, position_id, client).await?;
            Ok(positions)
        })
    }

    type GetPositionsFuture<'a> = Pin<
        Box<
            dyn Future<Output = Result<Vec<Position>, Box<dyn StdErrorTrait + Send + Sync>>>
                + Send
                + 'a,
        >,
    >;
    /// Get the `Position`(s) for multiple `Account`(s).
    fn get_positions<'a>(&'a self, client: &'a Client) -> Self::GetPositionsFuture<'a> {
        let account_ids: Vec<&str> = self
            .iter()
            .map(|account| account.account_id.as_str())
            .collect();

        Box::pin(async move {
            let positions = Account::get_positions_by_accounts(account_ids, client).await?;
            Ok(positions)
        })
    }

    /// Fetches specific `Position`(s) by their id for multiple `Account`(s).
    ///
    /// # Example
    /// ---
    ///
    /// Grab specific positions, say you need to check for updates on 2 specific
    /// positions within different accounts and you already know their position ids,
    /// but maybe not the account ids, here's how you would do it.
    ///
    /// ```ignore
    /// // Initialize the client
    /// let client = ClientBuilder::new()?
    ///     .credentials("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")?
    ///     .token(Token {
    ///         access_token: String::from("YOUR_ACCESS_TOKEN"),
    ///         refresh_token: String::from("YOUR_REFRESH_TOKEN"),
    ///         id_token: String::from("YOUR_ID_TOKEN"),
    ///         token_type: String::from("Bearer"),
    ///         scope: String::from("YOUR_SCOPES SPACE_SEPERATED FOR_EACH_SCOPE"),
    ///         expires_in: 1200,
    ///     })?
    ///     .build()
    ///     .await?;
    ///
    /// // Grab all accounts
    /// let accounts = client
    ///     .get_accounts()
    ///     .await?;
    ///
    /// let positions = accounts
    ///     .get_positions_by_ids(
    ///         vec!["YOUR_POSITION_ID_1", "YOUR_POSITION_ID_2"],
    ///         &client,
    ///     )
    ///     .await?;
    /// println!("Positions: {positions:?}");
    /// ```
    fn get_positions_by_ids<'a>(
        &'a self,
        position_ids: Vec<String>,
        client: &'a Client,
    ) -> Self::GetPositionsFuture<'a> {
        let account_ids = self
            .iter()
            .map(|account| account.account_id.clone())
            .collect::<Vec<String>>()
            .join(",");

        Box::pin(async move {
            let positions =
                Account::get_positions_by_id_for_accounts(account_ids, position_ids, client)
                    .await?;
            Ok(positions)
        })
    }

    type GetPositionsInSymbolsFuture<'a> = Pin<
        Box<
            dyn Future<Output = Result<Vec<Position>, Box<dyn StdErrorTrait + Send + Sync>>>
                + Send
                + 'a,
        >,
    >;
    /// Get the `Position`(s) in specific symbols for multiple `Account`(s).
    fn get_positions_in_symbols<'a>(
        &'a self,
        symbols: &'a str,
        client: &'a Client,
    ) -> Self::GetPositionsFuture<'a> {
        let account_ids: Vec<&str> = self
            .iter()
            .map(|account| account.account_id.as_str())
            .collect();

        Box::pin(async move {
            let positions =
                Account::get_positions_in_symbols_by_accounts(symbols, account_ids, client).await?;
            Ok(positions)
        })
    }

    type StreamOrdersFuture<'a> = Pin<
        Box<
            dyn Future<Output = Result<Vec<Order>, Box<dyn StdErrorTrait + Send + Sync>>>
                + Send
                + 'a,
        >,
    >;
    /// Stream `Order`(s) for the given `Account`
    ///
    /// NOTE: You need to pass a closure function that will handle
    /// each chunk of data (`StreamOrdersResp`) as it's streamed in.
    ///
    /// # Example
    /// ---
    ///
    /// Get the amount of funds allocated to open orders.
    /// ```ignore
    /// let mut funds_allocated_to_open_orders = 0.00;
    /// specific_account
    ///     .stream_orders(&client, |stream_data| {
    ///         // The response type is `responses::account::StreamOrdersResp`
    ///         // which has multiple variants the main one you care about is
    ///         // `Order` which will contain order data sent from the stream.
    ///         match stream_data {
    ///             StreamOrdersResp::Order(order) => {
    ///                 // Response for an `Order` streamed in
    ///                 println!("{order:?}");
    ///
    ///                 // keep a live sum of all the funds allocated to open orders
    ///                 let order_value = order.price_used_for_buying_power.parse::<f64>();
    ///                 if let Ok(value) = order_value {
    ///                     funds_allocated_to_open_orders += value;
    ///                 }
    ///             }
    ///             StreamOrdersResp::Heartbeat(heartbeat) => {
    ///                 // Response for periodic signals letting you know the connection is
    ///                 // still alive. A heartbeat is sent every 5 seconds of inactivity.
    ///                 println!("{heartbeat:?}");
    ///
    ///                 // for the sake of this example after we recieve the
    ///                 // tenth heartbeat, we will stop the stream session.
    ///                 if heartbeat.heartbeat > 10 {
    ///                     // Example: stopping a stream connection
    ///                     return Err(Error::StopStream);
    ///                 }
    ///             }
    ///             StreamOrdersResp::Status(status) => {
    ///                 // Signal sent on state changes in the stream
    ///                 // (closed, opened, paused, resumed)
    ///                 println!("{status:?}");
    ///             }
    ///             StreamOrdersResp::Error(err) => {
    ///                 // Response for when an error was encountered,
    ///                 // with details on the error
    ///                 println!("{err:?}");
    ///             }
    ///         }
    ///
    ///         Ok(())
    ///     })
    ///     .await?;
    /// ```
    fn stream_orders<'a, F>(
        &'a self,
        mut on_chunk: &'a mut F,
        client: &'a Client,
    ) -> Self::StreamOrdersFuture<'a>
    where
        F: FnMut(StreamOrdersResp) -> Result<(), Error> + Send + 'a,
    {
        let account_ids: Vec<&str> = self
            .iter()
            .map(|account| account.account_id.as_str())
            .collect();

        Box::pin(async move {
            let orders =
                Account::stream_orders_for_accounts(account_ids, client, &mut on_chunk).await?;
            Ok(orders)
        })
    }

    type StreamOrdersByIdFuture<'a> = Pin<
        Box<
            dyn Future<Output = Result<Vec<Order>, Box<dyn StdErrorTrait + Send + Sync>>>
                + Send
                + 'a,
        >,
    >;
    /// Stream `Order`s by order id's for the given `Account`(s)
    ///
    /// NOTE: order ids should be a comma delimited string slice `"xxxxx,xxxxx,xxxxx"`
    ///
    /// NOTE: You need to pass a closure function that will handle
    /// each chunk of data (`StreamOrdersResp`) as it's streamed in.
    ///
    /// # Example
    /// ---
    ///
    /// Do something until all order's in a trade are filled.
    /// ```ignore
    /// let mut some_trades_order_statuses: HashMap<String, OrderStatus> = HashMap::new();
    /// specific_account
    ///     // NOTE: The order ids "1111,1112,1113,1114" are fake and not to be used.
    ///     .stream_orders_by_id(&client, "1111,1112,1113,1114", |stream_data| {
    ///         // The response type is `responses::account::StreamOrdersResp`
    ///         // which has multiple variants the main one you care about is
    ///         // `Order` which will contain order data sent from the stream.
    ///         match stream_data {
    ///             StreamOrdersResp::Order(order) => {
    ///                 // Response for an `Order` streamed in
    ///                 println!("{order:?}");
    ///
    ///                 some_trades_order_statuses.insert(order.order_id, order.status);
    ///                 if some_trades_order_statuses
    ///                     .values()
    ///                     .all(|order_status| order_status == OrderStatus::FLL)
    ///                 {
    ///                     // When all order's are filled stop the stream
    ///                     return Err(Error::StopStream);
    ///                 } else {
    ///                     // Do something until all order's for a specific trade are filled
    ///                     // maybe update the limit price of the unfilled order's by 1 tick?
    ///                     //
    ///                     // NOTE: you can also "do nothing" essentially just waiting for some
    ///                     // scenario, maybe waiting for all order's to be filled to send an
    ///                     // email or text alerting that the trade is fully filled.
    ///                 }
    ///             }
    ///             StreamOrdersResp::Heartbeat(heartbeat) => {
    ///                 // Response for periodic signals letting you know the connection is
    ///                 // still alive. A heartbeat is sent every 5 seconds of inactivity.
    ///                 println!("{heartbeat:?}");
    ///
    ///                 // for the sake of this example after we recieve the
    ///                 // tenth heartbeat, we will stop the stream session.
    ///                 if heartbeat.heartbeat > 10 {
    ///                     // Example: stopping a stream connection
    ///                     return Err(Error::StopStream);
    ///                 }
    ///             }
    ///             StreamOrdersResp::Status(status) => {
    ///                 // Signal sent on state changes in the stream
    ///                 // (closed, opened, paused, resumed)
    ///                 println!("{status:?}");
    ///             }
    ///             StreamOrdersResp::Error(err) => {
    ///                 // Response for when an error was encountered,
    ///                 // with details on the error
    ///                 println!("{err:?}");
    ///             }
    ///         }
    ///
    ///         Ok(())
    ///     })
    ///     .await?;
    /// ```
    fn stream_orders_by_id<'a, F>(
        &'a self,
        order_ids: Vec<&'a str>,
        mut on_chunk: &'a mut F,
        client: &'a Client,
    ) -> Self::StreamOrdersByIdFuture<'a>
    where
        F: FnMut(StreamOrdersResp) -> Result<(), Error> + Send + 'a,
    {
        let account_ids: Vec<&str> = self
            .iter()
            .map(|account| account.account_id.as_str())
            .collect();

        Box::pin(async move {
            let orders = Account::stream_orders_by_id_for_accounts(
                order_ids,
                account_ids,
                client,
                &mut on_chunk,
            )
            .await?;
            Ok(orders)
        })
    }

    type StreamPositionsFuture<'a> = Pin<
        Box<
            dyn Future<Output = Result<Vec<Position>, Box<dyn StdErrorTrait + Send + Sync>>>
                + Send
                + 'a,
        >,
    >;
    /// Stream `Position`s for the given `Account`(s).
    ///
    /// NOTE: TODO: Currently does NOT support streaming `Position` changes.
    ///
    /// # Example
    /// ---
    ///
    /// Collect losing trades into a vector and do something with them.
    /// ```ignore
    /// let mut losing_positions: Vec<Position> = Vec::new();
    /// specific_account
    ///     .stream_positions(&client, |stream_data| {
    ///         // the response type is `responses::account::StreamPositionsResp`
    ///         // which has multiple variants the main one you care about is
    ///         // `order` which will contain order data sent from the stream.
    ///         match stream_data {
    ///             StreamPositionsResp::Position(position) => {
    ///                 // response for an `position` streamed in
    ///                 println!("{position:?}");
    ///
    ///                 if position.long_short == PositionType::Long {
    ///                     if position.last < position.average_price {
    ///                         losing_positions.push(*position)
    ///                     }
    ///                 } else if position.long_short == PositionType::Short {
    ///                     if position.last > position.average_price {
    ///                         losing_positions.push(*position)
    ///                     }
    ///                 }
    ///
    ///                 // do something with the list of losing trades
    ///                 // maybe send email or text of the positions
    ///                 println!("{losing_positions:?}");
    ///             }
    ///             StreamPositionsResp::Heartbeat(heartbeat) => {
    ///                 // response for periodic signals letting you know the connection is
    ///                 // still alive. a heartbeat is sent every 5 seconds of inactivity.
    ///                 println!("{heartbeat:?}");
    ///
    ///                 // for the sake of this example after we recieve the
    ///                 // tenth heartbeat, we will stop the stream session.
    ///                 if heartbeat.heartbeat > 10 {
    ///                     // example: stopping a stream connection
    ///                     return Err(Error::StopStream);
    ///                 }
    ///             }
    ///             StreamPositionsResp::Status(status) => {
    ///                 // signal sent on state changes in the stream
    ///                 // (closed, opened, paused, resumed)
    ///                 println!("{status:?}");
    ///             }
    ///             StreamPositionsResp::Error(err) => {
    ///                 // response for when an error was encountered,
    ///                 // with details on the error
    ///                 println!("{err:?}");
    ///             }
    ///         }
    ///
    ///         Ok(())
    ///     })
    ///     .await?;
    /// ```
    fn stream_positions<'a, F>(
        &'a self,
        mut on_chunk: &'a mut F,
        client: &'a Client,
    ) -> Self::StreamPositionsFuture<'a>
    where
        F: FnMut(StreamPositionsResp) -> Result<(), Error> + Send + 'a,
    {
        let account_ids: Vec<&str> = self
            .iter()
            .map(|account| account.account_id.as_str())
            .collect();

        Box::pin(async move {
            let positions =
                Account::stream_positions_for_accounts(account_ids, client, &mut on_chunk).await?;
            Ok(positions)
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// Deeper Details for an `Account`
pub struct AccountDetail {
    /// Can account locate securities for borrowing?
    ///
    /// For example if you want to short a stock, you need
    /// to "locate" shares to borrow and sell.
    is_stock_locate_eligible: bool,

    /// Is account enrolled with Regulation T ?
    ///
    /// Regulation T governs cash accounts and the amount of credit that
    /// broker-dealers can extend to investors for the purchase of securities.
    enrolled_in_reg_t_program: bool,

    /// Does the account require a buying power warning before order execution?
    ///
    /// TradeStation uses the greater of Overnight Buying Power or Day Trade
    /// Buying Power to determine if an order should be accepted or rejected.
    ///
    /// In cases where an order exceeds both values, the order will be rejected.
    /// If the order exceeds only one of the values, a Buying Power Warning will
    /// appear to notify you that the order could result in a margin call.
    requires_buying_power_warning: bool,

    /// Is the `Account` qualified for day trading?
    ///
    /// An `Account` MUST maintain a minimum equity balance of $25,000
    /// to be qualified for day trades. *(As per TradeStation compliance rules)*
    day_trading_qualified: bool,

    /// What options level is the `Account` approved for?
    ///
    /// The option approval level will determine what options strategies you will
    /// be able to employ on `Account`. In general terms, the levels are defined as:
    /// Level 0: No options trading allowed.
    /// Level 1: Writing of Covered Calls, Buying Protective Puts.
    /// Level 2: Level 1 + Buying Calls, Buying Puts, Writing Covered Puts.
    /// Level 3: Level 2 + Stock Option Spreads, Index Option Spreads, Butterfly Spreads, Condor Spreads, Iron Butterfly Spreads, Iron Condor Spreads.
    /// Level 4: Level 3 + Writing of Naked Puts (Stock Options).
    /// Level 5: Level 4 + Writing of Naked Puts (Index Options), Writing of Naked Calls (Stock Options), Writing of Naked Calls (Index Options).
    ///
    /// These levels vary depending on the funding and type of account.
    option_approval_level: OptionApprovalLevel,

    /// Is the `Account` a Pattern Day Trader?
    ///
    /// As per FINRA rules, an `Account` will be considered a pattern day trader
    /// if it day-trades 4 or more times in 5 business days and it's day-trading
    /// activities are greater than 6 percent of it's total trading activity for
    /// that same five-day period.
    ///
    /// A pattern day trader must maintain minimum equity of $25,000 on any day
    /// that the customer day trades. If the account falls below the $25,000
    /// requirement, the pattern day trader will not be permitted to day trade
    /// until the account is restored to the $25,000 minimum equity level.
    pattern_day_trader: bool,

    /// Is the `Account` enabled to trade crypto?
    ///
    /// NOTE: As of 2024 TradeStation no longer offer's crypto trading.
    crypto_enabled: bool,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
/// The different types of accounts.
pub enum AccountType {
    /// Cash Account
    Cash,

    /// Margin Account
    Margin,

    /// Futures Account
    Futures,

    /// Delivery Vs Payment Account
    DVP,
}

#[derive(Clone, Debug, Serialize)]
/// The different levels of options approval an account can have.
pub enum OptionApprovalLevel {
    /// Options Approval Level 0: No options trading allowed.
    Zero,

    /// Options Approval Level 1: Writing of Covered Calls, Buying Protective Puts.
    One,

    /// Options Approval Level 2: Level 1 + Buying Calls, Buying Puts, Writing Covered Puts.
    Two,

    /// Options Approval Level 3: Level 2 + Stock Option Spreads, Index Option Spreads,
    /// Butterfly Spreads, Condor Spreads, Iron Butterfly Spreads, Iron Condor Spreads.
    Three,

    /// Options Approval Level 4: Level 3 + Writing of Naked Puts (Stock Options).
    Four,

    /// Options Approval Level 5: Level 4 + Writing of Naked Puts (Index Options),
    /// Writing of Naked Calls (Stock Options), Writing of Naked Calls (Index Options).
    Five,
}
impl TryFrom<u8> for OptionApprovalLevel {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(OptionApprovalLevel::Zero),
            1 => Ok(OptionApprovalLevel::One),
            2 => Ok(OptionApprovalLevel::Two),
            3 => Ok(OptionApprovalLevel::Three),
            4 => Ok(OptionApprovalLevel::Four),
            5 => Ok(OptionApprovalLevel::Five),
            _ => Err(format!("Invalid OptionApprovalLevel: {}", value)),
        }
    }
}
impl From<OptionApprovalLevel> for u8 {
    fn from(level: OptionApprovalLevel) -> Self {
        match level {
            OptionApprovalLevel::Zero => 0,
            OptionApprovalLevel::One => 1,
            OptionApprovalLevel::Two => 2,
            OptionApprovalLevel::Three => 3,
            OptionApprovalLevel::Four => 4,
            OptionApprovalLevel::Five => 5,
        }
    }
}
impl<'de> Deserialize<'de> for OptionApprovalLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u8::deserialize(deserializer)?;
        OptionApprovalLevel::try_from(value).map_err(serde::de::Error::custom)
    }
}

impl Client {
    /// Get all of your registered TradeStation `Accounts`
    pub async fn get_accounts(&self) -> Result<Vec<Account>, Error> {
        Account::get_all(self).await
    }

    /// Get a specific TradeStation `Account` by it's account id
    pub async fn get_account(&self, account_id: &str) -> Result<Account, Error> {
        Account::get(account_id, self).await
    }
}
