use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Question {
    pub id: u32,
    pub question: String,
}