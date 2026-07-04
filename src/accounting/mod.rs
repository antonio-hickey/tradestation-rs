//! # Accounting Endpoints
//! - [Get Accounts](#endpoint-get-accounts): Fetches the list of Brokerage Accounts available for the current user.
//! - [Get Balances](#endpoint-get-balances): Fetches the brokerage account Balances for one or more given accounts.
//! - [Get BOD Balances](#endpoint-get-bod-balances): Fetches the brokerage account Balances for one or more given accounts.
//! - [Get Historical Orders](#endpoint-get-historical-orders): Fetches Historical Orders for the given Accounts except open orders.
//! - [Get Orders](#endpoint-get-orders): Fetches today's orders and open orders for the given Accounts.
//! - [Get Orders By Order ID](#endpoint-get-orders-by-order-id): Fetches today's orders and open orders for the given Accounts, filtered by given Order ID's.
//! - [Get Positions](#endpoint-get-positions): Fetches positions for the given Accounts.
//! - [Stream Orders](#endpoint-stream-orders): Stream orders for the given accounts.
//! - [Stream Orders By Order ID](#endpoint-stream-orders-by-order-id): Stream orders for the given accounts, filtered by given Order ID's.
//! - [Stream Positions](#endpoint-stream-positions): Stream positions for the given accounts.
//!
//! # Endpoint: Get Accounts
//! - Fetches the list of Brokerage Accounts available for the current user.
//! - Account Reference: [`crate::accounting::Account`]
//! - **Example**: Get a list of all your brokerage accounts and get a specific brokerage account.
//!
//! ```rust,no_run
//! # use tradestation::{Error, Client};
//! # async fn get_accounts_example(client: &Client) -> Result<(), Error> {
//! // Get all your accounts
//! let accounts = client.get_accounts().await?;
//!
//! // Get a specific account
//! let account = client.get_account("YOUR_ACCOUNT_ID").await?;
//! # Ok(()) }
//! ```
//!
//! ---
//!
//! # Endpoint: Get Balances
//! - Fetches the brokerage account Balances for one or more given accounts. Request valid for Cash, Margin, Futures, and DVP account types.
//! - Balance Reference: [`crate::accounting::Balance`]
//! - **Example**: Get the current balance of a specific account.
//!
//! ```rust,no_run
//! # use tradestation::{Error, Client, accounting::MultipleAccounts};
//! # async fn get_accounts_example(client: &Client) -> Result<(), Error> {
//! // Get the balance of a specific account
//! let account = client.get_account("YOUR_ACCOUNT_ID").await?;
//! let balance = account.get_balance(client).await?;
//!
//! // Get the balances of all accounts
//! let accounts = client.get_accounts().await?;
//! let balances = accounts.get_balances(client).await?;
//!
//! # Ok(()) }
//! ```
//!
//! ---
//!
//! # Endpoint: Get BOD Balances
//! - Fetches beginning-of-day balances for one or more brokerage accounts.
//! - BOD Balance Reference: [`crate::accounting::BODBalance`]
//! - **Example**: Compare current balances with beginning-of-day balances.
//!
//! ```rust,no_run
//! # use tradestation::{Error, Client, accounting::MultipleAccounts};
//! # async fn get_bod_balances_example(client: &Client) -> Result<(), Error> {
//! let account = client.get_account("YOUR_ACCOUNT_ID").await?;
//! let bod_balance = account.get_bod_balance(client).await?;
//! println!("BOD equity: {}", bod_balance.balance_detail.equity);
//!
//! let accounts = client.get_accounts().await?;
//! let bod_balances = accounts.get_bod_balances(client).await?;
//! println!("Loaded {} BOD balances", bod_balances.len());
//! # Ok(()) }
//! ```
//!
//! ---
//!
//! # Endpoint: Get Historical Orders
//! - Fetches historical orders since a given date. Open orders are excluded.
//! - Order Reference: [`crate::accounting::Order`]
//! - **Example**: Load closed orders from the last 90 days for one account and for all accounts.
//!
//! ```rust,no_run
//! # use tradestation::{Error, Client};
//! # use tradestation::accounting::MultipleAccounts;
//! # async fn get_historical_orders_example(client: &Client) -> Result<(), Error> {
//! let account = client.get_account("YOUR_ACCOUNT_ID").await?;
//! let account_history = account
//!     .get_historic_orders(client, "2026-06-01")
//!     .await?;
//!
//! let accounts = client.get_accounts().await?;
//! let all_history = accounts
//!     .get_historic_orders(client, "2026-06-01")
//!     .await?;
//!
//! println!(
//!     "Loaded {} account orders and {} cross-account orders",
//!     account_history.len(),
//!     all_history.len()
//! );
//! # Ok(()) }
//! ```
//!
//! ---
//!
//! # Endpoint: Get Historical Orders
//! - Fetches historical orders since a given date. Open orders are excluded.
//! - Order Reference: [`crate::accounting::Order`]
//! - **Example**: Load closed orders from the last 90 days for one account and for all accounts.
//!
//! ```rust,no_run
//! # use tradestation::{Error, Client, accounting::MultipleAccounts};
//! # async fn get_historical_orders_example(client: &Client) -> Result<(), Error> {
//! let account = client.get_account("YOUR_ACCOUNT_ID").await?;
//! let account_history = account
//!     .get_historic_orders(client, "2026-06-01")
//!     .await?;
//!
//! let accounts = client.get_accounts().await?;
//! let all_history = accounts
//!     .get_historic_orders(client, "2026-06-01")
//!     .await?;
//!
//! println!(
//!     "Loaded {} account orders and {} cross-account orders",
//!     account_history.len(),
//!     all_history.len()
//! );
//! # Ok(()) }
//! ```
//!
//! ---
//!
//! # Endpoint: Get Orders
//! - Fetches today's orders for one or more accounts.
//! - Order Reference: [`crate::accounting::Order`]
//! - **Example**: Load today's orders and keep only filled orders.
//!
//! ```rust,no_run
//! # use tradestation::{Error, Client};
//! # use tradestation::accounting::{MultipleAccounts, OrderStatus};
//! # async fn get_orders_example(client: &Client) -> Result<(), Error> {
//! let account = client.get_account("YOUR_ACCOUNT_ID").await?;
//! let filled_orders = account
//!     .get_orders(client)
//!     .await?
//!     .into_iter()
//!     .filter(|order| order.status == OrderStatus::FLL)
//!     .collect::<Vec<_>>();
//!
//! let accounts = client.get_accounts().await?;
//! let all_orders = accounts.get_orders(client).await?;
//!
//! println!(
//!     "Found {} filled orders in one account and {} total orders",
//!     filled_orders.len(),
//!     all_orders.len()
//! );
//! # Ok(()) }
//! ```
//!
//! ---
//!
//! # Endpoint: Get Orders By Order ID
//! - Fetches today's orders for one or more accounts, filtered by order id.
//! - Order Reference: [`crate::accounting::Order`]
//! - **Example**: Load today's orders and keep only filled orders.
//!
//! ```rust,no_run
//! # use tradestation::{Error, Client, accounting::{MultipleAccounts, OrderStatus}};
//! # async fn get_orders_by_id_example(client: &Client) -> Result<(), Error> {
//! let order_ids = vec!["ORDER_ID_1", "ORDER_ID_N"];
//!
//! let accounts = client.get_accounts().await?;
//! let orders = accounts.get_orders_by_id(client, &order_ids).await?;
//!
//! let filled_orders = orders
//!     .iter()
//!     .filter(|order| order.status == OrderStatus::FLL)
//!     .count();
//!
//! println!(
//!     "some trade status: {}/{} orders filled",
//!     filled_orders,
//!     orders.len()
//! );
//! # Ok(()) }
//! ```
//!
//! ---
//!
//! # Endpoint: Get Positions
//! - Fetches open positions for one or more accounts.
//! - Position Reference: [`crate::accounting::Position`]
//! - **Example**: Load open positions and filter by unrealized loss.
//!
//! ```rust,no_run
//! # use tradestation::{Error, Client};
//! # use tradestation::accounting::MultipleAccounts;
//! # async fn get_positions_example(client: &Client) -> Result<(), Error> {
//! let account = client.get_account("YOUR_ACCOUNT_ID").await?;
//! let losing_positions = account
//!     .get_positions(client)
//!     .await?
//!     .into_iter()
//!     .filter(|position| {
//!         position
//!             .unrealized_pnl
//!             .parse::<f64>()
//!             .map(|pnl| pnl < 0.0)
//!             .unwrap_or(false)
//!     })
//!     .collect::<Vec<_>>();
//!
//! let accounts = client.get_accounts().await?;
//! let all_positions = accounts.get_positions(client).await?;
//!
//! println!(
//!     "Found {} losing positions in one account and {} total positions",
//!     losing_positions.len(),
//!     all_positions.len()
//! );
//! # Ok(()) }
//! ```
//!
//! ---
//!
//! # Endpoint: Stream Orders
//! - Streams order events for one or more accounts.
//! - Stream Response Reference: [`crate::responses::account::StreamOrdersResp`]
//! - **Example**: Watch order events until a heartbeat limit is reached.
//!
//! ```rust,no_run
//! # use futures::StreamExt;
//! # use tradestation::{Error, Client, responses::account::StreamOrdersResp};
//! # async fn stream_orders_example(client: &Client) -> Result<(), Error> {
//! let account = client.get_account("YOUR_ACCOUNT_ID").await?;
//! let orders_stream = account.stream_orders(client);
//! tokio::pin!(orders_stream);
//!
//! while let Some(event) = orders_stream.next().await {
//!     match event? {
//!         StreamOrdersResp::Order(order) => {
//!             println!("Order {} is {:?}", order.order_id, order.status);
//!         }
//!         StreamOrdersResp::Heartbeat(heartbeat) if heartbeat.heartbeat > 10 => {
//!             return Err(Error::StopStream);
//!         }
//!         other => println!("{other:?}"),
//!     }
//! }
//! # Ok(()) }
//! ```
//!
//! ---
//!
//! # Endpoint: Stream Orders By ID
//! - Streams order events for one or more accounts filtered by given order id's.
//! - Stream Response Reference: [`crate::responses::account::StreamOrdersResp`]
//! - **Example**: Watch order events until a heartbeat limit is reached.
//!
//! ```rust,no_run
//! # use futures::StreamExt;
//! # use tradestation::{Error, Client, responses::account::StreamOrdersResp};
//! # async fn stream_orders_by_id_example(client: &Client) -> Result<(), Error> {
//! let order_ids = vec!["ORDER_ID_1", "ORDER_ID_N"];
//!
//! let account = client.get_account("YOUR_ACCOUNT_ID").await?;
//! let orders_stream = account.stream_orders_by_id(client, order_ids);
//! tokio::pin!(orders_stream);
//!
//! while let Some(event) = orders_stream.next().await {
//!     match event? {
//!         StreamOrdersResp::Order(order) => {
//!             println!("Order {} is {:?}", order.order_id, order.status);
//!         }
//!         StreamOrdersResp::Heartbeat(heartbeat) if heartbeat.heartbeat > 10 => {
//!             return Err(Error::StopStream);
//!         }
//!         other => println!("{other:?}"),
//!     }
//! }
//! # Ok(()) }
//! ```
//!
//! ---
//!
//! # Endpoint: Stream Positions
//! - Streams position events for one or more accounts.
//! - Stream Response Reference: [`crate::responses::account::StreamPositionsResp`]
//! - **Example**: Watch position events and stop after repeated inactivity.
//!
//! ```rust,no_run
//! # use futures::StreamExt;
//! # use tradestation::{Error, Client};
//! # use tradestation::responses::account::StreamPositionsResp;
//! # async fn stream_positions_example(client: &Client) -> Result<(), Error> {
//! let account = client.get_account("YOUR_ACCOUNT_ID").await?;
//! let positions_stream = account.stream_positions(client);
//! tokio::pin!(positions_stream);
//!
//! while let Some(event) = positions_stream.next().await {
//!     match event? {
//!         StreamPositionsResp::Position(position) => {
//!             println!(
//!                 "Position {} unrealized P/L: {}",
//!                 position.position_id,
//!                 position.unrealized_pnl
//!             );
//!         }
//!         StreamPositionsResp::Heartbeat(heartbeat) if heartbeat.heartbeat > 10 => {
//!             return Err(Error::StopStream);
//!         }
//!         other => println!("{other:?}"),
//!     }
//! }
//! # Ok(()) }
//! ```

/// Types and functionality for working with brokerage accounts.
pub mod accounts;

/// Types and functionality for working with account balances.
pub mod balances;

/// Types and functionality for working with account orders.
pub mod orders;

/// Types and functionality for working with account positions.
pub mod positions;

pub use accounts::{Account, MultipleAccounts};
pub use balances::{
    BODBalance, BODBalanceDetail, BODCurrencyDetails, Balance, BalanceDetail, CurrencyDetails,
};
pub use orders::{
    AssetType, LogicOp, OptionType, Order, OrderAction, OrderRelationship, OrderStage, OrderStatus,
    OrderType, Predicate, TickTrigger,
};
pub use positions::{Position, PositionType};
