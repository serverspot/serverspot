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
    embedded_pack_entries, read_theme_source_shared, theme_css_bundle, ThemeFileEntry, ACTIVE_THEME,
};
#[cfg(feature = "server")]
pub(crate) use loader::{
    delete_theme_file_from_disk, list_theme_file_entries, read_theme_file_from_pack,
    write_theme_file_to_disk,
};
pub use registry::is_required_theme_path;
pub use runtime::{render_template, TemplateContext};
pub use schema::{
    css_custom_properties, load_merged_config, SchemaOption, SchemaOptionType, ThemeConfigState,
    ThemeSchema,
};
#[cfg(feature = "server")]
pub(crate) use schema::{persist_theme_config, reset_theme_config_file, store_theme_config_image};
pub use security::public_content_security_policy;
#[cfg(feature = "server")]
pub use security::require_admin_editor_access;
#[cfg(feature = "server")]
pub use security::wrap_theme_javascript;
pub(crate) use security::{sanitize_css_bundle, sanitize_css_image_url};
