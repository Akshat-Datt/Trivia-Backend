use crate::models::question_data::Question;
use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, Clone, FromRow)]
pub struct QuestionPublic{
    pub id: i32,
    pub question: String,
    pub options: Vec<String>
}

#[derive(Serialize, Clone, FromRow)]
pub struct QuestionAdmin{
    pub id: i32,
    pub question: String,
    pub options: Vec<String>,
    pub answer: i32
}

impl From<Question> for QuestionPublic {
    fn from(question:Question) -> Self {
        QuestionPublic{
            id: question.id,
            question: question.question,
            options: question.options
        }
    } 
}

impl From<Question> for QuestionAdmin {
    fn from(question: Question)-> Self {
        QuestionAdmin{
            id: question.id,
            question: question.question,
            options: question.options,
            answer: question.answer
        }
    }
}
