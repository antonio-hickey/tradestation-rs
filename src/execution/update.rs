use crate::{accounting::orders::OrderType, execution::orders::AdvancedOrderOptions};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "PascalCase")]
/// An update for an existing [`crate::orders::Order`] open in the marketplace.
pub struct OrderUpdate {
    /// The limit price for this updated [`crate::orders::Order`].
    pub limit_price: Option<String>,

    /// The stop price for this updated [`crate::orders::Order`].
    pub stop_price: Option<String>,

    /// The order type for this updated [`crate::orders::Order`].
    pub order_type: Option<OrderType>,

    /// The quantity for this updated [`crate::orders::Order`].
    pub quantity: Option<String>,

    /// The advanced options of this updated [`crate::orders::Order`].
    pub advanced_options: Option<AdvancedOrderOptions>,
}
impl OrderUpdate {
    /// Create a new default [`OrderUpdate`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the limit price of the updated [`crate::orders::Order`].
    pub fn limit_price(mut self, price: impl Into<String>) -> Self {
        self.limit_price = Some(price.into());

        self
    }

    /// Set the stop price of the updated [`crate::orders::Order`].
    pub fn stop_price(mut self, price: impl Into<String>) -> Self {
        self.stop_price = Some(price.into());

        self
    }

    /// Set the order type of the updated [`crate::orders::Order`].
    pub fn order_type(mut self, order_type: OrderType) -> Self {
        self.order_type = Some(order_type);

        self
    }

    /// Set the quantity for the updated [`crate::orders::Order`].
    pub fn quantity(mut self, qty: impl Into<String>) -> Self {
        self.quantity = Some(qty.into());

        self
    }

    /// Set the advanced options of the updated [`crate::orders::Order`].
    pub fn advanced_options(mut self, opts: AdvancedOrderOptions) -> Self {
        self.advanced_options = Some(opts);

        self
    }
}
