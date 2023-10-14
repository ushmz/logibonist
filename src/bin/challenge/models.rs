use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ChallengeRequest {
    pub token: String,
    pub r#type: String,
    pub challenge: String,
}

impl From<serde_json::Value> for ChallengeRequest {
    fn from(value: serde_json::Value) -> Self {
        ChallengeRequest {
            token: value["token"].to_string(),
            r#type: value["type"].to_string(),
            challenge: value["challenge"].to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct SuccessResponse {
    pub challenge: String,
}

#[derive(Serialize, Debug)]
pub(crate) struct FailureResponse {
    pub body: String,
}

impl std::fmt::Display for FailureResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FailureResponse: {}", self.body)
    }
}

impl std::error::Error for FailureResponse {}
