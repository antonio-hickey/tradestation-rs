use crate::{
    token::{RefreshedToken, Token},
    Error,
};
use futures::{Stream, TryStreamExt};
use reqwest::{header, Method, Response};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use std::{collections::HashMap, fmt::Display, sync::Arc};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    sync::Mutex,
    time::Instant,
};
use tokio_util::{
    codec::{FramedRead, LinesCodec},
    io::StreamReader,
};
use tracing::{debug, error, warn};

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

    /// The API environment this client is configured to use.
    ///
    /// Determines whether requests are sent to the live, simulation, or mock
    /// TradeStation API environment. This is also useful for identifying whether
    /// transactional requests can affect real brokerage accounts.
    pub environment: ClientEnvironment,
}
impl Client {
    /// Send an HTTP request to TradeStation's API, with automatic
    /// token refreshing near, at, or after auth token expiration.
    ///
    /// NOTE: You should use `Client::post()` or `Client::get()` in favor of this method.
    pub async fn send_request<F, Fut>(
        &self,
        method: Method,
        endpoint: &str,
        request_fn: F,
    ) -> Result<Response, Error>
    where
        F: Fn(String) -> Fut,
        Fut: std::future::Future<Output = Result<Response, reqwest::Error>>,
    {
        let token_guard = self.token.lock().await;
        let access_token = token_guard.access_token.clone();
        drop(token_guard);

        debug!(
            target: "tradestation::http",
            method = %method,
            endpoint,
            attempt = 1,
            "sending request"
        );

        let started_at = Instant::now();
        match request_fn(access_token).await {
            Ok(resp) => {
                let status = resp.status();

                debug!(
                    target: "tradestation::http",
                    method = %method,
                    endpoint,
                    attempt = 1,
                    status = status.as_u16(),
                    latency_ms = started_at.elapsed().as_millis(),
                    "received response"
                );

                // Check if the client gets a 401 unauthorized to try and re auth the client
                // this happens when auth token expires.
                if status == reqwest::StatusCode::UNAUTHORIZED {
                    warn!(
                        target: "tradestation::http",
                        method = %method,
                        endpoint,
                        "received an unauthorized error response; attempting to refresh the token..."
                    );

                    // Refresh the clients token
                    self.refresh_token().await.inspect_err(|refresh_err| {
                        error!(
                            target: "tradestation::http",
                            method = %method,
                            endpoint,
                            error = %refresh_err,
                            "failed to refresh the access token"
                        );
                    })?;

                    let token_guard = self.token.lock().await;
                    let access_token = token_guard.access_token.clone();
                    drop(token_guard);

                    debug!(
                        target: "tradestation::http",
                        method = %method,
                        endpoint,
                        attempt = 2,
                        "retrying request"
                    );

                    let retry_started_at = Instant::now();
                    let retry_response =
                        request_fn(access_token).await.inspect_err(|request_err| {
                            error!(
                                target: "tradestation::http",
                                method = %method,
                                endpoint,
                                attempt = 2,
                                latency_ms = retry_started_at.elapsed().as_millis(),
                                error = %request_err,
                                "received error response"
                            )
                        })?;
                    let retry_status = retry_response.status();

                    debug!(
                        target: "tradestation::http",
                        method = %method,
                        endpoint,
                        attempt = 2,
                        status = retry_status.as_u16(),
                        latency_ms = retry_started_at.elapsed().as_millis(),
                        "received response"
                    );

                    if !retry_status.is_success() {
                        warn!(
                            target: "tradestation::http",
                            method = %method,
                            endpoint,
                            attempt = 2,
                            status = retry_status.as_u16(),
                            "retried request was unsuccessful"
                        );
                    }

                    Ok(retry_response)
                } else {
                    Ok(resp)
                }
            }
            Err(request_err) => {
                error!(
                    target: "tradestation::http",
                    method = %method,
                    endpoint,
                    attempt = 1,
                    latency_ms = started_at.elapsed().as_millis(),
                    error = %request_err,
                    "request failed"
                );

                Err(Error::Request(request_err))
            }
        }
    }

    /// Send a POST request from your `Client` to TradeStation's API
    pub async fn post<T: Serialize>(&self, endpoint: &str, payload: &T) -> Result<Response, Error> {
        let url = format!("{}/{}", self.environment.base_url(), endpoint);
        let resp = self
            .clone()
            .send_request(Method::POST, endpoint, |access_token| {
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
        let url = format!("{}/{}", self.environment.base_url(), endpoint);
        let resp = self
            .clone()
            .send_request(Method::GET, endpoint, |access_token| {
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
        let url = format!("{}/{}", self.environment.base_url(), endpoint);
        let resp = self
            .clone()
            .send_request(Method::PUT, endpoint, |access_token| {
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
        let url = format!("{}/{}", self.environment.base_url(), endpoint);
        let resp = self
            .clone()
            .send_request(Method::DELETE, endpoint, |access_token| {
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
            let url = format!("{}/{}", self.environment.base_url(), endpoint);
            let started_at = Instant::now();

            debug!(
                target: "tradestation::stream",
                endpoint,
                "opening stream connection"
            );

            let resp = self
            .clone()
            .send_request(Method::GET, &endpoint, |access_token| {
                self.http_client
                    .get(&url)
                    .header(
                        reqwest::header::AUTHORIZATION,
                        format!("Bearer {access_token}"),
                    )
                    .send()
            })
            .await?;

            let resp_status = resp.status();
            if !resp_status.is_success() {
                warn!(
                    target: "tradestation::stream",
                    endpoint,
                    status = resp_status.as_u16(),
                    elapsed_ms = started_at.elapsed().as_millis(),
                    "stream connection rejected"
                );

                Err(Error::StreamIssue(format!(
                    "Request failed with status: {}",
                    resp_status
                )))?
            }

            debug!(
                target: "tradestation::stream",
                endpoint,
                status = resp_status.as_u16(),
                elapsed_ms = started_at.elapsed().as_millis(),
                "stream connection opened"
            );

            let byte_stream = resp.bytes_stream().map_err(std::io::Error::other);
            let stream_reader = StreamReader::new(byte_stream);
            let buf_reader = BufReader::new(stream_reader);
            let mut lines = buf_reader.lines();

            while let Some(line) = lines.next_line().await.inspect_err(|e| {
                warn!(
                    target: "tradestation::stream",
                    endpoint,
                    elapsed_ms = started_at.elapsed().as_millis(),
                    reason = "read_error",
                    error = %e,
                    "stream connection closed"
                );
            })? {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }

                let json: Value = serde_json::from_str(trimmed)
                    .inspect_err(|e| {
                        warn!(
                            target: "tradestation::stream",
                            endpoint,
                            elapsed_ms = started_at.elapsed().as_millis(),
                            reason = "decode_error",
                            error = %e,
                            "stream connection closed"
                        );
                    })
                    .map_err(Error::Json)?;

                yield json;
            }

            warn!(
                target: "tradestation::stream",
                endpoint,
                elapsed_ms = started_at.elapsed().as_millis(),
                reason = "remote_eof",
                "stream connection closed"
            );
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
        let url = format!("{}/{}", self.environment.base_url(), endpoint);
        let started_at = Instant::now();

        debug!(
            target: "tradestation::stream",
            endpoint,
            "opening stream connection"
        );

        let resp = self
            .clone()
            .send_request(Method::GET, endpoint, |access_token| {
                self.http_client
                    .get(&url)
                    .header(
                        reqwest::header::AUTHORIZATION,
                        format!("Bearer {access_token}"),
                    )
                    .send()
            })
            .await?;

        let resp_status = resp.status();
        if !resp_status.is_success() {
            warn!(
                target: "tradestation::stream",
                endpoint,
                status = resp_status.as_u16(),
                elapsed_ms = started_at.elapsed().as_millis(),
                "stream connection rejected"
            );

            return Err(Error::StreamIssue(format!(
                "Request failed with status: {}",
                resp_status
            )));
        }

        debug!(
            target: "tradestation::stream",
            endpoint,
            status = resp_status.as_u16(),
            elapsed_ms = started_at.elapsed().as_millis(),
            "stream connection opened"
        );

        let byte_stream = resp.bytes_stream().map_err(std::io::Error::other);
        let reader = StreamReader::new(byte_stream);

        let mut lines = FramedRead::with_capacity(reader, LinesCodec::new(), 64 * 1024);

        while let Some(line) = lines
            .try_next()
            .await
            .inspect_err(|e| {
                warn!(
                    target: "tradestation::stream",
                    endpoint,
                    elapsed_ms = started_at.elapsed().as_millis(),
                    reason = "read_error",
                    error = %e,
                    "stream connection closed"
                );
            })
            .map_err(|e| Error::StreamIssue(e.to_string()))?
        {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            match serde_json::from_str::<T>(line) {
                Ok(json) => {
                    if let Err(e) = process_chunk(json) {
                        if matches!(&e, Error::StopStream) {
                            debug!(
                                target: "tradestation::stream",
                                endpoint,
                                elapsed_ms = started_at.elapsed().as_millis(),
                                reason = "requested",
                                "stream connection closed"
                            );

                            return Ok(());
                        } else {
                            warn!(
                                target: "tradestation::stream",
                                endpoint,
                                elapsed_ms = started_at.elapsed().as_millis(),
                                reason = "callback_error",
                                error = %e,
                                "stream connection closed"
                            );

                            return Err(e);
                        }
                    }
                }
                Err(e) => {
                    warn!(
                        target: "tradestation::stream",
                        endpoint,
                        elapsed_ms = started_at.elapsed().as_millis(),
                        reason = "decode_error",
                        error = %e,
                        "stream connection closed"
                    );

                    return Err(Error::Json(e));
                }
            }
        }

        warn!(
            target: "tradestation::stream",
            endpoint,
            elapsed_ms = started_at.elapsed().as_millis(),
            reason = "remote_eof",
            "stream connection closed"
        );

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

        let started_at = Instant::now();

        debug!(
            target: "tradestation::auth",
            "refreshing access token"
        );

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

        debug!(
            target: "tradestation::auth",
            elapsed_ms = started_at.elapsed().as_millis(),
            "access token refreshed"
        );

        Ok(())
    }
}

#[derive(Debug, Default)]
/// Initial builder state before the API environment has been selected.
pub struct Configure;
#[derive(Debug, Default)]
/// Builder state after the API environment has been selected.
pub struct Configured;
#[derive(Debug, Default)]
/// Authorization flow state.
pub struct Authorize;
#[derive(Debug, Default)]
/// Final builder state with a token available.
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
    environment: Option<ClientEnvironment>,
    base_url: String,
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
    pub fn credentials<S: Into<String>>(mut self, client_id: S, client_secret: S) -> Self {
        self.client_id = Some(client_id.into());
        self.client_secret = Some(client_secret.into());

        self
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
    /// Set the environment for the client to run in.
    ///
    /// There's only 3 valid client environments:
    /// - [`ClientEnvironment::Live`] uses real TradeStation accounts and can place
    ///   real orders.
    /// - [`ClientEnvironment::Simulation`] uses TradeStation's SIM API for paper
    ///   trading with simulated accounts and simulated fills.
    /// - [`ClientEnvironment::Mock`] is intended for local mock tests, CI, and
    ///   other flows that should not require or expose live credentials.
    pub fn environment(self, environment: ClientEnvironment) -> ClientBuilderStep<Configured> {
        let base_url = environment.base_url().to_owned();

        let ClientBuilderStep {
            _current_step: _,
            http_client,
            client_id,
            client_secret,
            redirect_uri,
            audience,
            scopes,
            token,
            environment: _,
            base_url: _,
        } = self;

        ClientBuilderStep {
            _current_step: std::marker::PhantomData::<Configured>,
            http_client,
            client_id,
            client_secret,
            redirect_uri,
            audience,
            scopes,
            token,
            environment: Some(environment),
            base_url,
        }
    }
}
impl ClientBuilderStep<Configured> {
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
            environment,
            base_url,
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
            environment,
            base_url,
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
            environment,
            base_url,
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
            environment,
            base_url,
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

        let started_at = Instant::now();
        debug!(
            target: "tradestation::auth",
            "exchanging authorization code for oauth token"
        );

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

        debug!(
            target: "tradestation::auth",
            elapsed_ms = started_at.elapsed().as_millis(),
            "exchanged authorization code for oauth token"
        );

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
            environment,
            base_url,
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
            environment,
            base_url,
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
            environment,
            base_url,
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
            environment,
            base_url,
        }
    }
}
impl ClientBuilderStep<Ready> {
    /// Finish building into a [`Client`].
    pub async fn build(self) -> Result<Client, Error> {
        let environment = self.environment.ok_or_else(|| Error::EnvironmentNotSet)?;
        let token = self.token.ok_or_else(|| {
            Error::TokenConfig("no token: use exchange_code() or with_token()".into())
        })?;

        let client = Client {
            http_client: self.http_client,
            client_id: self.client_id.unwrap_or_default(),
            client_secret: self.client_secret.unwrap_or_default(),
            token: Arc::new(Mutex::new(token)),
            redirect_uri: self
                .redirect_uri
                .unwrap_or_else(|| "http://localhost:8080/".to_string()),
            environment,
        };

        debug!(
            target: "tradestation::client",
            environment = %client.environment,
            "client initialized"
        );

        Ok(client)
    }
}

#[derive(Clone, Debug)]
/// Selects which TradeStation API environment a [`Client`] should use.
///
/// This controls the base URL and the safety level of account/order access.
///
/// - [`ClientEnvironment::Live`] uses real TradeStation accounts and can place
///   real orders.
/// - [`ClientEnvironment::Simulation`] uses TradeStation's SIM API for paper
///   trading with simulated accounts and simulated fills.
/// - [`ClientEnvironment::Mock`] is intended for local mock tests, CI, and
///   other flows that should not require or expose live credentials.
pub enum ClientEnvironment {
    /// The live TradeStation API environment.
    ///
    /// This environment uses the live API base URL:
    ///
    /// `https://api.tradestation.com/v3`
    ///
    /// NOTE: Transactional requests, such as order placement, can affect real
    /// brokerage accounts and real money.
    Live,

    /// The TradeStation simulator environment.
    ///
    /// This environment uses the SIM API base URL:
    ///
    /// `https://sim-api.tradestation.com/v3`
    ///
    /// NOTE: The simulator is intended for paper trading. It mirrors the live API
    /// behavior, but uses fake trading accounts seeded with fake money. Orders
    /// are not actually routed or executed in the market; simulated executions
    /// occur instead, typically with instant fills.
    ///
    /// This is useful for learning the API, testing application behavior,
    /// paper-trading workflows, demos, competitions, or validating order flows
    /// before exposing them to live users.
    Simulation,

    /// A local mock testing environment.
    ///
    /// This environment is intended for CI, unit tests, integration tests,
    /// mocked clients, fixtures, and other non-networked or non-production test
    /// flows.
    ///
    /// NOTE: Unlike [`ClientEnvironment::Simulation`], this is not TradeStation's SIM
    /// API. It should be used when tests should avoid real API calls, avoid
    /// real account access, and avoid exposing live or simulator credentials.
    Mock(String),
}
/// Format the [`ClientEnvironment`] as a short, human readable string.
impl Display for ClientEnvironment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Live => f.write_str("LIVE"),
            Self::Simulation => f.write_str("SIM"),
            Self::Mock(_) => f.write_str("MOCK"),
        }
    }
}
impl ClientEnvironment {
    /// Returns the base URL associated with the environment.
    ///
    /// Live and simulation environments use TradeStation's standard API URLs.
    /// Mock environments return the custom URL provided when the environment
    /// was created.
    #[must_use]
    pub fn base_url(&self) -> &str {
        match self {
            Self::Live => "https://api.tradestation.com/v3",
            Self::Simulation => "https://sim-api.tradestation.com/v3",
            Self::Mock(url) => url,
        }
    }
}
