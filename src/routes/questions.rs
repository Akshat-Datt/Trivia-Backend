use axum::{
    Router,
    routing::get
};
use crate::{handlers::questions_handler::{
    create_question, delete_question, get_question_by_id, get_questions, update_question
}, state::app_state::AppState};

pub fn questions_routes() -> Router<AppState>{
    Router::new()
    .route("/questions", get(get_questions).post(create_question))
    .route("/questions/{id}", get(get_question_by_id).put(update_question).delete(delete_question))
}