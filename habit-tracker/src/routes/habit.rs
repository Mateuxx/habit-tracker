use axum::{ routing::{ get, put }, Json, Router };
use serde::Serialize;
use std::sync::Arc;

use crate::{
    handlers::habit_handler::{ create_habit, list_habits, update_habit },
    models::habit::Habit,
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
//this is just a test handler
async fn habit_handler() -> Json<Habit> {
    Json(Habit { id: 1, title: "beber agua".to_string(), completed: false })
}

//routes for all the habits 
pub fn habit_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/habits", get(list_habits).post(create_habit))
        .route("/ping", get(ping_handler))
        .route("/habit-test", get(habit_handler))
        .route("/habits/{id}", put(update_habit))
        .with_state(state)
}
