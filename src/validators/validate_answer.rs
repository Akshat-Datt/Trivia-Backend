use sqlx::PgPool;

use crate::errors::errors::AppError;
use crate::repository::question_repository;

pub async fn validate_answer_data(
    db: &PgPool,
    fetched_map: &std::collections::HashMap<i32, i32>,
    questions_count: i64
) -> Result<(), AppError>{
    let question_ids: Vec<i32> = fetched_map.keys().cloned().collect();

    if fetched_map.is_empty(){
        return Err(AppError::ValidationError("Submission cannot be empty".to_string()));
    }

    if fetched_map.len() != questions_count as usize{
        return Err(AppError::ValidationError("Submission must contain answers for all questions".to_string()));
    }

    if question_repository::validation_questions(db, &question_ids).await.map_err(|_| AppError::DatabaseError)? == false{
        return Err(AppError::ValidationError("Submission contains invalid question IDs".to_string()));
    }

    let options_len_map = question_repository::each_question_options_count(db).await.map_err(|_| AppError::DatabaseError)?;

    for (key,value) in fetched_map.iter(){
        if *value != -1 && (*value < 0 || *value >= *options_len_map.get(key).unwrap_or(&0)){
            return Err(AppError::ValidationError(format!("Answer index for question ID {} is out of bounds", key)));
        }
    }

    Ok(())
}