use axum::{
    Json, Router, extract::{Path, path}, http::StatusCode, routing::get
};
use std::net::SocketAddr;
use serde::Serialize;

#[derive(Serialize)]
struct HealthStatus{
    status: String
}

#[derive(Serialize, Clone)]
struct Question{
    id: u32,
    question: String
}

#[tokio::main]
async fn main() {
    let app: Router<()> = Router::new().route("/", get(root)).route("/questions", get(get_questions)). route("/questions/{id}", get(get_question_by_id));

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

async fn get_question_by_id(Path(id): Path<u32>) -> Result<Json<Question>, StatusCode>{
    let questions = sample_questions();

    let question = questions.into_iter().find(|q| q.id == id);

    match question {
        Some(q) => Ok(Json(q)),
        None => Err(StatusCode::NOT_FOUND)
    }
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