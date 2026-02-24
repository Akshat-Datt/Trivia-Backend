use std::sync::Arc;
use tokio::sync::Mutex;

use axum::{
    Router
};
use crate::{routes::questions::questions_routes, state::app_state::AppState};

mod models;
mod handlers;
mod routes;
mod state;

#[tokio::main]
async fn main() {
    let shared_state = AppState{
        questions: Arc::new(Mutex::new(vec![]))
    };
    
    let app = Router::new().merge(questions_routes()).with_state(shared_state);

    axum::serve(
        tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap()
        ,app
    )
    .await.unwrap();
}