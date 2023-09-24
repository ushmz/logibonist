use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ChallengeRequest {
    pub token: String,
    pub r#type: String,
    pub challenge: String,
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
