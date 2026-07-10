use axum::{
    Json, extract::{Path, Query, State}, http::StatusCode
};
use crate::{dto::{question_request::QuestionChallengeDateRequest, question_response::{DailyQuestion, QuestionAdmin, QuestionChallengeDate, QuestionPublic, QuestionStatus}, score_response::ScoreResponse, submit_quiz_request::QuizSubmission}, errors::errors::AppError, models::question_data::{CreateQuestion, Question, QuestionQuery, UpdateQuestion}};
use crate::state::app_state::AppState;
use crate::services::question_service;

pub async fn get_questions_public(Query(params): Query<QuestionQuery>, State(state): State<AppState>) -> Result<Json<Vec<QuestionPublic>>, AppError>{
    let questions = question_service::get_questions(&state.db, params.limit).await?;

    let public: Vec<QuestionPublic> = questions.into_iter().map(QuestionPublic::from).collect();

    Ok(Json(public))
}

pub async fn get_questions_admin(Query(params): Query<QuestionQuery>, State(state): State<AppState>) -> Result<Json<Vec<QuestionAdmin>>, AppError>{
    let questions = question_service::get_questions(&state.db, params.limit).await?;

    let admin: Vec<QuestionAdmin> = questions.into_iter().map(QuestionAdmin::from).collect();

    Ok(Json(admin))
}

pub async fn get_question_by_id(State(state): State<AppState>, Path(id): Path<i32>) -> Result<Json<Question>, AppError>{
    let question = question_service::get_question_by_id(&state.db, id)
    .await?;

    Ok(Json(question))
}

pub async fn get_daily_questions(State(state): State<AppState>) -> Result<Json<Vec<DailyQuestion>>, AppError>{
    let daily_questions = question_service::get_daily_questions(&state.db).await?;

    Ok(Json(daily_questions))
}

pub async fn create_question( State(state): State<AppState>, Json(payload): Json<CreateQuestion>) -> Result<Json<Question>, AppError>{
    let question = question_service::create_question(&state.db, &payload.question_text, &payload.options, payload.answer_index, payload.platform_id, payload.content_type_id, &payload.difficulty, payload.challenge_date, payload.is_active).await?;

    Ok(Json(question))
}

pub async fn submit_quiz(State(state): State<AppState>, Json(payload): Json<QuizSubmission>) -> Result<Json<ScoreResponse>, AppError>{
    let score = question_service::get_answers(&state.db, &payload.answers).await?;

    Ok(Json(score))
}

pub async fn update_question( State(state): State<AppState>, Path(id): Path<i32>, Json(payload): Json<UpdateQuestion>) -> Result<Json<Question>, AppError>{
    let question = question_service::update_question(&state.db, id, &payload.question_text, &payload.options, payload.answer_index, payload.platform_id, payload.content_type_id, &payload.difficulty, payload.challenge_date, payload.is_active).await?;

    Ok(Json(question))
}

pub async fn delete_question(State(state): State<AppState>, Path(id): Path<i32>) -> Result<StatusCode, AppError>{
    let deleted = question_service::delete_question(&state.db, id).await?;

    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound("Question not found".to_string()))
    }
}

pub async fn toggle_question_status(State(state): State<AppState>, Path(id): Path<i32>) -> Result<Json<QuestionStatus>, AppError>{
    let question = question_service::toggle_active_status(&state.db, id).await?;

    Ok(Json(question))
}

pub async fn change_challenge_date(State(state): State<AppState>, Path(id): Path<i32>, Json(payload): Json<QuestionChallengeDateRequest>) -> Result<Json<QuestionChallengeDate>, AppError>{
    let question_challenge_date_response = question_service::change_question_challenge_date(&state.db, id, payload.challenge_date).await?;

    Ok(Json(question_challenge_date_response))
}
