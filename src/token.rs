use crate::Error;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
/// TradeStation API Bearer Token.
///
/// NOTE: You should never want to manually initialize `Token`. Instead let
/// any initialization of `Token` should be handled directly in the `TsClient`.
pub struct Token {
    /// Access token used to authenticate API requests.
    pub access_token: String,

    /// Refresh token used to obtain new access tokens.
    pub refresh_token: String,

    /// ID token used for identity verification.
    pub id_token: String,

    /// Token type.
    ///
    /// NOTE: Always `"Bearer"`.
    pub token_type: String,

    /// Scopes associated with the `Token`.
    #[serde(
        serialize_with = "serialize_scopes",
        deserialize_with = "deserialize_scopes"
    )]
    pub scope: Vec<Scope>,

    /// How many seconds until the `Token` expires.
    pub expires_in: u64,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
/// Builder for a TradeStation API Bearer Token.
pub struct TokenBuilder {
    /// Access token used to authenticate API requests.
    pub access_token: Option<String>,

    /// Refresh token used to obtain new access tokens.
    pub refresh_token: Option<String>,

    /// ID token used for identity verification.
    pub id_token: Option<String>,

    /// Scopes associated with the `Token`.
    #[serde(
        serialize_with = "serialize_scopes",
        deserialize_with = "deserialize_scopes"
    )]
    pub scope: Vec<Scope>,

    /// How many seconds until the `Token` expires.
    pub expires_in: Option<u64>,
}
impl TokenBuilder {
    /// Initialize a new `TokenBuilder` instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the access token for the `Token`.
    pub fn access_token(mut self, token: impl Into<String>) -> Self {
        self.access_token = Some(token.into());
        self
    }

    /// Set the refresh token for the `Token`.
    pub fn refresh_token(mut self, token: impl Into<String>) -> Self {
        self.refresh_token = Some(token.into());
        self
    }

    /// Set the id token for the `Token`.
    pub fn id_token(mut self, token: impl Into<String>) -> Self {
        self.id_token = Some(token.into());
        self
    }

    /// Set the scopes for the `Token`.
    pub fn scope(mut self, scopes: Vec<Scope>) -> Self {
        self.scope = scopes;
        self
    }

    /// Set the expiration for the `Token`.
    ///
    /// NOTE: Defaults to `1200`.
    pub fn expires_in(mut self, seconds: u64) -> Self {
        self.expires_in = Some(seconds);
        self
    }

    /// Try to build the `Token`.
    ///
    /// NOTE: `access_token`, `refresh_token`, and `id_token`
    /// are required to be set to successfully build a `Token`.
    pub fn build(self) -> Result<Token, Error> {
        if self.scope.is_empty() {
            return Err(Error::TokenConfig(
                "`Token::scope` is empty, but required.".into(),
            ));
        }

        Ok(Token {
            access_token: self.access_token.ok_or(Error::TokenConfig(
                "`Token::access_token` is not set".into(),
            ))?,
            refresh_token: self.refresh_token.ok_or(Error::TokenConfig(
                "`Token::refresh_token` is not set".into(),
            ))?,
            id_token: self
                .id_token
                .ok_or(Error::TokenConfig("`Token::id_token` is not set".into()))?,
            token_type: "Bearer".to_string(),
            scope: self.scope,
            expires_in: self.expires_in.unwrap_or(1200),
        })
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
/// Refreshed TradeStation API Bearer Token.
///
/// NOTE: There is no refresh token because you should reuse
/// the refresh token on your current `Token` for security.
pub struct RefreshedToken {
    /// Access token used to authenticate API requests.
    pub access_token: String,

    /// ID token used for identity verification.
    pub id_token: String,

    /// Token type.
    ///
    /// NOTE: Always "Bearer".
    pub token_type: String,

    /// Scopes associated with the `Token`.
    #[serde(
        serialize_with = "serialize_scopes",
        deserialize_with = "deserialize_scopes"
    )]
    pub scope: Vec<Scope>,

    /// How many seconds until the `Token` expires.
    pub expires_in: u64,
}

/// The different API Scopes a [`Token`] can be configured with.
///
/// NOTE: You should limit the level of scope a token has to only
/// what it needs, for security reasons. For example if your application
/// only needs to stream market data then it should not have the `Scope::Trade`
/// as it's not needed and could become dangerous if your leak your token.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Scope {
    /// Requests access to lookup or stream Market Data.
    MarketData,

    /// Requests access to view Brokerage Accounts belonging to the current user.
    ReadAccount,

    /// Requests access to execute options related endpoints.
    Trade,

    /// Requests access to execute options related endpoints.
    OptionSpreads,

    /// Request access to execute market depth related endpoints.
    Matrix,

    /// Returns the sub claim, which uniquely identifies the user.
    ///
    /// In an ID Token, `iss`, `aud`, `exp`, `iat`, and `at_hash`
    /// claims will also be present.
    ///
    /// NOTE: This scope is required.
    OpenId,

    /// Allows for use of Refresh Tokens.
    ///
    /// NOTE: This scope is required.
    OfflineAccess,

    /// Returns claims in the ID Token that represent basic profile information,
    /// including `name`, `family_name`, `given_name`, `middle_name`, `nickname`,
    /// `picture`, and `updated_at`.
    ///
    /// NOTE: This scope is optional.
    Profile,

    /// Returns the `email` claim in the ID Token, which contains the user's email
    /// address, and `email_verified`, which is a boolean indicating whether the email
    /// address was verified by the user.
    ///
    /// NOTE: This scope is optional.
    Email,
}
impl Scope {
    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "MarketData" => Ok(Scope::MarketData),
            "ReadAccount" => Ok(Scope::ReadAccount),
            "Trade" => Ok(Scope::Trade),
            "OptionSpreads" => Ok(Scope::OptionSpreads),
            "Matrix" => Ok(Scope::Matrix),
            "openid" => Ok(Scope::OpenId),
            "offline_access" => Ok(Scope::OfflineAccess),
            "profile" => Ok(Scope::Profile),
            "email" => Ok(Scope::Email),
            _ => Err(format!("unknown scope: {s}")),
        }
    }
}
impl std::fmt::Display for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Scope::MarketData => "MarketData",
            Scope::ReadAccount => "ReadAccount",
            Scope::Trade => "Trade",
            Scope::OptionSpreads => "OptionSpreads",
            Scope::Matrix => "Matrix",
            Scope::OpenId => "openid",
            Scope::OfflineAccess => "offline_access",
            Scope::Profile => "profile",
            Scope::Email => "email",
        };
        write!(f, "{s}")
    }
}

/// Serialize the vector of scopes into a space seperated string of scopes.
fn serialize_scopes<S>(scopes: &[Scope], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let scopes_str = scopes
        .iter()
        .map(|scope| scope.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    serializer.serialize_str(&scopes_str)
}

/// Deserialize the space seperated string into a vector of scopes.
fn deserialize_scopes<'de, D>(deserializer: D) -> Result<Vec<Scope>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    // If the string is empty, return an empty Vec.
    if s.trim().is_empty() {
        return Ok(Vec::new());
    }

    s.split_whitespace()
        .map(|token| Scope::from_str(token).map_err(de::Error::custom))
        .collect()
}
