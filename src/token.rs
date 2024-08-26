use serde::{Deserialize, Serialize};

/// TradeStation API Bearer Token
///
/// NOTE: You should never want to manually initialize `Token`. Instead let
/// any initialization of `Token` should be handled directly in the `TsClient`.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Token {
    /// Access token used to authenticate API requests
    access_token: String,
    /// Refresh token used to obtain new access tokens
    refresh_token: String,
    /// ID token used for identity verification
    id_token: String,
    /// Token type
    /// NOTE: Always "Bearer"
    token_type: String,
    /// Scopes associated with the `Token`
    /// TODO: Make types for scopes
    scope: String,
    /// How many seconds until the `Token` expires
    expires_in: u64,
}
