mod config;
mod handler;

use api::{init_api, DbConfig, VectorDb};
use axum::{routing::{delete,get,post}, Router};

use config::Config;
use defs::DbError;
use index::IndexType;
use storage::StorageType;
use tracing::info;

use std::sync::Arc;
use handler::{insert_point_handler,get_point_handler,delete_point_handler,search_points_handler};

#[derive(Clone)]
struct AppState {
    db: Arc<VectorDb>,
}

#[tokio::main]
async fn main() -> Result<(), DbError> {
    tracing_subscriber::fmt::init();

    let config = Config::from_env();
    info!("Loaded configuration: {:?}", config.db_path);
    info!("Vector dimension set to: {}", config.vector_dimension);

    if let Some(parent) = config.db_path.parent() {
        std::fs::create_dir_all(parent).expect("Failed to create database directory");
    }

    //  db init
    let db_config = DbConfig {
        storage_type: StorageType::RocksDb,
        index_type: IndexType::Flat,
        data_path: config.db_path,
        dimension: config.vector_dimension,
    };

    let db = init_api(db_config)?;
    let app_state = AppState { db: Arc::new(db) };

    // axum init
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/points", post(insert_point_handler))
        .route("/points/{id}", get(get_point_handler).delete(delete_point_handler),
        )
        .route("/points/search", post(search_points_handler))
        .with_state(app_state);

    info!(" Server listening on http://{}", config.listen_addr);

    let listener = tokio::net::TcpListener::bind(config.listen_addr)
        .await
        .unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn root_handler() -> &'static str {
    "Vector Database server is running!"
}