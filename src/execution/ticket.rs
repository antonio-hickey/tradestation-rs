use crate::{
    orders::{Order, OrderUpdate},
    responses::{
        execution::{ModifyOrderResp, ModifyOrderRespRaw},
        ApiResponse,
    },
    Client, Error,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// A transactional receipt from placing, replacing, or canceling an [`crate::orders::Order`].
pub struct OrderTicket {
    /// Short text summary/description of the [`crate::orders::Order`] execution result.
    pub message: String,

    #[serde(rename = "OrderID")]
    /// The id of the resulting [`crate::orders::Order`] from execution.
    pub order_id: String,

    /// The error for the [`crate::orders::Order`], if there was any errors.
    pub error: Option<String>,
}
impl OrderTicket {
    /// Instantiate an [`OrderTicket`] using a provided [`crate::orders::Order`] id.
    ///
    /// NOTE: The created [`OrderTicket`] is NOT guaranteed to be valid
    /// for use. The provided order id must be valid to do anything with
    /// the [`OrderTicket`] instance.
    ///
    /// # Example
    /// ---
    ///
    /// Create an instance of [`OrderTicket`] for an order id `11111111`
    /// which you can then use to cancel or replace the order.
    ///
    /// ```
    /// use tradestation::execution::OrderTicket;
    ///
    /// let order = OrderTicket::from_id("11111111");
    /// println!("{order:?}");
    /// ```
    pub fn from_id<S: Into<String>>(order_id: S) -> OrderTicket {
        OrderTicket {
            message: "".into(),
            order_id: order_id.into(),
            error: None,
        }
    }

    /// Find the [`Order`] associated with this [`OrderTicket`].
    ///
    /// Uses the stored order id from this [`OrderTicket`] to search for
    /// the order in the provided account via the given [`Client`].
    ///
    /// NOTE: This call will fail with [`Error::OrderNotFound`] if the order id
    /// does not correspond to any existing order in the account.
    ///
    /// # Example
    /// ---
    ///
    /// Attempt to find an order from an existing [`OrderTicket`] in the account
    /// `DU12345` using a [`Client`] instance.
    ///
    /// ```
    /// use tradestation::execution::OrderTicket;
    /// # use tradestation::{ClientBuilder, Error, token::{Scope, Token}};
    /// # use tokio;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Error> {
    /// # let client = ClientBuilder::new()
    /// #     .credentials("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")
    /// #     .with_token(Token {
    /// #         access_token: String::from("YOUR_ACCESS_TOKEN"),
    /// #         refresh_token: String::from("YOUR_REFRESH_TOKEN"),
    /// #         id_token: String::from("YOUR_ID_TOKEN"),
    /// #         token_type: String::from("Bearer"),
    /// #         scope: vec![
    /// #             Scope::Trade,
    /// #             /* ... Your Other Desired Scopes */
    /// #         ],
    /// #         expires_in: 1200,
    /// #     })
    /// #     .build()
    /// #     .await?;
    ///
    /// let ticket = OrderTicket::from_id("11111111");
    /// match ticket.find_order("ACCOUNT_ID", &client).await {
    ///     Ok(order) => println!("Found order: {order:?}"),
    ///     Err(e) => eprintln!("Error: {e}"),
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn find_order<S: Into<String>>(
        &self,
        account_id: S,
        client: &Client,
    ) -> Result<Order, Error> {
        let orders = Order::find(vec![&self.order_id], account_id.into(), client).await?;

        orders
            .into_iter()
            .next()
            .ok_or(Error::OrderNotFound(self.order_id.clone()))
    }

    /// Replace an [`crate::orders::Order`] with a new [`crate::orders::Order`].
    ///
    /// # Example
    /// ---
    /// Replace an order to buy 100 shares of Palantir `"PLTR"`
    /// at the limit price of $`"40.00"` to instead be 25 shares
    /// at the limit price of $`"42.50"`.
    ///
    /// ```ignore
    /// let order_req = OrderRequestBuilder::new()
    ///     .account_id("YOUR_EQUITIES_ACCOUNT_ID")
    ///     .symbol("PLTR")
    ///     .trade_action(TradeAction::Buy)
    ///     .quantity("100")
    ///     .order_type(OrderType::Limit)
    ///     .limit_price("40.00")
    ///     .time_in_force(OrderTimeInForce {
    ///         duration: Duration::GTC,
    ///         expiration: None,
    ///     })
    ///     .build()?;
    ///
    /// let order = Order::place(&order_req, &client)
    ///     .await?
    ///     .into_iter()
    ///     .next();
    ///
    /// if let Some(order) = order {
    ///     order
    ///         .clone()
    ///         .replace(
    ///             OrderUpdate::new().limit_price("42.50").quantity("25"),
    ///             &client,
    ///         )
    ///         .await?;
    /// }
    /// ```
    pub async fn replace(
        self,
        order_update: OrderUpdate,
        client: &Client,
    ) -> Result<OrderTicket, Error> {
        let endpoint = format!("orderexecution/orders/{}", self.order_id);

        match client
            .put(&endpoint, &order_update)
            .await?
            .json::<ApiResponse<ModifyOrderRespRaw>>()
            .await?
        {
            ApiResponse::Success(resp_raw) => {
                let resp: ModifyOrderResp = resp_raw.into();
                let order: OrderTicket = resp.into();

                Ok(order)
            }
            ApiResponse::Error(resp) => Err(Error::from_api_error(resp)),
        }
    }

    /// Cancel an active [`crate::orders::Order`].
    ///
    /// # Example
    /// ---
    ///
    /// ```ignore
    /// let order_req = OrderRequestBuilder::new()
    ///     .account_id("YOUR_EQUITIES_ACCOUNT_ID")
    ///     .symbol("JPM")
    ///     .trade_action(TradeAction::Buy)
    ///     .quantity("100")
    ///     .order_type(OrderType::Limit)
    ///     .limit_price("220.50")
    ///     .time_in_force(OrderTimeInForce {
    ///         duration: Duration::GTC,
    ///         expiration: None,
    ///     })
    ///     .build()?;
    ///
    /// let order = Order::place(&order_req, &client)
    ///     .await?
    ///     .into_iter()
    ///     .next();
    ///
    /// if let Some(order) = order {
    ///     order.cancel(&client).await?;
    /// }
    /// ```
    pub async fn cancel(self, client: &Client) -> Result<OrderTicket, Error> {
        let endpoint = format!("orderexecution/orders/{}", self.order_id);

        match client
            .delete(&endpoint)
            .await?
            .json::<ApiResponse<ModifyOrderRespRaw>>()
            .await?
        {
            ApiResponse::Success(resp_raw) => {
                let resp: ModifyOrderResp = resp_raw.into();
                let order: OrderTicket = resp.into();

                Ok(order)
            }
            ApiResponse::Error(resp) => Err(Error::from_api_error(resp)),
        }
    }
}
