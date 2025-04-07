//axum http framework dependencies
//use its like the import for rust
mod models;

use axum::{ routing::get, Json, Router };
use serde::Serialize;
use std::net::SocketAddr;
use crate::models::habit::Habit;

#[derive(Serialize)]
struct PingResponse {
    ok: bool,
}

//Ping Handler
async fn ping_handler() -> Json<PingResponse> {
    Json(PingResponse { ok: true })
}

async fn habit_handler() -> Json<Habit> {
    Json(Habit { id: 1, title: "beber agua".to_string(), completed: false })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/ping", get(ping_handler))
        .route("/habit-test", get(habit_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Servidor rodando em http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
