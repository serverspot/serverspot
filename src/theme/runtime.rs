use std::{
    collections::{BTreeMap, HashMap, VecDeque},
    hash::{Hash, Hasher},
    sync::{Arc, OnceLock, RwLock},
};

use dioxus::prelude::*;

use crate::theme::ast::{AttrValue, Node, Template, TemplateValue};
use crate::theme::loader::read_theme_source_shared;
use crate::theme::parser::{parse_template, ParseError};
use crate::theme::security::{
    is_forbidden_attr, is_forbidden_tag, resolve_theme_include, sanitize_inline_style,
    sanitize_public_url, sanitize_raw_html, MAX_FOREACH_ITEMS, MAX_INCLUDE_DEPTH,
};

#[derive(Clone, Default)]
pub struct TemplateContext {
    values: Arc<BTreeMap<String, TemplateValue>>,
    locals: BTreeMap<String, TemplateValue>,
    theme: Arc<str>,
    config: Arc<BTreeMap<String, String>>,
}

impl TemplateContext {
    pub fn new(theme: impl Into<String>) -> Self {
        Self {
            values: Arc::new(BTreeMap::new()),
            locals: BTreeMap::new(),
            theme: Arc::from(theme.into()),
            config: Arc::new(BTreeMap::new()),
        }
    }

    pub fn insert(&mut self, key: impl Into<String>, value: TemplateValue) {
        Arc::make_mut(&mut self.values).insert(key.into(), value);
    }

    pub fn set_config(&mut self, values: BTreeMap<String, String>) {
        self.config = Arc::new(values);
    }

    fn insert_local(&mut self, key: impl Into<String>, value: TemplateValue) {
        self.locals.insert(key.into(), value);
    }

    pub fn resolve(&self, expr: &str) -> TemplateValue {
        let expr = expr.trim();
        if expr.is_empty() {
            return TemplateValue::Null;
        }
        // Support `not foo`
        if let Some(rest) = expr.strip_prefix("not ") {
            return TemplateValue::Bool(!self.resolve(rest.trim()).as_bool());
        }
        if let Some(id) = parse_config_call(expr) {
            return self
                .config
                .get(id)
                .cloned()
                .map(TemplateValue::from)
                .unwrap_or(TemplateValue::Null);
        }
        if let Some(inner) = parse_display_url_call(expr) {
            let url = self.resolve(inner).as_string();
            return TemplateValue::from(strip_url_scheme(&url));
        }
        self.resolve_ref(expr)
            .cloned()
            .unwrap_or(TemplateValue::Null)
    }

    fn resolve_ref(&self, expr: &str) -> Option<&TemplateValue> {
        let expr = expr.trim();
        if expr.is_empty() {
            return None;
        }
        if let Some(value) = self.locals.get(expr).or_else(|| self.values.get(expr)) {
            return Some(value);
        }
        let (root, rest) = expr.split_once('.')?;
        self.locals
            .get(root)
            .or_else(|| self.values.get(root))?
            .get_path_ref(rest)
    }
}

const TEMPLATE_CACHE_CAPACITY: usize = 128;

#[derive(Clone)]
struct CachedTemplate {
    source: Arc<str>,
    parsed: Result<Arc<Template>, ParseError>,
}

#[derive(Default)]
struct TemplateCache {
    entries: HashMap<u64, CachedTemplate>,
    insertion_order: VecDeque<u64>,
}

impl TemplateCache {
    fn get(&self, key: u64, source: &str) -> Option<Result<Arc<Template>, ParseError>> {
        let cached = self.entries.get(&key)?;
        (cached.source.as_ref() == source).then(|| cached.parsed.clone())
    }

    fn insert(&mut self, key: u64, source: Arc<str>, parsed: Result<Arc<Template>, ParseError>) {
        if self.entries.remove(&key).is_some() {
            self.insertion_order.retain(|existing| *existing != key);
        }
        while self.entries.len() >= TEMPLATE_CACHE_CAPACITY {
            if let Some(oldest) = self.insertion_order.pop_front() {
                self.entries.remove(&oldest);
            } else {
                break;
            }
        }
        self.insertion_order.push_back(key);
        self.entries.insert(key, CachedTemplate { source, parsed });
    }
}

fn template_cache() -> &'static RwLock<TemplateCache> {
    static CACHE: OnceLock<RwLock<TemplateCache>> = OnceLock::new();
    CACHE.get_or_init(|| RwLock::new(TemplateCache::default()))
}

fn source_hash(source: &str) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    source.hash(&mut hasher);
    hasher.finish()
}

fn parse_cached(source: &str) -> Result<Arc<Template>, ParseError> {
    let key = source_hash(source);
    if let Some(parsed) = template_cache()
        .read()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .get(key, source)
    {
        return parsed;
    }

    let parsed = parse_template(source).map(Arc::new);
    template_cache()
        .write()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .insert(key, Arc::from(source), parsed.clone());
    parsed
}

#[cfg(feature = "server")]
pub(crate) fn clear_template_cache() {
    let mut cache = template_cache()
        .write()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    cache.entries.clear();
    cache.insertion_order.clear();
}

#[derive(Clone, Default)]
struct RenderState {
    include_stack: Vec<String>,
}

fn parse_config_call(expr: &str) -> Option<&str> {
    let rest = expr.strip_prefix("config(")?.trim();
    let rest = rest.strip_suffix(')')?.trim();
    if rest.len() < 2 {
        return None;
    }
    let quote = rest.chars().next()?;
    if quote != '"' && quote != '\'' {
        return None;
    }
    let inner = rest.strip_prefix(quote)?.strip_suffix(quote)?;
    if inner.is_empty()
        || inner.contains(quote)
        || !inner
            .chars()
            .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_')
    {
        return None;
    }
    Some(inner)
}

fn parse_display_url_call(expr: &str) -> Option<&str> {
    let rest = expr.strip_prefix("display_url(")?.trim();
    let rest = rest.strip_suffix(')')?.trim();
    if rest.is_empty() {
        return None;
    }
    Some(rest)
}

fn strip_url_scheme(url: &str) -> String {
    let url = url.trim();
    url.strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))
        .unwrap_or(url)
        .to_string()
}

pub fn render_template(source: &str, ctx: &TemplateContext) -> Element {
    match parse_cached(source) {
        Ok(template) => render_nodes(&template.nodes, ctx, &RenderState::default()),
        Err(err) => rsx! {
            div { class: "spot-error", "Template error: {err}" }
        },
    }
}

#[allow(dead_code)]
pub fn render_parsed(template: &Template, ctx: &TemplateContext) -> Element {
    render_nodes(&template.nodes, ctx, &RenderState::default())
}

fn render_nodes(nodes: &[Node], ctx: &TemplateContext, state: &RenderState) -> Element {
    rsx! {
        {nodes.iter().map(|node| render_node(node, ctx, state))}
    }
}

fn render_node(node: &Node, ctx: &TemplateContext, state: &RenderState) -> Element {
    match node {
        Node::Text(text) => {
            if text.is_empty() {
                return rsx! {};
            }
            rsx! { "{text}" }
        }
        Node::Interp { expr, raw } => {
            let value = ctx.resolve(expr).as_string();
            if *raw {
                let value = sanitize_raw_html(&value);
                rsx! {
                    span { dangerous_inner_html: "{value}" }
                }
            } else {
                rsx! { "{value}" }
            }
        }
        Node::If { arms, else_body } => {
            for (cond, body) in arms {
                if ctx.resolve(cond).as_bool() {
                    return render_nodes(body, ctx, state);
                }
            }
            render_nodes(else_body, ctx, state)
        }
        Node::ForEach {
            collection,
            item,
            body,
        } => {
            let Some(TemplateValue::Array(items)) = ctx.resolve_ref(collection) else {
                return rsx! {};
            };
            let rendered = items
                .iter()
                .take(MAX_FOREACH_ITEMS)
                .enumerate()
                .map(|(index, value)| {
                    let mut child_ctx = ctx.clone();
                    child_ctx.insert_local(item.clone(), value.clone());
                    child_ctx.insert_local("loop.index", TemplateValue::Number(index as f64));
                    render_nodes(body, &child_ctx, state)
                })
                .collect::<Vec<_>>();
            rsx! {
                {rendered.into_iter()}
            }
        }
        Node::Include { path } => {
            let normalized = match resolve_theme_include(&ctx.theme, path) {
                Ok(path) => path,
                Err(error) => {
                    return rsx! {
                        div { class: "spot-error", "Blocked include: {error}" }
                    };
                }
            };
            if state.include_stack.len() >= MAX_INCLUDE_DEPTH {
                return rsx! {
                    div { class: "spot-error", "Include depth exceeded while loading: {normalized}" }
                };
            }
            if state.include_stack.iter().any(|entry| entry == &normalized) {
                return rsx! {
                    div { class: "spot-error", "Circular include: {normalized}" }
                };
            }
            match read_theme_source_shared(&normalized) {
                Some(source) => match parse_cached(&source) {
                    Ok(template) => {
                        let mut child_state = state.clone();
                        child_state.include_stack.push(normalized);
                        render_nodes(&template.nodes, ctx, &child_state)
                    }
                    Err(err) => rsx! {
                        div { class: "spot-error", "Template error: {err}" }
                    },
                },
                None => rsx! {
                    div { class: "spot-error", "Missing include: {normalized}" }
                },
            }
        }
        Node::Element {
            tag,
            attrs,
            children,
            self_closing,
        } => render_element(tag, attrs, children, *self_closing, ctx, state),
    }
}

#[derive(Default)]
struct ResolvedAttrs {
    class: String,
    id: String,
    href: String,
    src: String,
    alt: String,
    style: String,
    title: String,
    target: String,
    rel: String,
    role: String,
    aria_label: String,
    input_type: String,
    placeholder: String,
    autocomplete: String,
}

fn resolve_attrs(attrs: &[(String, AttrValue)], ctx: &TemplateContext) -> ResolvedAttrs {
    let mut resolved = ResolvedAttrs::default();
    for (key, value) in attrs {
        if is_forbidden_attr(key) {
            continue;
        }
        let target = match key.as_str() {
            "class" => &mut resolved.class,
            "id" => &mut resolved.id,
            "href" => &mut resolved.href,
            "src" => &mut resolved.src,
            "alt" => &mut resolved.alt,
            "style" => &mut resolved.style,
            "title" => &mut resolved.title,
            "target" => &mut resolved.target,
            "rel" => &mut resolved.rel,
            "role" => &mut resolved.role,
            "aria-label" => &mut resolved.aria_label,
            "type" => &mut resolved.input_type,
            "placeholder" => &mut resolved.placeholder,
            "autocomplete" => &mut resolved.autocomplete,
            _ => continue,
        };
        *target = match value {
            AttrValue::Literal(value) => value.clone(),
            AttrValue::Interp { expr, .. } => ctx.resolve(expr).as_string(),
        };
    }
    if resolved.input_type.is_empty() {
        resolved.input_type.push_str("text");
    }
    resolved.href = sanitize_public_url(&resolved.href);
    resolved.src = sanitize_public_url(&resolved.src);
    resolved.style = sanitize_inline_style(&resolved.style);
    if resolved.target == "_blank" && !resolved.rel.contains("noopener") {
        if resolved.rel.is_empty() {
            resolved.rel = "noopener noreferrer".into();
        } else {
            resolved.rel.push_str(" noopener noreferrer");
        }
    }
    resolved
}

fn render_element(
    tag: &str,
    attrs: &[(String, AttrValue)],
    children: &[Node],
    self_closing: bool,
    ctx: &TemplateContext,
    state: &RenderState,
) -> Element {
    if is_forbidden_tag(tag) {
        return rsx! {
            div { class: "spot-error", "Blocked theme tag: {tag}" }
        };
    }

    let ResolvedAttrs {
        class,
        id,
        href,
        src,
        alt,
        style,
        title,
        target,
        rel,
        role,
        aria_label,
        input_type,
        placeholder,
        autocomplete,
    } = resolve_attrs(attrs, ctx);
    let body = if self_closing {
        rsx! {}
    } else {
        render_nodes(children, ctx, state)
    };

    match tag {
        "div" => rsx! {
            div { class: "{class}", id: "{id}", style: "{style}", {body} }
        },
        "span" => rsx! {
            span { class: "{class}", id: "{id}", style: "{style}", {body} }
        },
        "p" => rsx! {
            p { class: "{class}", id: "{id}", style: "{style}", {body} }
        },
        "a" => rsx! {
            a {
                class: "{class}",
                id: "{id}",
                href: "{href}",
                target: "{target}",
                rel: "{rel}",
                title: "{title}",
                aria_label: "{aria_label}",
                style: "{style}",
                {body}
            }
        },
        "h1" => rsx! {
            h1 { class: "{class}", id: "{id}", style: "{style}", {body} }
        },
        "h2" => rsx! {
            h2 { class: "{class}", id: "{id}", style: "{style}", {body} }
        },
        "h3" => rsx! {
            h3 { class: "{class}", id: "{id}", style: "{style}", {body} }
        },
        "h4" => rsx! {
            h4 { class: "{class}", id: "{id}", style: "{style}", {body} }
        },
        "ul" => rsx! {
            ul { class: "{class}", id: "{id}", style: "{style}", {body} }
        },
        "ol" => rsx! {
            ol { class: "{class}", id: "{id}", style: "{style}", {body} }
        },
        "li" => rsx! {
            li { class: "{class}", id: "{id}", style: "{style}", {body} }
        },
        "section" => rsx! {
            section { class: "{class}", id: "{id}", style: "{style}", {body} }
        },
        "header" => rsx! {
            header { class: "{class}", id: "{id}", style: "{style}", {body} }
        },
        "footer" => rsx! {
            footer { class: "{class}", id: "{id}", style: "{style}", {body} }
        },
        "main" => rsx! {
            main { class: "{class}", id: "{id}", style: "{style}", {body} }
        },
        "nav" => rsx! {
            nav { class: "{class}", id: "{id}", style: "{style}", {body} }
        },
        "article" => rsx! {
            article { class: "{class}", id: "{id}", style: "{style}", {body} }
        },
        "aside" => rsx! {
            aside { class: "{class}", id: "{id}", style: "{style}", {body} }
        },
        "button" => {
            rsx! {
                button {
                    class: "{class}",
                    id: "{id}",
                    r#type: "button",
                    style: "{style}",
                    {body}
                }
            }
        }
        "strong" => rsx! {
            strong { class: "{class}", {body} }
        },
        "em" => rsx! {
            em { class: "{class}", {body} }
        },
        "i" => rsx! {
            i { class: "{class}", title: "{title}", {body} }
        },
        "small" => rsx! {
            small { class: "{class}", {body} }
        },
        "code" => rsx! {
            code { class: "{class}", {body} }
        },
        "pre" => rsx! {
            pre { class: "{class}", {body} }
        },
        "blockquote" => rsx! {
            blockquote { class: "{class}", {body} }
        },
        "hr" => rsx! {
            hr { class: "{class}" }
        },
        "br" => rsx! {
            br {}
        },
        "img" => rsx! {
            img {
                class: "{class}",
                src: "{src}",
                alt: "{alt}",
                style: "{style}",
            }
        },
        "table" => rsx! {
            table { class: "{class}", {body} }
        },
        "thead" => rsx! {
            thead { class: "{class}", {body} }
        },
        "tbody" => rsx! {
            tbody { class: "{class}", {body} }
        },
        "tr" => rsx! {
            tr { class: "{class}", {body} }
        },
        "th" => rsx! {
            th { class: "{class}", {body} }
        },
        "td" => rsx! {
            td { class: "{class}", {body} }
        },
        "form" => rsx! {
            form { class: "{class}", {body} }
        },
        "label" => rsx! {
            label { class: "{class}", {body} }
        },
        "input" => rsx! {
            input {
                class: "{class}",
                id: "{id}",
                r#type: "{input_type}",
                placeholder: "{placeholder}",
                autocomplete: "{autocomplete}",
            }
        },
        "textarea" => rsx! {
            textarea { class: "{class}", {body} }
        },
        "time" => rsx! {
            time { class: "{class}", {body} }
        },
        _ => rsx! {
            div {
                class: "{class}",
                id: "{id}",
                style: "{style}",
                "data-spot-tag": "{tag}",
                role: "{role}",
                {body}
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_config_calls() {
        let mut ctx = TemplateContext::new("default");
        let mut values = BTreeMap::new();
        values.insert("hero_title".into(), "Season 4".into());
        values.insert("primary_colour".into(), "#4f46e5".into());
        values.insert("discord_url".into(), "https://discord.gg/serverspot".into());
        ctx.set_config(values);
        assert_eq!(
            ctx.resolve("config(\"hero_title\")").as_string(),
            "Season 4"
        );
        assert_eq!(
            ctx.resolve("config('primary_colour')").as_string(),
            "#4f46e5"
        );
        assert!(matches!(
            ctx.resolve("config(\"missing\")"),
            TemplateValue::Null
        ));
        assert!(matches!(
            ctx.resolve("config(hero_title)"),
            TemplateValue::Null
        ));
        assert_eq!(
            ctx.resolve("display_url(config(\"discord_url\"))")
                .as_string(),
            "discord.gg/serverspot"
        );
        assert_eq!(
            strip_url_scheme("http://example.com/path"),
            "example.com/path"
        );
    }

    #[test]
    fn parsed_templates_are_reused_from_the_cache() {
        let source = "<div class=\"cache-probe\">cached</div>";
        let first = parse_cached(source).expect("first parse should succeed");
        let second = parse_cached(source).expect("cached parse should succeed");
        assert!(Arc::ptr_eq(&first, &second));
    }

    #[test]
    fn cloned_contexts_share_base_values_and_support_loop_locals() {
        let mut parent = TemplateContext::new("default");
        parent.insert(
            "items",
            TemplateValue::Array(vec![TemplateValue::from("one")]),
        );
        let mut child = parent.clone();
        assert!(Arc::ptr_eq(&parent.values, &child.values));

        child.insert_local("item", TemplateValue::from("one"));
        child.insert_local("loop.index", TemplateValue::Number(0.0));
        assert_eq!(child.resolve("item").as_string(), "one");
        assert_eq!(child.resolve("loop.index").as_string(), "0");
        assert!(Arc::ptr_eq(&parent.values, &child.values));
    }

    #[test]
    #[ignore = "manual microbenchmark"]
    fn benchmark_cached_template_parsing() {
        use std::time::Instant;

        const ITERATIONS: usize = 2_000;
        let source = include_str!("../../themes/default/index.html");

        let uncached_start = Instant::now();
        for _ in 0..ITERATIONS {
            std::hint::black_box(parse_template(source).expect("template should parse"));
        }
        let uncached = uncached_start.elapsed();

        let cached_start = Instant::now();
        for _ in 0..ITERATIONS {
            std::hint::black_box(parse_cached(source).expect("template should be cached"));
        }
        let cached = cached_start.elapsed();

        println!(
            "{ITERATIONS} parses: uncached={uncached:?}, cached={cached:?}, speedup={:.1}x",
            uncached.as_secs_f64() / cached.as_secs_f64()
        );
    }
}
