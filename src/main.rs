use axum::{
    Json, Router, routing::get
};
use std::net::SocketAddr;
use serde::Serialize;

#[derive(Serialize)]
struct HealthStatus{
    status: String
}

#[derive(Serialize, Clone)]
struct Question{
    id: i32,
    question: String
}

#[tokio::main]
async fn main() {
    let app: Router<()> = Router::new().route("/", get(root)).route("/questions", get(get_questions));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("Server running at http://{}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap()
        ,app
    )
    .await.unwrap();
}

async fn root() -> Json<HealthStatus>{
    let status = HealthStatus{
        status: "Trivia Backend Running".to_owned(),
    };
    Json(status)
}

async fn get_questions() -> Json<Vec<Question>>{
    let questions = sample_questions();
    Json(questions)
}

fn sample_questions() -> Vec<Question>{
    vec![
        Question{
            id:1,
            question: "What is Android".to_owned()
        },
        Question{
            id:2,
            question: "What is Rust".to_owned()
        }
    ]
}