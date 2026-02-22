use axum::{
    Router,
    routing::get
};
use crate::handlers::questions_handler::{
    create_question, get_question_by_id, get_questions
};

pub fn questions_routes() -> Router{
    Router::new()
    .route("/questions", get(get_questions).post(create_question))
    .route("/questions/{id}", get(get_question_by_id))
}