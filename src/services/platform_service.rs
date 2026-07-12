use sqlx::PgPool;

use crate::{errors::errors::AppError, models::platforms_data::Platform, repository::platform_repository};

pub async fn get_platforms(
    db:&PgPool
)-> Result<Vec<Platform>, AppError>{
    return platform_repository::get_platforms(db)
    .await
    .map_err(|_| AppError::DatabaseError);
}