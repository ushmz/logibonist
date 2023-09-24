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

    match env::var("SLACK_VERIFICATION_TOKEN") {
        Ok(token) => {
            if req["token"].to_string() != token {
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

    let type_ = req["type"].as_str();
    match type_ {
        Some("url_verification") => {
            // Do nothing
        }
        Some(_) => {
            tracing::info!("challenge failed: type is not url_verification");
            let res = FailureResponse {
                body: "type is not url_verification".to_string(),
            };
            return Err(res);
        }
        None => {
            tracing::info!("challenge failed: type is not found");
            let res = FailureResponse {
                body: "type is not found".to_string(),
            };
            return Err(res);
        }
    }

    let challenge = req["challenge"].as_str();
    match challenge {
        Some(challenge) => {
            tracing::info!("challenge success");
            let body = json!(SuccessResponse {
                challenge: challenge.to_string(),
            });
            let res = Builder::new()
                .status(StatusCode::OK)
                .body(Body::Text(body.to_string()))
                .unwrap();
            Ok(res)
        }
        None => {
            tracing::info!("challenge failed: challenge is not found");
            let res = FailureResponse {
                body: "challenge is not found".to_string(),
            };
            Err(res)
        }
    }
}
