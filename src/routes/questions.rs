use axum::{
    Router,
    routing::{get, post}
};
use crate::{handlers::questions_handler::{
    change_challenge_date, create_question, delete_question, get_daily_questions, get_endless_questions, get_question_by_id, get_questions_admin, get_questions_public, submit_quiz, toggle_question_status, update_question
}, state::app_state::AppState};

pub fn questions_routes() -> Router<AppState>{
    Router::new()
    .route("/questions", get(get_questions_public).post(create_question))
    .route("/admin/questions",get(get_questions_admin))
    .route("/questions/daily-quiz", get(get_daily_questions))
    .route("/questions/endless-quiz", get(get_endless_questions))
    .route("/questions/{id}", get(get_question_by_id).put(update_question).delete(delete_question))
    .route("/submit/questions", post(submit_quiz))
    .route("/questions/{id}/toggle-status", post(toggle_question_status))
    .route("/questions/{id}/challenge-date", post(change_challenge_date))
}