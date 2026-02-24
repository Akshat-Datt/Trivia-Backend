use std::sync::Arc;
use tokio::sync::Mutex;

use crate::models::question_data::Question;


#[derive(Clone)]
pub struct AppState{
    pub questions: Arc<Mutex<Vec<Question>>>
}
