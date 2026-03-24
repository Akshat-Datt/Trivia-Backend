use dotenvy::dotenv;
use sqlx::{postgres::PgPoolOptions};
use std::env;

use crate::{routes::questions::questions_routes, state::app_state::AppState};
use axum::Router;

mod handlers;
mod models;
mod routes;
mod state;
mod repository;
mod services;
mod errors;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let shared_state = AppState { db: db_pool };

    let app = Router::new()
        .merge(questions_routes())
        .with_state(shared_state);

    axum::serve(
        tokio::net::TcpListener::bind("127.0.0.1:3000")
            .await
            .unwrap(),
        app,
    )
    .await
    .unwrap();
}
