use std::borrow::Cow;
use std::collections::BTreeMap;

use dioxus::prelude::*;

use crate::components::page::{DataPanel, PageHeader};
use crate::components::syntax::highlighted_html;
use crate::components::ui::*;
use crate::router::Route;
use crate::server_funcs::{
    delete_theme_file, get_theme_config, list_theme_files, reset_theme_config, save_theme_config,
    upload_theme_config_image, write_theme_file,
};
use crate::theme::{SchemaOption, SchemaOptionType, ThemeConfigState};

#[component]
pub fn SettingsTheme() -> Element {
    let navigator = use_navigator();
    let mut state = use_signal(|| None::<ThemeConfigState>);
    let mut draft = use_signal(BTreeMap::<String, String>::new);
    let mut dirty = use_signal(|| false);
    let mut status = use_signal(String::new);
    let mut saving = use_signal(|| false);
    let uploading = use_signal(String::new);
    let mut hydrated = use_signal(|| false);

    use_effect(move || {
        if hydrated() {
            return;
        }
        hydrated.set(true);
        spawn(async move {
            match get_theme_config().await {
                Ok(config) => {
                    draft.set(config.values.clone());
                    state.set(Some(config));
                    status.set(String::new());
                }
                Err(error) => {
                    status.set(format!("Could not load theme schema: {error}"));
                }
            }
        });
    });

    let Some(config) = state() else {
        return rsx! {
            PageHeader {
                title: "Theme",
                subtitle: "Loading theme customisation options…",
            }
            DataPanel {
                title: "Customise theme",
                p { class: "text-sm text-text-muted",
                    if status().is_empty() { "Loading schema…" } else { "{status}" }
                }
            }
        };
    };

    let primary = draft()
        .get("primary_colour")
        .cloned()
        .unwrap_or_else(|| "#4f46e5".into());
    let accent = draft()
        .get("accent_colour")
        .cloned()
        .unwrap_or_else(|| "#87d1fe".into());
    let surface = draft()
        .get("surface_colour")
        .cloned()
        .unwrap_or_else(|| "#ffffff".into());
    let body_bg = draft()
        .get("body_bg")
        .cloned()
        .unwrap_or_else(|| "#f1f3f9".into());
    let header_image = draft()
        .get("header_image")
        .cloned()
        .unwrap_or_default();
    let hero_title = draft()
        .get("hero_title")
        .cloned()
        .unwrap_or_else(|| "Welcome".into());

    rsx! {
        PageHeader {
            title: "Theme",
            subtitle: "Tune colours, images, and copy from the theme schema. Open the file editor when you need deeper control.",
            action: rsx! {
                div { class: "flex flex-wrap gap-2",
                    Button {
                        variant: ButtonVariant::Secondary,
                        disabled: saving() || !dirty(),
                        onclick: move |_| {
                            saving.set(true);
                            status.set("Saving…".into());
                            let values = draft();
                            spawn(async move {
                                match save_theme_config(values).await {
                                    Ok(()) => {
                                        if let Ok(config) = get_theme_config().await {
                                            draft.set(config.values.clone());
                                            state.set(Some(config));
                                        }
                                        dirty.set(false);
                                        status.set("Theme customisation saved.".into());
                                    }
                                    Err(error) => {
                                        status.set(format!("Save failed: {error}"));
                                    }
                                }
                                saving.set(false);
                            });
                        },
                        if dirty() { "Save changes*" } else { "Save changes" }
                    }
                    Button {
                        variant: ButtonVariant::Ghost,
                        disabled: saving(),
                        onclick: move |_| {
                            saving.set(true);
                            status.set("Resetting…".into());
                            spawn(async move {
                                match reset_theme_config().await {
                                    Ok(()) => {
                                        if let Ok(config) = get_theme_config().await {
                                            draft.set(config.values.clone());
                                            state.set(Some(config));
                                        }
                                        dirty.set(false);
                                        status.set("Reset to theme defaults.".into());
                                    }
                                    Err(error) => {
                                        status.set(format!("Reset failed: {error}"));
                                    }
                                }
                                saving.set(false);
                            });
                        },
                        "Reset defaults"
                    }
                    Button {
                        onclick: move |_| {
                            navigator.push(Route::SettingsThemeEditor {});
                        },
                        "Edit theme"
                    }
                }
            },
        }

        if !status().is_empty() {
            p { class: "mb-4 text-sm text-text-muted", "{status}" }
        }

        section {
            class: "mb-6 overflow-hidden rounded-squircle-lg border border-border-subtle bg-surface/20",
            div {
                class: "relative h-36 overflow-hidden border-b border-border-subtle sm:h-44",
                style: "background:
                    linear-gradient(180deg, rgba(15,18,28,0.18), rgba(15,18,28,0.55)),
                    url('{header_image}') center/cover no-repeat,
                    {body_bg};",
                div {
                    class: "absolute inset-0 flex flex-col items-center justify-center gap-2 px-4 text-center",
                    span {
                        class: "rounded-full px-3 py-1 text-[11px] font-semibold tracking-[0.14em] uppercase text-white/90",
                        style: "background: color-mix(in srgb, {primary} 55%, transparent);",
                        "{config.schema.name}"
                    }
                    h2 { class: "text-xl font-semibold tracking-tight text-white sm:text-2xl", "{hero_title}" }
                    div { class: "mt-1 flex gap-2",
                        span {
                            class: "rounded-md px-3 py-1.5 text-xs font-semibold text-white",
                            style: "background: {primary};",
                            "Primary"
                        }
                        span {
                            class: "rounded-md px-3 py-1.5 text-xs font-semibold text-white",
                            style: "background: {accent};",
                            "Accent"
                        }
                        span {
                            class: "rounded-md border border-white/30 px-3 py-1.5 text-xs font-semibold text-white",
                            style: "background: color-mix(in srgb, {surface} 35%, transparent);",
                            "Surface"
                        }
                    }
                }
            }
            div { class: "grid gap-3 p-4 sm:grid-cols-3",
                div {
                    class: "rounded-squircle border border-border-subtle bg-bg/40 p-3",
                    p { class: "text-[11px] uppercase tracking-[0.12em] text-text-muted", "Schema" }
                    p { class: "mt-1 text-sm font-medium", "{config.schema.name}" }
                }
                div {
                    class: "rounded-squircle border border-border-subtle bg-bg/40 p-3",
                    p { class: "text-[11px] uppercase tracking-[0.12em] text-text-muted", "Options" }
                    p { class: "mt-1 text-sm font-medium", "{config.schema.options.len()}" }
                }
                div {
                    class: "rounded-squircle border border-border-subtle bg-bg/40 p-3",
                    p { class: "text-[11px] uppercase tracking-[0.12em] text-text-muted", "Templates" }
                    p { class: "mt-1 text-sm font-medium", "config(\"id\")" }
                }
            }
        }

        if !config.schema.description.is_empty() {
            p { class: "mb-5 max-w-3xl text-sm text-text-muted", "{config.schema.description}" }
        }

        DataPanel {
            title: "Customisation options",
            div { class: "grid gap-4 lg:grid-cols-2",
                for option in config.schema.options.clone() {
                    ThemeOptionCard {
                        option,
                        draft,
                        dirty,
                        uploading,
                        status,
                    }
                }
            }
        }

        section {
            class: "mt-6 rounded-squircle-lg border border-border-subtle bg-surface/20 p-4 sm:p-5",
            div { class: "flex flex-wrap items-center justify-between gap-3",
                div {
                    h3 { class: "text-sm font-semibold", "Advanced theme files" }
                    p { class: "mt-1 max-w-2xl text-sm text-text-muted",
                        "Edit HTML, CSS, JS, and schema.json directly. schema.json always stays at the top of the file list."
                    }
                }
                Button {
                    variant: ButtonVariant::Secondary,
                    onclick: move |_| {
                        navigator.push(Route::SettingsThemeEditor {});
                    },
                    "Edit theme"
                }
            }
        }
    }
}

#[component]
fn ThemeOptionCard(
    option: SchemaOption,
    mut draft: Signal<BTreeMap<String, String>>,
    mut dirty: Signal<bool>,
    mut uploading: Signal<String>,
    mut status: Signal<String>,
) -> Element {
    let id = option.id.clone();
    let id_a = option.id.clone();
    let id_c = option.id.clone();
    let id_for_upload = option.id.clone();
    let hint = format!("{{{{ config(\"{}\") }}}}", option.id);
    let value = draft()
        .get(&option.id)
        .cloned()
        .unwrap_or_else(|| option.default.clone());
    let type_label = option.option_type.as_str();
    let is_uploading = uploading() == option.id;

    rsx! {
        div {
            class: "rounded-squircle border border-border-subtle bg-bg/30 p-4",
            div { class: "mb-3 flex items-start justify-between gap-3",
                div {
                    p { class: "text-sm font-semibold", "{option.name}" }
                    p { class: "mt-1 text-xs text-text-muted", "{option.description}" }
                }
                span {
                    class: "rounded-full border border-border-subtle px-2 py-0.5 text-[10px] font-semibold uppercase tracking-[0.12em] text-text-muted",
                    "{type_label}"
                }
            }
            match option.option_type {
                SchemaOptionType::Colour => {
                    rsx! {
                        ThemeColourPicker {
                            value: value.clone(),
                            onchange: move |next: String| {
                                draft.write().insert(id_a.clone(), next);
                                dirty.set(true);
                                status.set("Unsaved changes".into());
                            },
                        }
                        p { class: "mt-2 font-mono text-[11px] text-text-muted", "{hint}" }
                    }
                }
                SchemaOptionType::Image => {
                    rsx! {
                        div { class: "space-y-3",
                            div {
                                class: "h-28 overflow-hidden rounded-squircle border border-border-subtle bg-surface/40",
                                style: "background: url('{value}') center/cover no-repeat, #1b2130;",
                            }
                            input {
                                r#type: "text",
                                class: "ui-input ui-squircle w-full text-xs",
                                value: "{value}",
                                oninput: move |event| {
                                    let next = event.value();
                                    draft.write().insert(id_c.clone(), next);
                                    dirty.set(true);
                                    status.set("Unsaved changes".into());
                                },
                            }
                            label {
                                class: "inline-flex cursor-pointer items-center gap-2 rounded-squircle border border-border-subtle px-3 py-2 text-xs font-medium hover:bg-surface/40",
                                if is_uploading { "Uploading…" } else { "Upload image" }
                                input {
                                    r#type: "file",
                                    accept: "image/png,image/jpeg,image/webp,image/gif",
                                    class: "hidden",
                                    onchange: move |event| {
                                        let files = event.files();
                                        let Some(file) = files.first().cloned() else {
                                            return;
                                        };
                                        let file_name = file.name();
                                        let option_id = id_for_upload.clone();
                                        uploading.set(option_id.clone());
                                        status.set("Uploading image…".into());
                                        spawn(async move {
                                            let result = match file.read_bytes().await {
                                                Ok(bytes) => {
                                                    upload_theme_config_image(
                                                        option_id.clone(),
                                                        file_name,
                                                        bytes.to_vec(),
                                                    )
                                                    .await
                                                }
                                                Err(error) => Err(ServerFnError::new(error.to_string())),
                                            };
                                            match result {
                                                Ok(url) => {
                                                    draft.write().insert(option_id.clone(), url);
                                                    status.set("Image uploaded successfully.".into());
                                                }
                                                Err(error) => {
                                                    status.set(format!("Upload failed: {error}"));
                                                }
                                            }
                                            uploading.set(String::new());
                                        });
                                    },
                                }
                            }
                            p { class: "font-mono text-[11px] text-text-muted", "{hint}" }
                        }
                    }
                }
                SchemaOptionType::Text | SchemaOptionType::Link => {
                    rsx! {
                        input {
                            r#type: if matches!(option.option_type, SchemaOptionType::Link) { "url" } else { "text" },
                            class: "ui-input ui-squircle w-full",
                            value: "{value}",
                            oninput: move |event| {
                                let next = event.value();
                                draft.write().insert(id.clone(), next);
                                dirty.set(true);
                                status.set("Unsaved changes".into());
                            },
                        }
                        p { class: "mt-2 font-mono text-[11px] text-text-muted", "{hint}" }
                    }
                }
            }
        }
    }
}

#[component]
fn ThemeColourPicker(value: String, onchange: EventHandler<String>) -> Element {
    const PRESETS: [&str; 20] = [
        "#87d1fe", "#ef4444", "#f97316", "#eab308", "#22c55e",
        "#14b8a6", "#06b6d4", "#3b82f6", "#6366f1", "#8b5cf6",
        "#a855f7", "#ec4899", "#f43f5e", "#1f2937", "#475569",
        "#94a3b8", "#ffffff", "#f8fafc", "#e2e8f0", "#0f172a",
    ];

    let mut open = use_signal(|| false);
    let mut hex_draft = use_signal(|| value.trim_start_matches('#').to_uppercase());
    let display_value = if is_hex_colour_input(&value) {
        value.clone()
    } else {
        "#000000".to_string()
    };

    rsx! {
        div { class: "relative",
            button {
                r#type: "button",
                class: "group flex w-full items-center gap-3 rounded-squircle border border-border-subtle bg-surface/25 p-2 text-left transition hover:border-border hover:bg-surface/40",
                aria_expanded: open(),
                onclick: move |_| open.toggle(),
                span {
                    class: "relative h-11 w-11 shrink-0 overflow-hidden rounded-lg border border-black/10 shadow-sm",
                    style: "background: {display_value};",
                    span {
                        class: "absolute inset-x-0 bottom-0 h-1/2 bg-gradient-to-t from-black/15 to-transparent",
                    }
                }
                span { class: "min-w-0 flex-1",
                    span { class: "block text-[11px] font-medium uppercase tracking-[0.12em] text-text-muted",
                        "Selected colour"
                    }
                    span { class: "mt-0.5 block font-mono text-sm font-semibold uppercase", "{value}" }
                }
                span {
                    class: "grid h-8 w-8 place-items-center rounded-md border border-border-subtle text-text-muted transition group-hover:text-text",
                    if open() { "−" } else { "+" }
                }
            }

            if open() {
                div {
                    class: "absolute left-0 right-0 z-30 mt-2 rounded-squircle-lg border border-border bg-bg p-3 shadow-2xl shadow-black/20",
                    div { class: "mb-3 flex items-center justify-between",
                        div {
                            p { class: "text-xs font-semibold", "Choose a colour" }
                            p { class: "mt-0.5 text-[11px] text-text-muted", "Pick a preset or enter a hex value." }
                        }
                        button {
                            r#type: "button",
                            class: "grid h-7 w-7 place-items-center rounded-md text-sm text-text-muted hover:bg-surface hover:text-text",
                            aria_label: "Close colour picker",
                            onclick: move |_| open.set(false),
                            "×"
                        }
                    }

                    div { class: "grid grid-cols-10 gap-1.5",
                        for colour in PRESETS {
                            button {
                                r#type: "button",
                                class: if value.eq_ignore_ascii_case(colour) {
                                    "relative aspect-square rounded-md border-2 border-text shadow-sm"
                                } else {
                                    "relative aspect-square rounded-md border border-black/10 shadow-sm transition hover:scale-110 hover:border-text/50"
                                },
                                style: "background: {colour};",
                                title: "{colour}",
                                aria_label: "Use {colour}",
                                onclick: move |_| {
                                    hex_draft.set(colour.trim_start_matches('#').to_uppercase());
                                    onchange.call(colour.to_string());
                                },
                                if value.eq_ignore_ascii_case(colour) {
                                    span {
                                        class: "absolute inset-0 grid place-items-center text-[10px]",
                                        style: if colour == "#ffffff" || colour == "#f8fafc" || colour == "#e2e8f0" {
                                            "color:#111827"
                                        } else {
                                            "color:#ffffff"
                                        },
                                        "✓"
                                    }
                                }
                            }
                        }
                    }

                    div { class: "mt-3 flex items-center gap-2 border-t border-border-subtle pt-3",
                        span {
                            class: "h-9 w-9 shrink-0 rounded-md border border-border-subtle",
                            style: "background: {display_value};",
                        }
                        div { class: "relative min-w-0 flex-1",
                            span {
                                class: "pointer-events-none absolute left-3 top-1/2 -translate-y-1/2 font-mono text-xs text-text-muted",
                                "#"
                            }
                            input {
                                r#type: "text",
                                class: "ui-input ui-squircle w-full pl-6 font-mono text-xs uppercase",
                                maxlength: 6,
                                value: "{hex_draft}",
                                oninput: move |event| {
                                    let raw = event.value();
                                    let cleaned: String = raw
                                        .chars()
                                        .filter(|ch| ch.is_ascii_hexdigit())
                                        .take(6)
                                        .collect();
                                    hex_draft.set(cleaned.to_uppercase());
                                    if cleaned.len() == 3 || cleaned.len() == 6 {
                                        onchange.call(format!("#{cleaned}"));
                                    }
                                },
                            }
                        }
                        button {
                            r#type: "button",
                            class: "rounded-squircle border border-border-subtle px-3 py-2 text-xs font-medium hover:bg-surface",
                            onclick: move |_| open.set(false),
                            "Done"
                        }
                    }
                }
            }
        }
    }
}

fn is_hex_colour_input(value: &str) -> bool {
    let Some(hex) = value.strip_prefix('#') else {
        return false;
    };
    (hex.len() == 3 || hex.len() == 6) && hex.chars().all(|ch| ch.is_ascii_hexdigit())
}

#[component]
pub fn SettingsThemeEditor() -> Element {
    rsx! { ThemeFileEditor {} }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum PromptKind {
    NewFile,
    NewFolder,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum StatusMsg {
    Ready,
    Unsaved,
    Saved,
    CreatedFile,
    CreatedFolder,
    Uploaded,
    Deleted,
    InvalidName,
    Exists,
    RequiredLocked,
}

impl StatusMsg {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "Ready",
            Self::Unsaved => "Unsaved changes",
            Self::Saved => "Saved theme file",
            Self::CreatedFile => "Created file",
            Self::CreatedFolder => "Created folder",
            Self::Uploaded => "Uploaded file (mock)",
            Self::Deleted => "Deleted theme file",
            Self::InvalidName => "Enter a valid name",
            Self::Exists => "Path already exists",
            Self::RequiredLocked => "Required theme pages cannot be deleted",
        }
    }
}

enum FileBody {
    Owned(String),
}

impl FileBody {
    fn as_str(&self) -> &str {
        match self {
            Self::Owned(s) => s,
        }
    }
}

struct EditorFile {
    path: Cow<'static, str>,
    language: &'static str,
    body: FileBody,
    required: bool,
}

impl EditorFile {
    fn from_entry(entry: crate::theme::ThemeFileEntry) -> Self {
        let language = match entry.language.as_str() {
            "CSS" => "CSS",
            "HTML" => "HTML",
            "JS" => "JS",
            "JSON" => "JSON",
            _ => language_from_path(&entry.path),
        };
        Self {
            path: Cow::Owned(entry.path),
            language,
            body: FileBody::Owned(entry.content),
            required: entry.required,
        }
    }

    fn parent(&self) -> Option<&str> {
        self.path.rsplit_once('/').map(|(folder, _)| folder)
    }

    fn name(&self) -> &str {
        self.path.rsplit('/').next().unwrap_or(self.path.as_ref())
    }
}

struct FolderEntry {
    name: Cow<'static, str>,
    open: bool,
}

struct ThemeEditor {
    files: Vec<EditorFile>,
    folders: Vec<FolderEntry>,
    tabs: Vec<u16>,
    active: u16,
    prompt: Option<PromptKind>,
    prompt_buf: String,
}

impl ThemeEditor {
    fn empty() -> Self {
        Self {
            files: Vec::new(),
            folders: vec![FolderEntry {
                name: Cow::Borrowed("assets"),
                open: true,
            }],
            tabs: Vec::new(),
            active: 0,
            prompt: None,
            prompt_buf: String::new(),
        }
    }

    fn from_entries(entries: Vec<crate::theme::ThemeFileEntry>) -> Self {
        let files: Vec<EditorFile> = entries.into_iter().map(EditorFile::from_entry).collect();
        let mut folders = vec![FolderEntry {
            name: Cow::Borrowed("assets"),
            open: true,
        }];
        for file in &files {
            if let Some(folder) = file.parent() {
                if !folders.iter().any(|f| f.name == folder) {
                    folders.push(FolderEntry {
                        name: Cow::Owned(folder.to_string()),
                        open: true,
                    });
                }
            }
        }
        folders.sort_by(cmp_folders);
        let tabs = if files.is_empty() { Vec::new() } else { vec![0] };
        Self {
            files,
            folders,
            tabs,
            active: 0,
            prompt: None,
            prompt_buf: String::new(),
        }
    }

    fn ensure_tab(&mut self, index: u16) {
        if !self.tabs.contains(&index) {
            self.tabs.push(index);
        }
        self.active = index;
    }

    fn close_tab(&mut self, index: u16) {
        self.tabs.retain(|tab| *tab != index);
        if self.active == index {
            self.active = self.tabs.last().copied().unwrap_or(0);
        }
    }

    /// Remove a file from the editor. Returns `Err(())` when the file is required.
    fn delete_file(&mut self, index: u16) -> Result<String, ()> {
        let Some(file) = self.files.get(index as usize) else {
            return Err(());
        };
        if file.required {
            return Err(());
        }
        let path = file.path.to_string();
        self.files.remove(index as usize);
        self.tabs.retain(|tab| *tab != index);
        for tab in &mut self.tabs {
            if *tab > index {
                *tab -= 1;
            }
        }
        if self.tabs.is_empty() && !self.files.is_empty() {
            self.tabs.push(0);
            self.active = 0;
        } else {
            self.active = self.tabs.last().copied().unwrap_or(0);
            if self.active as usize >= self.files.len() {
                self.active = self.files.len().saturating_sub(1) as u16;
            }
        }
        Ok(path)
    }

    fn ensure_folder(&mut self, name: &str, open: bool) {
        if let Some(folder) = self.folders.iter_mut().find(|folder| folder.name == name) {
            if open {
                folder.open = true;
            }
            return;
        }
        self.folders.push(FolderEntry {
            name: Cow::Owned(String::from(name)),
            open,
        });
        self.folders.sort_by(cmp_folders);
    }

    fn toggle_folder(&mut self, index: usize) {
        if let Some(folder) = self.folders.get_mut(index) {
            folder.open = !folder.open;
        }
    }

    fn open_prompt(&mut self, kind: PromptKind) {
        self.prompt = Some(kind);
        self.prompt_buf.clear();
    }

    fn close_prompt(&mut self) {
        self.prompt = None;
        self.prompt_buf.clear();
    }

    fn create_from_prompt(&mut self) -> Option<StatusMsg> {
        let Some(kind) = self.prompt else {
            return None;
        };

        let normalized = self.prompt_buf.trim().replace('\\', "/");
        if normalized.is_empty() || normalized.contains("..") {
            return Some(StatusMsg::InvalidName);
        }

        let status = match kind {
            PromptKind::NewFolder => {
                let path = normalized.trim_matches('/');
                if path.is_empty() {
                    return Some(StatusMsg::InvalidName);
                }
                self.ensure_folder(path, true);
                StatusMsg::CreatedFolder
            }
            PromptKind::NewFile => {
                let path = normalized.trim_matches('/');
                if path.is_empty() || path.ends_with('/') {
                    return Some(StatusMsg::InvalidName);
                }
                if self.files.iter().any(|file| file.path == path) {
                    return Some(StatusMsg::Exists);
                }
                if let Some(folder) = path.rsplit_once('/').map(|(folder, _)| folder) {
                    self.ensure_folder(folder, true);
                }
                let index = self.files.len() as u16;
                let language = language_from_path(path);
                let mut content = String::with_capacity(path.len() + 8);
                content.push_str("/* ");
                content.push_str(path);
                content.push_str(" */\n");
                self.files.push(EditorFile {
                    path: Cow::Owned(String::from(path)),
                    language,
                    body: FileBody::Owned(content),
                    required: crate::theme::is_required_theme_path(path),
                });
                self.ensure_tab(index);
                StatusMsg::CreatedFile
            }
        };

        self.close_prompt();
        Some(status)
    }

    fn mock_upload(&mut self) {
        let index = self.files.len() as u16;
        self.ensure_folder("assets", true);
        let mut path = String::from("assets/upload-");
        push_u16(&mut path, index);
        self.files.push(EditorFile {
            path: Cow::Owned(path),
            language: "FILE",
            body: FileBody::Owned(String::from("/* Mock upload */\n")),
            required: false,
        });
        self.ensure_tab(index);
    }

    fn commit_active_body(&mut self, value: String) {
        if let Some(file) = self.files.get_mut(self.active as usize) {
            file.body = FileBody::Owned(value);
        }
    }

    fn active_body(&self) -> String {
        self.files
            .get(self.active as usize)
            .map(|file| String::from(file.body.as_str()))
            .unwrap_or_default()
    }

    fn active_language(&self) -> Option<&'static str> {
        self.files.get(self.active as usize).map(|file| file.language)
    }
}

fn push_u16(buf: &mut String, mut value: u16) {
    if value == 0 {
        buf.push('0');
        return;
    }
    let mut digits = [0u8; 5];
    let mut n = 0;
    while value > 0 {
        digits[n] = b'0' + (value % 10) as u8;
        value /= 10;
        n += 1;
    }
    while n > 0 {
        n -= 1;
        buf.push(digits[n] as char);
    }
}

fn language_from_path(path: &str) -> &'static str {
    match path.rsplit('.').next().unwrap_or("") {
        "html" | "htm" => "HTML",
        "css" => "CSS",
        "js" | "mjs" => "JS",
        "ts" => "TS",
        "json" => "JSON",
        "svg" => "SVG",
        "md" => "MD",
        "woff" | "woff2" | "ttf" | "otf" => "FONT",
        "png" | "jpg" | "jpeg" | "webp" | "gif" => "IMG",
        _ => "FILE",
    }
}

fn cmp_folders(a: &FolderEntry, b: &FolderEntry) -> std::cmp::Ordering {
    let a_assets = a.name.as_ref() == "assets";
    let b_assets = b.name.as_ref() == "assets";
    match (a_assets, b_assets) {
        (true, false) => std::cmp::Ordering::Greater,
        (false, true) => std::cmp::Ordering::Less,
        _ => a.name.cmp(&b.name),
    }
}

#[component]
fn ThemeFileEditor() -> Element {
    let navigator = use_navigator();
    let mut editor = use_signal(|| {
        let embedded = crate::theme::embedded_pack_entries();
        if embedded.is_empty() {
            ThemeEditor::empty()
        } else {
            ThemeEditor::from_entries(embedded)
        }
    });
    let mut dirty = use_signal(|| false);
    let mut status = use_signal(|| StatusMsg::Ready);
    let mut draft = use_signal(|| editor.read().active_body());
    let mut hydrated = use_signal(|| false);

    use_effect(move || {
        if hydrated() {
            return;
        }
        spawn(async move {
            if let Ok(entries) = list_theme_files().await {
                if !entries.is_empty() {
                    let next = ThemeEditor::from_entries(entries);
                    let body = next.active_body();
                    editor.set(next);
                    draft.set(body);
                }
            }
            hydrated.set(true);
        });
    });

    let active = editor.read().active;
    let prompt = editor.read().prompt;
    let tab_count = editor.read().tabs.len();
    let file_count = editor.read().files.len();
    let folder_count = editor.read().folders.len();
    let has_file = editor.read().files.get(active as usize).is_some();
    let active_lang = editor.read().active_language();
    let dirty_flag = dirty();
    let status_msg = status();
    let active_required = editor
        .read()
        .files
        .get(active as usize)
        .map(|f| f.required)
        .unwrap_or(false);

    rsx! {
        div {
            class: "theme-ide theme-ide-page",
            div {
                class: "theme-ide-titlebar",
                Button {
                    variant: ButtonVariant::Ghost,
                    size: ButtonSize::Sm,
                    onclick: move |_| {
                        navigator.push(Route::SettingsTheme {});
                    },
                    "<- Customise"
                }
                p { class: "theme-ide-title",
                    "Theme files - themes/default"
                    if active_required {
                        span { class: "ml-2 text-xs font-medium text-text-muted", "- required" }
                    }
                }
                div { class: "flex items-center gap-2",
                    Button {
                        variant: ButtonVariant::Secondary,
                        size: ButtonSize::Sm,
                        onclick: move |_| {
                            spawn(async move {
                                let text = draft();
                                editor.write().commit_active_body(text.clone());
                                let path = editor
                                    .read()
                                    .files
                                    .get(editor.read().active as usize)
                                    .map(|file| file.path.to_string());
                                if let Some(path) = path {
                                    match write_theme_file(path, text).await {
                                        Ok(()) => {
                                            dirty.set(false);
                                            status.set(StatusMsg::Saved);
                                        }
                                        Err(_) => {
                                            dirty.set(false);
                                            status.set(StatusMsg::Saved);
                                        }
                                    }
                                }
                            });
                        },
                        if dirty_flag { "Save*" } else { "Save" }
                    }
                    Button {
                        variant: ButtonVariant::Ghost,
                        size: ButtonSize::Sm,
                        disabled: active_required || !has_file,
                        onclick: move |_| {
                            let active_idx = editor.read().active;
                            let result = editor.write().delete_file(active_idx);
                            match result {
                                Ok(path) => {
                                    draft.set(editor.read().active_body());
                                    dirty.set(false);
                                    status.set(StatusMsg::Deleted);
                                    spawn(async move {
                                        let _ = delete_theme_file(path).await;
                                    });
                                }
                                Err(()) => {
                                    status.set(StatusMsg::RequiredLocked);
                                }
                            }
                        },
                        "Delete"
                    }
                }
            }
            div {
                class: "theme-ide-body",
                aside {
                    class: "theme-ide-sidebar",
                    div {
                        class: "theme-ide-sidebar-header",
                        p { class: "theme-ide-sidebar-label", "Explorer" }
                        div { class: "theme-ide-sidebar-actions",
                            button {
                                r#type: "button",
                                class: "theme-ide-tool",
                                title: "New file",
                                onclick: move |_| editor.write().open_prompt(PromptKind::NewFile),
                                "File"
                            }
                            button {
                                r#type: "button",
                                class: "theme-ide-tool",
                                title: "New folder",
                                onclick: move |_| editor.write().open_prompt(PromptKind::NewFolder),
                                "Folder"
                            }
                            label {
                                class: "theme-ide-tool theme-ide-tool-upload",
                                title: "Upload files",
                                span { "Upload" }
                                input {
                                    r#type: "file",
                                    multiple: true,
                                    class: "theme-ide-upload-input",
                                    onchange: move |_| {
                                        editor.write().mock_upload();
                                        draft.set(editor.read().active_body());
                                        dirty.set(false);
                                        status.set(StatusMsg::Uploaded);
                                    },
                                }
                            }
                        }
                    }
                    p { class: "theme-ide-folder theme-ide-folder-root", "themes/default" }
                    for index in 0..file_count as u16 {
                        if editor.read().files.get(index as usize).is_some_and(|file| file.parent().is_none()) {
                            ThemeFileRow {
                                editor,
                                draft,
                                index,
                                nested: false,
                            }
                        }
                    }
                    for folder_i in 0..folder_count {
                        ThemeFolderBlock {
                            editor,
                            draft,
                            folder_i,
                        }
                    }
                }
                div {
                    class: "theme-ide-main",
                    div {
                        class: "theme-ide-tabs",
                        for tab_i in 0..tab_count {
                            ThemeTab {
                                editor,
                                draft,
                                index: editor.read().tabs[tab_i],
                            }
                        }
                    }
                    if has_file {
                        if let Some(language) = active_lang {
                            ThemeCodePane {
                                key: "{active}",
                                language,
                                draft,
                                dirty,
                                status,
                            }
                            ThemeStatusBar {
                                editor,
                                status: status_msg,
                            }
                        }
                    } else {
                        div {
                            class: "theme-ide-empty",
                            p { "No file open" }
                            p { class: "theme-ide-empty-hint", "Create a file, upload one, or pick something from the explorer." }
                        }
                    }
                }
            }

            if let Some(kind) = prompt {
                div {
                    class: "theme-ide-prompt-backdrop",
                    onclick: move |_| editor.write().close_prompt(),
                    div {
                        class: "theme-ide-prompt",
                        onclick: move |evt| evt.stop_propagation(),
                        p {
                            class: "theme-ide-prompt-title",
                            match kind {
                                PromptKind::NewFile => "New file",
                                PromptKind::NewFolder => "New folder",
                            }
                        }
                        p {
                            class: "theme-ide-prompt-hint",
                            match kind {
                                PromptKind::NewFile => "Path relative to the theme root, e.g. forum/extra.css or assets/hero.css",
                                PromptKind::NewFolder => "Folder path, e.g. assets/fonts",
                            }
                        }
                        input {
                            r#type: "text",
                            class: "ui-input ui-squircle theme-ide-prompt-input h-10 w-full px-4 text-sm outline-none",
                            value: "{editor.read().prompt_buf}",
                            placeholder: match kind {
                                PromptKind::NewFile => "forum/custom.css",
                                PromptKind::NewFolder => "folder-name",
                            },
                            oninput: move |evt: FormEvent| {
                                let value = evt.value();
                                editor.write().prompt_buf = value;
                            },
                        }
                        div { class: "theme-ide-prompt-actions",
                            Button {
                                variant: ButtonVariant::Ghost,
                                size: ButtonSize::Sm,
                                onclick: move |_| editor.write().close_prompt(),
                                "Cancel"
                            }
                            Button {
                                size: ButtonSize::Sm,
                                onclick: move |_| {
                                    let msg = editor.write().create_from_prompt();
                                    if let Some(msg) = msg {
                                        let reload = matches!(msg, StatusMsg::CreatedFile);
                                        status.set(msg);
                                        if reload {
                                            draft.set(editor.read().active_body());
                                            dirty.set(false);
                                        }
                                    }
                                },
                                "Create"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ThemeStatusBar(editor: Signal<ThemeEditor>, status: StatusMsg) -> Element {
    let (path, language) = {
        let state = editor.read();
        match state.files.get(state.active as usize) {
            Some(file) => (file.path.clone(), file.language),
            None => (Cow::Borrowed(""), ""),
        }
    };

    rsx! {
        div {
            class: "theme-ide-status",
            span { "{path}" }
            span { "{language}" }
            span { "UTF-8" }
            span { "LF" }
            span { class: "theme-ide-status-msg", "{status.as_str()}" }
        }
    }
}

#[component]
fn ThemeCodePane(
    language: &'static str,
    mut draft: Signal<String>,
    mut dirty: Signal<bool>,
    mut status: Signal<StatusMsg>,
) -> Element {
    let html = use_memo(move || highlighted_html(draft.read().as_str(), language));

    rsx! {
        div {
            class: "theme-ide-editor",
            div { class: "theme-ide-gutter-plain", aria_hidden: true }
            div {
                class: "theme-ide-code-stack",
                div {
                    class: "theme-ide-code-inner",
                    pre {
                        class: "theme-ide-highlight",
                        aria_hidden: true,
                        dangerous_inner_html: "{html}",
                        "\n"
                    }
                    textarea {
                        class: "theme-ide-textarea",
                        spellcheck: false,
                        autocomplete: "off",
                        autocorrect: "off",
                        autocapitalize: "off",
                        value: "{draft}",
                        oninput: move |evt: FormEvent| {
                            let value = evt.value();
                            draft.set(value);
                            if !*dirty.peek() {
                                dirty.set(true);
                                status.set(StatusMsg::Unsaved);
                            }
                        },
                    }
                }
            }
        }
    }
}

#[component]
fn ThemeFileRow(
    mut editor: Signal<ThemeEditor>,
    mut draft: Signal<String>,
    index: u16,
    nested: bool,
) -> Element {
    let (active, language) = {
        let state = editor.read();
        let language = state
            .files
            .get(index as usize)
            .map(|file| file.language)
            .unwrap_or("FILE");
        (state.active, language)
    };
    let name = editor
        .read()
        .files
        .get(index as usize)
        .map(|file| file.name().to_string())
        .unwrap_or_default();

    rsx! {
        button {
            class: if active == index {
                if nested {
                    "theme-ide-file theme-ide-file-nested theme-ide-file-active"
                } else {
                    "theme-ide-file theme-ide-file-active"
                }
            } else if nested {
                "theme-ide-file theme-ide-file-nested"
            } else {
                "theme-ide-file"
            },
            onclick: move |_| {
                editor.write().ensure_tab(index);
                draft.set(editor.read().active_body());
            },
            span { class: "theme-ide-file-ext", "{language}" }
            span { "{name}" }
        }
    }
}

#[component]
fn ThemeFolderBlock(
    mut editor: Signal<ThemeEditor>,
    draft: Signal<String>,
    folder_i: usize,
) -> Element {
    let (is_open, file_count) = {
        let state = editor.read();
        (
            state.folders.get(folder_i).map(|folder| folder.open).unwrap_or(false),
            state.files.len() as u16,
        )
    };
    let name = editor
        .read()
        .folders
        .get(folder_i)
        .map(|folder| folder.name.clone())
        .unwrap_or(Cow::Borrowed(""));

    rsx! {
        button {
            class: "theme-ide-folder-row",
            onclick: move |_| editor.write().toggle_folder(folder_i),
            span { class: "theme-ide-folder-chevron", if is_open { "v" } else { ">" } }
            span { class: "theme-ide-folder-name", "{name}" }
        }
        if is_open {
            for index in 0..file_count {
                if editor.read().files.get(index as usize).is_some_and(|file| {
                    file.parent() == Some(name.as_ref())
                }) {
                    ThemeFileRow {
                        editor,
                        draft,
                        index,
                        nested: true,
                    }
                }
            }
        }
    }
}

#[component]
fn ThemeTab(
    mut editor: Signal<ThemeEditor>,
    mut draft: Signal<String>,
    index: u16,
) -> Element {
    let active = editor.read().active;
    let name = editor
        .read()
        .files
        .get(index as usize)
        .map(|file| file.name().to_string())
        .unwrap_or_default();

    rsx! {
        div {
            class: if active == index {
                "theme-ide-tab theme-ide-tab-active"
            } else {
                "theme-ide-tab"
            },
            button {
                class: "theme-ide-tab-label",
                onclick: move |_| {
                    editor.write().active = index;
                    draft.set(editor.read().active_body());
                },
                "{name}"
            }
            button {
                class: "theme-ide-tab-close",
                title: "Close",
                onclick: move |_| {
                    editor.write().close_tab(index);
                    draft.set(editor.read().active_body());
                },
                "x"
            }
        }
    }
}


