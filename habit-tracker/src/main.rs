//axum http framework dependencies
//use its like the import for rust
use axum::{ routing::get, Json, Router };

use serde::Serialize;
use std::net::SocketAddr;

#[derive(Serialize)]
struct PingResponse {
    ok: bool,
}

//Ping Handler
async fn ping_handler() -> Json<PingResponse> {
    Json(PingResponse { ok: true })
}


#[tokio::main]
async fn main() {
    let app = Router::new().route("/ping", get(ping_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Servidor rodando em http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


