use crate::{
    token::{RefreshedToken, Token},
    Error, Scope,
};
use futures::{Stream, TryStreamExt};
use reqwest::{header, Response};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use std::{collections::HashMap, sync::Arc};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::Mutex;
use tokio_util::{
    codec::{FramedRead, LinesCodec},
    io::StreamReader,
};

#[derive(Clone, Debug)]
/// TradeStation API Client
pub struct Client {
    /// The HTTP Client for sending requests
    http_client: reqwest::Client,

    /// The TradeStation API Client ID
    client_id: String,

    /// The TradeStation API Client Secret Key
    client_secret: String,

    /// Bearer Token for TradeStation's API
    pub token: Arc<Mutex<Token>>,

    /// OAuth redirect URI used during auth and refresh flows.
    redirect_uri: String,

    /// The base url used for all endpoints.
    ///
    /// NOTE: You should leave this default unless you
    /// specifically want to use a custom address for
    /// testing or mocking purposes.
    pub base_url: String,
}
impl Client {
    /// Send an HTTP request to TradeStation's API, with automatic
    /// token refreshing near, at, or after auth token expiration.
    ///
    /// NOTE: You should use `Client::post()` or `Client::get()` in favor of this method.
    pub async fn send_request<F, T>(&self, request_fn: F) -> Result<Response, Error>
    where
        F: Fn(String) -> T,
        T: std::future::Future<Output = Result<Response, reqwest::Error>>,
    {
        let token_guard = self.token.lock().await;
        let access_token = token_guard.access_token.clone();
        drop(token_guard);

        match request_fn(access_token).await {
            Ok(resp) => {
                // Check if the client gets a 401 unauthorized to try and re auth the client
                // this happens when auth token expires.
                if resp.status() == reqwest::StatusCode::UNAUTHORIZED {
                    // Refresh the clients token
                    self.refresh_token().await?;
                    let token_guard = self.token.lock().await;
                    let access_token = token_guard.access_token.clone();
                    drop(token_guard);

                    // Retry sending the request to TradeStation's API
                    let retry_response = request_fn(access_token).await?;
                    Ok(retry_response)
                } else {
                    Ok(resp)
                }
            }
            Err(e) => Err(Error::Request(e)),
        }
    }

    /// Send a POST request from your `Client` to TradeStation's API
    pub async fn post<T: Serialize>(&self, endpoint: &str, payload: &T) -> Result<Response, Error> {
        let url = format!("{}/{}", self.base_url, endpoint);
        let resp = self
            .clone()
            .send_request(|access_token| {
                self.http_client
                    .post(&url)
                    .header("Content-Type", "application/json")
                    .header(header::AUTHORIZATION, format!("Bearer {access_token}"))
                    .json(payload)
                    .send()
            })
            .await?;

        Ok(resp)
    }

    /// Send a GET request from your `Client` to TradeStation's API
    pub async fn get(&self, endpoint: &str) -> Result<Response, Error> {
        let url = format!("{}/{}", self.base_url, endpoint);
        let resp = self
            .clone()
            .send_request(|access_token| {
                self.http_client
                    .get(&url)
                    .header(header::AUTHORIZATION, format!("Bearer {access_token}"))
                    .send()
            })
            .await?;

        Ok(resp)
    }

    /// Send a PUT request from your `Client` to TradeStation's API
    pub async fn put<T: Serialize>(&self, endpoint: &str, payload: &T) -> Result<Response, Error> {
        let url = format!("{}/{}", self.base_url, endpoint);
        let resp = self
            .clone()
            .send_request(|access_token| {
                self.http_client
                    .put(&url)
                    .header("Content-Type", "application/json")
                    .header(header::AUTHORIZATION, format!("Bearer {access_token}"))
                    .json(&payload)
                    .send()
            })
            .await?;

        Ok(resp)
    }

    /// Send a DELETE request from your `Client` to TradeStation's API
    pub async fn delete(&self, endpoint: &str) -> Result<Response, Error> {
        let url = format!("{}/{}", self.base_url, endpoint);
        let resp = self
            .clone()
            .send_request(|access_token| {
                self.http_client
                    .delete(&url)
                    .header("Content-Type", "application/json")
                    .header(header::AUTHORIZATION, format!("Bearer {access_token}"))
                    .send()
            })
            .await?;

        Ok(resp)
    }

    /// Start a stream from the TradeStation API to the `Client`
    ///
    /// NOTE: You need to provide a processing function for handeling the stream chunks
    pub fn stream(&self, endpoint: String) -> impl Stream<Item = Result<Value, Error>> + '_ {
        async_stream::try_stream! {
            let url = format!("{}/{}", self.base_url, endpoint);

            let resp = self
            .clone()
            .send_request(|access_token| {
                self.http_client
                    .get(&url)
                    .header(
                        reqwest::header::AUTHORIZATION,
                        format!("Bearer {access_token}"),
                    )
                    .send()
            })
            .await?;

            if !resp.status().is_success() {
                Err(Error::StreamIssue(format!(
                    "Request failed with status: {}",
                    resp.status()
                )))?
            }

            let byte_stream = resp.bytes_stream().map_err(std::io::Error::other);
            let stream_reader = StreamReader::new(byte_stream);
            let buf_reader = BufReader::new(stream_reader);
            let mut lines = buf_reader.lines();

            while let Some(line) = lines.next_line().await? {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }

                let json: Value = serde_json::from_str(trimmed).map_err(Error::Json)?;
                yield json;
            }
        }
    }

    /// Streams a newline delimited JSON response into a provided callback.
    pub(crate) async fn stream_into<T, F>(
        &self,
        endpoint: &str,
        mut process_chunk: F,
    ) -> Result<(), Error>
    where
        T: DeserializeOwned,
        F: FnMut(T) -> Result<(), Error>,
    {
        let url = format!("{}/{}", self.base_url, endpoint);

        let resp = self
            .clone()
            .send_request(|access_token| {
                self.http_client
                    .get(&url)
                    .header(
                        reqwest::header::AUTHORIZATION,
                        format!("Bearer {access_token}"),
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

        let byte_stream = resp.bytes_stream().map_err(std::io::Error::other);
        let reader = StreamReader::new(byte_stream);

        let mut lines = FramedRead::with_capacity(reader, LinesCodec::new(), 64 * 1024);

        while let Some(line) = lines
            .try_next()
            .await
            .map_err(|e| Error::StreamIssue(e.to_string()))?
        {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            match serde_json::from_str::<T>(line) {
                Ok(json) => {
                    if let Err(e) = process_chunk(json) {
                        if matches!(e, Error::StopStream) {
                            return Ok(());
                        } else {
                            return Err(e);
                        }
                    }
                }
                Err(e) => return Err(Error::Json(e)),
            }
        }

        Ok(())
    }

    /// Refresh your clients bearer token used for
    /// authentication with TradeStation's API.
    pub async fn refresh_token(&self) -> Result<(), Error> {
        let mut token_guard = self.token.lock().await;

        let form_data: HashMap<String, String> = HashMap::from([
            ("grant_type".into(), "refresh_token".into()),
            ("client_id".into(), self.client_id.clone()),
            ("client_secret".into(), self.client_secret.clone()),
            ("refresh_token".into(), token_guard.refresh_token.clone()),
            ("redirect_uri".into(), self.redirect_uri.clone()),
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
        *token_guard = Token {
            refresh_token: token_guard.refresh_token.clone(),
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
/// First step to building a `Client`.
pub struct Configure;
#[derive(Debug, Default)]
/// Second step to building a `Client`.
pub struct Authorize;
#[derive(Debug, Default)]
/// Third step to building a `Client`.
pub struct Ready;

#[derive(Debug, Default)]
/// Phantom Type for compile time enforcement
/// on the order of builder steps used.
pub struct ClientBuilderStep<CurrentStep> {
    _current_step: std::marker::PhantomData<CurrentStep>,
    http_client: reqwest::Client,
    client_id: Option<String>,
    client_secret: Option<String>,
    redirect_uri: Option<String>,
    audience: Option<String>,
    scopes: Vec<String>,
    testing_url: Option<String>,
    token: Option<Token>,
}

#[derive(Debug, Default)]
/// Builder for [`Client`].
pub struct ClientBuilder;

impl ClientBuilder {
    #[allow(clippy::new_ret_no_self)]
    /// Instantiate a new instance of [`ClientBuilder`].
    pub fn new() -> ClientBuilderStep<Configure> {
        ClientBuilderStep {
            _current_step: std::marker::PhantomData::<Configure>,
            http_client: reqwest::Client::new(),
            ..Default::default()
        }
    }
}
impl ClientBuilderStep<Configure> {
    #[must_use]
    /// Set the API key and secret.
    pub fn credentials<S: Into<String>>(self, client_id: S, client_secret: S) -> Self {
        ClientBuilderStep {
            _current_step: std::marker::PhantomData::<Configure>,
            http_client: self.http_client,
            client_id: Some(client_id.into()),
            client_secret: Some(client_secret.into()),
            ..Default::default()
        }
    }

    #[must_use]
    /// Set the [`Token`] for the [`Client`] to use.
    ///
    /// This is very useful if you already have a token
    /// stored that you can use to skip the authorize
    /// redirect flow with.
    ///
    /// As long as the redirect token is not expired the
    /// [`Token`] itself is still valid, as we auto handle
    /// refreshing the token when the access token expires.
    pub fn with_token(self, token: Token) -> ClientBuilderStep<Ready> {
        let ClientBuilderStep {
            _current_step: _,
            http_client,
            client_id,
            client_secret,
            redirect_uri,
            audience,
            scopes,
            token: _,
            testing_url,
        } = self;

        ClientBuilderStep {
            _current_step: std::marker::PhantomData::<Ready>,
            http_client,
            client_id,
            client_secret,
            redirect_uri,
            audience,
            scopes,
            token: Some(token),
            testing_url,
        }
    }

    #[must_use]
    /// Set the redirect URI.
    ///
    /// NOTE:
    pub fn redirect_uri(mut self, uri: impl Into<String>) -> Self {
        self.redirect_uri = Some(uri.into());
        self
    }

    #[must_use]
    /// NOTE: Defaults to `https://api.tradestation.com` if omitted.
    pub fn audience(mut self, aud: impl Into<String>) -> Self {
        self.audience = Some(aud.into());
        self
    }

    #[must_use]
    /// Set the desired scopes for the [`Client`] to be authorized for.
    pub fn scopes<I, S>(mut self, scopes: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.scopes = scopes.into_iter().map(Into::into).collect();
        self
    }

    #[must_use]
    /// Set the testing url for the client to use for sending
    /// ALL the requests to your test/mock server instead of
    /// the default TradeStation API url.
    ///
    /// NOTE: This should ONLY be set for testing and
    /// mocking purposes. This should NOT be set used
    /// with a production `Client`.
    pub fn testing_url(self, url: impl Into<String>) -> ClientBuilderStep<Ready> {
        // Generate a test dummy token, when using a test url.
        let test_token = Token {
            access_token: "ACCESS_TOKEN".to_owned(),
            id_token: "ID_TOKEN".to_owned(),
            refresh_token: "REFRESH_TOKEN".to_owned(),
            token_type: "Bearer".to_owned(),
            expires_in: 3600,
            scope: vec![
                Scope::MarketData,
                Scope::Profile,
                Scope::OpenId,
                Scope::OfflineAccess,
            ],
        };

        let ClientBuilderStep {
            _current_step: _,
            http_client,
            client_id,
            client_secret,
            redirect_uri,
            audience,
            scopes,
            token: _,
            testing_url: _,
        } = self;

        ClientBuilderStep {
            _current_step: std::marker::PhantomData::<Ready>,
            http_client,
            client_id,
            client_secret,
            redirect_uri,
            audience,
            scopes,
            token: Some(test_token),
            testing_url: Some(url.into()),
        }
    }

    /// Start the authorization phase.
    ///
    /// NOTE: Freezes the [`Client`] configuration.
    pub fn start_authorization(self) -> ClientBuilderStep<Authorize> {
        let ClientBuilderStep {
            _current_step: _,
            http_client,
            client_id,
            client_secret,
            redirect_uri,
            audience,
            scopes,
            token: _,
            testing_url,
        } = self;

        ClientBuilderStep {
            _current_step: std::marker::PhantomData::<Authorize>,
            http_client,
            client_id,
            client_secret,
            redirect_uri,
            audience,
            scopes,
            token: self.token,
            testing_url,
        }
    }
}
impl ClientBuilderStep<Authorize> {
    /// Build the TradeStation `/authorize` URL.
    pub fn authorization_url(&self, state: &str) -> Result<url::Url, Error> {
        use url::Url;

        let client_id = self
            .client_id
            .as_deref()
            .ok_or_else(|| Error::TokenConfig("client_id not set".into()))?;
        let redirect = self
            .redirect_uri
            .as_deref()
            .ok_or_else(|| Error::TokenConfig("redirect_uri not set".into()))?;
        let audience = self
            .audience
            .as_deref()
            .unwrap_or("https://api.tradestation.com");

        let scope_str = if self.scopes.is_empty() {
            "openid offline_access profile MarketData ReadAccount Trade"
        } else {
            &self.scopes.join(" ")
        };

        let mut url = Url::parse("https://signin.tradestation.com/authorize")?;
        url.query_pairs_mut()
            .append_pair("response_type", "code")
            .append_pair("client_id", client_id)
            .append_pair("redirect_uri", redirect)
            .append_pair("audience", audience)
            .append_pair("scope", scope_str)
            .append_pair("state", state);
        Ok(url)
    }

    /// Exchange the `code` from the authorize redirect for a [`Token`].
    pub async fn exchange_code(mut self, code: &str) -> Result<ClientBuilderStep<Ready>, Error> {
        let client_id = self
            .client_id
            .as_deref()
            .ok_or_else(|| Error::TokenConfig("client_id not set".into()))?;
        let client_secret = self
            .client_secret
            .as_deref()
            .ok_or_else(|| Error::TokenConfig("client_secret not set".into()))?;
        let redirect = self
            .redirect_uri
            .as_deref()
            .ok_or_else(|| Error::TokenConfig("redirect_uri not set".into()))?;

        let form = [
            ("grant_type", "authorization_code"),
            ("client_id", client_id),
            ("client_secret", client_secret),
            ("code", code),
            ("redirect_uri", redirect),
        ];

        let token = self
            .http_client
            .post("https://signin.tradestation.com/oauth/token")
            .header(
                reqwest::header::CONTENT_TYPE,
                "application/x-www-form-urlencoded",
            )
            .form(&form)
            .send()
            .await?
            .error_for_status()?
            .json::<Token>()
            .await?;

        self.token = Some(token.clone());

        let ClientBuilderStep {
            _current_step: _,
            http_client,
            client_id,
            client_secret,
            redirect_uri,
            audience,
            scopes,
            token: _,
            testing_url,
        } = self;

        Ok(ClientBuilderStep {
            _current_step: std::marker::PhantomData::<Ready>,
            http_client,
            client_id,
            client_secret,
            redirect_uri,
            audience,
            scopes,
            token: Some(token),
            testing_url,
        })
    }

    #[must_use]
    /// Set the [`Token`] for the [`Client`] to use.
    ///
    /// This is very useful if you already have a token
    /// stored that you can use to skip the authorize
    /// redirect flow with.
    ///
    /// As long as the redirect token is not expired the
    /// [`Token`] itself is still valid, as we auto handle
    /// refreshing the token when the access token expires.
    pub fn with_token(self, token: Token) -> ClientBuilderStep<Ready> {
        let ClientBuilderStep {
            _current_step: _,
            http_client,
            client_id,
            client_secret,
            redirect_uri,
            audience,
            scopes,
            token: _,
            testing_url,
        } = self;

        ClientBuilderStep {
            _current_step: std::marker::PhantomData::<Ready>,
            http_client,
            client_id,
            client_secret,
            redirect_uri,
            audience,
            scopes,
            token: Some(token),
            testing_url,
        }
    }
}
impl ClientBuilderStep<Ready> {
    /// Finish building into a [`Client`].
    pub async fn build(self) -> Result<Client, Error> {
        let token = self.token.ok_or_else(|| {
            Error::TokenConfig("no token: call exchange_code() or with_token()".into())
        })?;

        let base_url = self
            .testing_url
            .unwrap_or_else(|| "https://api.tradestation.com/v3".to_string());

        Ok(Client {
            http_client: self.http_client,
            client_id: self.client_id.unwrap_or_default(),
            client_secret: self.client_secret.unwrap_or_default(),
            token: Arc::new(Mutex::new(token)),
            redirect_uri: self
                .redirect_uri
                .unwrap_or_else(|| "http://localhost:8080/".to_string()),
            base_url,
        })
    }
}
