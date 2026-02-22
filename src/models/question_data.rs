use serde::{Serialize, Deserialize};

#[derive(Serialize, Clone)]
pub struct Question {
    pub id: u32,
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