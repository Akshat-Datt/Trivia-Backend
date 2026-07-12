use axum::{
    Router,
    routing::get
};

use crate::{handlers::platform_handler::get_plaforms, state::app_state::AppState};

pub fn platfrom_routes() -> Router<AppState>{
    Router::new()
    .route("/platforms", get(get_plaforms))
}