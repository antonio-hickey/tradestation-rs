use serde::de::{self, DeserializeOwned, Deserializer};
use serde::{self, Deserialize, Serialize};

pub mod account;
pub mod execution;
pub mod market_data;
pub mod stream;

pub use market_data as MarketData;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// The raw error type provided by the api.
pub struct ApiError {
    /// The error.
    pub error: String,

    /// The error message.
    pub message: String,
}

#[derive(Debug)]
/// Generic wrapper around API responses for
/// detecting error vs successful responses.
pub enum ApiResponse<T> {
    /// A successful response of some type T.
    Success(T),

    /// A error response of `respones::ApiError`.
    Error(ApiError),
}
impl<'de, T> Deserialize<'de> for ApiResponse<T>
where
    T: DeserializeOwned,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let helper = serde_json::Value::deserialize(deserializer)?;

        // Check if "error" and "message" keys are available at the root
        // if so then it's an error response, else it's a success response
        if helper.get("error").is_some() && helper.get("message").is_some() {
            let err: ApiError = serde_json::from_value(helper).map_err(de::Error::custom)?;
            Ok(ApiResponse::Error(err))
        } else {
            let data: T = serde_json::from_value(helper).map_err(de::Error::custom)?;
            Ok(ApiResponse::Success(data))
        }
    }
}
