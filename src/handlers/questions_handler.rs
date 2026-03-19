use axum::{
    Json, extract::{Path, Query, State}, http::StatusCode
};
use crate::{models::question_data::{CreateQuestion, Question, QuestionQuery}};
use crate::state::app_state::AppState;
use crate::repository::question_repository;

pub async fn get_questions(Query(params): Query<QuestionQuery>, State(state): State<AppState>) -> Result<Json<Vec<Question>>, StatusCode>{

    
    let questions = question_repository::get_all_questions(&state.db, params.limit)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(questions))
}

pub async fn get_question_by_id(State(state): State<AppState>, Path(id): Path<i32>) -> Result<Json<Question>, StatusCode>{
    let question = question_repository::get_question_by_id(&state.db, id)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match question {
        Some(q) => Ok(Json(q)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn create_question( State(state): State<AppState>, Json(payload): Json<CreateQuestion>) -> Result<Json<Question>, StatusCode>{
    let question = question_repository::create_question(&state.db, &payload.question).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(question))
}