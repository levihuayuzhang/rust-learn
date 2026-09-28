use crate::state::AppState;

mod handlers;
mod models;
mod routes;
mod state;

#[tokio::main]
async fn main() {
    let state = AppState::new();

    let app = routes::app(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("taskr running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
