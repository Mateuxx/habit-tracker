mod handlers;
mod models;

//axum http framework dependencies
//use its like the import for rust
use axum::{ routing::get, Json, Router };
use axum::routing::post;
use serde::Serialize;
use std::sync::Mutex;
use std::{ net::SocketAddr, sync::Arc };
use crate::handlers::habit_handler::{ list_habits, create_habit };
use crate::models::habit::Habit;
use crate::models::state::AppState;

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

#[tokio::main]
async fn main() {
    //lista inicial de habitos -> preenchidos de forma dinamica
    //Arc garante que o estado seja compartilhado enter multiplcas threads
    //inicializa um vetor fazio com mutex que garante acesso seguro
    let state = Arc::new(AppState {
        habits: Mutex::new(vec![]),
    });

    let app = Router::new()
        .route("/ping", get(ping_handler))
        .route("/habit-test", get(habit_handler))
        .route("/habits", get(list_habits))
        .route("/habits", post(create_habit))
        .with_state(state); //injeta o Arc<AppState>

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Servidor rodando em http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
