use axum::{response::Html, routing::{get, post}, Router, http::Method};
use tower_http::cors::{CorsLayer, Any};
use std::time::Duration;

pub struct App;

impl App {
    pub async fn run() {
        let cors = CorsLayer::new()
            .allow_origin(Any) // 允许任何来源的请求（开发模式用）
            .allow_methods([Method::GET, Method::POST])
            .allow_headers(Any)
            // .allow_credentials(true)
            .max_age(Duration::from_secs(60)); // 可选，设置 CORS 预检请求缓存时间

        // build our application with a route
        let app = Router::new()
            .route("/", get(Self::handler))
            .route("/api/v1/compiler/compile", post(super::compiler::post_compile))
            .route("/api/v1/core/memory", get(super::core::get_memory))
            .route("/api/v1/core/registers", get(super::core::get_registers))
            .route("/api/v1/core/run", post(super::core::post_run))
            .layer(cors);
    
        // run it
        let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
            .await
            .unwrap();
        println!("listening on {}", listener.local_addr().unwrap());
        axum::serve(listener, app).await.unwrap();
    }
    
    async fn handler() -> Html<&'static str> {
        Html("<h1>Hello, World!</h1>")
    }
}
