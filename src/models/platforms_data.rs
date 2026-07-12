use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(Serialize, Clone, FromRow)]
pub struct Platform{
    pub id: i32,
    pub platform_name: String,
}