use derive_more::Display;
use serde::{Deserialize, Serialize};
use serde_json::Number;

#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{issues:?}")]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export))]
pub struct FailureResult {
    pub issues: Vec<Issue>,
}

impl std::error::Error for FailureResult {}

#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{path:?}: {message}")]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(optional_fields = nullable))]
pub struct Issue {
    /// The error message of the issue.
    pub message: String,
    /// The path of the issue, if any.
    #[cfg_attr(feature = "ts-rs", ts(type = "(string | { key: string })[]"))]
    pub path: Option<Vec<IssuePath>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[serde(untagged)]
pub enum PropertyKey {
    String(String),
    #[cfg_attr(feature = "ts-rs", ts(type = "number"))]
    Number(Number),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[serde(untagged)]
pub enum IssuePath {
    PropertyKey(PropertyKey),
    PathSegment(PathSegment),
}

/// The path segment interface of the issue.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
pub struct PathSegment {
    /// The key representing a path segment.
    pub key: PropertyKey,
}
