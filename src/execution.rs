use crate::{
    account::AssetType,
    responses::execution::{
        GetActivationTriggersResp, GetActivationTriggersRespRaw, GetExecutionRoutesResp,
        GetExecutionRoutesRespRaw,
    },
    Client, Error,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// A Route for Order Execution
pub struct Route {
    /// The ID that must be sent in the optional Route
    /// property of a POST order request, when specifying
    /// a route for an order.
    pub id: String,
    /// The name of the route.
    pub name: String,
    /// The asset type of the route
    pub asset_types: Vec<AssetType>,
}
impl Route {
    /// Fetch valid routes for sending an order for execution.
    ///
    /// # Example
    /// ---
    /// Example: Fetch a list of routes to send orders for execution.
    /// ```ignore
    /// use tradestation::{ClientBuilder, Error, Token};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     // Create client
    ///     let mut client = ClientBuilder::new()?
    ///         .set_credentials("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")?
    ///         .set_token(Token { /* YOUR BEARER AUTH TOKEN */ })?
    ///         .build()
    ///         .await?;
    ///
    ///     // Example: Fetch a list of routes to send orders for execution.
    ///     let routes = client.get_execution_routes().await?;
    ///     println!("Valid routes for order execution: {routes:?}");
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn fetch(client: &mut Client) -> Result<Vec<Route>, Error> {
        let endpoint = String::from("orderexecution/routes");
        let resp: GetExecutionRoutesResp = client
            .get(&endpoint)
            .await?
            .json::<GetExecutionRoutesRespRaw>()
            .await?
            .into();

        if let Some(routes) = resp.routes {
            Ok(routes)
        } else {
            Err(resp.error.unwrap_or(Error::UnknownTradeStationAPIError))
        }
    }
}
impl Client {
    /// Fetch valid routes for sending an order for execution.
    ///
    /// # Example
    /// ---
    /// Example: Fetch a list of routes to send orders for execution.
    /// ```ignore
    /// use tradestation::{ClientBuilder, Error, Token};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     // Create client
    ///     let mut client = ClientBuilder::new()?
    ///         .set_credentials("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")?
    ///         .set_token(Token { /* YOUR BEARER AUTH TOKEN */ })?
    ///         .build()
    ///         .await?;
    ///
    ///     // Example: Fetch a list of routes to send orders for execution.
    ///     let routes = client.get_execution_routes().await?;
    ///     println!("Valid routes for order execution: {routes:?}");
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_execution_routes(&mut self) -> Result<Vec<Route>, Error> {
        Route::fetch(self).await
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// Valid Activation Triggers for an Order.
pub struct ActivationTrigger {
    /// The Activation Trigger Key
    ///
    /// NOTE: This is what you with your orders.
    pub key: ActivationTriggerKey,

    /// Name of the Activation Trigger.
    pub name: String,

    /// Description of the Activation Trigger.
    pub description: String,
}
impl ActivationTrigger {
    /// Fetch Activation Triggers for Order Execution.
    ///
    /// NOTE: This provides the `key` that must be sent with an
    /// order to utilize and be triggered by the activation function.
    ///
    /// # Example
    /// ---
    /// Fetch valid activation triggers to utilize with your orders.
    ///
    /// ```ignore
    /// use tradestation::{ClientBuilder, Error, Token};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     // Initialize client
    ///     let mut client = ClientBuilder::new()?
    ///         .set_credentials("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")?
    ///         .set_token(Token {
    ///             access_token: String::from("YOUR_ACCESS_TOKEN"),
    ///             refresh_token: String::from("YOUR_REFRESH_TOKEN"),
    ///             id_token: String::from("YOUR_ID_TOKEN"),
    ///             token_type: String::from("Bearer"),
    ///             scope: String::from("YOUR_SCOPES SPACE_SEPERATED FOR_EACH_SCOPE"),
    ///             expires_in: 1200,
    ///         })?
    ///         .build()
    ///         .await?;
    ///
    ///     // Fetch a list of valid activation triggers for order execution.
    ///     let triggers = client.get_activation_triggers().await?;
    ///     println!("Valid activation triggers for order execution: {triggers:?}");
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn fetch(client: &mut Client) -> Result<Vec<ActivationTrigger>, Error> {
        let endpoint = String::from("orderexecution/activationtriggers");
        let resp: GetActivationTriggersResp = client
            .get(&endpoint)
            .await?
            .json::<GetActivationTriggersRespRaw>()
            .await?
            .into();

        if let Some(triggers) = resp.activation_triggers {
            Ok(triggers)
        } else {
            Err(resp.error.unwrap_or(Error::UnknownTradeStationAPIError))
        }
    }
}
impl Client {
    /// Fetch Activation Triggers for Order Execution.
    ///
    /// NOTE: This provides the `key` that must be sent with an
    /// order to utilize and be triggered by the activation function.
    ///
    /// # Example
    /// ---
    /// Fetch valid activation triggers to utilize with your orders.
    ///
    /// ```ignore
    /// use tradestation::{ClientBuilder, Error, Token};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     // Initialize client
    ///     let mut client = ClientBuilder::new()?
    ///         .set_credentials("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")?
    ///         .set_token(Token {
    ///             access_token: String::from("YOUR_ACCESS_TOKEN"),
    ///             refresh_token: String::from("YOUR_REFRESH_TOKEN"),
    ///             id_token: String::from("YOUR_ID_TOKEN"),
    ///             token_type: String::from("Bearer"),
    ///             scope: String::from("YOUR_SCOPES SPACE_SEPERATED FOR_EACH_SCOPE"),
    ///             expires_in: 1200,
    ///         })?
    ///         .build()
    ///         .await?;
    ///
    ///     // Fetch a list of valid activation triggers for order execution.
    ///     let triggers = client.get_activation_triggers().await?;
    ///     println!("Valid activation triggers for order execution: {triggers:?}");
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_activation_triggers(&mut self) -> Result<Vec<ActivationTrigger>, Error> {
        ActivationTrigger::fetch(self).await
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
/// The different types of activation trigger keys.
pub enum ActivationTriggerKey {
    /// Single Trade Tick, one trade tick must print
    /// within your stop price to trigger your stop.
    STT,

    /// Single Trade Tick Within NBBO, one trade tick
    /// within the National Best Bid or Offer (NBBO)
    /// must print within your stop price to trigger
    /// your stop.
    STTN,

    /// Single Bid/Ask Tick
    /// ---
    /// * Buy/Cover Orders: One Ask tick must print within
    ///   your stop price to trigger your stop.
    ///
    /// * Sell/Short Orders: One Bid tick must print within
    ///   your stop price to trigger your stop.
    SBA,

    /// Single Ask/Bid Tick
    /// ---
    /// * Buy/Cover Orders: One Bid tick must print within
    ///   your stop price to trigger your stop.
    ///
    /// * Sell/Short Orders: One Ask tick must print within
    ///   your stop price to trigger your stop.
    SAB,

    /// Double Trade Tick, two consecutive trade ticks must
    /// print within your stop price to trigger your stop.
    DTT,

    /// Double Trade Tick Within NBBO, two consecutive trade
    /// ticks within the National Best Bid or Offer (NBBO) must
    /// print within your stop price to trigger your stop.
    DTTN,

    /// Double Bid/Ask Tick
    /// ---
    /// * Buy/Cover Orders: Two consecutive Ask ticks must print
    ///   within your stop price to trigger your stop.
    ///
    /// * Sell/Short Orders: Two consecutive Bid ticks must print
    ///   within your stop price to trigger your stop.
    DBA,

    /// Double Ask/Bid Tick
    /// ---
    /// * Buy/Cover Orders: Two consecutive Bid ticks must print
    ///   within your stop price to trigger your stop.
    ///
    /// * Sell/Short Orders: Two consecutive Ask ticks must print
    ///   within your stop price to trigger your stop.
    DAB,

    /// Twice Trade Tick, two trade ticks must print within your
    /// stop price to trigger your stop.
    TTT,

    /// Twice Trade Tick Within NBBO, two trade ticks within the
    /// National Best Bid or Offer (NBBO) must print within your
    /// stop price to trigger your stop.
    TTTN,

    /// Twice Bid/Ask Tick
    /// ---
    /// * Buy/Cover Orders: Two Ask ticks must print within your
    ///   stop price to trigger your stop.
    ///
    /// * Sell/Short Orders: Two Bid ticks must print within your
    ///   stop price to trigger your stop.
    TBA,

    /// Twice Ask/Bid Tick
    /// ---
    /// * Buy/Cover Orders: Two Bid ticks must print within your
    ///   stop price to trigger your stop.
    ///
    /// * Sell/Short Orders: Two Ask ticks must print within your
    ///   stop price to trigger your stop.
    TAB,
}
