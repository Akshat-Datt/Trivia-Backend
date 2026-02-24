use axum::{
    Json, extract::{Path, Query, State}, http::StatusCode
};
use crate::{models::question_data::{CreateQuestion, Question, QuestionQuery}};
use crate::state::app_state::AppState;

pub fn sample_questions() -> Vec<Question>{
    vec![
        Question{
            id: 1,
            question: "What is Rust".to_owned()
        },
        Question{
            id: 2,
            question: "waht is Android ".to_owned()
        }
    ]
}

pub async fn get_questions(Query(params): Query<QuestionQuery>, State(state): State<AppState>) -> Json<Vec<Question>>{
    let mut questions = state.questions.lock().await;

    if let Some(limit) = params.limit{
        questions.truncate(limit);
    }

    Json(questions.clone())
}

pub async fn get_question_by_id(Path(id): Path<u32>) -> Result<Json<Question>, StatusCode>{
    let questions = sample_questions();

    let question = questions.into_iter().find(|q| q.id == id);

    match question{
        Some(q) => Ok(Json(q)),
        None => Err(StatusCode::NOT_FOUND)
    }
}

pub async fn create_question( State(state): State<AppState>, Json(payload): Json<CreateQuestion>) -> (StatusCode, Json<Question>){
    let mut questions = state.questions.lock().await;

    let new_id = questions.len() as u32 + 1;   

    let new_question = Question{
        id: new_id,
        question: payload.question
    };

    questions.push(new_question.clone());

    (StatusCode::CREATED, Json(new_question))
}