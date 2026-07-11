use serde::Serialize;
use sqlx::FromRow;
use chrono::{DateTime, Utc, NaiveDate};

#[derive(Serialize, Clone, FromRow)]
pub struct QuestionPublic{
    pub id: i32,
    pub question_text: String,
    pub options: Vec<String>,
    pub platform_name: String,
    pub content_type_name: String,

    pub difficulty: String,

    pub challenge_date: Option<NaiveDate>,
}

#[derive(Serialize, Clone, FromRow)]
pub struct QuestionAdmin{
    pub id: i32,
    pub question_text: String,
    pub options: Vec<String>,
    pub answer_index: i32,

    pub platform_name: String,
    pub content_type_name: String,

    pub difficulty: String,

    pub challenge_date: Option<NaiveDate>,

    pub is_active: bool,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Clone, FromRow)]
pub struct DailyQuestion {
    pub id: i32,

    pub question_text: String,
    pub options: Vec<String>,

    pub platform_name: String,
    pub content_type_name: String,

    pub difficulty: String,
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
