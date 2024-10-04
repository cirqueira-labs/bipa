use axum::{
    response::{IntoResponse, Response},
    routing::*,
    Router,
};

mod db;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let app = Router::new().route("/nodes", get(nodes));
    axum::serve(listener, app).await.unwrap();
}

pub async fn nodes() -> Response {
    let nodes = db::get_data().await.unwrap();
    nodes.into_response()
}
