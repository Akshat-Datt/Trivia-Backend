use axum::{
    Router,
    routing::get
};
use crate::handlers::questions_handler::{
    get_questions,
    get_question_by_id
};

pub fn questions_routes() -> Router{
    Router::new()
    .route("/questions", get(get_questions))
    .route("/questions/{id}", get(get_question_by_id))
}