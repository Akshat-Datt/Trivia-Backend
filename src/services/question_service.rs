use sqlx::{PgPool};
use chrono::{Local};
use crate::{
    dto::score_response::{ScoreResponse}, errors::errors::AppError, models::question_data::Question, repository::question_repository::{self}
};

const VALID_DIFFICULTIES: [&str; 3] = [ "Easy", "Medium", "Hard" ];

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
    answer: i32,
    platform_id: i32,
    content_type_id: i32,
    difficulty: &str,
    challenge_date: Option<chrono::NaiveDate>,
    is_active: bool
) -> Result<Question, AppError>{
    let question = question.trim();

    validate_question_data(db, question, options, &answer, &platform_id, &content_type_id, difficulty, &challenge_date).await?;

    if question_repository::question_duplicate_check(db, question).await.map_err(|_| AppError::DatabaseError)?{
        return Err(AppError::Conflict("Question already exists".to_string()));
    }

    return question_repository::create_question(db, question, options, &answer, &platform_id, &content_type_id, difficulty, challenge_date, &is_active)
    .await
    .map_err(|_| AppError::DatabaseError);
}

pub async fn update_question(
    db: &PgPool,
    id: i32,
    question: &str,
    options: &Vec<String>,
    answer: i32,
    platform_id: i32,
    content_type_id: i32,
    difficulty: &str,
    challenge_date: Option<chrono::NaiveDate>,
    is_active: bool
) -> Result<Question, AppError>{
    let question = question.trim();

    if id <= 0 {
        return Err(AppError::ValidationError("ID must be greater than 0".to_string()));
    }

    validate_question_data(db, question, options, &answer, &platform_id, &content_type_id, difficulty, &challenge_date).await?;

    let question = question_repository::update_question(db, id, question, options, &answer, &platform_id, &content_type_id, difficulty, challenge_date, &is_active).await.map_err(|_| AppError::DatabaseError)?;

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
        return Err(AppError::ValidationError("ID must be greater than 0".to_string()));
    }

   return question_repository::delete_question(db, id).await.map_err(|_| AppError::DatabaseError);
}

pub async fn get_answers(
    db: &PgPool,
    fetched_map: &std::collections::HashMap<i32, i32>
) -> Result<ScoreResponse, AppError>{

    let question_ids: Vec<i32> = fetched_map.keys().cloned().collect();

    let questions_count = question_repository::questions_count(db).await.map_err(|_| AppError::DatabaseError)?;

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
    
    let answers_map = question_repository::get_answers(db)
    .await
    .map_err(|_| AppError::DatabaseError)?;

    let mut score = 0;

    for(key, value) in answers_map.iter(){
        if fetched_map.get(key) == Some(value){
            score += 1;
        }
    }

    let accuracy = (score as f64 / answers_map.len() as f64) * 100.0;

    let score_response = ScoreResponse{
        score: score,
        total_questions: questions_count as i32,
        accuracy: accuracy as f32
    };

    return Ok(score_response);
}

async fn validate_question_data(
    db: &PgPool,
    question: &str,
    options: &Vec<String>,
    answer: &i32,
    platform_id: &i32,
    content_type_id: &i32,
    difficulty: &str,
    challenge_date: &Option<chrono::NaiveDate>
)-> Result<(), AppError>{
    if question.is_empty(){
        return Err(AppError::ValidationError("Question cannot be empty".to_string()));

    }

    if *answer < 0 || *answer >= options.len() as i32{
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
        let today = Local::now().format("%Y-%m-%d").to_string();
        
        if challenge_date.unwrap().to_string() < today{
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