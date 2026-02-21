use axum::{
    Json, extract::Path, http::StatusCode
};
use crate::models::question_data::Question;

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

pub async fn get_questions() -> Json<Vec<Question>>{
    let questions = sample_questions();
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