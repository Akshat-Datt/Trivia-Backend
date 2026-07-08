use sqlx::{PgPool};
use crate::{
    constants::quiz_constants::DAILY_QUIZ_QUESTION_COUNT, dto::{question_response::{QuestionChallengeDate, QuestionStatus}, score_response::ScoreResponse}, errors::errors::AppError, models::question_data::{DailyQuestion, Question}, repository::question_repository::{self}, validators::{validate_answer, validate_question}
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

pub async fn get_daily_questions(
    db: &PgPool
) -> Result<Vec<DailyQuestion>, AppError>{
    let daily_questions = question_repository::get_daily_questions(db).await.map_err(|_| AppError::DatabaseError)?;

    if daily_questions.is_empty() {
        return Err(AppError::NotFound("No daily questions found".to_string()));
    }

    if daily_questions.len() < DAILY_QUIZ_QUESTION_COUNT || daily_questions.len() > DAILY_QUIZ_QUESTION_COUNT {
        return Err(AppError::ValidationError(format!("Daily questions count must be exactly {}. Found: {}", DAILY_QUIZ_QUESTION_COUNT, daily_questions.len())));
    }

    return Ok(daily_questions);
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

    validate_question::validate_question_data(db, question, options, answer, platform_id, content_type_id, difficulty, &challenge_date).await?;

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

    validate_question::validate_question_data(db, question, options, answer, platform_id, content_type_id, difficulty, &challenge_date).await?;

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
    let questions_count = question_repository::questions_count(db).await.map_err(|_| AppError::DatabaseError)?;

    validate_answer::validate_answer_data(db, fetched_map, questions_count).await?;
    
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

pub async fn toggle_active_status(
    db: &PgPool,
    id: i32
) -> Result<QuestionStatus, AppError>{
    if id <= 0 {
        return Err(AppError::ValidationError("ID must be greater than 0".to_string()));
    }

    if validate_question::question_id_exists(db, id).await? == false{
        return Err(AppError::NotFound("Question ID not found".to_string()));
    }

    let question_status = validate_question::is_active_status(db, id).await?;

    let new_status = !question_status;

    let question = question_repository::change_question_active_status(db, id, new_status).await.map_err(|_| AppError::DatabaseError)?;

    Ok(question)
}

pub async fn change_question_challenge_date(
    db: &PgPool,
    id: i32,
    challenge_date: Option<chrono::NaiveDate>
) -> Result<QuestionChallengeDate, AppError>{
    if id <= 0 {
        return Err(AppError::ValidationError("ID must be greater than 0".to_string()));
    }

    if validate_question::question_id_exists(db, id).await? == false{
        return Err(AppError::NotFound("Question ID not found".to_string()));
    }

    if challenge_date.is_none() {
        return Err(AppError::ValidationError("Challenge date cannot be nothing".to_string()));
    }

    let question_status = validate_question::is_active_status(db, id).await?;

    if question_status == false{
        return Err(AppError::ValidationError("Cannot change challenge date of an inactive question".to_string()));
    }

    let question_challenge_date = question_repository::get_question_challenge_date(db, id).await.map_err(|_| AppError::DatabaseError)?;

    validate_question::challenge_date_staleness(&question_challenge_date).await?;

    validate_question::challenge_date_staleness(&challenge_date).await?;

    let question_challenge_response = question_repository::change_question_challenge_date(db, id, challenge_date).await.map_err(|_| AppError::DatabaseError)?;

    Ok(question_challenge_response)
}
