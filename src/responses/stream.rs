use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// A stream response to tell you a stream connection is still alive.
///
/// NOTE: Sent every 5 seconds of inactivity.
pub struct Heartbeat {
    /// The heartbeat count, sent to indicate that the stream is alive, although data is not actively being sent.
    ///
    /// NOTE: A heartbeat will be sent after 5 seconds on an idle stream.
    pub heartbeat: u64,

    /// Timestamp of the heartbeat.
    ///
    /// NOTE: Represented as an RFC3339 formatted date, a profile of ISO8601 date standard.
    /// E.g. `2030-01-01T23:30:30Z`
    pub timestamp: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// A stream response to tell you state changes
pub struct StreamStatus {
    /// The latest status of the stream.
    pub stream_status: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// A stream response to tell you of an error during stream
pub struct ErrorResp {
    /// The title of the error.
    pub error: String,

    /// The description of the error.
    pub message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// DEPRECATED: Removed by TradeStation.
    pub account_id: Option<String>,
}
