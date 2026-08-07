use anyhow::anyhow;
use async_trait::async_trait;
use axum_session_auth::{Authentication, HasPermission};
use chrono::{DateTime, Duration, Utc};
use dashmap::DashMap;
use dioxus::logger::tracing::error;
use rand::RngExt;
use surrealdb::types::SurrealValue;

use crate::{backend::Database, server_funcs::model::{AccountPermissions, Roles}};

pub struct OneTimePasscode {
    pub code: String,
    pub expiration: DateTime<Utc>,
}

impl OneTimePasscode {
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expiration
    }
}

impl From<String> for OneTimePasscode
{
    fn from(value: String) -> Self {
        Self {
            code: value.into(),
            expiration: Utc::now() + Duration::hours(1),
        }
    }
}

pub struct TwoFactorAuth {
    auth_codes: DashMap<String, OneTimePasscode>,
}

impl TwoFactorAuth {
    pub const OTP_VALID_CHARS: &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

    pub fn new() -> Self {
        Self {
            auth_codes: DashMap::new(),
        }
    }

    pub fn generate_code(&self, session_id: String) -> String  {
        let mut rng = rand::rng();

        let mut code = String::with_capacity(10);
        for _ in 0..10 {
            let char_idx: usize = rng.random_range(0..Self::OTP_VALID_CHARS.len());

            // safety is guaranteed since the bounds are restricted enough.
            code.push(unsafe { Self::OTP_VALID_CHARS.chars().nth(char_idx).unwrap_unchecked() });
        }

        let output = code.clone();
        let code = OneTimePasscode::from(code);

        self.auth_codes.insert(session_id.clone(), code);
    
        output
    }

    pub fn validate_code(&self, session_id: &str, code: &str) -> bool {
        if let Some(real_code) = self.auth_codes.get_mut(session_id) {
            if real_code.is_expired() {
                drop(real_code);
                self.auth_codes.remove(session_id);
                return false;
            }

            &real_code.code == code
        } else {
            false
        }
    }

    pub fn prune_expired_codes(&self) {
        self.auth_codes.retain(|_, code| !code.is_expired());
    }
}

#[derive(Clone, Debug)]
pub struct ActiveUser {
    id: String,
    perms: AccountPermissions,
    anonymous: bool, // TODO better system for tracking anonymous visitors
}

#[async_trait]
impl HasPermission<Database> for ActiveUser {
    async fn has(&self, perm: &str, pool: &Option<&Database>) -> bool {
        match perm.parse::<AccountPermissions>() {
            Ok(perm_flag) => self.perms.contains(perm_flag),
            Err(e) => {
                error!("Failed to validate permission: {e}");
                false
            }
        }
    }
}

#[async_trait]
impl Authentication<ActiveUser, String, Database> for ActiveUser {
    async fn load_user(userid: String, db: Option<&Database>) -> anyhow::Result<Self> {
        let db = db.ok_or(anyhow!("failed to authenticate user: db not yet loaded"))?;

        let mut res = db.query("type::record('account', $id).roles")
            .bind(("id", userid.as_str()))
            .await?;

        let roles: Option<Roles> = res.take(0)?;
        let roles = roles.unwrap();

        let perms = roles.total_permissions();

        Ok(Self {
            id: userid,
            perms,
            anonymous: false,
        })
    }

    fn is_authenticated(&self) -> bool {
        !self.anonymous
    }

    fn is_active(&self) -> bool {
        !self.anonymous
    }

    fn is_anonymous(&self) -> bool {
        self.anonymous
    }
}