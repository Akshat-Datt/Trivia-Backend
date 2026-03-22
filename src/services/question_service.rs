use sqlx::PgPool;
use crate::{
    models::question_data::Question,
    repository::question_repository
};

pub async fn get_questions(
    db: &PgPool,
    limit: Option<usize>
) -> Result<Vec<Question>, sqlx::Error>{
    return question_repository::get_all_questions(db, limit).await;
}

pub async fn get_question_by_id(
    db: &PgPool,
    id: i32
) -> Result<Option<Question>, sqlx::Error>{
    return question_repository::get_question_by_id(db, id).await;
}

pub async fn create_question(
    db: &PgPool,
    question: &str
) -> Result<Question, sqlx::Error>{
    return question_repository::create_question(db, question).await;
}

pub async fn update_question(
    db: &PgPool,
    id: i32,
    question: &str
) -> Result<Option<Question>, sqlx::Error>{
    return question_repository::update_question(db, id, question).await;
}

pub async fn delete_question(
    db: &PgPool,
    id: i32
) -> Result<bool, sqlx::Error>{
    return question_repository::delete_question(db, id).await;
}