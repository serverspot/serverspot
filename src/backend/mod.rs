use std::sync::Arc;

use dioxus::{
    prelude::*,
    server::axum::{
        Extension, Router,
        body::Body,
        http::{StatusCode, header},
        extract::{DefaultBodyLimit, Path},
        response::Response,
        routing::get,
    },
};
use surrealdb::{
    Surreal, engine::remote::ws::{Client, Ws}, opt::auth::Root,
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
        }).await?;

        db.use_ns(surreal_ns).use_db("serverspot").await?;

        info!("Connected to SurrealDB successfully");

        Ok(Arc::new(Self { db }))
    }
}

pub fn launch(app: fn() -> Element) {
    #[cfg(debug_assertions)]
    dotenvy::dotenv().ok();

    dioxus::serve(|| async move {
        let router = dioxus::server::router(app)
            .merge(Router::new().route("/uploads/site-logo", get(site_logo)))
            .merge(Router::new().route("/uploads/site-favicon", get(site_favicon)))
            .merge(
                Router::new().route(
                    "/uploads/theme-config/{id}",
                    get(theme_config_image),
                ),
            )
            .merge(
                Router::new().route(
                    "/theme-runtime/{feature}/scripts.js",
                    get(theme_javascript),
                ),
            )
            // Server functions encode byte vectors as JSON, which can be several
            // times larger than the source image. Keep this above every UI limit.
            .layer(DefaultBodyLimit::max(32 * 1024 * 1024))
            .layer(Extension(BackendState::new().await?));

        Ok(router)
    });
}

async fn theme_javascript(Path(feature): Path<String>) -> Response {
    const ALLOWED_FEATURES: &[&str] = &[
        "home",
        "forum",
        "store",
        "support",
        "blog",
        "players",
        "leaderboards",
        "votes",
        "applications",
        "analytics",
    ];

    if !ALLOWED_FEATURES.contains(&feature.as_str()) {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .unwrap_or_else(|_| Response::new(Body::empty()));
    }

    let theme = crate::theme::ACTIVE_THEME;
    let mut javascript = String::new();

    // Only allowlisted entrypoints — never recursively execute arbitrary .js drops.
    let mut allowlisted = vec![format!("{theme}/scripts.js")];
    if feature != "home" {
        allowlisted.push(format!("{theme}/{feature}/scripts.js"));
    }

    for relative in allowlisted {
        let Some(source) = crate::theme::read_theme_source_shared(&relative) else {
            continue;
        };
        javascript.push_str("\n/* ");
        javascript.push_str(&relative);
        javascript.push_str(" */\n");
        javascript.push_str(&source);
        javascript.push_str("\n;\n");
    }

    javascript.push_str(
        "\nwindow.dispatchEvent(new CustomEvent('spot:page-load', { detail: { feature: ",
    );
    javascript.push('"');
    javascript.push_str(&feature);
    javascript.push('"');
    javascript.push_str(" } }));\n");

    let javascript = crate::theme::wrap_theme_javascript(&javascript, &feature);

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/javascript; charset=utf-8")
        .header(header::CACHE_CONTROL, "no-cache, no-store, must-revalidate")
        .header(header::X_CONTENT_TYPE_OPTIONS, "nosniff")
        .body(Body::from(javascript))
        .unwrap_or_else(|_| Response::new(Body::empty()))
}

async fn theme_config_image(Path(id): Path<String>) -> Response {
    const ASSET_DIR: &str = "data/theme-assets";

    if id.is_empty()
        || !id
            .chars()
            .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_')
    {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .unwrap_or_else(|_| Response::new(Body::empty()));
    }

    let bin_path = format!("{ASSET_DIR}/{id}.bin");
    let mime_path = format!("{ASSET_DIR}/{id}.mime");
    let Ok(bytes) = std::fs::read(&bin_path) else {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .unwrap_or_else(|_| Response::new(Body::empty()));
    };
    let content_type = std::fs::read_to_string(mime_path)
        .unwrap_or_else(|_| "application/octet-stream".to_string());

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, content_type.trim())
        .header(header::CACHE_CONTROL, "no-cache, no-store, must-revalidate")
        .body(Body::from(bytes))
        .unwrap_or_else(|_| Response::new(Body::empty()))
}

async fn site_logo() -> Response {
    serve_brand_asset(
        "data/site-logo.bin",
        "data/site-logo.mime",
        "assets/favicon.svg",
    )
}

async fn site_favicon() -> Response {
    serve_brand_asset(
        "data/site-favicon.bin",
        "data/site-favicon.mime",
        "assets/favicon.svg",
    )
}

fn serve_brand_asset(asset_path: &str, mime_path: &str, fallback_path: &str) -> Response {
    let (bytes, content_type) = match std::fs::read(asset_path) {
        Ok(bytes) => {
            let mime = std::fs::read_to_string(mime_path)
                .unwrap_or_else(|_| "application/octet-stream".to_string());
            (bytes, mime)
        }
        Err(_) => match std::fs::read(fallback_path) {
            Ok(bytes) => (bytes, "image/svg+xml".to_string()),
            Err(_) => {
                return Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::empty())
                    .unwrap_or_else(|_| Response::new(Body::empty()));
            }
        },
    };

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, content_type.trim())
        .header(header::CACHE_CONTROL, "no-cache, no-store, must-revalidate")
        .body(Body::from(bytes))
        .unwrap_or_else(|_| Response::new(Body::empty()))
}
