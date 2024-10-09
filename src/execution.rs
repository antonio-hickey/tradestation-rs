use crate::{
    account::AssetType,
    responses::execution::{GetExecutionRoutesResp, GetExecutionRoutesRespRaw},
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
