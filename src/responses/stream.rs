use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// A stream response to tell you a stream connection is still alive.
///
/// NOTE: Sent every 5 seconds of inactivity.
pub struct Heartbeat {
    pub heartbeat: u64,
    pub timestamp: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// A stream response to tell you state changes
pub struct StreamStatus {
    pub stream_status: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// A stream response to tell you of an error during stream
pub struct ErrorResp {
    pub error: String,
    pub message: String,
    pub account_id: String,
}
