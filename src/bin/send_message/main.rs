use lambda_http::{
    http::{response::Builder, StatusCode},
    service_fn, Body, Error, Request, Response,
};

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

    print!("{:?}", req);

    let res = Builder::new()
        .status(StatusCode::ACCEPTED)
        .body(Body::Empty)
        .unwrap();
    Ok(res)
}
