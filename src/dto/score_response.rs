use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct ScoreResponse {
    pub score: i32,
    pub total_questions: i32,
    pub accuracy: f32
}