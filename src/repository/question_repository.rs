use std::{collections::HashMap};
use sqlx::{PgPool};
use chrono::{NaiveDate};
use crate::models::question_data::{Question, QuestionAnswer, QuestionMaxOptions};

pub async fn get_all_questions(
    db: &PgPool,
    limit: Option<usize>
) -> Result<Vec<Question>, sqlx::Error>{
    
    if let Some(limit) = limit{
        sqlx::query_as::<_, Question>(
            "SELECT id, question_text, options, answer_index, platform_id, content_type_id, difficulty, challenge_date, is_active, created_at, updated_at FROM question_bank LIMIT $1"
        )
        .bind(limit as i64)
        .fetch_all(db)
        .await
    }
    else{
        sqlx::query_as::<_, Question>(
            "SELECT id, question_text, options, answer_index, platform_id, content_type_id, difficulty, challenge_date, is_active, created_at, updated_at FROM question_bank"
        )
        .fetch_all(db)
        .await
    }
}

pub async fn get_question_by_id(
    db: &PgPool,
    id: i32
) -> Result<Option<Question>, sqlx::Error>{
    sqlx::query_as::<_, Question>(
        "SELECT id, question, options, answer FROM questions WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(db)
    .await
}

pub async fn create_question(
    db: &PgPool,
    question_text: &str,
    options: &Vec<String>,
    answer_index: &i32,
    platform_id: &i32,
    content_type_id: &i32,
    difficulty: &str,
    challenge_date: Option<NaiveDate>,
    is_active: &bool
) -> Result<Question, sqlx::Error>{
    sqlx::query_as::<_, Question>(
        "INSERT INTO question_bank (question_text, options, answer_index, platform_id, content_type_id, difficulty, challenge_date, is_active) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING id, question_text, options, answer_index, platform_id, content_type_id, difficulty, challenge_date, is_active, created_at, updated_at"
    )
    .bind(question_text)
    .bind(options)
    .bind(answer_index)
    .bind(platform_id)
    .bind(content_type_id)
    .bind(difficulty)
    .bind(challenge_date)
    .bind(is_active)
    .fetch_one(db)
    .await
}

pub async fn get_answers(
    db: &PgPool,
)->Result<HashMap<i32, i32>, sqlx::Error>{
    let rows = sqlx::query_as::<_, QuestionAnswer>(
        "SELECT id, answer FROM questions"
    )
    .fetch_all(db)
    .await?;

    let mut answers_map = HashMap::new();

    for row in rows{
        answers_map.insert(row.id, row.answer);
    }

    Ok(answers_map)
}

pub async fn questions_count(
    db: &PgPool
) -> Result<i64, sqlx::Error> {
    let result = sqlx::query_scalar(
        "SELECT COUNT(*) FROM questions"
    ).fetch_one(db).await?;

    Ok(result)
}

pub async fn validation_questions(
    db: &PgPool,
    question_ids: &Vec<i32>
) -> Result<bool, sqlx::Error>{
    let result = sqlx::query_scalar::<_, i32>(
        "SELECT id FROM questions WHERE id = ANY($1)"
    ).bind(question_ids)
    .fetch_all(db)
    .await?;

    // if questions length and fetched length are same, then all question ids are valid

    Ok(result.len() == question_ids.len())
}

pub async fn each_question_options_count(
    db: &PgPool
) -> Result<HashMap<i32, i32>, sqlx::Error>{
    let rows = sqlx::query_as::<_, QuestionMaxOptions>(
        "SELECT id, array_length(options, 1) as options_len
FROM questions"
    )
    .fetch_all(db)
    .await?;

    let mut options_count_map = HashMap::new();

    for row in rows{
        options_count_map.insert(row.id, row.options_len);
    }

    Ok(options_count_map)
}

pub async fn question_duplicate_check(
    db: &PgPool,
    question: &str
) -> Result<bool, sqlx::Error>{
    let result = sqlx::query(
        "SELECT 1 FROM questions WHERE LOWER(question) = LOWER($1)"
    )
    .bind(question)
    .fetch_optional(db)
    .await?;

    Ok(result.is_some())
}

pub async fn update_question(
    db: &PgPool,
    id: i32,
    question_text: &str,
    options: &Vec<String>,
    answer_index: &i32,
    platform_id: &i32,
    content_type_id: &i32,
    difficulty: &str,
    challenge_date: Option<NaiveDate>,
    is_active: &bool
) -> Result<Option<Question>, sqlx::Error>{
    sqlx::query_as::<_, Question>(
        "UPDATE question_bank SET question_text = $1, options = $3, answer_index =$4, platform_id =$5, content_type_id =$6, difficulty =$7, challenge_date =$8, is_active =$9 WHERE id = $2 RETURNING id, question_text, options, answer_index, platform_id, content_type_id, difficulty, challenge_date, is_active, created_at, updated_at"
    )
    .bind(question_text)
    .bind(id)
    .bind(options)
    .bind(answer_index)
    .bind(platform_id)
    .bind(content_type_id)
    .bind(difficulty)
    .bind(challenge_date)
    .bind(is_active)
    .fetch_optional(db)
    .await
}

pub async fn delete_question(
    db: &PgPool,
    id: i32
) -> Result<bool, sqlx::Error>{
    let result = sqlx::query(
        "DELETE FROM question_bank WHERE id = $1"
    )
    .bind(id)
    .execute(db)
    .await?;

    Ok(result.rows_affected() > 0)
}


