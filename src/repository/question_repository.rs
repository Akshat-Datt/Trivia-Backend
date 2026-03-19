use sqlx::{PgPool, Row};
use crate::models::question_data::Question;

pub async fn get_all_questions(
    db: &PgPool,
    limit: Option<usize>
) -> Result<Vec<Question>, sqlx::Error>{
    
    if let Some(limit) = limit{
        sqlx::query_as::<_, Question>(
            "SELECT id, question FROM questions LIMIT $1"
        )
        .bind(limit as i64)
        .fetch_all(db)
        .await
    }
    else{
        sqlx::query_as::<_, Question>(
            "SELECT id, question FROM questions"
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
        "SELECT id, question FROM questions WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(db)
    .await
}

pub async fn create_question(
    db: &PgPool,
    question: &str
) -> Result<Question, sqlx::Error>{
    sqlx::query_as::<_, Question>(
        "INSERT INTO questions (question) VALUES ($1) RETURNING id, question"
    )
    .bind(question)
    .fetch_one(db)
    .await
}

pub async fn update_question(
    db: &PgPool,
    id: i32,
    question: &str
) -> Result<Option<Question>, sqlx::Error>{
    sqlx::query_as::<_, Question>(
        "UPDATE questions SET question = $1 WHERE id = $2 RETURNING id, question"
    )
    .bind(question)
    .bind(id)
    .fetch_optional(db)
    .await
}


