use serde::Deserialize;
use chrono::NaiveDate;

#[derive(Deserialize)]
pub struct QuestionChallengeDateRequest{
    pub challenge_date: Option<NaiveDate>
}

#[derive(Deserialize)]
pub struct EndlessQuestionPlatfromIdRequest{
    pub platform_id: i32
}