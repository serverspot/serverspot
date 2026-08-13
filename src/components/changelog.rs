use dioxus::prelude::*;
use dioxus_i18n::prelude::*;
use dioxus_i18n::t;

use crate::components::ui::*;
use crate::i18n::t_key;

const DEFAULT_REPO_URL: &str = "https://github.com/serverspot/serverspot";

/// One bullet under a version heading.
#[derive(Clone, PartialEq, Eq)]
pub struct ChangelogItem {
    pub text: String,
}

impl ChangelogItem {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

/// A version block inside the changelog modal.
#[derive(Clone, PartialEq, Eq)]
pub struct ChangelogSection {
    pub version: String,
    pub date: String,
    pub items: Vec<ChangelogItem>,
}

impl ChangelogSection {
    pub fn new(
        version: impl Into<String>,
        date: impl Into<String>,
        items: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            version: version.into(),
            date: date.into(),
            items: items
                .into_iter()
                .map(|text| ChangelogItem::new(text))
                .collect(),
        }
    }
}

/// Dynamic body for the changelog modal — replace anytime from code.
#[derive(Clone, PartialEq, Eq)]
pub struct ChangelogContent {
    pub title: String,
    pub description: String,
    pub sections: Vec<ChangelogSection>,
    pub repo_url: String,
}

impl Default for ChangelogContent {
    fn default() -> Self {
        Self {
            title: String::new(),
            description: String::new(),
            sections: default_changelog_sections(),
            repo_url: DEFAULT_REPO_URL.to_string(),
        }
    }
}

impl ChangelogContent {
    pub fn with_sections(sections: Vec<ChangelogSection>) -> Self {
        Self {
            sections,
            ..Self::default()
        }
    }
}

fn default_changelog_sections() -> Vec<ChangelogSection> {
    vec![
        ChangelogSection::new(
            "0.2.0",
            "Aug 2026",
            [
                "Admin localisation catalogs and language picker",
                "Roles starting permissions expanded across modules",
                "Update available badge and changelog modal",
            ],
        ),
        ChangelogSection::new(
            "0.1.0",
            "Jul 2026",
            [
                "Admin reskin across store, support, forum, and community",
                "Theme editor and settings surfaces",
            ],
        ),
    ]
}

/// Copyable handle for opening / filling the changelog from anywhere with context.
#[derive(Clone, Copy)]
pub struct Changelog {
    open: Signal<bool>,
    content: Signal<ChangelogContent>,
}

impl Changelog {
    pub fn from_signals(open: Signal<bool>, content: Signal<ChangelogContent>) -> Self {
        Self { open, content }
    }

    pub fn is_open(&self) -> bool {
        (self.open)()
    }

    /// Show the modal with whatever content is already set.
    pub fn show(self) {
        self.open.clone().set(true);
    }

    /// Replace content, then show.
    pub fn show_with(self, content: ChangelogContent) {
        self.content.clone().set(content);
        self.open.clone().set(true);
    }

    /// Replace sections only, keep title/description/url, then show.
    pub fn show_sections(self, sections: Vec<ChangelogSection>) {
        self.content.clone().write().sections = sections;
        self.open.clone().set(true);
    }

    pub fn hide(self) {
        self.open.clone().set(false);
    }

    pub fn set_content(self, content: ChangelogContent) {
        self.content.clone().set(content);
    }

    pub fn set_sections(self, sections: Vec<ChangelogSection>) {
        self.content.clone().write().sections = sections;
    }
}

pub fn use_changelog() -> Changelog {
    use_context::<Changelog>()
}

#[component]
pub fn ChangelogHost() -> Element {
    let _lang = i18n();
    let changelog = use_context::<Changelog>();
    let open = changelog.open;
    let content = changelog.content;
    let body = content();

    let title = if body.title.is_empty() {
        t_key("changelog-title")
    } else {
        body.title.clone()
    };
    let description = if body.description.is_empty() {
        t_key("changelog-description")
    } else {
        body.description.clone()
    };
    let repo_url = body.repo_url.clone();
    let sections = body.sections.clone();

    rsx! {
        Modal {
            open,
            title,
            description,
            size: ModalSize::Md,
            footer: rsx! {
                Button {
                    variant: ButtonVariant::Ghost,
                    onclick: move |_| changelog.hide(),
                    { t!("changelog-close") }
                }
                a {
                    href: "{repo_url}",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    class: "ui-btn ui-squircle ui-btn-primary inline-flex h-10 items-center justify-center gap-2 px-4 text-sm font-semibold",
                    { t!("changelog-view-github") }
                }
            },
            div { class: "changelog-list",
                if sections.is_empty() {
                    p { class: "text-sm text-text-muted", { t!("changelog-empty") } }
                } else {
                    for section in sections {
                        section { class: "changelog-section",
                            div { class: "changelog-section-head",
                                h3 { class: "changelog-version", "{section.version}" }
                                if !section.date.is_empty() {
                                    span { class: "changelog-date", "{section.date}" }
                                }
                            }
                            ul { class: "changelog-items",
                                for item in section.items {
                                    li { "{item.text}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
