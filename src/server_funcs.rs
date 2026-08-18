//! Server funcs represent endpoints exposed by the server which can be accessed by the frontend.
//! You can call it like a regular function, but internally it sends a web request to the backend server.

use std::collections::BTreeMap;

use dioxus::prelude::*;

use crate::theme::{ThemeConfigState, ThemeFileEntry};

#[server]
pub async fn get_theme_config() -> Result<ThemeConfigState, ServerFnError> {
    crate::theme::load_merged_config(crate::theme::ACTIVE_THEME).map_err(ServerFnError::new)
}

#[server]
pub async fn save_theme_config(values: BTreeMap<String, String>) -> Result<(), ServerFnError> {
    crate::theme::require_admin_editor_access().await?;
    crate::theme::persist_theme_config(values).map_err(ServerFnError::new)
}

#[server]
pub async fn reset_theme_config() -> Result<(), ServerFnError> {
    crate::theme::require_admin_editor_access().await?;
    crate::theme::reset_theme_config_file().map_err(ServerFnError::new)
}

#[server]
pub async fn upload_theme_config_image(
    option_id: String,
    file_name: String,
    bytes: Vec<u8>,
) -> Result<String, ServerFnError> {
    crate::theme::require_admin_editor_access().await?;
    crate::theme::store_theme_config_image(&option_id, &file_name, &bytes)
        .map_err(ServerFnError::new)
}

#[server]
pub async fn list_theme_files() -> Result<Vec<ThemeFileEntry>, ServerFnError> {
    crate::theme::require_admin_editor_access().await?;
    crate::theme::list_theme_file_entries().map_err(ServerFnError::new)
}

#[server]
pub async fn read_theme_file(path: String) -> Result<String, ServerFnError> {
    crate::theme::require_admin_editor_access().await?;
    crate::theme::read_theme_file_from_pack(&path).map_err(ServerFnError::new)
}

#[server]
pub async fn write_theme_file(path: String, content: String) -> Result<(), ServerFnError> {
    crate::theme::require_admin_editor_access().await?;
    crate::theme::write_theme_file_to_disk(&path, &content).map_err(ServerFnError::new)
}

#[server]
pub async fn delete_theme_file(path: String) -> Result<(), ServerFnError> {
    crate::theme::require_admin_editor_access().await?;
    crate::theme::delete_theme_file_from_disk(&path).map_err(ServerFnError::new)
}

#[server]
pub async fn upload_site_logo(file_name: String, bytes: Vec<u8>) -> Result<(), ServerFnError> {
    const MAX_LOGO_SIZE: usize = 4 * 1024 * 1024;

    crate::theme::require_admin_editor_access().await?;
    if bytes.is_empty() {
        return Err(ServerFnError::new("The selected image is empty"));
    }
    if bytes.len() > MAX_LOGO_SIZE {
        return Err(ServerFnError::new("The logo must be 4 MB or smaller"));
    }

    let content_type = detect_brand_content_type(&bytes, false)
        .ok_or_else(|| ServerFnError::new("Use a valid PNG, JPEG, WebP, or GIF image"))?;

    #[cfg(feature = "server")]
    {
        let _ = file_name;
        std::fs::create_dir_all("data").map_err(|error| ServerFnError::new(error.to_string()))?;
        std::fs::write("data/site-logo.bin", bytes)
            .map_err(|error| ServerFnError::new(error.to_string()))?;
        std::fs::write("data/site-logo.mime", content_type)
            .map_err(|error| ServerFnError::new(error.to_string()))?;
        return Ok(());
    }

    #[cfg(not(feature = "server"))]
    {
        let _ = (file_name, bytes, content_type);
        Err(ServerFnError::new(
            "Uploading a site logo requires the server runtime",
        ))
    }
}

#[server]
pub async fn upload_site_favicon(file_name: String, bytes: Vec<u8>) -> Result<(), ServerFnError> {
    const MAX_FAVICON_SIZE: usize = 2 * 1024 * 1024;

    crate::theme::require_admin_editor_access().await?;
    if bytes.is_empty() {
        return Err(ServerFnError::new("The selected image is empty"));
    }
    if bytes.len() > MAX_FAVICON_SIZE {
        return Err(ServerFnError::new("The favicon must be 2 MB or smaller"));
    }

    let content_type = detect_brand_content_type(&bytes, true)
        .ok_or_else(|| ServerFnError::new("Use a valid PNG, JPEG, WebP, GIF, or ICO image"))?;

    #[cfg(feature = "server")]
    {
        let _ = file_name;
        std::fs::create_dir_all("data").map_err(|error| ServerFnError::new(error.to_string()))?;
        std::fs::write("data/site-favicon.bin", bytes)
            .map_err(|error| ServerFnError::new(error.to_string()))?;
        std::fs::write("data/site-favicon.mime", content_type)
            .map_err(|error| ServerFnError::new(error.to_string()))?;
        return Ok(());
    }

    #[cfg(not(feature = "server"))]
    {
        let _ = (file_name, bytes, content_type);
        Err(ServerFnError::new(
            "Uploading a site favicon requires the server runtime",
        ))
    }
}

#[cfg(feature = "server")]
fn detect_brand_content_type(bytes: &[u8], allow_ico: bool) -> Option<&'static str> {
    if bytes.starts_with(b"\x89PNG\r\n\x1a\n") {
        Some("image/png")
    } else if bytes.starts_with(b"\xff\xd8\xff") {
        Some("image/jpeg")
    } else if bytes.len() >= 12 && bytes.starts_with(b"RIFF") && &bytes[8..12] == b"WEBP" {
        Some("image/webp")
    } else if bytes.starts_with(b"GIF87a") || bytes.starts_with(b"GIF89a") {
        Some("image/gif")
    } else if allow_ico && bytes.starts_with(b"\x00\x00\x01\x00") {
        Some("image/x-icon")
    } else {
        None
    }
}
