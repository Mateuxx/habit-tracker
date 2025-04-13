mod handlers;
mod models;
mod routes;

use std::sync::Mutex;
use std::{ net::SocketAddr, sync::Arc };
use crate::models::state::AppState;
use crate::routes::create_routes;

#[tokio::main]
async fn main() {
    //lista inicial de habitos -> preenchidos de forma dinamica
    //Arc garante que o estado seja compartilhado enter multiplcas threads
    //inicializa um vetor fazio com mutex que garante acesso seguro
    let state = Arc::new(AppState {
        habits: Mutex::new(vec![]),
    });

    //router more clean
    let app = create_routes(state.clone());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Servidor rodando em http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
