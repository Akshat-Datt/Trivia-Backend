use serde::Deserialize;
use chrono::NaiveDate;

#[derive(Deserialize)]
pub struct QuestionChallengeDateRequest{
    pub challenge_date: Option<NaiveDate>
}

#[derive(Deserialize)]
pub struct EndlessQuestionRequest{
    pub platform_id: i32,
    pub page: Option<u32>,
    pub limit: Option<u32>
}