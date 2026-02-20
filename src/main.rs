use axum::{
    Json, Router, routing::get
};
use std::net::SocketAddr;
use serde::Serialize;

#[derive(Serialize)]
struct HealthStatus{
    status:String,
}

#[tokio::main]
async fn main() {
    let app: Router<()> = Router::new().route("/", get(root));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("Server running at http://{}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap()
        ,app
    )
    .await.unwrap();
}

async fn root() -> Json<HealthStatus>{
    let response = HealthStatus{
        status: "Trivia backend running".to_string(),
    };
    Json(response)
}