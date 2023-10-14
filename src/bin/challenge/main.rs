mod models;
use lambda_http::{
    http::{response::Builder, StatusCode},
    service_fn, Body, Error, Request, Response,
};
use models::*;
use serde_json::json;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .event_format(tracing_subscriber::fmt::format::json())
        .with_target(false)
        .without_time()
        .init();

    lambda_http::run(service_fn(handler)).await?;
    Ok(())
}

pub(crate) async fn handler(request: Request) -> Result<Response<Body>, FailureResponse> {
    let req: serde_json::Value = match request.body() {
        Body::Text(text) => serde_json::from_str(text).unwrap_or(serde_json::Value::Null),
        _ => serde_json::Value::Null,
    };
    let challenge = ChallengeRequest::from(req);

    match env::var("SLACK_VERIFICATION_TOKEN") {
        Ok(token) => {
            if challenge.token != token {
                tracing::warn!("Authentication failed: Invalid token");
                let res = FailureResponse {
                    body: "Invalid token".to_string(),
                };
                return Err(res);
            }
        }
        Err(_) => {
            tracing::error!("SLACK_VERIFICATION_TOKEN is not set");
            let res = FailureResponse {
                body: "SLACK_VERIFICATION_TOKEN is not set".to_string(),
            };
            return Err(res);
        }
    }

    tracing::info!("challenge success");
    let body = json!(SuccessResponse {
        challenge: challenge.challenge,
    });
    let res = Builder::new()
        .status(StatusCode::OK)
        .body(Body::Text(body.to_string()))
        .unwrap();
    Ok(res)
}
