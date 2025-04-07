//axum http framework dependencies
//use its like the import for rust
use axum::{ routing::get, Json, Router };

use serde::Serialize;
use std::net::SocketAddr;

#[derive(Serialize)]
struct PingResponse {
    ok: bool,
}

struct Habit {
    id: u32,
    title: String,
    completed: bool,
}

//Ping Handler
async fn ping_handler() -> Json<PingResponse> {
    Json(PingResponse { ok: true })
}

async fn habit_handler() -> Json<Habit> {}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/ping", get(ping_handler)).route(path, method_router);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Servidor rodando em http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
