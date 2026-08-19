use std::collections::HashSet;
use std::sync::OnceLock;

use dioxus_i18n::prelude::*;
use dioxus_i18n::unic_langid::{langid, LanguageIdentifier};
use dioxus_i18n::{t, tid};

use crate::nav::Section;
use crate::router::Route;

pub const DEFAULT_LOCALE: &str = "en-US";

const EN_US_FTL: &str = include_str!("../i18n/en-US.ftl");
const FR_FR_FTL: &str = include_str!("../i18n/fr-FR.ftl");
const DE_DE_FTL: &str = include_str!("../i18n/de-DE.ftl");
const ES_ES_FTL: &str = include_str!("../i18n/es-ES.ftl");

pub fn init_i18n_config() -> I18nConfig {
    I18nConfig::new(langid!("en-US"))
        .with_locale((langid!("en-US"), EN_US_FTL))
        .with_locale((langid!("fr-FR"), FR_FR_FTL))
        .with_locale((langid!("de-DE"), DE_DE_FTL))
        .with_locale((langid!("es-ES"), ES_ES_FTL))
}

pub fn locale_to_langid(locale: &str) -> LanguageIdentifier {
    match locale {
        "fr-FR" => langid!("fr-FR"),
        "de-DE" => langid!("de-DE"),
        "es-ES" => langid!("es-ES"),
        _ => langid!("en-US"),
    }
}

pub fn apply_user_locale(locale: &str) {
    i18n().set_language(locale_to_langid(locale));
}

pub fn t_key(id: &str) -> String {
    tid!(id).to_string()
}

/// A translation catalog shown on Settings → Localisation.
#[derive(Clone, PartialEq)]
pub struct TranslationCatalog {
    pub code: String,
    pub filename: String,
    pub key_count: usize,
    pub matching_keys: usize,
    pub is_source: bool,
    pub is_uploaded: bool,
    pub accent: &'static str,
}

impl TranslationCatalog {
    pub fn completion_pct(&self, source_total: usize) -> u8 {
        if source_total == 0 {
            return 0;
        }
        ((self.matching_keys * 100) / source_total).min(100) as u8
    }

    pub fn display_name(&self) -> String {
        locale_display_name(&self.code)
    }
}

pub fn locale_display_name(code: &str) -> String {
    match code {
        "en-US" => t_key("lang-en"),
        "fr-FR" => t_key("lang-fr"),
        "de-DE" => t_key("lang-de"),
        "es-ES" => t_key("lang-es"),
        other => other.to_string(),
    }
}

pub fn parse_ftl_keys(source: &str) -> HashSet<String> {
    source
        .lines()
        .filter_map(|line| {
            if line.is_empty() || line.starts_with('#') || line.starts_with(char::is_whitespace) {
                return None;
            }
            let (key, _) = line.split_once('=')?;
            let key = key.trim();
            if key.is_empty()
                || !key
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
            {
                return None;
            }
            Some(key.to_string())
        })
        .collect()
}

pub fn source_ftl_keys() -> &'static HashSet<String> {
    static KEYS: OnceLock<HashSet<String>> = OnceLock::new();
    KEYS.get_or_init(|| parse_ftl_keys(EN_US_FTL))
}

pub fn source_key_count() -> usize {
    source_ftl_keys().len()
}

fn catalog_from_ftl(
    code: &str,
    filename: &str,
    source: &str,
    is_source: bool,
    accent: &'static str,
) -> TranslationCatalog {
    let keys = parse_ftl_keys(source);
    let matching = keys.intersection(source_ftl_keys()).count();
    TranslationCatalog {
        code: code.to_string(),
        filename: filename.to_string(),
        key_count: keys.len(),
        matching_keys: matching,
        is_source,
        is_uploaded: false,
        accent,
    }
}

pub fn builtin_translation_catalogs() -> Vec<TranslationCatalog> {
    vec![
        catalog_from_ftl("en-US", "en-US.ftl", EN_US_FTL, true, "#34d399"),
        catalog_from_ftl("fr-FR", "fr-FR.ftl", FR_FR_FTL, false, "#c4b5fd"),
        catalog_from_ftl("de-DE", "de-DE.ftl", DE_DE_FTL, false, "#87d1fe"),
        catalog_from_ftl("es-ES", "es-ES.ftl", ES_ES_FTL, false, "#5b9dff"),
    ]
}

pub fn catalog_from_uploaded_ftl(
    filename: &str,
    source: &str,
) -> Result<TranslationCatalog, &'static str> {
    let code = locale_code_from_filename(filename).ok_or("settings-locale-upload-bad-name")?;
    if source.trim().is_empty() {
        return Err("settings-locale-upload-empty");
    }
    let keys = parse_ftl_keys(source);
    if keys.is_empty() {
        return Err("settings-locale-upload-no-keys");
    }
    let matching = keys.intersection(source_ftl_keys()).count();
    Ok(TranslationCatalog {
        code,
        filename: filename.to_string(),
        key_count: keys.len(),
        matching_keys: matching,
        is_source: false,
        is_uploaded: true,
        accent: "#f0a35e",
    })
}

pub fn locale_code_from_filename(filename: &str) -> Option<String> {
    let name = filename
        .rsplit(['/', '\\'])
        .next()
        .unwrap_or(filename)
        .trim();
    let stem = name
        .strip_suffix(".ftl")
        .or_else(|| name.strip_suffix(".FTL"))?;
    let code = stem.replace('_', "-");
    let mut parts = code.split('-');
    let lang = parts.next()?;
    if lang.len() < 2 || !lang.chars().all(|c| c.is_ascii_alphabetic()) {
        return None;
    }
    let mut normalized = lang.to_ascii_lowercase();
    if let Some(region) = parts.next() {
        if region.is_empty() || !region.chars().all(|c| c.is_ascii_alphanumeric()) {
            return None;
        }
        normalized.push('-');
        normalized.push_str(&region.to_ascii_uppercase());
    }
    if parts.next().is_some() {
        return None;
    }
    Some(normalized)
}

pub fn settings_locale_keys_note(matched: usize, source_total: usize, filename: String) -> String {
    t!(
        "settings-locale-keys-note",
        matched: matched,
        source_total: source_total,
        filename: filename
    )
}

pub fn settings_locale_pct_complete(pct: u8) -> String {
    t!("settings-locale-pct-complete", pct: pct)
}

pub fn settings_locale_upload_success(locale: String) -> String {
    t!("settings-locale-upload-success", locale: locale)
}

pub fn community_case_joined(case_id: String, joined_at: String) -> String {
    t!("community-case-joined", case_id: case_id, joined_at: joined_at)
}

pub fn community_status_last_seen(status: String, last_seen: String) -> String {
    t!("community-status-last-seen", status: status, last_seen: last_seen)
}

pub fn section_label(section: Section) -> String {
    match section {
        Section::Dashboard => t_key("nav-section-dashboard"),
        Section::Store => t_key("nav-section-store"),
        Section::Forum => t_key("nav-section-forum"),
        Section::Support => t_key("nav-section-support"),
        Section::Content => t_key("nav-section-content"),
        Section::Players => t_key("nav-section-players"),
        Section::Leaderboards => t_key("nav-section-leaderboards"),
        Section::Votes => t_key("nav-section-votes"),
        Section::Applications => t_key("nav-section-applications"),
        Section::Analytics => t_key("nav-section-analytics"),
        Section::Settings => t_key("nav-section-settings"),
        Section::Account => t_key("nav-section-account"),
    }
}

pub fn section_document_title(section: Section) -> String {
    t!("nav-document-title", section: section_label(section))
}

pub fn subnav_label(id: &str) -> String {
    t_key(id)
}

pub fn crumb_label(route: &Route) -> String {
    match route {
        Route::Login {} => t_key("crumb-login"),
        Route::LoginOtp {} => t_key("crumb-verification"),
        Route::LoginReset {} => t_key("crumb-reset-password"),
        Route::Dashboard {} => t_key("crumb-overview"),
        Route::DashboardActivity {} => t_key("crumb-activity"),
        Route::StoreOverview {} => t_key("crumb-overview"),
        Route::StoreProducts {} => t_key("crumb-products"),
        Route::StoreProductNew {} => t_key("crumb-new-product"),
        Route::StoreProductEdit { .. } => t_key("crumb-edit-product"),
        Route::StoreCategories {} => t_key("crumb-categories"),
        Route::StoreCategoryNew {} => t_key("crumb-new-category"),
        Route::StoreCategoryEdit { .. } => t_key("crumb-edit-category"),
        Route::StoreCoupons {} => t_key("crumb-coupons"),
        Route::StoreCouponNew {} => t_key("crumb-new-coupon"),
        Route::StoreCouponEdit { .. } => t_key("crumb-edit-coupon"),
        Route::StoreOrders {} => t_key("crumb-orders"),
        Route::StoreSettings {} => t_key("crumb-settings"),
        Route::ForumOverview {} => t_key("crumb-overview"),
        Route::ForumBoards {} => t_key("crumb-boards"),
        Route::ForumBoardNew {} => t_key("crumb-new-board"),
        Route::ForumBoardEdit { .. } => t_key("crumb-edit-board"),
        Route::ForumThreads {} => t_key("crumb-threads"),
        Route::ForumThreadNew {} => t_key("crumb-new-thread"),
        Route::ForumThread { .. } => t_key("crumb-thread"),
        Route::ForumModeration {} => t_key("crumb-moderation"),
        Route::ForumAutoModeration {} => t_key("crumb-auto-moderation"),
        Route::ForumSiteSettings {} => t_key("crumb-settings"),
        Route::SupportOverview {} => t_key("crumb-overview"),
        Route::SupportTickets {} => t_key("crumb-tickets"),
        Route::SupportTicket { .. } => t_key("crumb-ticket"),
        Route::SupportHelpCentre {} => t_key("crumb-help-centre"),
        Route::SupportHelpNew {} => t_key("crumb-new-article"),
        Route::SupportHelpEdit { .. } => t_key("crumb-edit-article"),
        Route::SupportAutomation {} => t_key("crumb-automation"),
        Route::SupportSiteSettings {} => t_key("crumb-settings"),
        Route::ContentOverview {} => t_key("crumb-overview"),
        Route::ContentBlog {} => t_key("crumb-posts"),
        Route::ContentPostNew {} => t_key("crumb-new-post"),
        Route::ContentPostEdit { .. } => t_key("crumb-edit-post"),
        Route::ContentSiteSettings {} => t_key("crumb-settings"),
        Route::PlayersOverview {} => t_key("crumb-overview"),
        Route::CommunityPlayers {} => t_key("crumb-profiles"),
        Route::PlayersProfileDetail { .. } => t_key("crumb-player-file"),
        Route::PlayersSiteSettings {} => t_key("crumb-settings"),
        Route::LeaderboardsOverview {} => t_key("crumb-overview"),
        Route::CommunityLeaderboards {} => t_key("crumb-boards"),
        Route::LeaderboardsBoardNew {} => t_key("crumb-new-board"),
        Route::LeaderboardsBoardEdit { .. } => t_key("crumb-edit-board"),
        Route::LeaderboardsSiteSettings {} => t_key("crumb-settings"),
        Route::VotesOverview {} => t_key("crumb-overview"),
        Route::CommunityVotes {} => t_key("crumb-rewards"),
        Route::VoteRewardNew {} => t_key("crumb-new-reward"),
        Route::VoteRewardEdit { .. } => t_key("crumb-edit-reward"),
        Route::VotesSiteSettings {} => t_key("crumb-settings"),
        Route::ApplicationsOverview {} => t_key("crumb-overview"),
        Route::CommunityApplications {} => t_key("crumb-inbox"),
        Route::ApplicationReview { .. } => t_key("crumb-review-application"),
        Route::ApplicationFormNew {} => t_key("crumb-new-form"),
        Route::ApplicationsSiteSettings {} => t_key("crumb-settings"),
        Route::AnalyticsOverview {} => t_key("crumb-overview"),
        Route::AnalyticsWebsite {} => t_key("crumb-website"),
        Route::AnalyticsCommunity {} => t_key("crumb-community"),
        Route::AnalyticsGaming {} => t_key("crumb-gaming"),
        Route::AnalyticsSiteSettings {} => t_key("crumb-settings"),
        Route::SettingsGeneral {} => t_key("crumb-general"),
        Route::AccountsAuth {} => t_key("crumb-authentication"),
        Route::AccountsStaff {} => t_key("crumb-staff"),
        Route::AccountsRoles {} => t_key("crumb-roles"),
        Route::AccountsRoleNew {} => t_key("crumb-new-role"),
        Route::SettingsLocalisation {} => t_key("crumb-localisation"),
        Route::SettingsDeveloper {} => t_key("crumb-developer"),
        Route::SettingsIntegrations {} => t_key("crumb-integrations"),
        Route::SettingsSecurity {} => t_key("crumb-security"),
        Route::SettingsHosting {} => t_key("crumb-hosting"),
        Route::SettingsTheme {} => t_key("crumb-theme"),
        Route::Account {} => t_key("crumb-profile"),
        Route::AdminNotFound { .. } => t_key("crumb-not-found"),
    }
}
