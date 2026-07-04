use crate::{
    accounting::orders::AssetType,
    responses::{
        execution::{GetExecutionRoutesResp, GetExecutionRoutesRespRaw},
        ApiResponse,
    },
    Client, Error,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// A Route for [`crate::orders::Order`] Execution.
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
    /// Fetch valid routes for sending an [`crate::orders::Order`] for execution.
    ///
    /// # Example
    /// ---
    /// Example: Fetch a list of routes to send orders for execution.
    /// ```rust,no_run
    /// # use tradestation::{Client, Error};
    /// # async fn get_routes_example(client: &Client) -> Result<(), Error> {
    /// let routes = client.get_execution_routes().await?;
    /// println!("Valid routes for order execution: {routes:?}");
    /// # Ok(()) }
    /// ```
    pub async fn fetch(client: &Client) -> Result<Vec<Route>, Error> {
        let endpoint = String::from("orderexecution/routes");

        match client
            .get(&endpoint)
            .await?
            .json::<ApiResponse<GetExecutionRoutesRespRaw>>()
            .await?
        {
            ApiResponse::Success(resp_raw) => {
                let resp: GetExecutionRoutesResp = resp_raw.into();

                if let Some(routes) = resp.routes {
                    Ok(routes)
                } else {
                    Err(resp.error.unwrap_or(Error::UnknownTradeStationAPIError(
                        "Unknown TradeStation Error While Fetching Execution Routes.".into(),
                    )))
                }
            }
            ApiResponse::Error(resp) => Err(Error::from_api_error(resp)),
        }
    }
}
impl Client {
    /// Fetch valid routes for sending an [`crate::orders::Order`] for execution.
    ///
    /// # Example
    /// ---
    /// Example: Fetch a list of routes to send orders for execution.
    /// ```rust,no_run
    /// use tradestation::{ClientBuilder, ClientEnvironment, Error, Scope, Token};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     // Create client
    ///     let client = ClientBuilder::new()
    ///         .credentials("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")
    ///         .environment(ClientEnvironment::Simulation)
    ///         .with_token(Token {
    ///             access_token: String::from("YOUR_ACCESS_TOKEN"),
    ///             refresh_token: String::from("YOUR_REFRESH_TOKEN"),
    ///             id_token: String::from("YOUR_ID_TOKEN"),
    ///             token_type: String::from("Bearer"),
    ///             scope: vec![
    ///                 Scope::Trade,
    ///                 /* ... Your Other Desired Scopes */
    ///             ],
    ///             expires_in: 1200,
    ///         })
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
    pub async fn get_execution_routes(&self) -> Result<Vec<Route>, Error> {
        Route::fetch(self).await
    }
}
