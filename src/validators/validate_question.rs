use chrono::Local;
use sqlx::PgPool;
use crate::{repository::question_repository, errors::errors::AppError};

const VALID_DIFFICULTIES: [&str; 3] = [ "Easy", "Medium", "Hard" ];

pub async fn validate_question_data(
    db: &PgPool,
    question: &str,
    options: &Vec<String>,
    answer: i32,
    platform_id: i32,
    content_type_id: i32,
    difficulty: &str,
    challenge_date: &Option<chrono::NaiveDate>
)-> Result<(), AppError>{
    if question.is_empty(){
        return Err(AppError::ValidationError("Question cannot be empty".to_string()));

    }

    if answer < 0 || answer >= options.len() as i32{
        return Err(AppError::ValidationError("Answer index is out of bounds".to_string()));
    }

    if options.len() < 4 || options.len() > 4 {
        return Err(AppError::ValidationError("Four options are required".to_string()));
    }

    if check_platform_id_exists(db, &platform_id).await? == false{
        return Err(AppError::NotFound("Platform ID not found it must be within bounds".to_string()));
    }

    if check_content_type_if_exists(db, &content_type_id).await? == false{
        return Err(AppError::NotFound("Content Type ID not found it must be within bounds".to_string()));
    }

    for option_item in options{
        if option_item.trim().is_empty(){
            return Err(AppError::ValidationError("Options cannot be empty".to_string()));
        }
    }

    validate_difficulty(&difficulty)?;

    if challenge_date.is_some(){
        let today = Local::now().date_naive();
        
        if challenge_date.unwrap() < today{
            return Err(AppError::ValidationError("Challenge date cannot be in the past".to_string()));
        }
    }

    Ok(())
}

async fn check_platform_id_exists(
    db: &PgPool,
    platform_id: &i32
) -> Result<bool, AppError>{
    return question_repository::platform_id_exists(db, platform_id)
    .await
    .map_err(|_| AppError::NotFound("Platform ID not found".to_string()));
}

async fn check_content_type_if_exists(
    db: &PgPool,
    content_type_id: &i32
) -> Result<bool, AppError>{
    return question_repository::content_type_id_exists(db, content_type_id)
    .await
    .map_err(|_| AppError::NotFound("Content Type ID not found".to_string()));
}

fn validate_difficulty(
    difficulty: &str
) -> Result<(), AppError>{
    if difficulty.is_empty(){
        return Err(AppError::ValidationError("Difficulty cannot be empty".to_string()));
    }

    else if !VALID_DIFFICULTIES.contains(&difficulty) {
        return Err(AppError::ValidationError(
            format!(
                "Difficulty must be one of: {:?}",
                VALID_DIFFICULTIES
            )
        ));
    }

    Ok(())
}