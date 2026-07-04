//! # Order Execution Endpoints
//! - [Confirm Order](#endpoint-confirm-order): Previews an order and returns estimated cost, commission, and validation details.
//! - [Confirm Group Order](#endpoint-confirm-group-order): Previews a grouped order request.
//! - [Place Order](#endpoint-place-order): Submits a single order for execution.
//! - [Place Group Order](#endpoint-place-group-order): Submits a group of orders for execution.
//! - [Replace Order](#endpoint-replace-order): Replaces an open order with updated order values.
//! - [Cancel Order](#endpoint-cancel-order): Cancels an active order.
//! - [Get Routes](#endpoint-get-routes): Fetches valid execution routes.
//! - [Get Activation Triggers](#endpoint-get-activation-triggers): Fetches valid stop activation trigger keys.
//!
//! # Endpoint: Confirm Order
//! - Previews an order before placing it and returns estimated cost, commission, and validation details.
//! - Order Request Reference: [`crate::execution::OrderRequest`]
//! - Confirmation Reference: [`crate::execution::confirm::OrderConfirmation`]
//! - **Example**: Confirm a limit buy order without submitting it.
//!
//! ```rust,no_run
//! # use tradestation::{Client, Error};
//! # use tradestation::orders::{
//! #     Duration, Order, OrderRequestBuilder, OrderTimeInForce, OrderType, TradeAction,
//! # };
//! # async fn confirm_order_example(client: &Client) -> Result<(), Error> {
//! let order = OrderRequestBuilder::new()
//!     .account_id("YOUR_ACCOUNT_ID")
//!     .symbol("MSFT")
//!     .trade_action(TradeAction::Buy)
//!     .quantity("10")
//!     .order_type(OrderType::Limit)
//!     .limit_price("420.00")
//!     .time_in_force(OrderTimeInForce {
//!         duration: Duration::DAY,
//!         expiration: None,
//!     })
//!     .build()?;
//!
//! let confirmations = Order::confirm(client, &order).await?;
//! println!("Loaded {} confirmations", confirmations.len());
//! # Ok(()) }
//! ```
//!
//! ---
//!
//! # Endpoint: Confirm Group Order
//! - Previews a grouped order request before placing it.
//! - Order Group Reference: [`crate::execution::OrderRequestGroup`]
//! - Confirmation Reference: [`crate::execution::confirm::OrderConfirmation`]
//! - **Example**: Confirm an OCO group containing a limit order and a stop order.
//!
//! ```rust,no_run
//! # use tradestation::{Client, Error};
//! # use tradestation::orders::{
//! #     Duration, Order, OrderRelationship, OrderRequestBuilder, OrderRequestGroupBuilder,
//! #     OrderTimeInForce, OrderType, TradeAction,
//! # };
//! # async fn confirm_group_order_example(client: &Client) -> Result<(), Error> {
//! let time_in_force = OrderTimeInForce {
//!     duration: Duration::GTC,
//!     expiration: None,
//! };
//!
//! let limit_order = OrderRequestBuilder::new()
//!     .account_id("YOUR_ACCOUNT_ID")
//!     .symbol("MSFT")
//!     .trade_action(TradeAction::Sell)
//!     .quantity("10")
//!     .order_type(OrderType::Limit)
//!     .limit_price("450.00")
//!     .time_in_force(time_in_force.clone())
//!     .build()?;
//!
//! let stop_order = OrderRequestBuilder::new()
//!     .account_id("YOUR_ACCOUNT_ID")
//!     .symbol("MSFT")
//!     .trade_action(TradeAction::Sell)
//!     .quantity("10")
//!     .order_type(OrderType::StopMarket)
//!     .stop_price("390.00")
//!     .time_in_force(time_in_force)
//!     .build()?;
//!
//! let group = OrderRequestGroupBuilder::new()
//!     .order_requests(vec![limit_order, stop_order])
//!     .group_type(OrderRelationship::OCO)
//!     .build()?;
//!
//! let confirmations = Order::confirm_group(client, &group).await?;
//! println!("Loaded {} group confirmations", confirmations.len());
//! # Ok(()) }
//! ```
//!
//! ---
//!
//! # Endpoint: Place Order
//! - Submits a single order for execution.
//! - Order Request Reference: [`crate::execution::OrderRequest`]
//! - Ticket Reference: [`crate::execution::OrderTicket`]
//! - **Example**: Place a limit buy order.
//!
//! ```rust,no_run
//! # use tradestation::{Client, Error, orders::{Duration, Order, OrderRequestBuilder, OrderTimeInForce, OrderType, TradeAction}};
//! # async fn place_order_example(client: &Client) -> Result<(), Error> {
//! let order = OrderRequestBuilder::new()
//!     .account_id("YOUR_ACCOUNT_ID")
//!     .symbol("MSFT")
//!     .trade_action(TradeAction::Buy)
//!     .quantity("10")
//!     .order_type(OrderType::Limit)
//!     .limit_price("420.00")
//!     .time_in_force(OrderTimeInForce {
//!         duration: Duration::DAY,
//!         expiration: None,
//!     })
//!     .build()?;
//!
//! let tickets = Order::place(client, &order).await?;
//! println!("Placed {} order tickets", tickets.len());
//! # Ok(()) }
//! ```
//!
//! ---
//!
//! # Endpoint: Place Group Order
//! - Submits a grouped order request for execution.
//! - Order Group Reference: [`crate::execution::OrderRequestGroup`]
//! - Ticket Reference: [`crate::execution::OrderTicket`]
//! - **Example**: Place an OCO group containing a limit order and a stop order.
//!
//! ```rust,no_run
//! # use tradestation::{
//! #     Client, Error, orders::{
//! #         Duration, Order, OrderRelationship, OrderRequestBuilder,
//! #         OrderRequestGroupBuilder, OrderTimeInForce, OrderType, TradeAction
//! #     }
//! # };
//! # async fn place_group_order_example(client: &Client) -> Result<(), Error> {
//! let time_in_force = OrderTimeInForce {
//!     duration: Duration::GTC,
//!     expiration: None,
//! };
//!
//! let limit_order = OrderRequestBuilder::new()
//!     .account_id("YOUR_ACCOUNT_ID")
//!     .symbol("MSFT")
//!     .trade_action(TradeAction::Sell)
//!     .quantity("10")
//!     .order_type(OrderType::Limit)
//!     .limit_price("450.00")
//!     .time_in_force(time_in_force.clone())
//!     .build()?;
//!
//! let stop_order = OrderRequestBuilder::new()
//!     .account_id("YOUR_ACCOUNT_ID")
//!     .symbol("MSFT")
//!     .trade_action(TradeAction::Sell)
//!     .quantity("10")
//!     .order_type(OrderType::StopMarket)
//!     .stop_price("390.00")
//!     .time_in_force(time_in_force)
//!     .build()?;
//!
//! let group = OrderRequestGroupBuilder::new()
//!     .order_requests(vec![limit_order, stop_order])
//!     .group_type(OrderRelationship::OCO)
//!     .build()?;
//!
//! let tickets = Order::place_group(client, &group).await?;
//! println!("Placed {} grouped order tickets", tickets.len());
//! # Ok(()) }
//! ```
//!
//! ---
//!
//! # Endpoint: Replace Order
//! - Replaces an active order with updated order values.
//! - Ticket Reference: [`crate::execution::OrderTicket`]
//! - Update Reference: [`crate::execution::update::OrderUpdate`]
//! - **Example**: Replace an order by ID with a new limit price and quantity.
//!
//! ```rust,no_run
//! # use tradestation::{Client, Error, orders::{OrderTicket, OrderUpdate}};
//! # async fn replace_order_example(client: &Client) -> Result<(), Error> {
//! let ticket = OrderTicket::from_id("ORDER_ID");
//! let replacement = ticket
//!     .replace(client, OrderUpdate::new().limit_price("421.50").quantity("5"))
//!     .await?;
//!
//! println!("Replacement result: {}", replacement.message);
//! # Ok(()) }
//! ```
//!
//! ---
//!
//! # Endpoint: Cancel Order
//! - Cancels an active order.
//! - Ticket Reference: [`crate::execution::OrderTicket`]
//! - **Example**: Cancel an order by ID.
//!
//! ```rust,no_run
//! # use tradestation::{Client, Error, orders::OrderTicket};
//! # async fn cancel_order_example(client: &Client) -> Result<(), Error> {
//! let ticket = OrderTicket::from_id("ORDER_ID");
//! let cancellation = ticket.cancel(client).await?;
//! println!("Cancellation result: {}", cancellation.message);
//! # Ok(()) }
//! ```
//!
//! ---
//!
//! # Endpoint: Get Routes
//! - Fetches valid routes for order execution.
//! - Route Reference: [`crate::execution::Route`]
//! - **Example**: Load available execution routes.
//!
//! ```rust,no_run
//! # use tradestation::{Client, Error};
//! # async fn get_routes_example(client: &Client) -> Result<(), Error> {
//! let routes = client.get_execution_routes().await?;
//! println!("Loaded {} routes", routes.len());
//! # Ok(()) }
//! ```
//!
//! ---
//!
//! # Endpoint: Get Activation Triggers
//! - Fetches valid stop activation trigger keys for order execution.
//! - Activation Trigger Reference: [`crate::execution::ActivationTrigger`]
//! - **Example**: Load activation trigger definitions.
//!
//! ```rust,no_run
//! # use tradestation::{Client, Error};
//! # async fn get_activation_triggers_example(client: &Client) -> Result<(), Error> {
//! let triggers = client.get_activation_triggers().await?;
//! println!("Loaded {} activation triggers", triggers.len());
//! # Ok(()) }
//! ```
//!

/// Functionality and primitives around [`crate::accounting::orders::Order`] confirmations (pre execution).
pub mod confirm;
/// Functionality and primitives around [`crate::accounting::orders::Order`] specifically at the execution level.
pub mod orders;
/// Functionality and abstractions around [`crate::accounting::orders::Order`] requests.
pub mod request;
/// Functionality and primitives around execution routes.
pub mod route;
/// Functionality and primitives around [`crate::accounting::orders::Order`] tickets (post execution).
pub mod ticket;
/// Functionality and primitives around [`crate::accounting::orders::Order`] execution triggers.
pub mod trigger;
/// Functionality and primitives around [`crate::accounting::orders::Order`] updating/replacing.
pub mod update;

// Expose these directly from the [`crate::execution`] level
pub use orders::{
    AdvancedOrderOptions, BPWarningStatus, Duration, OrderRequestLeg, OrderTimeInForce, Oso,
    PegValue, TradeAction,
};
pub use request::{OrderRequest, OrderRequestBuilder, OrderRequestGroup, OrderRequestGroupBuilder};
pub use route::Route;
pub use ticket::OrderTicket;
pub use trigger::{ActivationTrigger, ActivationTriggerKey};
