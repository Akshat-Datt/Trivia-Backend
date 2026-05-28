use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct QuizSubmission{
    pub answers: HashMap<i32, i32>
}