use dioxus::prelude::*;

use crate::components::syntax::highlighted_html;
use crate::components::ui::*;
use crate::router::Route;

#[derive(Clone, PartialEq, Eq)]
struct ThemeFile {
    path: String,
    language: String,
    content: String,
}

fn theme_file(path: &str, language: &str, content: &str) -> ThemeFile {
    ThemeFile {
        path: path.to_owned(),
        language: language.to_owned(),
        content: content.to_owned(),
    }
}

fn site_theme_files() -> Vec<ThemeFile> {
    vec![
        theme_file("theme.css", "CSS", SITE_THEME_CSS),
        theme_file("layout.css", "CSS", SITE_LAYOUT_CSS),
        theme_file("preview.html", "HTML", SITE_PREVIEW_HTML),
    ]
}

#[component]
pub fn SettingsTheme() -> Element {
    rsx! {
        ThemeFileEditor {}
    }
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
    InvalidName,
    Exists,
}

impl StatusMsg {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "Ready",
            Self::Unsaved => "Unsaved changes",
            Self::Saved => "Saved theme files (mock)",
            Self::CreatedFile => "Created file",
            Self::CreatedFolder => "Created folder",
            Self::Uploaded => "Uploaded file (mock)",
            Self::InvalidName => "Enter a valid name",
            Self::Exists => "Path already exists",
        }
    }
}

struct EditorFile {
    path: String,
    language: String,
    body: String,
}

impl EditorFile {
    fn from_seed(file: ThemeFile) -> Self {
        Self {
            path: file.path,
            language: file.language,
            body: file.content,
        }
    }

    fn parent(&self) -> Option<&str> {
        self.path.rsplit_once('/').map(|(folder, _)| folder)
    }

    fn name(&self) -> &str {
        self.path.rsplit('/').next().unwrap_or(self.path.as_str())
    }
}

struct FolderEntry {
    name: String,
    open: bool,
}

struct ThemeEditor {
    files: Vec<EditorFile>,
    folders: Vec<FolderEntry>,
    tabs: Vec<u16>,
    active: u16,
}

impl ThemeEditor {
    fn new(seed: Vec<ThemeFile>) -> Self {
        let files: Vec<EditorFile> = seed.into_iter().map(EditorFile::from_seed).collect();
        let tabs = if files.is_empty() {
            Vec::new()
        } else {
            vec![0]
        };
        Self {
            files,
            folders: vec![
                FolderEntry {
                    name: String::from("assets"),
                    open: true,
                },
                FolderEntry {
                    name: String::from("partials"),
                    open: true,
                },
            ],
            tabs,
            active: 0,
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

    fn ensure_folder(&mut self, name: &str, open: bool) {
        if let Some(folder) = self.folders.iter_mut().find(|folder| folder.name == name) {
            if open {
                folder.open = true;
            }
            return;
        }
        self.folders.push(FolderEntry {
            name: String::from(name),
            open,
        });
        self.folders.sort_by(|a, b| a.name.cmp(&b.name));
    }

    fn toggle_folder(&mut self, index: usize) {
        if let Some(folder) = self.folders.get_mut(index) {
            folder.open = !folder.open;
        }
    }

    fn create_from_prompt(&mut self, kind: PromptKind, prompt_buf: &str) -> Option<StatusMsg> {
        let normalized = prompt_buf.trim().replace('\\', "/");
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
                    path: String::from(path),
                    language,
                    body: content,
                });
                self.ensure_tab(index);
                StatusMsg::CreatedFile
            }
        };

        Some(status)
    }

    fn mock_upload(&mut self) {
        let index = self.files.len() as u16;
        self.ensure_folder("assets", true);
        let mut path = String::from("assets/upload-");
        push_u16(&mut path, index);
        self.files.push(EditorFile {
            path,
            language: String::from("FILE"),
            body: String::from("/* Mock upload */\n"),
        });
        self.ensure_tab(index);
    }

    fn commit_active_body(&mut self, value: String) {
        if let Some(file) = self.files.get_mut(self.active as usize) {
            file.body = value;
        }
    }

    fn active_body(&self) -> String {
        self.files
            .get(self.active as usize)
            .map(|file| file.body.clone())
            .unwrap_or_default()
    }

    fn active_language(&self) -> Option<String> {
        self.files
            .get(self.active as usize)
            .map(|file| file.language.clone())
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

fn language_from_path(path: &str) -> String {
    let language = match path.rsplit('.').next().unwrap_or("") {
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
    };
    String::from(language)
}

#[component]
fn ThemeFileEditor() -> Element {
    let navigator = use_navigator();
    let mut editor = use_signal(|| ThemeEditor::new(site_theme_files()));
    let mut dirty = use_signal(|| false);
    let mut status = use_signal(|| StatusMsg::Ready);
    let mut draft = use_signal(|| editor.read().active_body());
    let mut prompt = use_signal(|| Option::<PromptKind>::None);
    let mut prompt_buf = use_signal(String::new);

    let active = editor.read().active;
    let prompt_kind = prompt();
    let tab_count = editor.read().tabs.len();
    let file_count = editor.read().files.len();
    let folder_count = editor.read().folders.len();
    let has_file = editor.read().files.get(active as usize).is_some();
    let active_lang = editor.read().active_language();
    let dirty_flag = dirty();
    let status_msg = status();

    rsx! {
        div { class: "theme-ide theme-ide-page",
            div { class: "theme-ide-titlebar",
                Button {
                    variant: ButtonVariant::Ghost,
                    size: ButtonSize::Sm,
                    onclick: move |_| {
                        navigator.push(Route::SettingsGeneral {});
                    },
                    "← Settings"
                }
                p { class: "theme-ide-title", "Site theme · themes/site" }
                div { class: "flex items-center gap-2",
                    Button {
                        variant: ButtonVariant::Secondary,
                        size: ButtonSize::Sm,
                        onclick: move |_| {
                            let text = draft();
                            editor.write().commit_active_body(text);
                            dirty.set(false);
                            status.set(StatusMsg::Saved);
                        },
                        if dirty_flag {
                            "Save*"
                        } else {
                            "Save"
                        }
                    }
                }
            }
            div { class: "theme-ide-body",
                aside { class: "theme-ide-sidebar",
                    div { class: "theme-ide-sidebar-header",
                        p { class: "theme-ide-sidebar-label", "Explorer" }
                        div { class: "theme-ide-sidebar-actions",
                            button {
                                r#type: "button",
                                class: "theme-ide-tool",
                                title: "New file",
                                onclick: move |_| {
                                    prompt_buf.set(String::new());
                                    prompt.set(Some(PromptKind::NewFile));
                                },
                                "File"
                            }
                            button {
                                r#type: "button",
                                class: "theme-ide-tool",
                                title: "New folder",
                                onclick: move |_| {
                                    prompt_buf.set(String::new());
                                    prompt.set(Some(PromptKind::NewFolder));
                                },
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
                    p { class: "theme-ide-folder theme-ide-folder-root", "themes/site" }
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
                        ThemeFolderBlock { editor, draft, folder_i }
                    }
                }
                div { class: "theme-ide-main",
                    div { class: "theme-ide-tabs",
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
                            ThemeStatusBar { editor, status: status_msg }
                        }
                    } else {
                        div { class: "theme-ide-empty",
                            p { "No file open" }
                            p { class: "theme-ide-empty-hint",
                                "Create a file, upload one, or pick something from the explorer."
                            }
                        }
                    }
                }
            }

            if let Some(kind) = prompt_kind {
                ThemePromptDialog {
                    kind,
                    prompt,
                    prompt_buf,
                    editor,
                    draft,
                    dirty,
                    status,
                }
            }
        }
    }
}

#[component]
fn ThemePromptDialog(
    kind: PromptKind,
    mut prompt: Signal<Option<PromptKind>>,
    mut prompt_buf: Signal<String>,
    mut editor: Signal<ThemeEditor>,
    mut draft: Signal<String>,
    mut dirty: Signal<bool>,
    mut status: Signal<StatusMsg>,
) -> Element {
    rsx! {
        div {
            class: "theme-ide-prompt-backdrop",
            onclick: move |_| {
                prompt.set(None);
                prompt_buf.set(String::new());
            },
            div {
                class: "theme-ide-prompt",
                onclick: move |evt| evt.stop_propagation(),
                p { class: "theme-ide-prompt-title",
                    match kind {
                        PromptKind::NewFile => "New file",
                        PromptKind::NewFolder => "New folder",
                    }
                }
                p { class: "theme-ide-prompt-hint",
                    match kind {
                        PromptKind::NewFile => "Path relative to the theme root, e.g. assets/hero.css",
                        PromptKind::NewFolder => "Folder path, e.g. assets/fonts",
                    }
                }
                input {
                    r#type: "text",
                    class: "ui-input ui-squircle theme-ide-prompt-input h-10 w-full px-4 text-sm outline-none",
                    value: "{prompt_buf}",
                    placeholder: match kind {
                        PromptKind::NewFile => "filename.css",
                        PromptKind::NewFolder => "folder-name",
                    },
                    oninput: move |evt: FormEvent| {
                        prompt_buf.set(evt.value());
                    },
                }
                div { class: "theme-ide-prompt-actions",
                    Button {
                        variant: ButtonVariant::Ghost,
                        size: ButtonSize::Sm,
                        onclick: move |_| {
                            prompt.set(None);
                            prompt_buf.set(String::new());
                        },
                        "Cancel"
                    }
                    Button {
                        size: ButtonSize::Sm,
                        onclick: move |_| {
                            let buf = prompt_buf();
                            let msg = editor.write().create_from_prompt(kind, &buf);
                            prompt.set(None);
                            prompt_buf.set(String::new());
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

#[component]
fn ThemeStatusBar(editor: Signal<ThemeEditor>, status: StatusMsg) -> Element {
    let (path, language) = {
        let state = editor.read();
        match state.files.get(state.active as usize) {
            Some(file) => (file.path.clone(), file.language.clone()),
            None => (String::new(), String::new()),
        }
    };

    rsx! {
        div { class: "theme-ide-status",
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
    #[props(into)] language: String,
    mut draft: Signal<String>,
    mut dirty: Signal<bool>,
    mut status: Signal<StatusMsg>,
) -> Element {
    let html = use_memo(move || highlighted_html(draft.read().as_str(), language.as_str()));

    rsx! {
        div { class: "theme-ide-editor",
            div { class: "theme-ide-gutter-plain", aria_hidden: true }
            div { class: "theme-ide-code-stack",
                div { class: "theme-ide-code-inner",
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
            .map(|file| file.language.clone())
            .unwrap_or_else(|| String::from("FILE"));
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
            class: if active == index { if nested {
                "theme-ide-file theme-ide-file-nested theme-ide-file-active"
            } else {
                "theme-ide-file theme-ide-file-active"
            } } else if nested { "theme-ide-file theme-ide-file-nested" } else { "theme-ide-file" },
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
            state
                .folders
                .get(folder_i)
                .map(|folder| folder.open)
                .unwrap_or(false),
            state.files.len() as u16,
        )
    };
    let name = editor
        .read()
        .folders
        .get(folder_i)
        .map(|folder| folder.name.clone())
        .unwrap_or_default();

    rsx! {
        button {
            class: "theme-ide-folder-row",
            onclick: move |_| editor.write().toggle_folder(folder_i),
            span { class: "theme-ide-folder-chevron",
                if is_open {
                    "▾"
                } else {
                    "▸"
                }
            }
            span { class: "theme-ide-folder-name", "{name}" }
        }
        if is_open {
            for index in 0..file_count {
                if editor
                    .read()
                    .files
                    .get(index as usize)
                    .is_some_and(|file| { file.parent() == Some(name.as_str()) })
                {
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
fn ThemeTab(mut editor: Signal<ThemeEditor>, mut draft: Signal<String>, index: u16) -> Element {
    let active = editor.read().active;
    let name = editor
        .read()
        .files
        .get(index as usize)
        .map(|file| file.name().to_string())
        .unwrap_or_default();

    rsx! {
        div { class: if active == index { "theme-ide-tab theme-ide-tab-active" } else { "theme-ide-tab" },
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
                "×"
            }
        }
    }
}

const SITE_THEME_CSS: &str = r#":root {
  --color-bg: #0b0f14;
  --color-surface: #12181f;
  --color-surface-2: #1a222c;
  --color-border: #2a3441;
  --color-border-subtle: #1f2833;
  --color-text: #f4f7fb;
  --color-text-secondary: #c2ccd8;
  --color-text-muted: #8b97a8;
  --color-accent: #5b9dff;
  --color-accent-soft: color-mix(in srgb, var(--color-accent) 18%, transparent);
  --radius-sm: 8px;
  --radius-md: 12px;
  --radius-lg: 16px;
  --font-sans: Outfit, system-ui, sans-serif;
  --font-mono: "JetBrains Mono", ui-monospace, monospace;
}

.site-shell {
  background: var(--color-bg);
  color: var(--color-text);
  font-family: var(--font-sans);
}
"#;

const SITE_LAYOUT_CSS: &str = r#".site-shell {
  min-height: 100vh;
}

.site-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 1rem;
  padding: 1rem 1.25rem;
  border-bottom: 1px solid var(--color-border-subtle);
  background: color-mix(in srgb, var(--color-surface) 92%, transparent);
}

.site-main {
  width: min(1120px, 100%);
  margin: 0 auto;
  padding: 1.5rem 1.25rem 3rem;
}

.site-card {
  border: 1px solid var(--color-border-subtle);
  border-radius: var(--radius-lg);
  background: var(--color-surface);
  padding: 1.1rem 1.2rem;
}

.site-btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 0.4rem;
  padding: 0.55rem 0.95rem;
  border: none;
  border-radius: var(--radius-sm);
  background: var(--color-accent);
  color: #fff;
  font: inherit;
  font-weight: 600;
  cursor: pointer;
}
"#;

const SITE_PREVIEW_HTML: &str = r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <title>ServerSpot</title>
    <link rel="stylesheet" href="theme.css" />
    <link rel="stylesheet" href="layout.css" />
  </head>
  <body class="site-shell">
    <header class="site-header">
      <strong>Your server</strong>
      <button class="site-btn" type="button">Open store</button>
    </header>
    <main class="site-main">
      <section class="site-card">
        <h1>Welcome back</h1>
        <p>One site theme drives colours, type, and radius across every public page.</p>
      </section>
    </main>
  </body>
</html>
"#;
