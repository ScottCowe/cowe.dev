use axum::{Json, Router, extract::Path, routing::get};
use serde_json::{Value, json};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/post", get(get_posts))
        .route("/post/{id}", get(get_post));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_posts() -> Json<Value> {
    Json(json!({}))
}

async fn get_post(Path(path): Path<String>) -> Json<Value> {
    Json(json!({ "id": path }))
}
