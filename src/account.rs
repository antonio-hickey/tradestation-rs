use crate::responses::account::{StreamOrdersResp, StreamPositionsResp};
use crate::{responses::account as responses, Client, Error};
use serde::{Deserialize, Serialize};
use std::{error::Error as StdErrorTrait, future::Future, pin::Pin};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// TradeStation Account
pub struct Account {
    #[serde(rename = "AccountID")]
    /// The main identifier for a TradeStation account
    account_id: String,
    /// The currency the account is based on.
    currency: String,
    /// The type of account, examples: "Cash" or "Margin"
    // TODO: Make enum for this
    account_type: String,
    /// The account details, stuff like options level and day trading approval
    ///
    /// NOTE: This will always be `None` if it's a Futures `Account`
    account_detail: Option<AccountDetail>,
}
impl Account {
    /// Get a specific TradeStation `Account` by it's account id
    pub async fn get(client: &mut Client, account_id: &str) -> Result<Account, Error> {
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

    /// Get all of your registered TradeStation `Account`(s)
    pub async fn get_all(client: &mut Client) -> Result<Vec<Account>, Error> {
        let endpoint = "brokerage/accounts";

        let resp = client
            .get(endpoint)
            .await?
            .json::<responses::GetAccountsResp>()
            .await?;

        Ok(resp.accounts)
    }

    /// Get the current balance of an `Account`
    pub async fn get_balance(&self, client: &mut Client) -> Result<Balance, Error> {
        let endpoint = format!("brokerage/accounts/{}/balances", self.account_id);

        if let Some(balance) = client
            .get(&endpoint)
            .await?
            .json::<responses::GetBalanceResp>()
            .await?
            .balances
            .pop()
        {
            Ok(balance)
        } else {
            Err(Error::AccountNotFound)
        }
    }

    /// Get the current balance of all `Account`(s) by account ids
    ///
    /// NOTE: If you have `Vec<Account>` you should instead use `Vec<Account>::get_balances()`
    /// this method should only be used in cases where you ONLY have account id's.
    pub async fn get_balances_by_ids(
        client: &mut Client,
        account_ids: Vec<&str>,
    ) -> Result<Vec<Balance>, Error> {
        let endpoint = format!("brokerage/accounts/{}/balances", account_ids.join(","));

        let resp = client
            .get(&endpoint)
            .await?
            .json::<responses::GetBalanceResp>()
            .await?;

        Ok(resp.balances)
    }

    /// Get the beginning of day balance of an `Account`
    pub async fn get_bod_balance(&self, client: &mut Client) -> Result<BODBalance, Error> {
        let endpoint = format!("brokerage/accounts/{}/bodbalances", self.account_id);

        if let Some(balance) = client
            .get(&endpoint)
            .await?
            .json::<responses::GetBODBalanceResp>()
            .await?
            .bod_balances
            .pop()
        {
            Ok(balance)
        } else {
            Err(Error::AccountNotFound)
        }
    }

    /// Get the beginning of day balances for multiple `Account`(s) by account id
    ///
    /// NOTE: If you have `Vec<Account>` you should instead use `Vec<Account>::get_bod_balances()`
    /// this method should only be used if you ONLY have account id's.
    pub async fn get_bod_balances_by_ids(
        client: &mut Client,
        account_ids: Vec<&str>,
    ) -> Result<Vec<BODBalance>, Error> {
        let endpoint = format!("brokerage/accounts/{}/bodbalances", account_ids.join(","));

        let resp = client
            .get(&endpoint)
            .await?
            .json::<responses::GetBODBalanceResp>()
            .await?;

        Ok(resp.bod_balances)
    }

    /// Fetches Historical `Order`(s) since a specific date for the given `Account`.
    ///
    /// NOTE: Date format is {YEAR-MONTH-DAY} ex: `"2024-07-09"`, and is limited to 90
    /// days prior to the current date.
    ///
    /// NOTE: Excludes open `Order`(s) and is sorted in descending order of time closed.
    pub async fn get_historic_orders(
        &self,
        client: &mut Client,
        since_date: &str,
    ) -> Result<Vec<Order>, Error> {
        let endpoint = format!(
            "brokerage/accounts/{}/historicalorders?since={}",
            self.account_id, since_date
        );

        let resp = client
            .get(&endpoint)
            .await?
            .json::<responses::GetOrdersResp>()
            .await?;

        Ok(resp.orders)
    }

    /// Fetches Historical `Order`(s) for the given `Account`(s) by id.
    ///
    /// NOTE: Date format is {YEAR-MONTH-DAY} ex: `"2024-07-09"`, and is limited to 90
    /// days prior to the current date.
    ///
    /// NOTE: Excludes open `Order`(s) and is sorted in descending order of time closed.
    pub async fn get_historic_orders_by_ids(
        client: &mut Client,
        account_ids: Vec<&str>,
        since_date: &str,
    ) -> Result<Vec<Order>, Error> {
        let endpoint = format!(
            "brokerage/accounts/{}/historicalorders?since={}",
            account_ids.join(","),
            since_date,
        );

        let resp = client
            .get(&endpoint)
            .await?
            .json::<responses::GetOrdersResp>()
            .await?;

        Ok(resp.orders)
    }

    /// Fetches positions for the given `Account`.
    pub async fn get_positions(&self, client: &mut Client) -> Result<Vec<Position>, Error> {
        let endpoint = format!("brokerage/accounts/{}/positions", self.account_id);

        let resp = client
            .get(&endpoint)
            .await?
            .json::<responses::GetPositionsResp>()
            .await?;

        Ok(resp.positions)
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
        client: &mut Client,
    ) -> Result<Vec<Position>, Error> {
        let endpoint = format!(
            "brokerage/accounts/{}/positions?symbol={}",
            self.account_id, symbols
        );

        let resp = client
            .get(&endpoint)
            .await?
            .json::<responses::GetPositionsResp>()
            .await?;

        Ok(resp.positions)
    }

    /// Fetches positions for the given `Account`.
    ///
    /// NOTE: If you have `Vec<Account>` you should instead use `Vec<Account>::get_positions()`
    /// this method should only be used if you ONLY have account id's.
    pub async fn get_positions_by_ids(
        client: &mut Client,
        account_ids: Vec<&str>,
    ) -> Result<Vec<Position>, Error> {
        let endpoint = format!("brokerage/accounts/{}/positions", account_ids.join(","));

        let resp = client
            .get(&endpoint)
            .await?
            .json::<responses::GetPositionsResp>()
            .await?;

        Ok(resp.positions)
    }

    /// Fetches positions for the given `Account`.
    ///
    /// NOTE: If you have `Vec<Account>` you should instead use `Vec<Account>::get_positions_in_symbols()`
    /// this method should only be used if you ONLY have account id's.
    ///
    /// NOTE: symbol should be a str of valid symbols in comma separated format;
    /// for example: `"MSFT,MSFT *,AAPL"`.
    ///
    /// NOTE: You can use an * as wildcard to make more complex filters.
    pub async fn get_positions_in_symbols_by_ids(
        client: &mut Client,
        symbols: &str,
        account_ids: Vec<&str>,
    ) -> Result<Vec<Position>, Error> {
        let endpoint = format!(
            "brokerage/accounts/{}/positions?symbol={}",
            account_ids.join(","),
            symbols
        );

        let resp = client
            .get(&endpoint)
            .await?
            .json::<responses::GetPositionsResp>()
            .await?;

        Ok(resp.positions)
    }

    /// Stream `Order`(s) for the given `Account`
    ///
    /// NOTE: You need to pass a closure function that will handle
    /// each chunk of data (`StreamOrdersResp`) as it's streamed in.
    ///
    /// Example: Get the amount of funds allocated to open orders.
    /// ```rust
    /// let mut funds_allocated_to_open_orders = 0.00;
    /// specific_account
    ///     .stream_orders(&mut client, |stream_data| {
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
    pub async fn stream_orders<F>(
        &self,
        client: &mut Client,
        mut on_chunk: F,
    ) -> Result<Vec<Order>, Error>
    where
        F: FnMut(StreamOrdersResp) -> Result<(), Error>,
    {
        let endpoint = format!("brokerage/stream/accounts/{}/orders", self.account_id);

        let mut collected_orders: Vec<Order> = Vec::new();
        client
            .stream(&endpoint, |chunk| {
                let parsed_chunk = serde_json::from_value::<StreamOrdersResp>(chunk)?;
                on_chunk(parsed_chunk.clone())?;

                // Only collect orders, so when the stream is done
                // all the orders that were streamed can be returned
                if let StreamOrdersResp::Order(order) = parsed_chunk {
                    collected_orders.push(*order);
                }

                Ok(())
            })
            .await?;

        Ok(collected_orders)
    }

    /// Stream `Order`(s) by order id's for the given `Account`
    ///
    /// NOTE: order ids should be a comma delimited string slice `"xxxxx,xxxxx,xxxxx"`
    ///
    /// NOTE: You need to pass a closure function that will handle
    /// each chunk of data (`StreamOrdersResp`) as it's streamed in.
    ///
    /// Example: Do something until all order's in a trade are filled.
    /// ```rust
    /// let mut some_trades_order_statuses: HashMap<String, String> = HashMap::new();
    /// specific_account
    ///     // NOTE: The order ids "1111,1112,1113,1114" are fake and not to be used.
    ///     .stream_orders_by_id(&mut client, "1111,1112,1113,1114", |stream_data| {
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
    ///                     .all(|order_status| order_status.eq("FLL"))
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
    pub async fn stream_orders_by_id<F>(
        &self,
        client: &mut Client,
        order_ids: &str,
        mut on_chunk: F,
    ) -> Result<Vec<Order>, Error>
    where
        F: FnMut(StreamOrdersResp) -> Result<(), Error>,
    {
        let endpoint = format!(
            "brokerage/stream/accounts/{}/orders/{}",
            self.account_id, order_ids
        );

        let mut collected_orders: Vec<Order> = Vec::new();
        client
            .stream(&endpoint, |chunk| {
                let parsed_chunk = serde_json::from_value::<StreamOrdersResp>(chunk)?;
                on_chunk(parsed_chunk.clone())?;

                // Only collect orders, so when the stream is done
                // all the orders that were streamed can be returned
                if let StreamOrdersResp::Order(order) = parsed_chunk {
                    collected_orders.push(*order);
                }

                Ok(())
            })
            .await?;

        Ok(collected_orders)
    }

    /// Stream `Order`(s) for the given `Account`
    ///
    /// NOTE: You need to pass a closure function that will handle
    /// each chunk of data (`StreamOrdersResp`) as it's streamed in.
    ///
    /// Example: Get the amount of funds allocated to open orders.
    /// ```rust
    /// let mut funds_allocated_to_open_orders = 0.00;
    /// specific_account
    ///     .stream_orders(&mut client, |stream_data| {
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
        client: &mut Client,
        account_ids: Vec<&str>,
        mut on_chunk: F,
    ) -> Result<Vec<Order>, Error>
    where
        F: FnMut(StreamOrdersResp) -> Result<(), Error>,
    {
        let endpoint = format!("brokerage/stream/accounts/{}/orders", account_ids.join(","));

        let mut collected_orders: Vec<Order> = Vec::new();
        client
            .stream(&endpoint, |chunk| {
                let parsed_chunk = serde_json::from_value::<StreamOrdersResp>(chunk)?;
                on_chunk(parsed_chunk.clone())?;

                // Only collect orders so when the stream is done
                // all the orders that were streamed can be returned
                if let StreamOrdersResp::Order(order) = parsed_chunk {
                    collected_orders.push(*order);
                }

                Ok(())
            })
            .await?;

        Ok(collected_orders)
    }

    /// Stream `Order`s by order id's for the given `Account`(s)
    ///
    /// NOTE: order ids should be a comma delimited string slice `"xxxxx,xxxxx,xxxxx"`
    ///
    /// NOTE: You need to pass a closure function that will handle
    /// each chunk of data (`StreamOrdersResp`) as it's streamed in.
    ///
    /// Example: Do something until all order's in a trade are filled.
    /// ```rust
    /// let mut some_trades_order_statuses: HashMap<String, String> = HashMap::new();
    /// specific_account
    ///     // NOTE: The order ids "1111,1112,1113,1114" are fake and not to be used.
    ///     .stream_orders_by_id(&mut client, "1111,1112,1113,1114", |stream_data| {
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
    ///                     .all(|order_status| order_status.eq("FLL"))
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
        client: &mut Client,
        order_ids: &str,
        account_ids: Vec<&str>,
        mut on_chunk: F,
    ) -> Result<Vec<Order>, Error>
    where
        F: FnMut(StreamOrdersResp) -> Result<(), Error>,
    {
        let endpoint = format!(
            "brokerage/stream/accounts/{}/orders/{}",
            account_ids.join(","),
            order_ids
        );

        let mut collected_orders: Vec<Order> = Vec::new();
        client
            .stream(&endpoint, |chunk| {
                let parsed_chunk = serde_json::from_value::<StreamOrdersResp>(chunk)?;
                on_chunk(parsed_chunk.clone())?;

                // Only collect orders so when the stream is done
                // all the orders that were streamed can be returned
                if let StreamOrdersResp::Order(order) = parsed_chunk {
                    collected_orders.push(*order);
                }

                Ok(())
            })
            .await?;

        Ok(collected_orders)
    }

    /// Stream `Position`s for the given `Account`
    ///
    /// NOTE: TODO: Currently does NOT support streaming `Position` changes.
    ///
    /// Example: Collect losing trades into a vector and do something with them.
    /// ```rust
    /// let mut losing_positions: Vec<Position> = Vec::new();
    /// specific_account
    ///     .stream_positions(&mut client, |stream_data| {
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
        client: &mut Client,
        mut on_chunk: F,
    ) -> Result<Vec<Position>, Error>
    where
        F: FnMut(StreamPositionsResp) -> Result<(), Error>,
    {
        let endpoint = format!("brokerage/stream/accounts/{}/positions", self.account_id);

        let mut collected_positions: Vec<Position> = Vec::new();
        client
            .stream(&endpoint, |chunk| {
                let parsed_chunk = serde_json::from_value::<StreamPositionsResp>(chunk)?;
                on_chunk(parsed_chunk.clone())?;

                // Only collect orders, so when the stream is done
                // all the orders that were streamed can be returned
                if let StreamPositionsResp::Position(position) = parsed_chunk {
                    collected_positions.push(*position);
                }

                Ok(())
            })
            .await?;

        Ok(collected_positions)
    }

    /// Stream `Position`s for the given `Account`(s)
    ///
    /// NOTE: TODO: Currently does NOT support streaming `Position` changes.
    ///
    /// Example: Collect losing trades into a vector and do something with them.
    /// ```rust
    /// let mut losing_positions: Vec<Position> = Vec::new();
    /// specific_account
    ///     .stream_positions(&mut client, |stream_data| {
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
    pub async fn stream_positions_for_accounts<F>(
        client: &mut Client,
        account_ids: Vec<&str>,
        mut on_chunk: F,
    ) -> Result<Vec<Position>, Error>
    where
        F: FnMut(StreamPositionsResp) -> Result<(), Error>,
    {
        let endpoint = format!(
            "brokerage/stream/accounts/{}/positions",
            account_ids.join(",")
        );

        let mut collected_positions: Vec<Position> = Vec::new();
        client
            .stream(&endpoint, |chunk| {
                let parsed_chunk = serde_json::from_value::<StreamPositionsResp>(chunk)?;
                on_chunk(parsed_chunk.clone())?;

                // Only collect orders, so when the stream is done
                // all the orders that were streamed can be returned
                if let StreamPositionsResp::Position(position) = parsed_chunk {
                    collected_positions.push(*position);
                }

                Ok(())
            })
            .await?;

        Ok(collected_positions)
    }
}

pub trait MultipleAccounts {
    /// Find an `Account` by it's id
    fn find_by_id(&self, id: &str) -> Option<Account>;

    type GetBalanceFuture<'a>: Future<Output = Result<Vec<Balance>, Box<dyn StdErrorTrait + Send + Sync>>>
        + Send
        + 'a
    where
        Self: 'a;
    /// Get the current balance of multiple `Account`(s)
    fn get_balances<'a>(&'a self, client: &'a mut Client) -> Self::GetBalanceFuture<'a>;

    type GetBODBalanceFuture<'a>: Future<Output = Result<Vec<BODBalance>, Box<dyn StdErrorTrait + Send + Sync>>>
        + Send
        + 'a
    where
        Self: 'a;
    /// Get the beginning of day balances for multiple `Account`(s) by account id
    fn get_bod_balances<'a>(&'a self, client: &'a mut Client) -> Self::GetBODBalanceFuture<'a>;

    type GetHistoricOrdersFuture<'a>: Future<Output = Result<Vec<Order>, Box<dyn StdErrorTrait + Send + Sync>>>
        + Send
        + 'a
    where
        Self: 'a;
    /// Get the historical `Order`(s) for multiple `Account`(s)
    ///
    /// NOTE: Date format is {YEAR-MONTH-DAY} ex: `"2024-07-09"`, and is limited to 90
    /// days prior to the current date.
    ///
    /// NOTE: Excludes open `Order`(s) and is sorted in descending order of time closed.
    fn get_historic_orders<'a>(
        &'a self,
        client: &'a mut Client,
        since_date: &'a str,
    ) -> Self::GetHistoricOrdersFuture<'a>;

    type GetPositionsFuture<'a>: Future<Output = Result<Vec<Position>, Box<dyn StdErrorTrait + Send + Sync>>>
        + Send
        + 'a
    where
        Self: 'a;
    /// Get the `Position`(s) for multiple `Account`(s)
    fn get_positions<'a>(&'a self, client: &'a mut Client) -> Self::GetPositionsFuture<'a>;

    type GetPositionsInSymbolsFuture<'a>: Future<Output = Result<Vec<Position>, Box<dyn StdErrorTrait + Send + Sync>>>
        + Send
        + 'a
    where
        Self: 'a;
    /// Get the `Position`(s) in specific symbols for multiple `Account`(s)
    fn get_positions_in_symbols<'a>(
        &'a self,
        symbols: &'a str,
        client: &'a mut Client,
    ) -> Self::GetPositionsFuture<'a>;

    type StreamOrdersFuture<'a>: Future<Output = Result<Vec<Order>, Box<dyn StdErrorTrait + Send + Sync>>>
        + Send
        + 'a
    where
        Self: 'a;
    /// Stream `Order`(s) for the given `Account`
    ///
    /// NOTE: You need to pass a closure function that will handle
    /// each chunk of data (`StreamOrdersResp`) as it's streamed in.
    ///
    /// Example: Get the amount of funds allocated to open orders.
    /// ```rust
    /// let mut funds_allocated_to_open_orders = 0.00;
    /// specific_account
    ///     .stream_orders(&mut client, |stream_data| {
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
        client: &'a mut Client,
    ) -> Self::StreamOrdersFuture<'a>
    where
        F: FnMut(StreamOrdersResp) -> Result<(), Error> + Send + 'a;

    type StreamOrdersByIdFuture<'a>: Future<Output = Result<Vec<Order>, Box<dyn StdErrorTrait + Send + Sync>>>
        + Send
        + 'a
    where
        Self: 'a;
    /// Stream `Order`s by order id's for the given `Account`(s)
    ///
    /// NOTE: order ids should be a comma delimited string slice `"xxxxx,xxxxx,xxxxx"`
    ///
    /// NOTE: You need to pass a closure function that will handle
    /// each chunk of data (`StreamOrdersResp`) as it's streamed in.
    ///
    /// Example: Do something until all order's in a trade are filled.
    /// ```rust
    /// let mut some_trades_order_statuses: HashMap<String, String> = HashMap::new();
    /// specific_account
    ///     // NOTE: The order ids "1111,1112,1113,1114" are fake and not to be used.
    ///     .stream_orders_by_id(&mut client, "1111,1112,1113,1114", |stream_data| {
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
    ///                     .all(|order_status| order_status.eq("FLL"))
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
        order_ids: &'a str,
        on_chunk: &'a mut F,
        client: &'a mut Client,
    ) -> Self::StreamOrdersByIdFuture<'a>
    where
        F: FnMut(StreamOrdersResp) -> Result<(), Error> + Send + 'a;

    type StreamPositionsFuture<'a>: Future<Output = Result<Vec<Position>, Box<dyn StdErrorTrait + Send + Sync>>>
        + Send
        + 'a
    where
        Self: 'a;
    /// Stream `Position`s for the given `Account`(s)
    ///
    /// NOTE: TODO: Currently does NOT support streaming `Position` changes.
    ///
    /// Example: Collect losing trades into a vector and do something with them.
    /// ```rust
    /// let mut losing_positions: Vec<Position> = Vec::new();
    /// specific_account
    ///     .stream_positions(&mut client, |stream_data| {
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
        client: &'a mut Client,
    ) -> Self::StreamPositionsFuture<'a>
    where
        F: FnMut(StreamPositionsResp) -> Result<(), Error> + Send + 'a;
}
impl MultipleAccounts for Vec<Account> {
    fn find_by_id(&self, id: &str) -> Option<Account> {
        self.iter()
            .filter(|account| account.account_id == id)
            .collect::<Vec<&Account>>()
            .pop()
            .cloned()
    }

    type GetBalanceFuture<'a> = Pin<
        Box<
            dyn Future<Output = Result<Vec<Balance>, Box<dyn StdErrorTrait + Send + Sync>>>
                + Send
                + 'a,
        >,
    >;
    /// Get the beginning of day balances for multiple `Account`(s)
    fn get_balances<'a>(&'a self, client: &'a mut Client) -> Self::GetBalanceFuture<'a> {
        let account_ids: Vec<&str> = self
            .iter()
            .map(|account| account.account_id.as_str())
            .collect();

        Box::pin(async move {
            let balances = Account::get_balances_by_ids(client, account_ids).await?;
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
    fn get_bod_balances<'a>(&'a self, client: &'a mut Client) -> Self::GetBODBalanceFuture<'a> {
        let account_ids: Vec<&str> = self
            .iter()
            .map(|account| account.account_id.as_str())
            .collect();

        Box::pin(async move {
            let balances = Account::get_bod_balances_by_ids(client, account_ids).await?;
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
        client: &'a mut Client,
        since_date: &'a str,
    ) -> Self::GetHistoricOrdersFuture<'a> {
        let account_ids: Vec<&str> = self
            .iter()
            .map(|account| account.account_id.as_str())
            .collect();

        Box::pin(async move {
            let balances =
                Account::get_historic_orders_by_ids(client, account_ids, since_date).await?;
            Ok(balances)
        })
    }

    type GetPositionsFuture<'a> = Pin<
        Box<
            dyn Future<Output = Result<Vec<Position>, Box<dyn StdErrorTrait + Send + Sync>>>
                + Send
                + 'a,
        >,
    >;
    fn get_positions<'a>(&'a self, client: &'a mut Client) -> Self::GetPositionsFuture<'a> {
        let account_ids: Vec<&str> = self
            .iter()
            .map(|account| account.account_id.as_str())
            .collect();

        Box::pin(async move {
            let positions = Account::get_positions_by_ids(client, account_ids).await?;
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
    fn get_positions_in_symbols<'a>(
        &'a self,
        symbols: &'a str,
        client: &'a mut Client,
    ) -> Self::GetPositionsFuture<'a> {
        let account_ids: Vec<&str> = self
            .iter()
            .map(|account| account.account_id.as_str())
            .collect();

        Box::pin(async move {
            let positions =
                Account::get_positions_in_symbols_by_ids(client, symbols, account_ids).await?;
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
    /// Example: Get the amount of funds allocated to open orders.
    /// ```rust
    /// let mut funds_allocated_to_open_orders = 0.00;
    /// specific_account
    ///     .stream_orders(&mut client, |stream_data| {
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
        client: &'a mut Client,
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
                Account::stream_orders_for_accounts(client, account_ids, &mut on_chunk).await?;
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
    /// Example: Do something until all order's in a trade are filled.
    /// ```rust
    /// let mut some_trades_order_statuses: HashMap<String, String> = HashMap::new();
    /// specific_account
    ///     // NOTE: The order ids "1111,1112,1113,1114" are fake and not to be used.
    ///     .stream_orders_by_id(&mut client, "1111,1112,1113,1114", |stream_data| {
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
    ///                     .all(|order_status| order_status.eq("FLL"))
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
        order_ids: &'a str,
        mut on_chunk: &'a mut F,
        client: &'a mut Client,
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
                client,
                order_ids,
                account_ids,
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
    /// Stream `Position`s for the given `Account`(s)
    ///
    /// NOTE: TODO: Currently does NOT support streaming `Position` changes.
    ///
    /// Example: Collect losing trades into a vector and do something with them.
    /// ```rust
    /// let mut losing_positions: Vec<Position> = Vec::new();
    /// specific_account
    ///     .stream_positions(&mut client, |stream_data| {
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
        client: &'a mut Client,
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
                Account::stream_positions_for_accounts(client, account_ids, &mut on_chunk).await?;
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
    // TODO: Make enum for this
    option_approval_level: u8,
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

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Balance {
    #[serde(rename = "AccountID")]
    /// The main identifier for a TradeStation account
    pub account_id: String,
    /// The type of account, examples: "Cash" or "Margin"
    pub account_type: String,
    /// The real time Cash Balance value for the `Account`
    pub cash_balance: String,
    /// The real time Buying Power value for the `Account`
    pub buying_power: String,
    /// The real time Equity value for the `Account`
    pub equity: String,
    /// The real time Market Value for the `Account`
    pub market_value: String,
    #[serde(rename = "TodaysProfitLoss")]
    /// The real time (profit - loss) value for the `Account` over a 24 hour period
    pub todays_pnl: String,
    /// The value of uncleared funds for the `Account`
    pub uncleared_deposit: String,
    /// Deeper details on the `Balance` of an `Account`
    pub balance_detail: BalanceDetail,
    /// The amount paid in brokerage commissions.
    ///
    /// NOTE: This value does not include slippage.
    pub commission: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BalanceDetail {
    /// The real time cost for all positions open in the `Account`
    ///
    /// NOTE: Positions are based on the actual entry price
    pub cost_of_positions: Option<String>,
    /// The number of day trades the `Account` has taken over the previous 4 days
    ///
    /// NOTE: This updates daily
    ///
    /// NOTE: This is always None for futures `Account`
    pub day_trades: Option<String>,
    /// The real time dollar amount of required funds for `Account` margin maintenance
    ///
    /// NOTE: SUM(maintenance margin of all open positions in the account)
    ///
    /// NOTE: This is always None for futures `Account`
    pub maintenance_rate: Option<String>,
    /// The real time value of intraday buying power for options
    ///
    /// NOTE: This is always None for futures `Account`
    pub option_buying_power: Option<String>,
    /// The real time Market Value of current open option positions in an `Account`
    pub options_market_value: Option<String>,
    /// The real time Buying Power value that can be held overnight w/o triggering a margin call
    ///
    /// NOTE: (Equity - Overnight Requirement %) / 50 %
    pub overnight_buying_power: Option<String>,
    /// The real time dollar value of open order Day Trade Margins for an `Account`
    ///
    /// NOTE: SUM(Day Trade Margin of all open orders in the account)
    ///
    /// NOTE: Always `None` for cash & margin accounts
    pub day_trade_open_order_margin: Option<String>,
    /// The real time dollar value of open order Initial Margin for an `Account`
    ///
    /// NOTE: SUM(Initial Margin of all open orders in the account)
    ///
    /// NOTE: Always `None` for cash & margin accounts
    pub open_order_margin: Option<String>,
    /// The real time dollar value of Initial Margin for an `Account`
    ///
    /// NOTE: SUM(Initial Margin of all open positions in the account)
    pub initial_margin: Option<String>,
    /// The real time dollar value of Maintenance Margin for an `Account`
    ///
    /// NOTE: SUM(Maintenance Margins of all open positions in the account)
    ///
    /// NOTE: Always `None` for cash & margin accounts
    pub maintenance_margin: Option<String>,
    /// The real time dollar amount of Trade Equity for an `Account`
    ///
    /// NOTE: Always `None` for cash & margin accounts
    pub trade_equity: Option<String>,
    /// The value of special securities deposited with the clearing firm
    /// for the sole purpose of increasing purchasing power in `Account`
    ///
    /// NOTE: This number will be reset daily by the account balances clearing file
    ///
    /// NOTE: The entire value of this field will increase purchasing power
    ///
    /// NOTE: Always `None` for cash & margin accounts
    pub security_on_deposit: Option<String>,
    /// The real time dollar value of Today's Trade Equity for an `Account`
    ///
    /// NOTE: (Beginning Day Trade Equity - Real Time Trade Equity)
    pub today_real_time_trade_equity: Option<String>,
    /// Deeper details on base currency
    ///
    /// NOTE: Always `None` for cash & margin accounts
    pub currency_details: Option<CurrencyDetails>,
    /// The real time amount of required funds for `Account` margin maintenance
    ///
    /// NOTE: The currency denomination is dependant on `Account::currency`
    ///
    /// NOTE: SUM(maintenance margin of all open positions in the account)
    /// NOTE: Always `None` for futures accounts
    pub required_margin: Option<String>,
    /// Funds received by TradeStation that are not settled from a transaction in the `Account`
    /// NOTE: Always `None` for futures accounts
    pub unsettled_funds: Option<String>,
    /// Maintenance Excess
    ///
    /// NOTE: (Cash Balance + Long Market Value + Short Credit - Maintenance Requirement - Margin Debt - Short Market Value)
    pub day_trade_excess: String,
    #[serde(rename = "RealizedProfitLoss")]
    /// The net Realized Profit or Loss of an `Account` for the current trading day
    ///
    /// NOTE: This includes all commissions and routing fees
    pub realized_pnl: String,
    #[serde(rename = "UnrealizedProfitLoss")]
    /// The net Unrealized Profit or Loss of an `Account` for all currently open positions
    ///
    /// NOTE: This does not include commissions or routing fees
    pub unrealized_pnl: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BODBalance {
    #[serde(rename = "AccountID")]
    /// The main identifier for a TradeStation account
    pub account_id: String,
    /// The type of account, examples: "Cash" or "Margin"
    pub account_type: String,
    /// Deeper details on the `Balance` of an `Account`
    pub balance_detail: BODBalanceDetail,
    /// Deeper details on the `Currency` local of an `Account`
    ///
    /// NOTE: Only applies to futures
    pub currency_details: Option<Vec<BODCurrencyDetails>>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BODBalanceDetail {
    /// The amount of cash in the account at the beginning of the day
    ///
    /// NOTE: Only applies to equities
    pub account_balance: Option<String>,
    /// Beginning of day value for cash available to withdraw
    pub cash_available_to_withdraw: Option<String>,
    /// The number of day trades placed in the account within the previous
    /// 4 trading days.
    ///
    /// NOTE: Only applies to equities
    pub day_trades: Option<String>,
    /// The Intraday Buying Power with which the account started the trading day
    ///
    /// NOTE: Only applies to equities
    pub day_trading_marginable_buying_power: Option<String>,
    /// The total amount of equity with which you started the current trading day
    pub equity: String,
    /// The amount of cash in the account at the beginning of the day
    pub net_cash: String,
    /// Unrealized profit and loss at the beginning of the day
    ///
    /// NOTE: Only applies to futures
    pub open_trade_equity: Option<String>,
    /// Option buying power at the start of the trading day
    ///
    /// NOTE: Only applies to equities
    pub option_buying_power: Option<String>,
    /// Intraday liquidation value of option positions
    ///
    /// NOTE: Only applies to equities
    pub option_value: Option<String>,
    /// Overnight Buying Power (Regulation T) at the start of the trading day
    ///
    /// NOTE: Only applies to equities
    pub overnight_buying_power: Option<String>,
    /// The value of special securities that are deposited by the customer with
    /// the clearing firm for the sole purpose of increasing purchasing power in
    /// their trading account.
    ///
    /// NOTE: Only applies to futures
    pub security_on_deposit: Option<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BODCurrencyDetails {
    /// The dollar amount of Beginning Day Margin for the given forex account
    pub account_margin_requirement: Option<String>,
    /// The dollar amount of Beginning Day Trade Equity for the given account
    pub account_open_trade_equity: String,
    /// The value of special securities that are deposited by the customer with
    /// the clearing firm for the sole purpose of increasing purchasing power in
    /// their trading account.
    ///
    /// NOTE: This number will be reset daily by the account balances
    /// clearing file.
    ///
    /// NOTE: The entire value of this field will increase purchasing power
    pub account_securities: String,
    /// The dollar amount of the Beginning Day Cash Balance for the given account
    pub cash_balance: String,
    /// The currency of the entity
    pub currency: String,
    /// The dollar amount of Beginning Day Margin for the given forex account
    pub margin_requirement: Option<String>,
    /// The dollar amount of Beginning Day Trade Equity for the given account
    pub open_trade_equity: String,
    /// Indicates the dollar amount of Beginning Day Securities
    pub securities: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CurrencyDetails {
    /// Base currency
    currency: String,
    /// The net Unrealized Profit or Loss for all currently open positions
    ///
    /// NOTE: This does not include commissions or routing fees
    commission: String,
    /// The real time value of an `Account`(s) Cash Balance
    cash_balance: String,
    #[serde(rename = "RealizedProfitLoss")]
    /// The net Realized Profit or Loss of an `Account` for the current trading day
    ///
    /// NOTE: This includes all commissions and routing fees
    realized_pnl: String,
    #[serde(rename = "UnrealizedProfitLoss")]
    /// The net Unrealized Profit or Loss of an `Account` for all currently open positions
    ///
    /// NOTE: This does not include commissions or routing fees
    unrealized_pnl: String,
    /// The real time dollar value of Initial Margin for an `Account`
    ///
    /// NOTE: SUM(Initial Margin of all open positions in the account)
    initial_margin: String,
    /// The real time dollar value of Maintenance Margin for an `Account`
    ///
    /// NOTE: SUM(Maintenance Margins of all open positions in the account)
    maintenance_margin: String,
    /// The real time conversion rate used to translate value from symbol currency to `Account` currency
    account_conversion_rate: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Order {
    #[serde(rename = "AccountID")]
    /// The `Account` id to the this `Order` belongs to
    pub account_id: String,
    /// The `Order rules` or brackets
    pub advanced_options: Option<String>,
    /// The Closed Date Time of this `order`
    pub closed_date_time: Option<String>,
    /// The actual brokerage commission cost and routing fees
    /// for a trade based on the number of shares or contracts.
    pub commission_fee: String,
    /// The relationship between linked `Order`(s) in a group
    /// and this `Order` in specific.
    pub conditional_orders: Option<ConditionalOrder>,
    /// The rate used to convert from the currency of
    /// the symbol to the currency of the account.
    pub conversion_rate: Option<String>,
    /// The currency used to complete the `Order`.
    pub currency: String,
    /// The amount of time for which the `Order` is valid.
    pub duration: String,
    /// At the top level, this is the average fill price.
    ///
    /// At the expanded levels, this is the actual execution price.
    pub filled_price: Option<String>,
    /// The expiration date-time for the `Order`
    ///
    /// NOTE: The time portion, if "T:00:00:00Z", should be ignored.
    pub good_till_date: Option<String>,
    /// An identifier for `Order`(s) that are part of the same bracket.
    pub group_name: Option<String>,
    /// Legs (multi step/part trade) associated with this `Order`
    pub legs: Vec<OrderLeg>,
    /// Allows you to specify when an order will be placed based on
    /// the price action of one or more symbols.
    // TODO: Should I convert None to empty vector ?
    pub market_activation_rules: Option<Vec<MarketActivationRule>>,
    /// Allows you to specify a time that an `Order` will be placed.
    // TODO: Should I convert None to empty vector ?
    pub time_activation_rules: Option<Vec<TimeActivationRule>>,
    /// The limit price for Limit and Stop Limit `Order`(s).
    pub limit_price: Option<String>,
    /// Time the `Order` was placed.
    pub opened_date_time: String,
    #[serde(rename = "OrderID")]
    /// The `order` id.
    pub order_id: String,
    /// The type of `Order` this is.
    pub order_type: OrderType,
    /// Price used for the buying power calculation of the `Order`.
    pub price_used_for_buying_power: String,
    /// Identifies the routing selection made by the customer when
    /// placing the `Order`.
    pub routing: String,
    /// Hides the true number of shares intended to be bought or sold.
    ///
    /// NOTE: ONLY valid for `OrderType::Limit` or `Order::Type::StopLimit`
    /// `Order`(s).
    ///
    /// NOTE: Not valid for all exchanges.
    pub show_only_quantity: Option<String>,
    /// The spread type for an option `Order`
    pub spread: Option<String>,
    /// The status of an `Order`
    // TODO: make enum for this.
    pub status: String,
    /// Description of the `Order` status
    pub status_description: String,
    /// The stop price for `OrderType::StopLimit` and
    /// `OrderType::StopMarket` orders.
    pub stop_price: Option<String>,
    /// TrailingStop offset.
    ///
    /// NOTE: amount or percent.
    pub trailing_stop: Option<TrailingStop>,
    /// Only applies to equities.
    ///
    /// NOTE: Will contain a value if the order has received a routing fee.
    pub unbundled_route_fee: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct TrailingStop {
    /// Currency Offset from current price.
    ///
    /// NOTE: Mutually exclusive with Percent.
    pub amount: Option<String>,
    /// Percentage offset from current price.
    ///
    /// NOTE: Mutually exclusive with Amount.
    pub percent: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
/// Types of `Order`(s).
pub enum OrderType {
    Limit,
    Market,
    StopMarket,
    StopLimit,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// Allows you to specify a time that an `Order` will be placed.
pub struct TimeActivationRule {
    /// Timestamp represented as an RFC3339 formatted date.
    time_utc: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// Allows you to specify when an order will be placed
/// based on the price action of one or more symbols.
pub struct MarketActivationRule {
    /// The type of activation rule.
    ///
    /// NOTE: Currently only supports `"Price"` for now.
    pub rule_type: String,
    /// The symbol that the rule is based on.
    pub symbol: String,
    /// The type of comparison predicate the rule is based on.
    pub predicate: Predicate,
    /// The ticks behavior for the activation rule.
    pub tigger_key: TickTrigger,
    /// The price at which the rule will trigger.
    pub price: Option<String>,
    /// Relation with the previous activation rule.
    ///
    /// NOTE: The first rule will never have a logic operator.
    pub logic_operator: Option<LogicOp>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
/// Logic Operators
pub enum LogicOp {
    And,
    Or,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
/// Types of tick triggers
pub enum TickTrigger {
    /// Single Trade Tick
    STT,
    /// Single Trade Tick Within NBBO
    STTN,
    /// Single Bid/Ask Tick
    SBA,
    /// Single Ask/Bid Tick
    SAB,
    /// Double Trade Tick
    DTT,
    /// Double Trade Tick Within NBBO
    DTTN,
    /// Double Bid/Ask Tick
    DBA,
    /// Double Ask/Bid Tick
    DAB,
    /// Triple Trade Tick
    TTT,
    /// Triple Trade Tick Within NBBO
    TTTN,
    /// Triple Bid/Ask Tick
    TBA,
    /// Triple Ask/Bid Tick
    TAB,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
/// Types of comparison predicates
pub enum Predicate {
    /// Less than
    Lt,
    /// Less than or Equal
    Lte,
    /// Greater than
    Gt,
    /// Greater than or Equal
    Gte,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// Order Leg associated with this `Order`
pub struct OrderLeg {
    /// Indicates the asset type of the `Order`.
    pub asset_type: AssetType,
    /// identifier for the `Order` action (buying or selling)
    pub buy_or_sell: OrderAction,
    /// Number of shares or contracts that have been executed.
    pub exec_quantity: String,
    /// The price at which `Order` execution occurred.
    pub execution_price: Option<String>,
    /// The expiration date of the future or option contract.
    pub expiration_date: Option<String>,
    /// The stage of the `Order` , is it opening or closing?
    pub open_or_close: Option<OrderStage>,
    /// The type of option
    pub option_type: Option<OptionType>,
    /// Number of shares or contracts being purchased or sold.
    pub quantity_ordered: String,
    /// In a partially filled `Order` , this is the number of shares
    /// or contracts the have NOT yet been filled.
    pub quantity_remaining: String,
    /// The price at which the holder of an options contract can buy
    /// or sell the underlying asset.
    ///
    /// NOTE: ONLY for option `Order`(s).
    pub strike_price: Option<String>,
    /// The securities symbol the `Order` is for.
    pub symbol: String,
    /// The underlying securities symbol the `Order` is for.
    ///
    /// NOTE: ONLY for futures and options.
    pub underlying: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
/// The type of option
pub enum OptionType {
    #[serde(rename = "CALL")]
    Call,
    #[serde(rename = "PUT")]
    Put,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
/// The stage of the `Order` , is it opening or closing?
pub enum OrderStage {
    Open,
    Close,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum OrderAction {
    Buy,
    Sell,
    /// Open a short position
    SellShort,
    /// Closing a short position
    BuyToCover,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum AssetType {
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "STOCK")]
    Stock,
    #[serde(rename = "STOCKOPTION")]
    StockOption,
    #[serde(rename = "FUTURE")]
    Future,
    #[serde(rename = "FUTUREOPTION")]
    FutureOption,
    #[serde(rename = "FOREX")]
    Forex,
    #[serde(rename = "CURRENCYOPTION")]
    CurrencyOption,
    #[serde(rename = "INDEX")]
    Index,
    #[serde(rename = "INDEXOPTION")]
    IndexOption,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConditionalOrder {
    #[serde(rename = "OrderID")]
    /// The id of the linked `Order`.
    pub order_id: String,
    /// The relationship of a linked order within a group order
    /// to the current returned `Order`.
    pub relationship: OrderRelationship,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
/// Types of `Order` relationships
pub enum OrderRelationship {
    BRK,
    OSP,
    OSO,
    OCO,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Position {
    #[serde(rename = "AccountID")]
    /// The `Account` id the `Position` belongs to.
    pub account_id: String,
    /// Indicates the asset type of the position.
    // NOTE: use enum
    pub asset_type: String,
    /// The average price of the position currently held.
    pub average_price: String,
    /// The highest price a prospective buyer is prepared to pay at
    /// a particular time for a trading unit of a given symbol.
    pub bid: String,
    /// The price at which a security, futures contract, or other
    /// financial instrument is offered for sale.
    pub ask: String,
    /// The currency conversion rate that is used in order to convert
    /// from the currency of the symbol to the currency of the account.
    pub conversion_rate: String,
    /// DayTradeMargin used on open positions.
    ///
    /// NOTE: Currently only calculated for futures positions.
    /// Other asset classes will have a 0 for this value.
    pub day_trade_requirement: String,
    /// The UTC formatted expiration date of the future or option symbol,
    /// in the country the contract is traded in.
    ///
    /// NOTE: The time portion of the value should be ignored.
    pub expiration_date: Option<String>,
    /// The margin account balance denominated in the symbol currency required
    /// for entering a position on margin.
    ///
    /// NOTE: Only applies to future and option positions.
    pub initial_requirement: String,
    /// The last price at which the symbol traded.
    pub last: String,
    /// Specifies if the position is Long or Short.
    pub long_short: PositionType,
    /// The MarkToMarketPrice value is the weighted average of the previous close
    /// price for the position quantity held overnight and the purchase price of the
    /// position quantity opened during the current market session.
    ///
    /// NOTE: This value is used to calculate TodaysProfitLoss.
    ///
    /// NOTE: Only applies to equity and option positions.
    pub mark_to_market_price: String,
    /// The actual market value denominated in the symbol currency of the open position.
    ///
    /// NOTE: This value is updated in real-time.
    pub market_value: String,
    #[serde(rename = "PositionID")]
    /// A unique identifier for the position.
    pub position_id: String,
    /// The number of shares or contracts for a particular position.
    ///
    /// NOTE: This value is negative for short positions.
    pub quantity: String,
    /// Symbol of the position.
    pub symbol: String,
    /// Time the position was entered.
    pub timestamp: String,
    /// The unrealized profit or loss denominated in the account currency on the position
    /// held, calculated using the MarkToMarketPrice.
    ///
    /// NOTE: Only applies to equity and option positions.
    #[serde(rename = "TodaysProfitLoss")]
    pub todays_pnl: String,
    /// The total cost denominated in the account currency of the open position.
    pub total_cost: String,
    #[serde(rename = "UnrealizedProfitLoss")]
    /// The unrealized profit or loss denominated in the symbol currency on the position
    /// held, calculated based on the average price of the position.
    pub unrealized_pnl: String,
    #[serde(rename = "UnrealizedProfitLossPercent")]
    /// The unrealized profit or loss on the position expressed as a percentage of the
    /// initial value of the position.
    pub unrealized_pnl_percent: String,
    #[serde(rename = "UnrealizedProfitLossQty")]
    /// The unrealized profit or loss denominated in the account currency divided by the
    /// number of shares, contracts or units held.
    pub unrealized_pnl_qty: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
/// A position type can either be short or long
pub enum PositionType {
    /// Long a share, or futures/options contract
    Long,
    /// Short a share, or futures/options contract
    Short,
}

impl Client {
    /// Get all of your registered TradeStation `Accounts`
    pub async fn get_accounts(&mut self) -> Result<Vec<Account>, Error> {
        Account::get_all(self).await
    }

    /// Get a specific TradeStation `Account` by it's account id
    pub async fn get_account(&mut self, account_id: &str) -> Result<Account, Error> {
        Account::get(self, account_id).await
    }

    /// Get the current balance of a specific `Account` by it's account id
    pub async fn get_account_balance(&mut self, account_id: &str) -> Result<Balance, Error> {
        let mut balances = Account::get_balances_by_ids(self, vec![account_id]).await?;
        if balances.len() == 1 {
            // NOTE: This unwrap is panic safe due to invariant above
            let balance = balances.pop().unwrap();
            Ok(balance)
        } else {
            Err(Error::AccountNotFound)
        }
    }

    /// Get the current balance of all `Account`(s)
    pub async fn get_account_balances(
        &mut self,
        account_ids: Vec<&str>,
    ) -> Result<Vec<Balance>, Error> {
        Account::get_balances_by_ids(self, account_ids).await
    }
}
