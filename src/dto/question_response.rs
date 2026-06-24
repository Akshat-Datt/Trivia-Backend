use crate::models::question_data::Question;
use serde::Serialize;
use sqlx::FromRow;
use chrono::{DateTime, Utc, NaiveDate};

#[derive(Serialize, Clone, FromRow)]
pub struct QuestionPublic{
    pub id: i32,
    pub question_text: String,
    pub options: Vec<String>,
    pub platform_id: i32,
    pub content_type_id: i32,

    pub difficulty: String,

    pub challenge_date: Option<NaiveDate>,
}

#[derive(Serialize, Clone, FromRow)]
pub struct QuestionAdmin{
    pub id: i32,
    pub question: String,
    pub options: Vec<String>,
    pub answer: i32,

    pub platform_id: i32,
    pub content_type_id: i32,

    pub difficulty: String,

    pub challenge_date: Option<NaiveDate>,

    pub is_active: bool,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Clone, FromRow)]
pub struct QuestionStatus{
    pub id: i32,
    pub question_text: String,
    pub is_active: bool
}

#[derive(Serialize, Clone, FromRow)]
pub struct QuestionChallengeDate{
    pub id: i32,
    pub question_text: String,
    pub challenge_date: Option<NaiveDate>
}

impl From<Question> for QuestionPublic {
    fn from(question:Question) -> Self {
        QuestionPublic{
            id: question.id,
            question_text: question.question_text,
            options: question.options,
            platform_id: question.platform_id,
            content_type_id: question.content_type_id,
            difficulty: question.difficulty,
            challenge_date: question.challenge_date
        }
    } 
}

impl From<Question> for QuestionAdmin {
    fn from(question: Question)-> Self {
        QuestionAdmin{
            id: question.id,
            question: question.question_text,
            options: question.options,
            answer: question.answer_index,
            platform_id: question.platform_id,
            content_type_id: question.content_type_id,
            difficulty: question.difficulty,
            challenge_date: question.challenge_date,
            is_active: question.is_active,
            created_at: question.created_at,
            updated_at: question.updated_at
        }
    }
}
