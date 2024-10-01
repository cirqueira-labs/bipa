use axum::{error_handling, routing::*, Error, Router};
use reqwest::Client;

const ENDPOINT_REQ: &str = "https://mempool.space/api/v1/lightning/nodes/rankings/connectivity";

#[tokio::main]
async fn main() {
    let client = Client::new();
    let response = client
        .get(ENDPOINT_REQ)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    dbg!(response);
}
