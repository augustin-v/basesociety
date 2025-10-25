pub mod handlers;
pub mod models;

use axum::{
    Router,
    routing::{delete, get, post},
};
use dotenv::dotenv;
use tower_http::cors::{Any, CorsLayer};
use models::AppState;
use sqlx::{Sqlite, SqlitePool, migrate::MigrateDatabase};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, RwLock},
    env
};
use tracing::info;

use crate::handlers::{delete_agent, get_history, interact_agent, launch_agent, list_agents, get_agent};

fn get_db_url() -> String {
    env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:./crates/ai_execution/agents.db".to_string())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv().ok();

    if !Sqlite::database_exists(&get_db_url()).await.unwrap_or(false) {
        info!("Creating database {}", &get_db_url());
        match Sqlite::create_database(&get_db_url()).await {
            Ok(_) => info!("Create database success"),
            Err(error) => panic!("error: {}", error),
        }
    }

    let db_pool = SqlitePool::connect(&get_db_url()).await.unwrap();

    sqlx::migrate!("./migrations").run(&db_pool).await.unwrap();

    let api_key = env::var("OPENAI_API_KEY")
        .expect("OPENAI_API_KEY must be set in .env");

    let state = AppState {
        db_pool,
        agents: Arc::new(RwLock::new(HashMap::new())),
        api_key
    };

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<reqwest::header::HeaderValue>().unwrap())
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(root))
        .nest(
            "/agents",
            Router::new()
                .route("/", post(launch_agent).get(list_agents)) // POST/GET /agents
                .route("/{id}/interact", post(interact_agent)) // POST /agents/{id}/interact
                .route("/{id}", get(get_agent).delete(delete_agent)) // DELETE /agents/{id}
                .route("/{id}/history", get(get_history)), // GET
        )
        .with_state(state)
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World! The AI Execution Crate is running."
}
