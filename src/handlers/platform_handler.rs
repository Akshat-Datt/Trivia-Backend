use axum::{
    Json, extract::State
};
use crate::services::platform_service::get_platforms;
use crate::state::app_state::AppState;
use crate::{errors::errors::AppError, models::platforms_data::Platform};

pub async fn get_plaforms(State(state): State<AppState>) -> Result<Json<Vec<Platform>>, AppError>{
    let platforms = get_platforms(&state.db).await?;

    Ok(Json(platforms))
}