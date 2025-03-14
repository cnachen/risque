use axum::{
    http::Method,
    response::Html,
    routing::{get, post},
    Extension, Router,
};
use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};

pub struct App;

impl App {
    pub async fn run() {
        let cpu = Arc::new(Mutex::new(crate::Cpu::new(Vec::new())));

        let cors = CorsLayer::new()
            .allow_origin(Any) // 允许任何来源的请求（开发模式用）
            .allow_methods([Method::GET, Method::POST])
            .allow_headers(Any)
            // .allow_credentials(true)
            .max_age(Duration::from_secs(60)); // 可选，设置 CORS 预检请求缓存时间

        // build our application with a route
        let app = Router::new()
            .route("/", get(Self::handler))
            .route(
                "/api/v1/compiler/compile",
                post(super::compiler::post_compile),
            )
            .route("/api/v1/core/memory", post(super::core::post_memory))
            .route("/api/v1/core/registers", post(super::core::post_registers))
            .route("/api/v1/core/run", post(super::core::post_run))
            .route("/api/v1/core/step", post(super::core::post_step))
            .layer(cors)
            .layer(Extension(cpu));

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
