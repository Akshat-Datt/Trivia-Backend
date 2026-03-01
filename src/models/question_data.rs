use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Clone, FromRow)]
pub struct Question {
    pub id: i32,
    pub question: String,
}

#[derive(Deserialize)]
pub struct QuestionQuery{
    pub limit: Option<usize>
}

#[derive(Deserialize)]
pub struct CreateQuestion{
    pub question:String
}