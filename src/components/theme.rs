use std::borrow::Cow;
use dioxus::prelude::*;
use crate::components::syntax::highlighted_html;
use crate::components::ui::*;
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ThemeFeature {
    Store,
    Forum,
    Support,
    Content,
    Players,
    Leaderboards,
    Votes,
    Applications,
    Analytics,
}
impl ThemeFeature {
    fn label(self) -> &'static str {
        match self {
            Self::Store => "Store",
            Self::Forum => "Forum",
            Self::Support => "Support",
            Self::Content => "Blog",
            Self::Players => "Players",
            Self::Leaderboards => "Leaderboards",
            Self::Votes => "Vote rewards",
            Self::Applications => "Applications",
            Self::Analytics => "Analytics",
        }
    }
    fn slug(self) -> &'static str {
        match self {
            Self::Store => "store",
            Self::Forum => "forum",
            Self::Support => "support",
            Self::Content => "blog",
            Self::Players => "players",
            Self::Leaderboards => "leaderboards",
            Self::Votes => "votes",
            Self::Applications => "applications",
            Self::Analytics => "analytics",
        }
    }
    fn overview_route(self) -> crate::router::Route {
        use crate::router::Route;
        match self {
            Self::Store => Route::StoreOverview {},
            Self::Forum => Route::ForumOverview {},
            Self::Support => Route::SupportOverview {},
            Self::Content => Route::ContentOverview {},
            Self::Players => Route::PlayersOverview {},
            Self::Leaderboards => Route::LeaderboardsOverview {},
            Self::Votes => Route::VotesOverview {},
            Self::Applications => Route::ApplicationsOverview {},
            Self::Analytics => Route::AnalyticsOverview {},
        }
    }
    fn files(self) -> &'static [ThemeFile] {
        match self {
            Self::Store => {
                &[
                    ThemeFile {
                        path: "theme.css",
                        language: "CSS",
                        content: STORE_THEME_CSS,
                    },
                    ThemeFile {
                        path: "product-card.css",
                        language: "CSS",
                        content: STORE_CARD_CSS,
                    },
                    ThemeFile {
                        path: "checkout.html",
                        language: "HTML",
                        content: STORE_CHECKOUT_HTML,
                    },
                ]
            }
            Self::Forum => {
                &[
                    ThemeFile {
                        path: "theme.css",
                        language: "CSS",
                        content: FORUM_THEME_CSS,
                    },
                    ThemeFile {
                        path: "thread.html",
                        language: "HTML",
                        content: FORUM_THREAD_HTML,
                    },
                    ThemeFile {
                        path: "category.css",
                        language: "CSS",
                        content: FORUM_CATEGORY_CSS,
                    },
                ]
            }
            Self::Support => {
                &[
                    ThemeFile {
                        path: "theme.css",
                        language: "CSS",
                        content: SUPPORT_THEME_CSS,
                    },
                    ThemeFile {
                        path: "ticket-portal.html",
                        language: "HTML",
                        content: SUPPORT_PORTAL_HTML,
                    },
                    ThemeFile {
                        path: "reply.css",
                        language: "CSS",
                        content: SUPPORT_REPLY_CSS,
                    },
                ]
            }
            Self::Content => {
                &[
                    ThemeFile {
                        path: "theme.css",
                        language: "CSS",
                        content: CONTENT_THEME_CSS,
                    },
                    ThemeFile {
                        path: "article.html",
                        language: "HTML",
                        content: CONTENT_ARTICLE_HTML,
                    },
                    ThemeFile {
                        path: "page-hero.css",
                        language: "CSS",
                        content: CONTENT_HERO_CSS,
                    },
                ]
            }
            Self::Players => {
                &[
                    ThemeFile {
                        path: "theme.css",
                        language: "CSS",
                        content: COMMUNITY_THEME_CSS,
                    },
                    ThemeFile {
                        path: "profile.html",
                        language: "HTML",
                        content: COMMUNITY_PROFILE_HTML,
                    },
                    ThemeFile {
                        path: "stats.css",
                        language: "CSS",
                        content: PLAYERS_STATS_CSS,
                    },
                ]
            }
            Self::Leaderboards => {
                &[
                    ThemeFile {
                        path: "theme.css",
                        language: "CSS",
                        content: LEADERBOARDS_THEME_CSS,
                    },
                    ThemeFile {
                        path: "board.html",
                        language: "HTML",
                        content: LEADERBOARDS_BOARD_HTML,
                    },
                    ThemeFile {
                        path: "rank-row.css",
                        language: "CSS",
                        content: LEADERBOARDS_ROW_CSS,
                    },
                ]
            }
            Self::Votes => {
                &[
                    ThemeFile {
                        path: "theme.css",
                        language: "CSS",
                        content: VOTES_THEME_CSS,
                    },
                    ThemeFile {
                        path: "claim.html",
                        language: "HTML",
                        content: VOTES_CLAIM_HTML,
                    },
                    ThemeFile {
                        path: "streak.css",
                        language: "CSS",
                        content: VOTES_STREAK_CSS,
                    },
                ]
            }
            Self::Applications => {
                &[
                    ThemeFile {
                        path: "theme.css",
                        language: "CSS",
                        content: APPLICATIONS_THEME_CSS,
                    },
                    ThemeFile {
                        path: "form.html",
                        language: "HTML",
                        content: APPLICATIONS_FORM_HTML,
                    },
                    ThemeFile {
                        path: "application.css",
                        language: "CSS",
                        content: COMMUNITY_APP_CSS,
                    },
                ]
            }
            Self::Analytics => {
                &[
                    ThemeFile {
                        path: "theme.css",
                        language: "CSS",
                        content: ANALYTICS_THEME_CSS,
                    },
                    ThemeFile {
                        path: "report.html",
                        language: "HTML",
                        content: ANALYTICS_REPORT_HTML,
                    },
                    ThemeFile {
                        path: "charts.css",
                        language: "CSS",
                        content: ANALYTICS_CHARTS_CSS,
                    },
                ]
            }
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
struct ThemeFile {
    path: &'static str,
    language: &'static str,
    content: &'static str,
}
#[component]
pub fn StoreTheme() -> Element {
    rsx! {
        FeatureTheme { feature: ThemeFeature::Store }
    }
}
#[component]
pub fn ForumTheme() -> Element {
    rsx! {
        FeatureTheme { feature: ThemeFeature::Forum }
    }
}
#[component]
pub fn SupportTheme() -> Element {
    rsx! {
        FeatureTheme { feature: ThemeFeature::Support }
    }
}
#[component]
pub fn ContentTheme() -> Element {
    rsx! {
        FeatureTheme { feature: ThemeFeature::Content }
    }
}
#[component]
pub fn PlayersTheme() -> Element {
    rsx! {
        FeatureTheme { feature: ThemeFeature::Players }
    }
}
#[component]
pub fn LeaderboardsTheme() -> Element {
    rsx! {
        FeatureTheme { feature: ThemeFeature::Leaderboards }
    }
}
#[component]
pub fn VotesTheme() -> Element {
    rsx! {
        FeatureTheme { feature: ThemeFeature::Votes }
    }
}
#[component]
pub fn ApplicationsTheme() -> Element {
    rsx! {
        FeatureTheme { feature: ThemeFeature::Applications }
    }
}
#[component]
pub fn AnalyticsTheme() -> Element {
    rsx! {
        FeatureTheme { feature: ThemeFeature::Analytics }
    }
}
#[component]
fn FeatureTheme(feature: ThemeFeature) -> Element {
    rsx! {
        ThemeFileEditor { feature }
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
enum FileBody {
    Static(&'static str),
    Owned(String),
}
impl FileBody {
    fn as_str(&self) -> &str {
        match self {
            Self::Static(s) => s,
            Self::Owned(s) => s,
        }
    }
}
struct EditorFile {
    path: Cow<'static, str>,
    language: &'static str,
    body: FileBody,
}
impl EditorFile {
    fn from_seed(file: ThemeFile) -> Self {
        Self {
            path: Cow::Borrowed(file.path),
            language: file.language,
            body: FileBody::Static(file.content),
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
    fn new(seed: &'static [ThemeFile]) -> Self {
        let files: Vec<EditorFile> = seed
            .iter()
            .copied()
            .map(EditorFile::from_seed)
            .collect();
        let tabs = if files.is_empty() { Vec::new() } else { vec![0] };
        Self {
            files,
            folders: vec![
                FolderEntry {
                    name: Cow::Borrowed("assets"),
                    open: true,
                },
                FolderEntry {
                    name: Cow::Borrowed("partials"),
                    open: true,
                },
            ],
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
    fn ensure_folder(&mut self, name: &str, open: bool) {
        if let Some(folder) = self.folders.iter_mut().find(|folder| folder.name == name)
        {
            if open {
                folder.open = true;
            }
            return;
        }
        self.folders
            .push(FolderEntry {
                name: Cow::Owned(String::from(name)),
                open,
            });
        self.folders.sort_by(|a, b| a.name.cmp(&b.name));
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
                self.files
                    .push(EditorFile {
                        path: Cow::Owned(String::from(path)),
                        language,
                        body: FileBody::Owned(content),
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
        self.files
            .push(EditorFile {
                path: Cow::Owned(path),
                language: "FILE",
                body: FileBody::Owned(String::from("/* Mock upload */\n")),
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
#[component]
fn ThemeFileEditor(feature: ThemeFeature) -> Element {
    let feature_label = feature.label();
    let feature_slug = feature.slug();
    let overview = feature.overview_route();
    let navigator = use_navigator();
    let mut editor = use_signal(|| ThemeEditor::new(feature.files()));
    let mut dirty = use_signal(|| false);
    let mut status = use_signal(|| StatusMsg::Ready);
    let mut draft = use_signal(|| editor.read().active_body());
    let active = editor.read().active;
    let prompt = editor.read().prompt;
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
                    onclick: move |_| { navigator.push(overview);
                        navigator.push(overview);
                    },
                    "← Overview"
                }
                p { class: "theme-ide-title", "{feature_label} · themes/{feature_slug}" }
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
                                onclick: move | _ | editor.write()
                                        .open_prompt(PromptKind::NewFile),
                                "File"
                            }
                            button {
                                r#type: "button",
                                class: "theme-ide-tool",
                                title: "New folder",
                                onclick: move | _ | editor.write()
                                        .open_prompt(PromptKind::NewFolder),
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
                    p { class: "theme-ide-folder theme-ide-folder-root", "themes/{feature_slug}" }
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
            if let Some(kind) = prompt {
                div {
                    class: "theme-ide-prompt-backdrop",
                    onclick: move |_| editor.write().close_prompt(),
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
                            value: "{editor.read().prompt_buf}",
                            placeholder: match kind {
                                PromptKind::NewFile => "filename.css",
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
                                onclick: move | _ |
                                        editor.write().close_prompt(),
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
    language: &'static str,
    mut draft: Signal<String>,
    mut dirty: Signal<bool>,
    mut status: Signal<StatusMsg>,
) -> Element {
    let html = use_memo(move || highlighted_html(draft.read().as_str(), language));
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
            onclick: move | _ | editor.write()
                    .toggle_folder(folder_i),
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
                    .is_some_and(|file| { file.parent() == Some(name.as_ref()) })
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
const STORE_THEME_CSS: &str = r#":root {
  --store-primary: #3ecf8e;
  --store-accent: #87d1fe;
  --store-bg: #12161a;
  --store-radius: 12px;
}

.store-shell {
  background: var(--store-bg);
  color: #f4f7f5;
  font-family: Outfit, sans-serif;
}
"#;
const STORE_CARD_CSS: &str = r#".product-card {
  border-radius: var(--store-radius);
  border: 1px solid color-mix(in srgb, var(--store-primary) 24%, transparent);
  background: #1c242c;
  padding: 1rem;
}

.product-card__price {
  color: var(--store-primary);
  font-weight: 600;
}
"#;
const STORE_CHECKOUT_HTML: &str = r#"<section class="checkout">
  <h1>Checkout</h1>
  <div class="checkout__summary">
    <p>VIP Rank</p>
    <strong>£29.99</strong>
  </div>
  <button class="btn-primary">Pay now</button>
</section>
"#;
const FORUM_THEME_CSS: &str = r#":root {
  --forum-primary: #5b9dff;
  --forum-surface: #1e2230;
  --forum-radius: 8px;
}

.forum-shell {
  background: #14161f;
  font-family: "IBM Plex Sans", sans-serif;
}
"#;
const FORUM_THREAD_HTML: &str = r#"<article class="thread">
  <header>
    <h1>Welcome to the forums</h1>
    <span class="meta">Posted by NovaCraft</span>
  </header>
  <div class="thread__body">
    Share builds, events, and server news.
  </div>
</article>
"#;
const FORUM_CATEGORY_CSS: &str = r#".category-row {
  display: grid;
  grid-template-columns: 1fr auto;
  gap: 1rem;
  padding: 0.85rem 1rem;
  border-radius: var(--forum-radius);
  background: var(--forum-surface);
}
"#;
const SUPPORT_THEME_CSS: &str = r#":root {
  --support-primary: #f0a35e;
  --support-accent: #f5c14a;
  --support-bg: #181410;
}

.support-shell {
  background: var(--support-bg);
  color: #f7f3ee;
}
"#;
const SUPPORT_PORTAL_HTML: &str = r#"<main class="ticket-portal">
  <h1>Help Center</h1>
  <form class="ticket-form">
    <label>Subject</label>
    <input placeholder="Briefly describe the issue" />
    <button type="button">Submit ticket</button>
  </form>
</main>
"#;
const SUPPORT_REPLY_CSS: &str = r#".ticket-reply {
  border-left: 3px solid var(--support-primary);
  background: #262018;
  padding: 0.75rem 1rem;
  border-radius: 10px;
}
"#;
const CONTENT_THEME_CSS: &str = r#":root {
  --content-primary: #87d1fe;
  --content-display: Fraunces, serif;
  --content-radius: 14px;
}

.content-shell {
  background: #101418;
  font-family: Outfit, sans-serif;
}
"#;
const CONTENT_ARTICLE_HTML: &str = r#"<article class="article">
  <p class="eyebrow">Patch notes</p>
  <h1>Season 4 launch</h1>
  <p>Read about new ranks, crates, and world events.</p>
</article>
"#;
const CONTENT_HERO_CSS: &str = r#".page-hero {
  border-radius: var(--content-radius);
  background:
    linear-gradient(180deg, transparent, #101418),
    radial-gradient(circle at 20% 20%, color-mix(in srgb, var(--content-primary) 30%, transparent), transparent 55%);
  padding: 3rem 1.5rem;
}
"#;
const COMMUNITY_THEME_CSS: &str = r#":root {
  --community-primary: #69bdf2;
  --community-accent: #3ecf8e;
  --community-radius: 16px;
}

.community-shell {
  background: #12161c;
  color: #f2f5fa;
}
"#;
const COMMUNITY_PROFILE_HTML: &str = r#"<section class="profile">
  <header>
    <h1>NovaCraft</h1>
    <span class="rank">VIP</span>
  </header>
  <p>Joined Mar 2024 · 128 play sessions</p>
</section>
"#;
const PLAYERS_STATS_CSS: &str = r#".player-stats {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 0.75rem;
}

.player-stats__item {
  border-radius: var(--community-radius);
  background: #1c2430;
  padding: 0.85rem;
}
"#;
const LEADERBOARDS_THEME_CSS: &str = r#":root {
  --boards-primary: #5eead4;
  --boards-accent: #5b9dff;
  --boards-radius: 8px;
}

.leaderboards-shell {
  background: #101618;
  color: #eef8f6;
}
"#;
const LEADERBOARDS_BOARD_HTML: &str = r#"<section class="board">
  <h1>Top players</h1>
  <ol>
    <li>NovaCraft · 1,842 pts</li>
    <li>SkyBuilder · 1,640 pts</li>
  </ol>
</section>
"#;
const LEADERBOARDS_ROW_CSS: &str = r#".rank-row {
  display: grid;
  grid-template-columns: 2.5rem 1fr auto;
  gap: 0.75rem;
  border-radius: var(--boards-radius);
  padding: 0.7rem 0.85rem;
  background: #172226;
}
"#;
const VOTES_THEME_CSS: &str = r#":root {
  --votes-primary: #fbbf24;
  --votes-accent: #f0a35e;
  --votes-radius: 12px;
}

.votes-shell {
  background: #16120a;
  color: #faf6ee;
}
"#;
const VOTES_CLAIM_HTML: &str = r#"<section class="vote-claim">
  <h1>Claim rewards</h1>
  <p>Streak day 7 · Ready to claim</p>
  <button type="button">Claim now</button>
</section>
"#;
const VOTES_STREAK_CSS: &str = r#".vote-streak {
  border-radius: var(--votes-radius);
  border: 1px solid color-mix(in srgb, var(--votes-primary) 30%, transparent);
  background: #242016;
  padding: 1rem;
}
"#;
const APPLICATIONS_THEME_CSS: &str = r#":root {
  --apps-primary: #fb7185;
  --apps-accent: #f0a35e;
  --apps-radius: 10px;
}

.applications-shell {
  background: #161014;
  color: #faf2f4;
}
"#;
const APPLICATIONS_FORM_HTML: &str = r#"<form class="application-form">
  <h1>Moderator application</h1>
  <label>Why do you want to join staff?</label>
  <textarea rows="4"></textarea>
  <button type="button">Submit</button>
</form>
"#;
const COMMUNITY_APP_CSS: &str = r#".application-card {
  border-radius: var(--apps-radius, 16px);
  border: 1px solid color-mix(in srgb, var(--apps-primary, #3ecf8e) 28%, transparent);
  padding: 1rem;
  background: #241820;
}
"#;
const ANALYTICS_THEME_CSS: &str = r#":root {
  --analytics-primary: #f5c14a;
  --analytics-accent: #87d1fe;
  --analytics-grid: #1a1e24;
}

.analytics-shell {
  background: #101214;
  font-family: "JetBrains Mono", monospace;
}
"#;
const ANALYTICS_REPORT_HTML: &str = r#"<section class="report">
  <h1>Weekly overview</h1>
  <div class="report__metrics">
    <div>Revenue · £4,281</div>
    <div>Tickets · 37</div>
  </div>
</section>
"#;
const ANALYTICS_CHARTS_CSS: &str = r#".chart-panel {
  background: var(--analytics-grid);
  border: 1px solid color-mix(in srgb, var(--analytics-primary) 22%, transparent);
  border-radius: 6px;
  padding: 1rem;
}

.chart-panel__series {
  stroke: var(--analytics-accent);
}
"#;
