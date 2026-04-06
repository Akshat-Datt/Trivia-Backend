use sqlx::{PgPool};
use crate::{
    errors::errors::AppError, models::question_data::Question, repository::question_repository
};

pub async fn get_questions(
    db: &PgPool,
    limit: Option<usize>
) -> Result<Vec<Question>, AppError>{
    return question_repository::get_all_questions(db, limit)
    .await
    .map_err(|_| AppError::DatabaseError);
}

pub async fn get_question_by_id(
    db: &PgPool,
    id: i32
) -> Result<Question, AppError>{

    if id <= 0 {
        println!("{:?}", AppError::ValidationError("ID must be greater than 0".to_string()));

        return Err(AppError::ValidationError("ID must be greater than 0".to_string()));
    }

    let question =  question_repository::get_question_by_id(db, id).await.map_err(|_| AppError::DatabaseError)?;

    match question {
        Some(q) => Ok(q),
        None => Err(AppError::NotFound("Question not found".to_string()))
    }
}

pub async fn create_question(
    db: &PgPool,
    question: &str,
    options: &Vec<String>,
    answer: i32
) -> Result<Question, AppError>{
    let question = question.trim();

    if question.is_empty(){
        println!("{:?}", AppError::ValidationError("Question cannot be empty".to_string()));

        return Err(AppError::ValidationError("Question cannot be empty".to_string()));
    }

    if answer < 0 || answer >= options.len() as i32{
        return Err(AppError::ValidationError("Answer index is out of bounds".to_string()));
    }

    if options.len() < 4 || options.len() > 4 {
        return Err(AppError::ValidationError("Four options are required".to_string()));
    }

    for option_item in options{
        if option_item.trim().is_empty(){
            return Err(AppError::ValidationError("Options cannot be empty".to_string()));
        }
    }

    return question_repository::create_question(db, question, options, &answer)
    .await
    .map_err(|_| AppError::DatabaseError);
}

pub async fn update_question(
    db: &PgPool,
    id: i32,
    question: &str,
    options: &Vec<String>,
    answer: i32
) -> Result<Question, AppError>{
    let question = question.trim();

    if id <= 0 {
        println!("{:?}", AppError::ValidationError("ID must be greater than 0".to_string()));

        return Err(AppError::ValidationError("ID must be greater than 0".to_string()));
    }

    if question.is_empty(){
        println!("{:?}", AppError::ValidationError("Question cannot be empty".to_string()));

        return Err(AppError::ValidationError("Question cannot be empty".to_string()));

    }

    if answer < 0 || answer >= options.len() as i32{
        return Err(AppError::ValidationError("Answer index is out of bounds".to_string()));
    }

    if options.len() < 4 || options.len() > 4 {
        return Err(AppError::ValidationError("Four options are required".to_string()));
    }

    for option_item in options{
        if option_item.trim().is_empty(){
            return Err(AppError::ValidationError("Options cannot be empty".to_string()));
        }
    }

    let question = question_repository::update_question(db, id, question, options, &answer).await.map_err(|_| AppError::DatabaseError)?;

    match question {
        Some(q) => Ok(q),
        None => Err(AppError::NotFound("Question not found".to_string()))
    }
}

pub async fn delete_question(
    db: &PgPool,
    id: i32
) -> Result<bool, AppError>{
    if id <= 0 {
        println!("{:?}", AppError::ValidationError("ID must be greater than 0".to_string()));

        return Err(AppError::ValidationError("ID must be greater than 0".to_string()));
    }

   return question_repository::delete_question(db, id).await.map_err(|_| AppError::DatabaseError);
}