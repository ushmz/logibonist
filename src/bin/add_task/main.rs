use lambda_http::{
    http::{response::Builder, StatusCode},
    service_fn, Body, Error, Request, Response,
};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Task {
    user: String,
    title: String,
    description: String,
}

impl TryFrom<Request> for Task {
    type Error = Error;

    fn try_from(req: Request) -> Result<Self, Self::Error> {
        let body = match req.body() {
            Body::Text(text) => text,
            _ => return Err(Error::from("Invalid body")),
        };
        let task: Task = serde_json::from_str(body).unwrap();
        Ok(task)
    }
}

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

pub(crate) async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let req: Task = req.try_into()?;
    print!("{:?}", req);
    let res = Builder::new().status(StatusCode::CREATED).body(Body::Empty).unwrap();

    Ok(res)
}
