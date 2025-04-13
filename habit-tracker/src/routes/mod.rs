use axum::Router;
use std::sync::Arc;

use crate::AppState;
mod habit;

// Esta função será usada no main.rs pra montar o app
pub fn create_routes(state: Arc<AppState>) -> Router {
    habit::habit_routes(state)
}
