use sqlx::PgPool;
use crate::models::question_data::Question;

pub async fn get_all_questions(
    db: &PgPool,
    limit: Qption<usize>
) -> Result<Vec<Question>, sqlx::error>{
    
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