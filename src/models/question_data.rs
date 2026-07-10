use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc, NaiveDate};


#[derive(Serialize, Clone, FromRow)]
pub struct Question {
    pub id: i32,

    pub question_text: String,
    pub options: Vec<String>,
    pub answer_index: i32,

    pub platform_id: i32,
    pub content_type_id: i32,

    pub difficulty: String,

    pub challenge_date: Option<NaiveDate>,

    pub is_active: bool,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct QuestionQuery{
    pub limit: Option<usize>
}

#[derive(Deserialize)]
pub struct CreateQuestion{
    pub question_text:String,
    pub options: Vec<String>,
    pub answer_index: i32,
    pub platform_id: i32,
    pub content_type_id: i32,

    pub difficulty: String,

    pub challenge_date: Option<NaiveDate>,

    pub is_active: bool,
}

#[derive(Deserialize)]
pub struct UpdateQuestion{
    pub question_text:String,
    pub options: Vec<String>,
    pub answer_index: i32,
    pub platform_id: i32,
    pub content_type_id: i32,

    pub difficulty: String,

    pub challenge_date: Option<NaiveDate>,

    pub is_active: bool,
}

#[derive(FromRow)]
pub struct QuestionAnswer{
    pub id: i32,
    pub answer_index: i32
}

#[derive(FromRow)]
pub struct QuestionMaxOptions{
    pub id: i32,
    pub options_len: i32
}