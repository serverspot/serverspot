//! Spot: Blade-like theme templates rendered into Dioxus Elements.

mod ast;
mod loader;
mod parser;
mod registry;
mod runtime;
mod schema;
mod security;

pub use ast::TemplateValue;
pub use loader::{
    delete_theme_file, embedded_pack_entries, list_theme_files, read_theme_source_shared,
    theme_css_bundle, write_theme_file, ThemeFileEntry, ACTIVE_THEME,
};
pub use registry::is_required_theme_path;
pub use runtime::{render_template, TemplateContext};
pub use schema::{
    css_custom_properties, get_theme_config, load_merged_config, reset_theme_config,
    save_theme_config, upload_theme_config_image, SchemaOption, SchemaOptionType, ThemeConfigState,
    ThemeSchema,
};
pub use security::public_content_security_policy;
#[cfg(feature = "server")]
pub use security::require_admin_editor_access;
#[cfg(feature = "server")]
pub use security::wrap_theme_javascript;
pub(crate) use security::{sanitize_css_bundle, sanitize_css_image_url};
