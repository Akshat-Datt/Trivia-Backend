use axum::{
    Json, extract::{Path, Query, State}, http::StatusCode
};
use sqlx::Row;
use crate::{models::question_data::{CreateQuestion, Question, QuestionQuery}};
use crate::state::app_state::AppState;

pub async fn get_questions(Query(params): Query<QuestionQuery>, State(state): State<AppState>) -> Result<Json<Vec<Question>>, StatusCode>{

    
    let questions = if let Some(limit) = params.limit{
    sqlx::query_as::<_, Question>("SELECT id, question FROM questions LIMIT $1")
        .bind(limit as i64)
        .fetch_all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    }
    
    else{
    sqlx::query_as::<_, Question>("SELECT id, question FROM questions")
        .fetch_all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    };

    Ok(Json(questions))
}

pub async fn get_question_by_id(State(state): State<AppState>, Path(id): Path<i32>) -> Result<Json<Question>, StatusCode>{
    let question = sqlx::query_as::<_, Question>(
        "SELECT id, question FROM questions WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match question {
        Some(q) => Ok(Json(q)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn create_question( State(state): State<AppState>, Json(payload): Json<CreateQuestion>) -> Result<Json<Question>, StatusCode>{
    let row = sqlx::query(
        "INSERT INTO questions (question) VALUES ($1) RETURNING id, question"
    )
    .bind(&payload.question)
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let question = Question {
        id: row.get("id"),
        question: row.get("question"),
    };

    Ok(Json(question))
}