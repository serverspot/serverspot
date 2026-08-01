use std::sync::Arc;
use dioxus::{prelude::*, server::axum::Extension};
use surrealdb::{
    Surreal, engine::remote::ws::{Client, Ws},
    opt::auth::Root,
};
pub mod util;
pub type AppState = Arc<BackendState>;
pub struct BackendState {
    #[allow(dead_code)]
    pub db: Surreal<Client>,
}
impl BackendState {
    pub async fn new() -> anyhow::Result<AppState> {
        let surreal_url = util::get_env("SURREAL_URL")?;
        let surreal_ns = util::get_env("SURREAL_NS")?;
        let surreal_user = util::get_env("SURREAL_USER")?;
        let surreal_pass = util::get_env("SURREAL_PASS")?;
        let db = Surreal::new::<Ws>(surreal_url).await?;
        db.signin(Root {
                username: surreal_user,
                password: surreal_pass,
            })
            .await?;
        db.use_ns(surreal_ns).use_db("serverspot").await?;
        info!("Connected to SurrealDB successfully");
        Ok(Arc::new(Self { db }))
    }
}
pub fn launch(app: fn() -> Element) {
    #[cfg(debug_assertions)] dotenvy::dotenv().ok();
    dioxus::serve(|| async move {
        let router = dioxus::server::router(app)
            .layer(Extension(BackendState::new().await?)); // TODO figure out why it keeps running BackendState::new() over and over
        Ok(router)
    });
}
