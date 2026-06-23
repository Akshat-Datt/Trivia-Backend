use axum::{
    Router,
    routing::{get, post}
};
use crate::{handlers::questions_handler::{
    create_question, delete_question, get_question_by_id, get_questions_admin, get_questions_public, submit_quiz, update_question, toggle_question_status
}, state::app_state::AppState};

pub fn questions_routes() -> Router<AppState>{
    Router::new()
    .route("/questions", get(get_questions_public).post(create_question))
    .route("/admin/questions",get(get_questions_admin))
    .route("/questions/{id}", get(get_question_by_id).put(update_question).delete(delete_question))
    .route("/submit/questions", post(submit_quiz))
    .route("/questions/{id}/toggle-status", post(toggle_question_status))
}