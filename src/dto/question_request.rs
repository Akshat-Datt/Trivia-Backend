use serde::Deserialize;
use chrono::NaiveDate;

#[derive(Deserialize)]
pub struct QuestionChallengeDateRequest{
    pub challenge_date: Option<NaiveDate>
}