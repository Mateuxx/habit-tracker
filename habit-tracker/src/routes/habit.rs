use axum::{ routing::{ get, patch, post, put }, Json, Router };
use serde::Serialize;
use std::sync::Arc;

use crate::{
    handlers::habit_handler::{ create_habit, list_habits, update_habit },
    AppState,
};

//just for test
#[derive(Serialize)]
struct PingResponse {
    ok: bool,
}

//Ping Handler
async fn ping_handler() -> Json<PingResponse> {
    Json(PingResponse { ok: true })
}
//routes for all the habits
pub fn habit_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/habits", post(create_habit).get(list_habits))
        .route("/ping", get(ping_handler))
        .route("/habits/{id}", patch(update_habit))
        .with_state(state)
}
