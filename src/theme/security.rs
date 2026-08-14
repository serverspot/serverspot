//! Theme sandbox: untrusted theme HTML/CSS/JS must stay on the public site and
//! must not escape into admin surfaces, the filesystem, or process-killing DoS.

use crate::theme::loader::ACTIVE_THEME;

pub const MAX_THEME_FILE_BYTES: usize = 512 * 1024;
pub const MAX_FOREACH_ITEMS: usize = 250;
pub const MAX_INCLUDE_DEPTH: usize = 16;
pub const MAX_RAW_HTML_BYTES: usize = 64 * 1024;
pub const MAX_PARSE_DEPTH: usize = 64;

const THEME_FILE_EXTENSIONS: &[&str] = &["html", "css", "js", "json"];

/// Theme editor / brand mutations must come from the admin UI, not public theme code.
#[cfg(feature = "server")]
pub async fn require_admin_editor_access() -> Result<(), dioxus::prelude::ServerFnError> {
    use dioxus::fullstack::FullstackContext;
    use dioxus::server::axum::http::header;

    if std::env::var("SERVERSPOT_ALLOW_OPEN_THEME_ADMIN")
        .ok()
        .as_deref()
        == Some("1")
    {
        return Ok(());
    }

    let Some(ctx) = FullstackContext::current() else {
        return Err(dioxus::prelude::ServerFnError::new(
            "admin access required for theme editor actions",
        ));
    };

    let parts = ctx.parts_mut().clone();

    // In-process SSR of admin pages may not include a Referer. The request path itself
    // is authoritative for those calls.
    if is_admin_request_path(parts.uri.path()) {
        return Ok(());
    }

    let referer = parts
        .headers
        .get(header::REFERER)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("");
    if is_admin_ui_referer(&referer.to_ascii_lowercase()) {
        return Ok(());
    }

    Err(dioxus::prelude::ServerFnError::new(
        "admin access required for theme editor actions",
    ))
}

#[cfg(not(feature = "server"))]
#[allow(dead_code)]
pub async fn require_admin_editor_access() -> Result<(), dioxus::prelude::ServerFnError> {
    Err(dioxus::prelude::ServerFnError::new(
        "admin access required for theme editor actions",
    ))
}

fn is_admin_request_path(path: &str) -> bool {
    let path = path.split(['?', '#']).next().unwrap_or(path);
    path == "/admin" || path.starts_with("/admin/")
}

fn is_admin_ui_referer(referer: &str) -> bool {
    // Accept only admin UI pages. Public theme pages must never satisfy this.
    let path = if let Some(idx) = referer.find("://") {
        let after_scheme = &referer[idx + 3..];
        match after_scheme.find('/') {
            Some(path_start) => &after_scheme[path_start..],
            None => return false,
        }
    } else if referer.starts_with('/') {
        referer
    } else {
        return false;
    };
    is_admin_request_path(path)
}

pub fn has_allowed_theme_extension(path: &str) -> bool {
    path.rsplit_once('.')
        .map(|(_, ext)| THEME_FILE_EXTENSIONS.iter().any(|allowed| *allowed == ext))
        .unwrap_or(false)
}

/// Logical theme path like `default/assets/header.html`.
pub fn sanitize_logical_theme_path(path: &str) -> Option<String> {
    let path = path.trim().trim_start_matches('/').replace('\\', "/");
    if path.is_empty() || path.contains('\0') || path.contains("..") {
        return None;
    }
    if !path
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '/' | '_' | '-' | '.'))
    {
        return None;
    }

    let mut parts = path.split('/');
    let theme = parts.next()?;
    if theme != ACTIVE_THEME {
        return None;
    }

    let rest: Vec<&str> = parts.collect();
    if rest.is_empty() || rest.iter().any(|part| part.is_empty() || *part == "." || *part == "..")
    {
        return None;
    }

    Some(path)
}

/// Pack-relative path used by the theme editor (`assets/header.html`).
#[cfg_attr(not(feature = "server"), allow(dead_code))]
pub fn sanitize_pack_relative_path(path: &str) -> Result<String, String> {
    let path = path.trim().replace('\\', "/");
    if path.is_empty() || path.starts_with('/') || path.contains('\0') || path.contains("..") {
        return Err("invalid theme path".into());
    }
    if !path
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '/' | '_' | '-' | '.'))
    {
        return Err("invalid theme path".into());
    }
    let parts: Vec<&str> = path.split('/').collect();
    if parts.is_empty() || parts.iter().any(|part| part.is_empty() || *part == "." || *part == "..")
    {
        return Err("invalid theme path".into());
    }
    if !has_allowed_theme_extension(&path) {
        return Err("theme files must be .html, .css, .js, or .json".into());
    }

    let logical = format!("{ACTIVE_THEME}/{path}");
    sanitize_logical_theme_path(&logical).ok_or_else(|| "invalid theme path".to_string())?;
    Ok(path)
}

/// Resolve `@include('…')` into a sandboxed logical theme path.
pub fn resolve_theme_include(theme: &str, path: &str) -> Result<String, String> {
    if theme != ACTIVE_THEME {
        return Err("invalid theme".into());
    }
    let path = path.trim().trim_matches(|c| c == '\'' || c == '"');
    let path = path.strip_suffix(".html").unwrap_or(path).replace('\\', "/");
    if path.is_empty() || path.contains("..") || path.starts_with('/') || path.contains('\0') {
        return Err("invalid include path".into());
    }
    if !path
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '/' | '_' | '-'))
    {
        return Err("invalid include path".into());
    }

    let logical = if path.contains('/') {
        format!("{theme}/{path}.html")
    } else {
        format!("{theme}/assets/{path}.html")
    };
    sanitize_logical_theme_path(&logical).ok_or_else(|| "invalid include path".into())
}

#[cfg(feature = "server")]
pub fn resolve_disk_path_inside_theme(pack_relative: &str) -> Result<std::path::PathBuf, String> {
    let clean = sanitize_pack_relative_path(pack_relative)?;
    let root = std::path::PathBuf::from(format!("themes/{ACTIVE_THEME}"));
    let candidate = root.join(&clean);
    if candidate
        .components()
        .any(|component| matches!(component, std::path::Component::ParentDir))
    {
        return Err("invalid theme path".into());
    }

    let root_canon = std::fs::canonicalize(&root).unwrap_or(root.clone());
    if let Some(parent) = candidate.parent() {
        std::fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    if let Ok(canon) = std::fs::canonicalize(&candidate) {
        if !canon.starts_with(&root_canon) {
            return Err("path escapes theme directory".into());
        }
        return Ok(canon);
    }

    // File may not exist yet (writes). Validate the parent instead.
    if let Some(parent) = candidate.parent() {
        if let Ok(parent_canon) = std::fs::canonicalize(parent) {
            if !parent_canon.starts_with(&root_canon) {
                return Err("path escapes theme directory".into());
            }
        }
    }
    Ok(candidate)
}

pub fn is_forbidden_tag(tag: &str) -> bool {
    matches!(
        tag,
        "script"
            | "iframe"
            | "object"
            | "embed"
            | "applet"
            | "link"
            | "meta"
            | "base"
            | "frame"
            | "frameset"
            | "portal"
            | "svg"
            | "math"
            | "template"
            | "foreignobject"
    )
}

pub fn is_forbidden_attr(name: &str) -> bool {
    let name = name.trim();
    name.starts_with("on")
        || matches!(
            name,
            "srcdoc"
                | "formaction"
                | "xlink:href"
                | "hreflang"
                | "http-equiv"
                | "integrity"
                | "nonce"
        )
}

pub fn is_admin_or_internal_path(url: &str) -> bool {
    let path = url.split(['?', '#']).next().unwrap_or(url).to_ascii_lowercase();
    path == "/admin"
        || path.starts_with("/admin/")
        || path.starts_with("/_dioxus")
        || path.starts_with("/api/")
}

/// Allow only safe public navigations/resources from theme markup.
pub fn sanitize_public_url(url: &str) -> String {
    let url = url.trim();
    if url.is_empty() {
        return String::new();
    }

    let lower = url.to_ascii_lowercase();
    if lower.starts_with("javascript:")
        || lower.starts_with("data:")
        || lower.starts_with("vbscript:")
        || lower.starts_with("blob:")
    {
        return "#".into();
    }

    if url.starts_with('#') {
        return url.to_string();
    }

    if url.starts_with('/') && !url.starts_with("//") {
        if is_admin_or_internal_path(url) {
            return "/".into();
        }
        return url.to_string();
    }

    if lower.starts_with("https://") || lower.starts_with("http://") || lower.starts_with("mailto:")
    {
        return url.to_string();
    }

    "#".into()
}

pub fn sanitize_css_image_url(url: &str) -> Option<String> {
    let url = url.trim();
    if url.is_empty() {
        return None;
    }
    let lower = url.to_ascii_lowercase();
    if lower.starts_with("javascript:")
        || lower.starts_with("data:")
        || lower.contains(")")
        || lower.contains("(")
        || lower.contains(';')
        || lower.contains('{')
        || lower.contains('}')
    {
        return None;
    }
    if url.starts_with('/') && !url.starts_with("//") {
        if is_admin_or_internal_path(url) {
            return None;
        }
        // Prefer known upload/asset paths for theme-controlled images.
        if url.starts_with("/uploads/theme-config/")
            || url.starts_with("/uploads/site-")
            || url.starts_with("/assets/")
            || url.starts_with("/_dioxus/")
        {
            return Some(url.to_string());
        }
        // Allow other same-origin image paths that are not admin/internal.
        if url.rsplit_once('.').is_some_and(|(_, ext)| {
            matches!(
                ext.to_ascii_lowercase().as_str(),
                "png" | "jpg" | "jpeg" | "gif" | "webp" | "svg" | "ico"
            )
        }) {
            return Some(url.to_string());
        }
        return None;
    }
    if lower.starts_with("https://") || lower.starts_with("http://") {
        return Some(url.to_string());
    }
    None
}

pub fn sanitize_inline_style(style: &str) -> String {
    let lower = style.to_ascii_lowercase();
    if lower.contains("expression(")
        || lower.contains("javascript:")
        || lower.contains("-moz-binding")
        || lower.contains("behavior:")
        || lower.contains("@import")
    {
        return String::new();
    }
    style.to_string()
}

/// Prevent theme CSS from breaking out of an inline `<style>` element.
pub fn sanitize_css_bundle(css: &str) -> String {
    let mut out = css.replace('\0', "");
    for needle in ["</style", "</STYLE", "</Style", "<script", "<SCRIPT", "<Script"] {
        while let Some(idx) = out.find(needle) {
            out.replace_range(idx..idx + needle.len(), "/* blocked */");
        }
    }
    out
}

/// Strip executable HTML from `{!! !!}` interpolations.
pub fn sanitize_raw_html(input: &str) -> String {
    if input.len() > MAX_RAW_HTML_BYTES {
        return String::new();
    }

    let mut output = String::with_capacity(input.len());
    let lower = input.to_ascii_lowercase();
    let bytes = input.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        if bytes[i] == b'<' {
            let rest = &lower[i..];
            if rest.starts_with("<script")
                || rest.starts_with("</script")
                || rest.starts_with("<iframe")
                || rest.starts_with("</iframe")
                || rest.starts_with("<object")
                || rest.starts_with("</object")
                || rest.starts_with("<embed")
                || rest.starts_with("</embed")
                || rest.starts_with("<link")
                || rest.starts_with("<meta")
                || rest.starts_with("<base")
                || rest.starts_with("<svg")
                || rest.starts_with("</svg")
                || rest.starts_with("<math")
                || rest.starts_with("</math")
                || rest.starts_with("<form")
                || rest.starts_with("</form")
            {
                if let Some(end) = rest.find('>') {
                    i += end + 1;
                    continue;
                }
                break;
            }
        }
        output.push(input[i..].chars().next().unwrap_or('?'));
        i += input[i..].chars().next().map(|ch| ch.len_utf8()).unwrap_or(1);
    }

    // Drop inline event handlers that survived tag stripping.
    let mut cleaned = output;
    for attr in [
        "onclick",
        "onload",
        "onerror",
        "onmouseover",
        "onfocus",
        "onblur",
        "onsubmit",
    ] {
        cleaned = strip_attr_ci(&cleaned, attr);
    }
    cleaned
}

fn strip_attr_ci(input: &str, attr: &str) -> String {
    let lower = input.to_ascii_lowercase();
    let mut out = String::with_capacity(input.len());
    let mut idx = 0usize;
    while idx < input.len() {
        if let Some(found) = lower[idx..].find(attr) {
            let start = idx + found;
            out.push_str(&input[idx..start]);
            let after_name = start + attr.len();
            let rest = &input[after_name..];
            let trimmed = rest.trim_start();
            if let Some(rest_after_eq) = trimmed.strip_prefix('=') {
                let value = rest_after_eq.trim_start();
                let consumed = rest.len() - value.len();
                if let Some(quote) = value.chars().next().filter(|ch| *ch == '"' || *ch == '\'') {
                    if let Some(end) = value[1..].find(quote) {
                        idx = after_name + consumed + end + 2;
                        continue;
                    }
                } else {
                    let end = value
                        .find(|ch: char| ch.is_whitespace() || ch == '>')
                        .unwrap_or(value.len());
                    idx = after_name + consumed + end;
                    continue;
                }
            }
            idx = after_name;
            continue;
        }
        out.push_str(&input[idx..]);
        break;
    }
    out
}

/// Wrap theme JavaScript so a thrown error cannot tear down the page bootstrap.
#[cfg_attr(not(feature = "server"), allow(dead_code))]
pub fn wrap_theme_javascript(source: &str, feature: &str) -> String {
    let feature = feature
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric() || *ch == '_' || *ch == '-')
        .collect::<String>();
    let mut out = String::with_capacity(source.len() + 160);
    out.push_str("(function(){\n\"use strict\";\ntry {\n");
    out.push_str(source);
    out.push_str("\n} catch (error) {\n");
    out.push_str("console.error('[spot-theme:', '");
    out.push_str(&feature);
    out.push_str("]', error);\n");
    out.push_str("}\n})();\n");
    out
}

pub fn public_content_security_policy() -> &'static str {
    // Theme scripts stay same-origin. Admin routes use a different layout and do not
    // load theme runtime scripts. Remote styles are limited to the fonts/icons CDNs
    // already used by the public shell.
    "default-src 'self'; \
     base-uri 'self'; \
     object-src 'none'; \
     frame-ancestors 'none'; \
     form-action 'self'; \
     script-src 'self'; \
     style-src 'self' 'unsafe-inline' https://fonts.googleapis.com https://cdnjs.cloudflare.com; \
     font-src 'self' https://fonts.gstatic.com https://cdnjs.cloudflare.com data:; \
     img-src 'self' data: https: blob:; \
     connect-src 'self'; \
     frame-src 'none'"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_path_traversal_and_admin_includes() {
        assert!(sanitize_logical_theme_path("../Cargo.toml").is_none());
        assert!(sanitize_logical_theme_path("default/../../Cargo.toml").is_none());
        assert!(sanitize_logical_theme_path("other/index.html").is_none());
        assert_eq!(
            sanitize_logical_theme_path("default/assets/header.html").as_deref(),
            Some("default/assets/header.html")
        );
        assert!(resolve_theme_include("default", "../../secrets").is_err());
        assert_eq!(
            resolve_theme_include("default", "header").as_deref().ok(),
            Some("default/assets/header.html")
        );
    }

    #[test]
    fn blocks_admin_and_javascript_urls() {
        assert_eq!(sanitize_public_url("javascript:alert(1)"), "#");
        assert_eq!(sanitize_public_url("/admin/settings"), "/");
        assert_eq!(sanitize_public_url("/admin"), "/");
        assert_eq!(sanitize_public_url("/forum"), "/forum");
        assert_eq!(
            sanitize_public_url("https://discord.gg/serverspot"),
            "https://discord.gg/serverspot"
        );
        assert!(sanitize_css_image_url("/uploads/theme-config/header_image").is_some());
        assert!(sanitize_css_image_url("url(javascript:alert(1))").is_none());
        assert!(sanitize_css_image_url("/admin/secret.png").is_none());
    }

    #[test]
    fn strips_executable_raw_html() {
        let cleaned = sanitize_raw_html(
            "<p>hi</p><script>alert(1)</script><img src=x onerror=\"alert(1)\">",
        );
        assert!(!cleaned.to_ascii_lowercase().contains("<script"));
        assert!(!cleaned.to_ascii_lowercase().contains("onerror"));
        assert!(cleaned.contains("<p>hi</p>"));
    }

    #[test]
    fn rejects_absolute_and_drive_paths() {
        assert!(sanitize_pack_relative_path("C:/Windows/system32").is_err());
        assert!(sanitize_pack_relative_path("/etc/passwd").is_err());
        assert!(sanitize_pack_relative_path("..\\secrets.html").is_err());
        assert!(sanitize_pack_relative_path("assets/header.exe").is_err());
        assert!(sanitize_pack_relative_path("assets/header.html").is_ok());
    }

    #[test]
    fn admin_referer_detection() {
        assert!(is_admin_ui_referer("http://127.0.0.1:8080/admin/settings/theme"));
        assert!(is_admin_ui_referer("https://example.com/admin"));
        assert!(!is_admin_ui_referer("http://127.0.0.1:8080/"));
        assert!(!is_admin_ui_referer("http://127.0.0.1:8080/forum"));
        assert!(!is_admin_ui_referer("http://evil.test/admin.example.com/"));
        assert!(!is_admin_ui_referer("http://evil.test/foo?next=/admin/settings"));
    }
}
