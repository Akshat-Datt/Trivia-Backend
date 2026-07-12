use sqlx::PgPool;

use crate::models::platforms_data::Platform;

pub async fn get_platforms(
    db: &PgPool
) -> Result<Vec<Platform>, sqlx::Error>{
    sqlx::query_as::<_, Platform>(
        "SELECT id, platform_name FROM platforms"
    )
    .fetch_all(db)
    .await
}