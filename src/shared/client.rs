use lambda_http::Body;
use reqwest::header;
use serde::Serialize;
use serde_json::json;
use std::env;

fn client() -> Result<reqwest::Client, reqwest::Error> {
    let token = env::var("BOT_USER_OAUTH_TOKEN").expect("BOT_USER_OAUTH_TOKEN must be set");
    let bearer_token: &str = &format!("Bearer {}", token);

    let mut headers = header::HeaderMap::new();
    headers.insert(
        "AUTHORIZATION",
        header::HeaderValue::from_str(bearer_token).unwrap(),
    );

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;
    Ok(client)
}

async fn post(endpoint: &str, body: String) -> Result<reqwest::Response, reqwest::Error> {
    let client = client()?;
    client.post(endpoint).body(body).send().await
}

#[derive(Debug, Serialize)]
pub struct PostMessageArguments {
    channel: String,
    // used as a fallback string of notifications
    text: Option<String>,
    blocks: Vec<String>,
    thread_ts: Option<String>,
}

async fn post_message(arg: PostMessageArguments) -> Result<reqwest::Response, reqwest::Error> {
    let body = json!(arg);
    post("https://slack.com/api/chat.postMessage", body.to_string()).await
}
