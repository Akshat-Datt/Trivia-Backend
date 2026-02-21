use axum::{
    Router
};
use crate::routes::questions::questions_routes;

mod models;
mod handlers;
mod routes;

#[tokio::main]
async fn main() {
    let app: Router<()> = Router::new().merge(questions_routes());

    axum::serve(
        tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap()
        ,app
    )
    .await.unwrap();
}