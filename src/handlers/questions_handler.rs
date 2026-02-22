use axum::{
    Json, extract::{Path, Query}, http::StatusCode
};
use crate::models::question_data::{Question, QuestionQuery, CreateQuestion};

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

pub async fn get_questions(Query(params): Query<QuestionQuery>) -> Json<Vec<Question>>{
    let mut questions = sample_questions();

    if let Some(limit) = params.limit{
        questions.truncate(limit);
    }

    Json(questions)
}

pub async fn get_question_by_id(Path(id): Path<u32>) -> Result<Json<Question>, StatusCode>{
    let questions = sample_questions();

    let question = questions.into_iter().find(|q| q.id == id);

    match question{
        Some(q) => Ok(Json(q)),
        None => Err(StatusCode::NOT_FOUND)
    }
}

pub async fn create_question(Json(payload): Json<CreateQuestion>) -> (StatusCode, Json<Question>){
    let new_question = Question{
        id: 999,
        question: payload.question
    };

    (StatusCode::CREATED, Json(new_question))
}