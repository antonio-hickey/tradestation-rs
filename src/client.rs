use crate::{Error, Token};
use std::collections::HashMap;

/// TradeStation API Client
pub struct Client {
    _http_client: reqwest::Client,
    _client_id: String,
    _client_secret: String,
    /// Bearer Token for the TradeStation API
    pub token: Token,
}

#[derive(Debug, Default)]
/// Builder for `Client`
pub struct ClientBuilder;

#[derive(Debug, Default)]
pub struct Step1;
#[derive(Debug, Default)]
pub struct Step2;
#[derive(Debug, Default)]
pub struct Step3;

#[derive(Debug, Default)]
/// Phantom Type for compile time enforcement
/// on the order of builder steps used.
pub struct ClientBuilderStep<CurrentStep> {
    _current_step: CurrentStep,
    http_client: Option<reqwest::Client>,
    client_id: Option<String>,
    client_secret: Option<String>,
    token: Option<Token>,
}

impl ClientBuilder {
    /// Instantiate a new instance of `ClientBuilder`
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Result<ClientBuilderStep<Step1>, Error> {
        Ok(ClientBuilderStep {
            _current_step: Step1,
            http_client: Some(reqwest::Client::new()),
            ..Default::default()
        })
    }
}
impl ClientBuilderStep<Step1> {
    /// Set your client id/key and secret
    pub fn set_credentials(
        self,
        client_id: &str,
        client_secret: &str,
    ) -> Result<ClientBuilderStep<Step2>, Error> {
        Ok(ClientBuilderStep {
            _current_step: Step2,
            http_client: Some(self.http_client.unwrap()),
            client_id: Some(client_id.into()),
            client_secret: Some(client_secret.into()),
            ..Default::default()
        })
    }
}
impl ClientBuilderStep<Step2> {
    /// Use your authorization code to get and set auth token
    pub async fn authorize(
        self,
        authorization_code: &str,
    ) -> Result<ClientBuilderStep<Step3>, Error> {
        // NOTE: These unwraps are panic safe due to type invariant
        // with compile time enforced order of steps for `ClientBuilderStep`
        let http_client = self.http_client.unwrap();
        let client_id = self.client_id.as_ref().unwrap();
        let client_secret = self.client_secret.as_ref().unwrap();

        // Send HTTP request to TradeStation API to get auth token
        let form_data = HashMap::from([
            ("grant_type", "authorization_code"),
            ("client_id", client_id),
            ("client_secret", client_secret),
            ("code", authorization_code),
            ("redirect_uri", "http://localhost:8080/"),
        ]);
        let token = http_client
            .post("https://signin.tradestation.com/oauth/token")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&form_data)
            .send()
            .await?
            .json::<Token>()
            .await?;

        Ok(ClientBuilderStep {
            _current_step: Step3,
            http_client: Some(http_client),
            client_id: self.client_id,
            client_secret: self.client_secret,
            token: Some(token),
        })
    }
}
impl ClientBuilderStep<Step3> {
    pub async fn build(self) -> Result<Client, Error> {
        let _http_client = self.http_client.unwrap();
        let _client_id = self.client_id.unwrap();
        let _client_secret = self.client_secret.unwrap();
        let token = self.token.unwrap();

        Ok(Client {
            _http_client,
            _client_id,
            _client_secret,
            token,
        })
    }
}
