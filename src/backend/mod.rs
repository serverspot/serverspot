//! Put backend-specific logic that you don't want the frontend to see here.

pub mod util;

use std::sync::Arc;

use dioxus::{prelude::*, server::axum::Extension};
use surrealdb::{Surreal, engine::local::{Db, RocksDb}};

/// Type alias for the backend state actually applied via the extensions
pub type AppState = Arc<BackendState>;

/// The backend state for things like database connections.
pub struct BackendState {
    /// Embedded SurrealDB instance using RocksDB.
    pub db: Surreal<Db>,
}

impl BackendState {
    /// Initialize global state.
    pub async fn new() -> anyhow::Result<AppState> {
        let surreal_path = util::get_env("SURREAL_PATH")?;
        let surreal_ns = util::get_env("SURREAL_NS")?;

        let db = Surreal::new::<RocksDb>(surreal_path).await?;
        db.use_ns(surreal_ns).use_db("serverspot").await?;

        info!("Connected to SurrealDB successfully");
        
        Ok(Arc::new(Self {
            db,
        }))
    }
}

/// Initializes global state and launches the server.
pub fn launch(app: fn() -> Element) {
    #[cfg(debug_assertions)]
    dotenvy::dotenv().ok();
    
    dioxus::serve(|| async move {
        let router = dioxus::server::router(app)
            .layer(Extension(BackendState::new().await?));

        Ok(router)
    });
}