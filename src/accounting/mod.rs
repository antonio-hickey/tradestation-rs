//! Types and functionality related to the accounting endpoints.
//!
//! ---
//!
//!
//! **Endpoint: Get Accounts**
//! - Fetches the list of Brokerage Accounts available for the current user.
//! - Account Reference: [`Account`]
//! - **Example**: Get a list of all your brokerage accounts and get a specific brokerage account.
//!
//! ```rust,no_run
//! # use tradestation::{Error, Client, ClientBuilder, ClientEnvironment, Token, Scope};
//! # async fn get_accounts_example() -> Result<(), Error> {
//! let client = ClientBuilder::new()
//!     .credentials("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")
//!     .environment(ClientEnvironment::Live)
//!     .with_token(Token {
//!         access_token: String::from("YOUR_ACCESS_TOKEN"),
//!         refresh_token: String::from("YOUR_REFRESH_TOKEN"),
//!         id_token: String::from("YOUR_ID_TOKEN"),
//!         token_type: String::from("Bearer"),
//!         scope: vec![Scope::ReadAccount],
//!         expires_in: 1200,
//!     })
//!     .build()
//!     .await?;
//!
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
//!
//! **Endpoint: Balances**
//! - Fetches the brokerage account Balances for one or more given accounts. Request valid for Cash, Margin, Futures, and DVP account types.
//! - Balance Reference: [`Balance`]
//! - **Example**: Get the current balance of a specific account.
//!
//! ```rust,no_run
//! # use tradestation::{Error, Client, ClientBuilder, ClientEnvironment, Token, Scope};
//! # async fn get_accounts_example() -> Result<(), Error> {
//! let client = ClientBuilder::new()
//!     .credentials("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")
//!     .environment(ClientEnvironment::Live)
//!     .with_token(Token {
//!         access_token: String::from("YOUR_ACCESS_TOKEN"),
//!         refresh_token: String::from("YOUR_REFRESH_TOKEN"),
//!         id_token: String::from("YOUR_ID_TOKEN"),
//!         token_type: String::from("Bearer"),
//!         scope: vec![Scope::ReadAccount],
//!         expires_in: 1200,
//!     })
//!     .build()
//!     .await?;
//!
//! // Get the balance of a specific account
//! let balance = client.get_balance("YOUR_ACCOUNT_ID").await?;
//!
//! // Get the balances of all accounts
//! let balances = client.get_balances().await?;
//!
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
