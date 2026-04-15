


use sqlx::{PgPool};
use crate::models::question_data::Question;

pub async fn get_all_questions(
    db: &PgPool,
    limit: Option<usize>
) -> Result<Vec<Question>, sqlx::Error>{
    
    if let Some(limit) = limit{
        sqlx::query_as::<_, Question>(
            "SELECT id, question, options, answer FROM questions LIMIT $1"
        )
        .bind(limit as i64)
        .fetch_all(db)
        .await
    }
    else{
        sqlx::query_as::<_, Question>(
            "SELECT id, question, options, answer FROM questions"
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
    question: &str,
    options: &Vec<String>,
    answer: &i32
) -> Result<Question, sqlx::Error>{
    sqlx::query_as::<_, Question>(
        "INSERT INTO questions (question, options, answer) VALUES ($1, $2, $3) RETURNING id, question, options, answer"
    )
    .bind(question)
    .bind(options)
    .bind(answer)
    .fetch_one(db)
    .await
}

pub async fn question_duplicate_check(
    db: &PgPool,
    question: &str
) -> Result<bool, sqlx::Error>{
    let result = sqlx::query(
        "SELECT 1 FROM questions WHERE question = $1"
    )
    .bind(question)
    .fetch_optional(db)
    .await?;

    Ok(result.is_some())
}

pub async fn update_question(
    db: &PgPool,
    id: i32,
    question: &str,
    options: &Vec<String>,
    answer: &i32
) -> Result<Option<Question>, sqlx::Error>{
    sqlx::query_as::<_, Question>(
        "UPDATE questions SET question = $1, options = $3, answer =$4 WHERE id = $2 RETURNING id, question"
    )
    .bind(question)
    .bind(id)
    .bind(options)
    .bind(answer)
    .fetch_optional(db)
    .await
}

pub async fn delete_question(
    db: &PgPool,
    id: i32
) -> Result<bool, sqlx::Error>{
    let result = sqlx::query(
        "DELETE FROM questions WHERE id = $1"
    )
    .bind(id)
    .execute(db)
    .await?;

    Ok(result.rows_affected() > 0)
}


