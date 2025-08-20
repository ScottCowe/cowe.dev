use axum::Router;
use sqlx::{
    PgPool,
    postgres::{PgConnectOptions, PgPoolOptions},
};

pub mod routes;
pub mod types;

use crate::routes::posts;

#[derive(Clone)]
pub struct AppState {
    pool: PgPool,
}

#[tokio::main]
async fn main() {
    // TODO: Proper error handling
    // TODO: Options file or smth
    let opts = PgConnectOptions::new().socket("/tmp/").database("db");

    let pool = PgPoolOptions::new()
        .max_connections(5) // <- Seems low (in theory anyway)
        .connect_with(opts)
        .await
        .unwrap();

    let state = AppState { pool };

    let router = Router::new().merge(posts::router()).with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();
}
