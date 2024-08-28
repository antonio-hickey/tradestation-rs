use serde::{Deserialize, Serialize};

/// TradeStation API Bearer Token
///
/// NOTE: You should never want to manually initialize `Token`. Instead let
/// any initialization of `Token` should be handled directly in the `TsClient`.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Token {
    /// Access token used to authenticate API requests
    pub access_token: String,
    /// Refresh token used to obtain new access tokens
    pub refresh_token: String,
    /// ID token used for identity verification
    pub id_token: String,
    /// Token type
    /// NOTE: Always "Bearer"
    token_type: String,
    /// Scopes associated with the `Token`
    /// TODO: Make types for scopes
    pub scope: String,
    /// How many seconds until the `Token` expires
    pub expires_in: u64,
}
