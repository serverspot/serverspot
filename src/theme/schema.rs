//! Theme pack schema definitions and site-level config values.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use crate::theme::loader::{read_theme_source, ACTIVE_THEME};

#[allow(dead_code)]
pub const CONFIG_PATH: &str = "data/theme-config.json";
#[allow(dead_code)]
pub const ASSET_DIR: &str = "data/theme-assets";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SchemaOptionType {
    Text,
    Colour,
    Image,
    Link,
}

impl SchemaOptionType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Colour => "colour",
            Self::Image => "image",
            Self::Link => "link",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaOption {
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(rename = "type")]
    pub option_type: SchemaOptionType,
    pub default: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThemeSchema {
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub options: Vec<SchemaOption>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ThemeConfigValues {
    #[serde(default)]
    pub values: BTreeMap<String, String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThemeConfigState {
    pub schema: ThemeSchema,
    pub values: BTreeMap<String, String>,
}

impl ThemeSchema {
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("schema name is required".into());
        }
        let mut seen = BTreeMap::new();
        for option in &self.options {
            validate_option_id(&option.id)?;
            if seen.insert(option.id.clone(), ()).is_some() {
                return Err(format!("duplicate option id `{}`", option.id));
            }
            if option.name.trim().is_empty() {
                return Err(format!("option `{}` needs a name", option.id));
            }
            validate_value_for_type(option.option_type.clone(), &option.default)?;
        }
        Ok(())
    }

    pub fn defaults(&self) -> BTreeMap<String, String> {
        self.options
            .iter()
            .map(|option| (option.id.clone(), option.default.clone()))
            .collect()
    }

    #[allow(dead_code)]
    pub fn option(&self, id: &str) -> Option<&SchemaOption> {
        self.options.iter().find(|option| option.id == id)
    }
}

pub fn validate_option_id(id: &str) -> Result<(), String> {
    if id.is_empty() {
        return Err("option id cannot be empty".into());
    }
    if !id
        .chars()
        .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_')
    {
        return Err(format!(
            "option id `{id}` must use lowercase letters, numbers, and underscores"
        ));
    }
    if id.starts_with('_') || id.ends_with('_') || id.contains("__") {
        return Err(format!("option id `{id}` is not a valid identifier"));
    }
    Ok(())
}

pub fn validate_value_for_type(option_type: SchemaOptionType, value: &str) -> Result<(), String> {
    let value = value.trim();
    match option_type {
        SchemaOptionType::Text => {
            if value.chars().count() > 500 {
                return Err("text values must be 500 characters or fewer".into());
            }
            Ok(())
        }
        SchemaOptionType::Colour => {
            if is_hex_colour(value) {
                Ok(())
            } else {
                Err("colour values must look like #RGB or #RRGGBB".into())
            }
        }
        SchemaOptionType::Image => {
            if value.is_empty() {
                return Err("image values cannot be empty".into());
            }
            crate::theme::security::sanitize_css_image_url(value)
                .map(|_| ())
                .ok_or_else(|| "image values must be a safe site path or http(s) URL".into())
        }
        SchemaOptionType::Link => {
            if value.is_empty() {
                return Err("link values cannot be empty".into());
            }
            let sanitized = crate::theme::security::sanitize_public_url(value);
            if sanitized == "#" || sanitized.is_empty() || crate::theme::security::is_admin_or_internal_path(&sanitized)
            {
                return Err("link values must be a safe public path, http(s) URL, or mailto link".into());
            }
            if sanitized == value || sanitized == value.trim() {
                Ok(())
            } else {
                Err("link values must be a safe public path, http(s) URL, or mailto link".into())
            }
        }
    }
}

fn is_hex_colour(value: &str) -> bool {
    let Some(rest) = value.strip_prefix('#') else {
        return false;
    };
    (rest.len() == 3 || rest.len() == 6) && rest.chars().all(|ch| ch.is_ascii_hexdigit())
}

pub fn parse_schema_json(source: &str) -> Result<ThemeSchema, String> {
    let schema: ThemeSchema =
        serde_json::from_str(source).map_err(|error| format!("invalid schema.json: {error}"))?;
    schema.validate()?;
    Ok(schema)
}

pub fn load_theme_schema(theme: &str) -> Result<ThemeSchema, String> {
    let relative = format!("{theme}/schema.json");
    let source = read_theme_source_prefer_disk(&relative)
        .ok_or_else(|| format!("missing theme schema at {relative}"))?;
    parse_schema_json(&source)
}

pub fn read_theme_source_prefer_disk(relative: &str) -> Option<String> {
    // Public render, includes, and CSS already prefer disk through the shared loader.
    read_theme_source(relative.trim_start_matches('/'))
}

pub fn load_config_overrides() -> ThemeConfigValues {
    #[cfg(feature = "server")]
    {
        match std::fs::read_to_string(CONFIG_PATH) {
            Ok(raw) => serde_json::from_str(&raw).unwrap_or_default(),
            Err(_) => ThemeConfigValues::default(),
        }
    }
    #[cfg(not(feature = "server"))]
    {
        ThemeConfigValues::default()
    }
}

pub fn merge_config_values(
    schema: &ThemeSchema,
    overrides: &ThemeConfigValues,
) -> BTreeMap<String, String> {
    let mut merged = schema.defaults();
    for option in &schema.options {
        if let Some(value) = overrides.values.get(&option.id) {
            if validate_value_for_type(option.option_type.clone(), value).is_ok() {
                merged.insert(option.id.clone(), value.trim().to_string());
            }
        }
    }
    merged
}

pub fn load_merged_config(theme: &str) -> Result<ThemeConfigState, String> {
    let schema = load_theme_schema(theme)?;
    let overrides = load_config_overrides();
    let values = merge_config_values(&schema, &overrides);
    Ok(ThemeConfigState { schema, values })
}

pub fn css_custom_properties(schema: &ThemeSchema, values: &BTreeMap<String, String>) -> String {
    let mut rules = String::from(":root {\n");
    for option in &schema.options {
        let Some(value) = values.get(&option.id) else {
            continue;
        };
        let css_id = option.id.replace('_', "-");
        match option.option_type {
            SchemaOptionType::Colour => {
                rules.push_str(&format!("  --config-{css_id}: {value};\n"));
                if let Some(rgb) = hex_to_rgb_components(value) {
                    rules.push_str(&format!("  --config-{css_id}-rgb: {rgb};\n"));
                }
            }
            SchemaOptionType::Image => {
                let Some(safe_url) = crate::theme::security::sanitize_css_image_url(value) else {
                    continue;
                };
                let escaped = safe_url.replace('\\', "\\\\").replace('"', "\\\"");
                rules.push_str(&format!("  --config-{css_id}: url(\"{escaped}\");\n"));
            }
            SchemaOptionType::Text | SchemaOptionType::Link => {}
        }
    }
    rules.push_str("}\n");
    rules
}

fn hex_to_rgb_components(value: &str) -> Option<String> {
    let rest = value.trim().strip_prefix('#')?;
    let (r, g, b) = match rest.len() {
        3 => {
            let r = u8::from_str_radix(&rest[0..1].repeat(2), 16).ok()?;
            let g = u8::from_str_radix(&rest[1..2].repeat(2), 16).ok()?;
            let b = u8::from_str_radix(&rest[2..3].repeat(2), 16).ok()?;
            (r, g, b)
        }
        6 => {
            let r = u8::from_str_radix(&rest[0..2], 16).ok()?;
            let g = u8::from_str_radix(&rest[2..4], 16).ok()?;
            let b = u8::from_str_radix(&rest[4..6], 16).ok()?;
            (r, g, b)
        }
        _ => return None,
    };
    Some(format!("{r}, {g}, {b}"))
}

#[allow(dead_code)]
pub fn public_image_url(option_id: &str) -> String {
    format!("/uploads/theme-config/{option_id}")
}

#[allow(dead_code)]
pub fn sanitize_option_id_for_path(id: &str) -> Result<String, String> {
    validate_option_id(id)?;
    Ok(id.to_string())
}

#[cfg(feature = "server")]
fn detect_image_content_type(bytes: &[u8]) -> Option<&'static str> {
    if bytes.starts_with(b"\x89PNG\r\n\x1a\n") {
        Some("image/png")
    } else if bytes.starts_with(b"\xff\xd8\xff") {
        Some("image/jpeg")
    } else if bytes.len() >= 12 && bytes.starts_with(b"RIFF") && &bytes[8..12] == b"WEBP" {
        Some("image/webp")
    } else if bytes.starts_with(b"GIF87a") || bytes.starts_with(b"GIF89a") {
        Some("image/gif")
    } else {
        None
    }
}

#[cfg(feature = "server")]
pub(crate) fn persist_theme_config(values: BTreeMap<String, String>) -> Result<(), String> {
    let schema = load_theme_schema(ACTIVE_THEME)?;
    let mut cleaned = BTreeMap::new();
    for option in &schema.options {
        let Some(value) = values.get(&option.id) else {
            continue;
        };
        validate_value_for_type(option.option_type.clone(), value)?;
        cleaned.insert(option.id.clone(), value.trim().to_string());
    }

    std::fs::create_dir_all("data").map_err(|error| error.to_string())?;
    let payload = ThemeConfigValues { values: cleaned };
    let raw = serde_json::to_string_pretty(&payload).map_err(|error| error.to_string())?;
    std::fs::write(CONFIG_PATH, raw).map_err(|error| error.to_string())?;
    Ok(())
}

#[cfg(feature = "server")]
pub(crate) fn reset_theme_config_file() -> Result<(), String> {
    if std::path::Path::new(CONFIG_PATH).exists() {
        std::fs::remove_file(CONFIG_PATH).map_err(|error| error.to_string())?;
    }
    Ok(())
}

#[cfg(feature = "server")]
pub(crate) fn store_theme_config_image(
    option_id: &str,
    file_name: &str,
    bytes: &[u8],
) -> Result<String, String> {
    const MAX_IMAGE_SIZE: usize = 6 * 1024 * 1024;

    let option_id = sanitize_option_id_for_path(option_id.trim())?;
    let schema = load_theme_schema(ACTIVE_THEME)?;
    let option = schema
        .option(&option_id)
        .ok_or_else(|| format!("unknown option `{option_id}`"))?;
    if option.option_type != SchemaOptionType::Image {
        return Err(format!("option `{option_id}` is not an image field"));
    }
    if bytes.is_empty() {
        return Err("The selected image is empty".into());
    }
    if bytes.len() > MAX_IMAGE_SIZE {
        return Err("Images must be 6 MB or smaller".into());
    }

    let content_type = detect_image_content_type(bytes)
        .ok_or_else(|| "Use a valid PNG, JPEG, WebP, or GIF image".to_string())?;
    let _ = file_name;
    std::fs::create_dir_all(ASSET_DIR).map_err(|error| error.to_string())?;
    let bin_path = format!("{ASSET_DIR}/{option_id}.bin");
    let mime_path = format!("{ASSET_DIR}/{option_id}.mime");
    std::fs::write(&bin_path, bytes).map_err(|error| error.to_string())?;
    std::fs::write(&mime_path, content_type).map_err(|error| error.to_string())?;

    let mut overrides = load_config_overrides();
    let public_url = public_image_url(&option_id);
    overrides
        .values
        .insert(option_id.clone(), public_url.clone());
    let raw = serde_json::to_string_pretty(&overrides).map_err(|error| error.to_string())?;
    std::fs::create_dir_all("data").map_err(|error| error.to_string())?;
    std::fs::write(CONFIG_PATH, raw).map_err(|error| error.to_string())?;
    Ok(public_url)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_schema() -> ThemeSchema {
        ThemeSchema {
            name: "Default".into(),
            description: "Test".into(),
            options: vec![
                SchemaOption {
                    id: "primary_colour".into(),
                    name: "Primary".into(),
                    description: "Primary colour".into(),
                    option_type: SchemaOptionType::Colour,
                    default: "#4f46e5".into(),
                },
                SchemaOption {
                    id: "hero_title".into(),
                    name: "Hero title".into(),
                    description: "Title".into(),
                    option_type: SchemaOptionType::Text,
                    default: "Hello".into(),
                },
            ],
        }
    }

    #[test]
    fn validates_unique_ids_and_colours() {
        assert!(sample_schema().validate().is_ok());
        let mut bad = sample_schema();
        bad.options[0].default = "blue".into();
        assert!(bad.validate().is_err());
    }

    #[test]
    fn merges_overrides_over_defaults() {
        let schema = sample_schema();
        let overrides = ThemeConfigValues {
            values: BTreeMap::from([("hero_title".into(), "Season 4".into())]),
        };
        let merged = merge_config_values(&schema, &overrides);
        assert_eq!(merged.get("hero_title").map(String::as_str), Some("Season 4"));
        assert_eq!(
            merged.get("primary_colour").map(String::as_str),
            Some("#4f46e5")
        );
    }

    #[test]
    fn schema_json_sorts_first() {
        let mut entries = vec![
            crate::theme::ThemeFileEntry {
                path: "index.html".into(),
                language: "HTML".into(),
                required: true,
                content: String::new(),
            },
            crate::theme::ThemeFileEntry {
                path: "schema.json".into(),
                language: "JSON".into(),
                required: true,
                content: String::new(),
            },
            crate::theme::ThemeFileEntry {
                path: "styles.css".into(),
                language: "CSS".into(),
                required: true,
                content: String::new(),
            },
        ];
        crate::theme::loader::sort_theme_entries(&mut entries);
        assert_eq!(entries[0].path, "schema.json");
    }

    #[test]
    fn emits_rgb_companions_for_colours() {
        let schema = sample_schema();
        let values = BTreeMap::from([("primary_colour".into(), "#87d1fe".into())]);
        let css = css_custom_properties(&schema, &values);
        assert!(css.contains("--config-primary-colour: #87d1fe;"));
        assert!(css.contains("--config-primary-colour-rgb: 135, 209, 254;"));
    }

    #[test]
    fn rejects_bad_option_ids() {
        assert!(validate_option_id("primary_colour").is_ok());
        assert!(validate_option_id("Primary").is_err());
        assert!(validate_option_id("bad-id").is_err());
    }
}
