use axum::{
    Json, extract::{Path, Query, State}, http::StatusCode
};
use crate::{errors::errors::AppError, models::question_data::{CreateQuestion, Question, QuestionQuery, UpdateQuestion}};
use crate::state::app_state::AppState;
use crate::services::question_service;

pub async fn get_questions(Query(params): Query<QuestionQuery>, State(state): State<AppState>) -> Result<Json<Vec<Question>>, AppError>{
    let questions = question_service::get_questions(&state.db, params.limit).await?;

    Ok(Json(questions))
}

pub async fn get_question_by_id(State(state): State<AppState>, Path(id): Path<i32>) -> Result<Json<Question>, AppError>{
    let question = question_service::get_question_by_id(&state.db, id)
    .await?;

    Ok(Json(question))
}

pub async fn create_question( State(state): State<AppState>, Json(payload): Json<CreateQuestion>) -> Result<Json<Question>, AppError>{
    let question = question_service::create_question(&state.db, &payload.question).await?;

    Ok(Json(question))
}

pub async fn update_question( State(state): State<AppState>, Path(id): Path<i32>, Json(payload): Json<UpdateQuestion>) -> Result<Json<Question>, AppError>{
    let question = question_service::update_question(&state.db, id, &payload.question).await?;

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