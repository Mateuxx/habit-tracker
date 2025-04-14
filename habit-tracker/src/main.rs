mod handlers;
mod models;
mod routes;

use std::env;
use std::{ net::SocketAddr, sync::Arc };
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;

use crate::models::state::AppState;
use crate::routes::create_routes;

#[tokio::main]
async fn main() {
    dotenv().ok();

    //Database connection
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL não definida no .env");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url).await
        .expect("Erro ao conectar no banco de dados");

    // now our databse connection receives
    let state = Arc::new(AppState {
        db: pool,
    });

    //router more clean
    let app = create_routes(state.clone());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Servidor rodando em http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
