use std::{
    collections::HashMap,
    sync::{Arc, OnceLock, RwLock},
};

use dioxus::prelude::*;

#[allow(unused_imports)]
use crate::theme::registry::{is_required_theme_path, root_required_files, theme_features};

pub const ACTIVE_THEME: &str = "default";

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ThemeFileEntry {
    pub path: String,
    pub language: String,
    pub required: bool,
    pub content: String,
}

/// All files in the active theme pack (root + feature folders + assets).
pub fn embedded_pack_entries() -> Vec<ThemeFileEntry> {
    let mut entries = Vec::new();

    for file in root_required_files() {
        let rel = format!("{ACTIVE_THEME}/{file}");
        if let Some(content) = read_theme_source(&rel) {
            entries.push(ThemeFileEntry {
                path: file.to_string(),
                language: language_for(file).to_string(),
                required: true,
                content,
            });
        }
    }

    if let Some(content) = read_theme_source(&format!("{ACTIVE_THEME}/scripts.js")) {
        entries.push(ThemeFileEntry {
            path: "scripts.js".to_string(),
            language: "JS".to_string(),
            required: false,
            content,
        });
    }

    for feature in theme_features() {
        let slug = feature.slug();
        for file in feature.required_files() {
            let rel = format!("{ACTIVE_THEME}/{slug}/{file}");
            if let Some(content) = read_theme_source(&rel) {
                entries.push(ThemeFileEntry {
                    path: format!("{slug}/{file}"),
                    language: language_for(file).to_string(),
                    required: true,
                    content,
                });
            }
        }
    }

    for file in [
        "header.html",
        "footer.html",
        "rail.html",
        "sidebar.html",
        "discord.html",
        "top-donors.html",
        "recent-purchases.html",
        "community-cta.html",
        "utils.css",
    ] {
        let rel = format!("{ACTIVE_THEME}/assets/{file}");
        if let Some(content) = read_theme_source(&rel) {
            entries.push(ThemeFileEntry {
                path: format!("assets/{file}"),
                language: language_for(file).to_string(),
                required: false,
                content,
            });
        }
    }

    sort_theme_entries(&mut entries);
    entries
}

pub fn sort_theme_entries(entries: &mut [ThemeFileEntry]) {
    entries.sort_by(|a, b| compare_theme_paths(&a.path, &b.path));
}

fn compare_theme_paths(left: &str, right: &str) -> std::cmp::Ordering {
    match (left == "schema.json", right == "schema.json") {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => left.cmp(right),
    }
}

fn source_cache() -> &'static RwLock<HashMap<String, Arc<str>>> {
    static CACHE: OnceLock<RwLock<HashMap<String, Arc<str>>>> = OnceLock::new();
    CACHE.get_or_init(|| RwLock::new(HashMap::new()))
}

fn css_cache() -> &'static RwLock<HashMap<(String, String), Arc<str>>> {
    static CACHE: OnceLock<RwLock<HashMap<(String, String), Arc<str>>>> = OnceLock::new();
    CACHE.get_or_init(|| RwLock::new(HashMap::new()))
}

pub fn read_theme_source(relative: &str) -> Option<String> {
    read_theme_source_shared(relative).map(|source| source.to_string())
}

pub fn read_theme_source_shared(relative: &str) -> Option<Arc<str>> {
    let path = crate::theme::security::sanitize_logical_theme_path(relative)?;
    if let Some(content) = source_cache()
        .read()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .get(&path)
        .cloned()
    {
        return Some(content);
    }

    let content: Arc<str> = Arc::from(load_theme_source_fresh(&path)?);
    if content.len() > crate::theme::security::MAX_THEME_FILE_BYTES {
        return None;
    }
    source_cache()
        .write()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .insert(path, content.clone());
    Some(content)
}

fn load_theme_source_fresh(path: &str) -> Option<String> {
    let path = crate::theme::security::sanitize_logical_theme_path(path)?;
    #[cfg(feature = "server")]
    {
        let disk = std::path::PathBuf::from(format!("themes/{path}"));
        if disk.exists() {
            if let Ok(content) = std::fs::read_to_string(disk) {
                if content.len() <= crate::theme::security::MAX_THEME_FILE_BYTES {
                    return Some(content);
                }
                return None;
            }
        }
    }
    embedded_theme(&path).map(str::to_string)
}

/// Drop cached theme sources/CSS/parsed ASTs so editor saves become live immediately.
#[cfg(feature = "server")]
pub fn invalidate_theme_caches() {
    source_cache()
        .write()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .clear();
    css_cache()
        .write()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .clear();
    crate::theme::runtime::clear_template_cache();
}

/// CSS for a feature folder (`forum`, `store`, …) or `"home"` / `""` for root styles.
pub fn theme_css_bundle(theme: &str, feature: &str) -> Arc<str> {
    let feature = feature.trim();
    let key = (theme.to_string(), feature.to_string());
    if let Some(css) = css_cache()
        .read()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .get(&key)
        .cloned()
    {
        return css;
    }

    let mut css = String::new();
    if let Some(utils) = read_theme_source_shared(&format!("{theme}/assets/utils.css")) {
        css.push_str(&utils);
        css.push('\n');
    }
    let styles_path = if feature.is_empty() || feature == "home" {
        format!("{theme}/styles.css")
    } else {
        format!("{theme}/{feature}/styles.css")
    };
    if let Some(styles) = read_theme_source_shared(&styles_path) {
        css.push_str(&styles);
    }
    let css: Arc<str> = Arc::from(css);
    css_cache()
        .write()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .insert(key, css.clone());
    css
}

fn embedded_theme(path: &str) -> Option<&'static str> {
    match path {
        "default/schema.json" => Some(include_str!("../../themes/default/schema.json")),
        "default/index.html" => Some(include_str!("../../themes/default/index.html")),
        "default/login.html" => Some(include_str!("../../themes/default/login.html")),
        "default/profile.html" => Some(include_str!("../../themes/default/profile.html")),
        "default/styles.css" => Some(include_str!("../../themes/default/styles.css")),
        "default/scripts.js" => Some(include_str!("../../themes/default/scripts.js")),
        "default/forum/index.html" => Some(include_str!("../../themes/default/forum/index.html")),
        "default/forum/thread.html" => Some(include_str!("../../themes/default/forum/thread.html")),
        "default/forum/styles.css" => Some(include_str!("../../themes/default/forum/styles.css")),
        "default/store/index.html" => Some(include_str!("../../themes/default/store/index.html")),
        "default/store/product.html" => {
            Some(include_str!("../../themes/default/store/product.html"))
        }
        "default/store/styles.css" => Some(include_str!("../../themes/default/store/styles.css")),
        "default/support/index.html" => {
            Some(include_str!("../../themes/default/support/index.html"))
        }
        "default/support/ticket.html" => {
            Some(include_str!("../../themes/default/support/ticket.html"))
        }
        "default/support/styles.css" => {
            Some(include_str!("../../themes/default/support/styles.css"))
        }
        "default/blog/index.html" => Some(include_str!("../../themes/default/blog/index.html")),
        "default/blog/post.html" => Some(include_str!("../../themes/default/blog/post.html")),
        "default/blog/styles.css" => Some(include_str!("../../themes/default/blog/styles.css")),
        "default/players/index.html" => {
            Some(include_str!("../../themes/default/players/index.html"))
        }
        "default/players/profile.html" => {
            Some(include_str!("../../themes/default/players/profile.html"))
        }
        "default/players/styles.css" => {
            Some(include_str!("../../themes/default/players/styles.css"))
        }
        "default/leaderboards/index.html" => {
            Some(include_str!("../../themes/default/leaderboards/index.html"))
        }
        "default/leaderboards/board.html" => {
            Some(include_str!("../../themes/default/leaderboards/board.html"))
        }
        "default/leaderboards/styles.css" => {
            Some(include_str!("../../themes/default/leaderboards/styles.css"))
        }
        "default/votes/index.html" => Some(include_str!("../../themes/default/votes/index.html")),
        "default/votes/claim.html" => Some(include_str!("../../themes/default/votes/claim.html")),
        "default/votes/styles.css" => Some(include_str!("../../themes/default/votes/styles.css")),
        "default/applications/index.html" => {
            Some(include_str!("../../themes/default/applications/index.html"))
        }
        "default/applications/form.html" => {
            Some(include_str!("../../themes/default/applications/form.html"))
        }
        "default/applications/styles.css" => {
            Some(include_str!("../../themes/default/applications/styles.css"))
        }
        "default/analytics/index.html" => {
            Some(include_str!("../../themes/default/analytics/index.html"))
        }
        "default/analytics/styles.css" => {
            Some(include_str!("../../themes/default/analytics/styles.css"))
        }
        "default/assets/header.html" => {
            Some(include_str!("../../themes/default/assets/header.html"))
        }
        "default/assets/footer.html" => {
            Some(include_str!("../../themes/default/assets/footer.html"))
        }
        "default/assets/community-cta.html" => Some(include_str!(
            "../../themes/default/assets/community-cta.html"
        )),
        "default/assets/rail.html" => Some(include_str!("../../themes/default/assets/rail.html")),
        "default/assets/sidebar.html" => {
            Some(include_str!("../../themes/default/assets/sidebar.html"))
        }
        "default/assets/discord.html" => {
            Some(include_str!("../../themes/default/assets/discord.html"))
        }
        "default/assets/top-donors.html" => {
            Some(include_str!("../../themes/default/assets/top-donors.html"))
        }
        "default/assets/recent-purchases.html" => Some(include_str!(
            "../../themes/default/assets/recent-purchases.html"
        )),
        "default/assets/utils.css" => Some(include_str!("../../themes/default/assets/utils.css")),
        _ => None,
    }
}

fn language_for(path: &str) -> &'static str {
    if path.ends_with(".css") {
        "CSS"
    } else if path.ends_with(".html") {
        "HTML"
    } else if path.ends_with(".js") {
        "JS"
    } else if path.ends_with(".json") {
        "JSON"
    } else {
        "TEXT"
    }
}

#[server]
pub async fn list_theme_files() -> Result<Vec<ThemeFileEntry>, ServerFnError> {
    crate::theme::security::require_admin_editor_access().await?;
    let mut entries = Vec::new();

    #[cfg(feature = "server")]
    {
        let root = std::path::PathBuf::from(format!("themes/{ACTIVE_THEME}"));
        if root.exists() {
            collect_pack_dir(&root, "", &mut entries)?;
        }
    }

    if entries.is_empty() {
        return Ok(embedded_pack_entries());
    }

    sort_theme_entries(&mut entries);
    Ok(entries)
}

#[cfg(feature = "server")]
fn collect_pack_dir(
    dir: &std::path::Path,
    prefix: &str,
    out: &mut Vec<ThemeFileEntry>,
) -> Result<(), ServerFnError> {
    use std::fs;
    for entry in fs::read_dir(dir).map_err(|e| ServerFnError::new(e.to_string()))? {
        let entry = entry.map_err(|e| ServerFnError::new(e.to_string()))?;
        let path = entry.path();
        let name = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string();
        if name.is_empty() || name.starts_with('.') {
            continue;
        }
        let rel = if prefix.is_empty() {
            name.clone()
        } else {
            format!("{prefix}/{name}")
        };
        if path.is_dir() {
            collect_pack_dir(&path, &rel, out)?;
            continue;
        }
        let content = fs::read_to_string(&path).map_err(|e| ServerFnError::new(e.to_string()))?;
        out.push(ThemeFileEntry {
            path: rel.clone(),
            language: language_for(&name).to_string(),
            required: is_required_theme_path(&rel),
            content,
        });
    }
    Ok(())
}

#[server]
pub async fn read_theme_file(path: String) -> Result<String, ServerFnError> {
    crate::theme::security::require_admin_editor_access().await?;
    let path = crate::theme::security::sanitize_pack_relative_path(&path)
        .map_err(ServerFnError::new)?;
    let logical = format!("{ACTIVE_THEME}/{path}");

    #[cfg(feature = "server")]
    {
        let disk = crate::theme::security::resolve_disk_path_inside_theme(&path)
            .map_err(ServerFnError::new)?;
        if disk.exists() {
            let content =
                std::fs::read_to_string(disk).map_err(|e| ServerFnError::new(e.to_string()))?;
            if content.len() > crate::theme::security::MAX_THEME_FILE_BYTES {
                return Err(ServerFnError::new("theme file is too large"));
            }
            return Ok(content);
        }
    }

    read_theme_source(&logical).ok_or_else(|| ServerFnError::new("theme file not found"))
}

#[server]
pub async fn write_theme_file(path: String, content: String) -> Result<(), ServerFnError> {
    crate::theme::security::require_admin_editor_access().await?;
    let path = crate::theme::security::sanitize_pack_relative_path(&path)
        .map_err(ServerFnError::new)?;
    if content.len() > crate::theme::security::MAX_THEME_FILE_BYTES {
        return Err(ServerFnError::new("theme file is too large"));
    }
    if path == "schema.json" {
        crate::theme::schema::parse_schema_json(&content).map_err(ServerFnError::new)?;
    }

    #[cfg(feature = "server")]
    {
        let disk = crate::theme::security::resolve_disk_path_inside_theme(&path)
            .map_err(ServerFnError::new)?;
        if let Some(parent) = disk.parent() {
            std::fs::create_dir_all(parent).map_err(|e| ServerFnError::new(e.to_string()))?;
        }
        std::fs::write(disk, content).map_err(|e| ServerFnError::new(e.to_string()))?;
        invalidate_theme_caches();
        return Ok(());
    }

    #[cfg(not(feature = "server"))]
    {
        let _ = (path, content);
        Err(ServerFnError::new(
            "writing theme files requires the server runtime",
        ))
    }
}

#[server]
pub async fn delete_theme_file(path: String) -> Result<(), ServerFnError> {
    crate::theme::security::require_admin_editor_access().await?;
    let path = crate::theme::security::sanitize_pack_relative_path(&path)
        .map_err(ServerFnError::new)?;
    if is_required_theme_path(&path) {
        return Err(ServerFnError::new("required theme pages cannot be deleted"));
    }

    #[cfg(feature = "server")]
    {
        let disk = crate::theme::security::resolve_disk_path_inside_theme(&path)
            .map_err(ServerFnError::new)?;
        if disk.exists() {
            std::fs::remove_file(&disk).map_err(|e| ServerFnError::new(e.to_string()))?;
        }
        invalidate_theme_caches();
        return Ok(());
    }

    #[cfg(not(feature = "server"))]
    {
        let _ = path;
        Err(ServerFnError::new(
            "deleting theme files requires the server runtime",
        ))
    }
}
