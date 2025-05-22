// use axum::{Router, routing::get, response::IntoResponse};

// async fn health_check() -> impl IntoResponse {
//     "OK"
// }

// pub async fn run_server() {
//     let app = Router::new()
//         .route("/health", get(health_check));
    
//     let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
//     axum::serve(listener, app).await.unwrap();
// }