use serde::{Deserialize, Serialize};

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
    ///
    /// TODO: Make types for scopes.
    pub scope: String,
    /// How many seconds until the `Token` expires.
    pub expires_in: u64,
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
    /// TODO: Make types for scopes.
    pub scope: String,
    /// How many seconds until the `Token` expires.
    pub expires_in: u64,
}
