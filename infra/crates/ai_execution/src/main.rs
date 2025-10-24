pub mod handlers;
pub mod models;

use axum::{
    Router,
    routing::{delete, get, post},
};
use models::AppState;
use sqlx::{Sqlite, SqlitePool, migrate::MigrateDatabase};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, RwLock},
};
use tracing::info;

use crate::handlers::{delete_agent, get_history, interact_agent, launch_agent, list_agents};

const DB_URL: &str = "sqlite:agents.db";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        info!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => info!("Create database success"),
            Err(error) => panic!("error: {}", error),
        }
    }

    let db_pool = SqlitePool::connect(DB_URL).await.unwrap();

    sqlx::migrate!("./migrations").run(&db_pool).await.unwrap();

    let state = AppState {
        db_pool,
        agents: Arc::new(RwLock::new(HashMap::new())),
    };

    let app = Router::new()
        .route("/", get(root))
        .nest(
            "/agents",
            Router::new()
                .route("/", post(launch_agent).get(list_agents)) // POST/GET /agents
                .route("/{id}/interact", post(interact_agent)) // POST /agents/{id}/interact
                .route("/{id}", delete(delete_agent)) // DELETE /agents/{id}
                .route("/{id}/history", get(get_history)), // GET
        )
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World! The AI Execution Crate is running."
}
