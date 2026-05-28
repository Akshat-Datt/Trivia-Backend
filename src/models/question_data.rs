use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Clone, FromRow)]
pub struct Question {
    pub id: i32,
    pub question: String,
    pub options: Vec<String>,
    pub answer: i32
}

#[derive(Deserialize)]
pub struct QuestionQuery{
    pub limit: Option<usize>
}

#[derive(Deserialize)]
pub struct CreateQuestion{
    pub question:String,
    pub options: Vec<String>,
    pub answer: i32
}

#[derive(Deserialize)]
pub struct UpdateQuestion{
    pub question:String,
    pub options: Vec<String>,
    pub answer: i32
}

#[derive(Serialize)]
pub struct Score{
    pub score: i32
}

#[derive(FromRow)]
pub struct QuestionAnswer{
    pub id: i32,
    pub answer: i32
}

#[derive(FromRow)]
pub struct QuestionMaxOptions{
    pub id: i32,
    pub options_len: i32
}