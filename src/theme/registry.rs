//! Which theme files are required (undeletable) in the active theme pack.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThemeFeatureId {
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

impl ThemeFeatureId {
    pub fn slug(self) -> &'static str {
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

    pub fn from_slug(slug: &str) -> Option<Self> {
        Some(match slug {
            "store" => Self::Store,
            "forum" => Self::Forum,
            "support" => Self::Support,
            "blog" | "content" => Self::Content,
            "players" => Self::Players,
            "leaderboards" => Self::Leaderboards,
            "votes" => Self::Votes,
            "applications" => Self::Applications,
            "analytics" => Self::Analytics,
            _ => return None,
        })
    }

    /// Required files inside this feature folder (e.g. `forum/index.html`).
    pub fn required_files(self) -> &'static [&'static str] {
        match self {
            Self::Forum => &["index.html", "thread.html", "styles.css"],
            Self::Support => &["index.html", "ticket.html", "styles.css"],
            Self::Store => &["index.html", "product.html", "styles.css"],
            Self::Content => &["index.html", "post.html", "styles.css"],
            Self::Players => &["index.html", "profile.html", "styles.css"],
            Self::Leaderboards => &["index.html", "board.html", "styles.css"],
            Self::Votes => &["index.html", "claim.html", "styles.css"],
            Self::Applications => &["index.html", "form.html", "styles.css"],
            Self::Analytics => &["index.html", "styles.css"],
        }
    }
}

/// Root-level required files (not inside a feature folder).
pub fn root_required_files() -> &'static [&'static str] {
    &[
        "schema.json",
        "index.html",
        "login.html",
        "profile.html",
        "styles.css",
    ]
}

pub fn theme_features() -> &'static [ThemeFeatureId] {
    &[
        ThemeFeatureId::Store,
        ThemeFeatureId::Forum,
        ThemeFeatureId::Support,
        ThemeFeatureId::Content,
        ThemeFeatureId::Players,
        ThemeFeatureId::Leaderboards,
        ThemeFeatureId::Votes,
        ThemeFeatureId::Applications,
        ThemeFeatureId::Analytics,
    ]
}

pub fn feature_required_files(slug: &str) -> &'static [&'static str] {
    ThemeFeatureId::from_slug(slug)
        .map(ThemeFeatureId::required_files)
        .unwrap_or(&[])
}

/// `path` is relative to the theme root, e.g. `index.html`, `forum/thread.html`, `assets/header.html`.
pub fn is_required_theme_path(path: &str) -> bool {
    let path = path.trim().trim_start_matches('/').replace('\\', "/");
    if path.starts_with("assets/") {
        return false;
    }
    if !path.contains('/') {
        return root_required_files().iter().any(|f| *f == path);
    }
    let Some((folder, file)) = path.split_once('/') else {
        return false;
    };
    if file.contains('/') {
        // nested optional files are never required
        return false;
    }
    feature_required_files(folder)
        .iter()
        .any(|required| *required == file)
}
