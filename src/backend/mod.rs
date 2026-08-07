use std::sync::Arc;
use async_once_cell::OnceCell;
use axum_session::{SessionConfig, SessionLayer, SessionStore};
use axum_session_auth::AuthSessionLayer;
use axum_session_surreal::{SessionSurrealPool, SessionSurrealSession};
use dioxus::{prelude::*, server::axum::Extension};
use surrealdb::{
    Surreal, engine::remote::ws::{Client, Ws},
    opt::auth::Root,
};

use crate::backend::auth::ActiveUser;


pub mod util;
pub mod auth;
pub mod db_model;

pub type AppState = Arc<BackendState>;
pub type AuthSession = SessionSurrealSession<Client>;
pub type SessionPool = SessionSurrealPool<Client>;
pub type Database = Surreal<Client>;

pub struct BackendState {
    #[allow(dead_code)]
    pub db: Database,
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
        }).await?;
        db.use_ns(surreal_ns).use_db("serverspot").await?;
        info!("Connected to SurrealDB successfully");
        Ok(Arc::new(Self { db }))
    }
}

pub fn launch(app: fn() -> Element) -> anyhow::Result<()> {
   let setup = Arc::new(OnceCell::<(AppState, SessionStore<SessionPool>)>::new()); 

    #[cfg(debug_assertions)] dotenvy::dotenv().ok();
    dioxus::serve(|| {
        // clone bc this may be executed multiple times and moving it won't work.
        // also some lazy fuck added 'static to the closure requirements even though
        // it's guaranteed to never use-after-free since it would always be done executing by the time it escapes
        // the dioxus::serve function.
        let setup = setup.clone();

        async move {
            // since we don't really want to reconnect to the db every hot reload,
            // it's better to just use a OnceCell and copy an Arc into there
            // than to put the setup logic directly within this async block.
            let (state, session_store ) = setup.get_or_try_init(async {
                let state = BackendState::new().await?;

                let session_config = SessionConfig::new()
                    .with_table_name("auth_session");
                let session_pool = SessionSurrealPool::new(state.db.clone());
                let session_store = SessionStore::new(Some(session_pool), session_config).await?;

                Ok::<_, anyhow::Error>((state, session_store))
            }).await?;

            let router = dioxus::server::router(app)
                .layer(Extension(state.clone()))
                .layer(AuthSessionLayer::<ActiveUser, String, SessionPool, Database>::new(Some(state.db.clone())))
                .layer(SessionLayer::new(session_store.clone()));
            Ok(router)
        }
    });
}