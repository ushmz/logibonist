mod client;
mod model;

use client::post_message;
use lambda_http::{
    http::{response::Builder, StatusCode},
    service_fn, Body, Error, Request, Response,
};
use model::PostMessageArguments;

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

async fn handler(request: Request) -> Result<Response<Body>, Error> {
    let req: serde_json::Value = match request.body() {
        Body::Text(text) => serde_json::from_str(text).unwrap_or(serde_json::Value::Null),
        _ => serde_json::Value::Null,
    };
    tracing::debug!("{:?}", req);

    let arg = PostMessageArguments::new()
        .channel(req["channel"].to_string())
        .text(req["text"].to_string())
        .blocks(
            req["blocks"]
                .as_array()
                .unwrap()
                .iter()
                .map(|x| x.to_string())
                .collect(),
        )
        .thread_ts(req["thread_ts"].to_string());

    let api_res = post_message(&arg).await;
    match api_res {
        Ok(res) => tracing::info!("{:?}", res),
        Err(e) => tracing::error!("{:?}", e),
    }

    let res = Builder::new()
        .status(StatusCode::ACCEPTED)
        .body(Body::Empty)
        .unwrap();
    Ok(res)
}
