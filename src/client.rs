use crate::token::RefreshedToken;
use crate::{Error, Token};
use reqwest::{header, Response};
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone, Debug)]
/// TradeStation API Client
pub struct Client {
    http_client: reqwest::Client,
    client_id: String,
    client_secret: String,
    /// Bearer Token for TradeStation's API
    pub token: Token,
}
impl Client {
    /// Send an HTTP request to TradeStation's API, with automatic
    /// token refreshing near, at, or after auth token expiration.
    ///
    /// NOTE: You should use `Client::post()` or `Client::get()` in favor of this method.
    pub async fn send_request<F, T>(&mut self, request_fn: F) -> Result<Response, Error>
    where
        F: Fn(&Token) -> T,
        T: std::future::Future<Output = Result<Response, reqwest::Error>>,
    {
        match request_fn(&self.token).await {
            Ok(resp) => {
                // Check if the client gets a 401 unauthorized to try and re auth the client
                // this happens when auth token expires.
                if resp.status() == reqwest::StatusCode::UNAUTHORIZED {
                    // Refresh the clients token
                    self.refresh_token().await?;

                    // Retry sending the request to TradeStation's API
                    let retry_response = request_fn(&self.token).await?;
                    Ok(retry_response)
                } else {
                    Ok(resp)
                }
            }
            Err(e) => Err(Error::Request(e)),
        }
    }

    /// Send a POST request from your `Client` to TradeStation's API
    pub async fn post<T: Serialize>(
        &mut self,
        endpoint: &str,
        payload: &T,
    ) -> Result<Response, Error> {
        let url = format!("https://api.tradestation.com/v3/{endpoint}");
        let resp = self
            .clone()
            .send_request(|token| {
                self.http_client
                    .post(&url)
                    .header("Content-Type", "application/json")
                    .header(
                        header::AUTHORIZATION,
                        format!("Bearer {}", token.access_token),
                    )
                    .json(payload)
                    .send()
            })
            .await?;

        Ok(resp)
    }

    /// Send a GET request from your `Client` to TradeStation's API
    pub async fn get(&mut self, endpoint: &str) -> Result<Response, Error> {
        let url = format!("https://api.tradestation.com/v3/{endpoint}");
        let resp = self
            .clone()
            .send_request(|token| {
                self.http_client
                    .get(&url)
                    .header(
                        header::AUTHORIZATION,
                        format!("Bearer {}", token.access_token),
                    )
                    .send()
            })
            .await?;

        Ok(resp)
    }

    /// Send a PUT request from your `Client` to TradeStation's API
    pub async fn put<T: Serialize>(
        &mut self,
        endpoint: &str,
        payload: &T,
    ) -> Result<Response, Error> {
        let url = format!("https://api.tradestation.com/v3/{endpoint}");
        let resp = self
            .clone()
            .send_request(|token| {
                self.http_client
                    .put(&url)
                    .header("Content-Type", "application/json")
                    .header(
                        header::AUTHORIZATION,
                        format!("Bearer {}", token.access_token),
                    )
                    .json(&payload)
                    .send()
            })
            .await?;

        Ok(resp)
    }

    /// Start a stream from the TradeStation API to the `Client`
    ///
    /// NOTE: You need to provide a processing function for handeling the stream chunks
    pub async fn stream<F>(&mut self, endpoint: &str, mut process_chunk: F) -> Result<(), Error>
    where
        F: FnMut(Value) -> Result<(), Error>,
    {
        let url = format!("https://api.tradestation.com/v3/{endpoint}");
        let mut resp = self
            .clone()
            .send_request(|token| {
                self.http_client
                    .get(&url)
                    .header(
                        reqwest::header::AUTHORIZATION,
                        format!("Bearer {}", token.access_token),
                    )
                    .send()
            })
            .await?;

        if !resp.status().is_success() {
            return Err(Error::StreamIssue(format!(
                "Request failed with status: {}",
                resp.status()
            )));
        }

        let mut buffer = String::new();
        while let Some(chunk) = resp.chunk().await? {
            let chunk_str = std::str::from_utf8(&chunk).unwrap_or("");
            buffer.push_str(chunk_str);

            while let Some(pos) = buffer.find("\n") {
                let json_str = buffer[..pos].trim().to_string();
                buffer = buffer[pos + 1..].to_string();
                if json_str.is_empty() {
                    continue;
                }

                match serde_json::from_str::<Value>(&json_str) {
                    Ok(json_value) => {
                        if let Err(e) = process_chunk(json_value) {
                            if matches!(e, Error::StopStream) {
                                return Ok(());
                            } else {
                                return Err(e);
                            }
                        }
                    }
                    Err(e) => {
                        return Err(Error::Json(e));
                    }
                }
            }
        }

        // Handle any leftover data in the buffer
        if !buffer.trim().is_empty() {
            match serde_json::from_str::<Value>(&buffer) {
                Ok(json_value) => {
                    if let Err(e) = process_chunk(json_value) {
                        if matches!(e, Error::StopStream) {
                            return Ok(());
                        } else {
                            return Err(e);
                        }
                    }
                }
                Err(e) => {
                    return Err(Error::Json(e));
                }
            }
        }

        Ok(())
    }

    /// Refresh your clients bearer token used for authentication
    /// with TradeStation's API.
    pub async fn refresh_token(&mut self) -> Result<(), Error> {
        let form_data: HashMap<String, String> = HashMap::from([
            ("grant_type".into(), "refresh_token".into()),
            ("client_id".into(), self.client_id.clone()),
            ("client_secret".into(), self.client_secret.clone()),
            ("refresh_token".into(), self.token.refresh_token.clone()),
            ("redirect_uri".into(), "http://localhost:8080/".into()),
        ]);

        let new_token = self
            .http_client
            .post("https://signin.tradestation.com/oauth/token")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&form_data)
            .send()
            .await?
            .json::<RefreshedToken>()
            .await?;

        // Update the clients token
        self.token = Token {
            refresh_token: self.token.refresh_token.clone(),
            access_token: new_token.access_token,
            id_token: new_token.id_token,
            scope: new_token.scope,
            token_type: new_token.token_type,
            expires_in: new_token.expires_in,
        };

        Ok(())
    }
}

#[derive(Debug, Default)]
/// Builder for `Client`
pub struct ClientBuilder;

#[derive(Debug, Default)]
/// First step to building a `Client`.
pub struct Step1;
#[derive(Debug, Default)]
/// Second step to building a `Client`.
pub struct Step2;
#[derive(Debug, Default)]
/// Third step to building a `Client`.
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
    #[allow(clippy::new_ret_no_self)]
    /// Instantiate a new instance of `ClientBuilder`
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

    /// Set the current `Token` for the `Client` to use
    pub fn set_token(self, token: Token) -> Result<ClientBuilderStep<Step3>, Error> {
        Ok(ClientBuilderStep {
            _current_step: Step3,
            http_client: self.http_client,
            client_id: self.client_id,
            client_secret: self.client_secret,
            token: Some(token),
        })
    }
}
impl ClientBuilderStep<Step3> {
    /// Finish building into a `Client`.
    pub async fn build(self) -> Result<Client, Error> {
        let http_client = self.http_client.unwrap();
        let client_id = self.client_id.unwrap();
        let client_secret = self.client_secret.unwrap();
        let token = self.token.unwrap();

        Ok(Client {
            http_client,
            client_id,
            client_secret,
            token,
        })
    }
}
