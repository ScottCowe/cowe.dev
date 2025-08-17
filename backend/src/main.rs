use axum::{Json, Router, extract::Path, routing::get};
use serde_json::{Value, json};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/post", get(get_posts))
        .route("/api/post/{id}", get(get_post));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_posts() -> Json<Value> {
    Json(json!(
        [
            {
                "id": "post-1",
                "title": "Post 1",
                "format": "plaintext",
            },
            {
                "id": "post-2",
                "title": "Post 2",
                "format": "plaintext",
            },
            {
                "id": "post-3",
                "title": "Post 3",
                "format": "plaintext",
            },
            {
                "id": "post-4",
                "title": "Post 4",
                "format": "plaintext",
            }
        ]
    ))
}

async fn get_post(Path(path): Path<String>) -> Json<Value> {
    let title = format!("Title: {}", path);
    let content = format!("Content: {}", path);

    Json(json!({
        "id": path,
        "title": title,
        "format": "plaintext",
        "content": content
    }))
}
