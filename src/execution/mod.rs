//! # Execution
//! ---
//! This module contains functionality and primitives around execution
//! within the TradeStation API. The API is mostly exposed via the
//! [`crate::accounting::orders::Order`] struct, but also accessible
//! via their native structs like [`crate::execution::ticket::OrderTicket`],
//! [`crate::execution::confirmation::OrderConfirmation`], or
//! [`crate::execution::request::OrderRequest`] for example.
//!
//! See individual modules for more information on specific components.
/// Functionality and primitives around [`crate::accounting::orders::Order`] confirmations (pre execution).
pub mod confirm;
/// Functionality and primitives around [`crate::accounting::orders::Order`] specifically at the execution level.
pub mod order;
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
pub use crate::accounting::orders::{
    AssetType, ConditionalOrder, OrderAction, OrderLeg, OrderRelationship, OrderStage, OrderStatus,
    OrderType,
};
pub use confirm::OrderConfirmation;
pub use order::{
    AdvancedOrderOptions, BPWarningStatus, Duration, OrderRequestLeg, OrderTimeInForce, Oso,
    PegValue, TradeAction,
};
pub use request::{OrderRequest, OrderRequestBuilder, OrderRequestGroup, OrderRequestGroupBuilder};
pub use route::Route;
pub use ticket::OrderTicket;
pub use trigger::{ActivationTrigger, ActivationTriggerKey};
pub use update::OrderUpdate;
