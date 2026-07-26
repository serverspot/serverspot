use std::sync::Arc;

use dioxus::{prelude::*, server::axum::Extension};
use surrealdb::{
    engine::local::{Db, RocksDb},
    Surreal,
};

pub mod util;

pub type AppState = Arc<BackendState>;

pub struct BackendState {
    #[allow(dead_code)]
    pub db: Surreal<Db>,
}

impl BackendState {
    pub async fn new() -> anyhow::Result<AppState> {
        let surreal_path = util::get_env("SURREAL_PATH")?;
        let surreal_ns = util::get_env("SURREAL_NS")?;

        let db = Surreal::new::<RocksDb>(surreal_path).await?;
        db.use_ns(surreal_ns).use_db("serverspot").await?;

        info!("Connected to SurrealDB successfully");

        Ok(Arc::new(Self { db }))
    }
}

pub fn launch(app: fn() -> Element) {
    #[cfg(debug_assertions)]
    dotenvy::dotenv().ok();

    dioxus::serve(|| async move {
        let router = dioxus::server::router(app).layer(Extension(BackendState::new().await?));

        Ok(router)
    });
}
