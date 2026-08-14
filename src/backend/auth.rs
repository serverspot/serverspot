use std::collections::HashSet;

use anyhow::anyhow;
use async_trait::async_trait;
use axum_session_auth::{Authentication, HasPermission};
use chrono::{DateTime, Duration, Utc};
use dioxus::logger::tracing::error;
use hmac::{Hmac, KeyInit, Mac};
use rand::{Rng, RngExt};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use surrealdb::types::SurrealValue;

use crate::{backend::Database, server_funcs::{auth::TwoFactorMethod, model::{AccountPermissions, Roles}}};

pub const OTP_LEN: usize = 8;

/// Generates a one-time passcode. Should be passed into a hash
/// function before being stored on the session.
pub fn generate_otp(rng: &mut impl Rng) -> String  {
    let mut code = String::with_capacity(OTP_LEN);
    for _ in 0..OTP_LEN {
        let char_byte = b'0' + rng.random_range(0..=9);
        code.push(char_byte as char);
    }

    code
}

fn hash_otp(code: &str, secret: &[u8]) -> [u8; 32] {
    let mut mac = Hmac::<Sha256>::new_from_slice(secret)
        .expect("HMAC accepts arbitrary key sizes");

    mac.update(code.as_bytes());

    mac.finalize().into_bytes().into()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingTwoFactor {
    pub account_id: String,
    pub method: TwoFactorMethod,
    pub otp_mac: [u8; 32],
    pub attempts: u8,
    pub expires_at: DateTime<Utc>,
}

impl PendingTwoFactor {
    pub const MAX_ATTEMPTS: u8 = 5;
    pub const OTP_LIFETIME: Duration = Duration::minutes(10);

    pub fn new(account_id: String, method: TwoFactorMethod, otp: &str, secret: &[u8]) -> Self {
        Self {
            account_id,
            method,
            otp_mac: hash_otp(otp, secret),
            attempts: 0,
            expires_at: Utc::now() + Self::OTP_LIFETIME,
        }
    }

    pub fn validate(&mut self, attempt: &str, secret: &[u8]) -> bool {
        if self.is_expired() {
            return false;
        }

        let attempt_mac = hash_otp(attempt, secret);

        if attempt_mac == self.otp_mac {
            return true;
        }

        self.attempts += 1;
        false
    }

    pub fn is_expired(&self) -> bool {
        self.attempts > Self::MAX_ATTEMPTS || Utc::now() > self.expires_at
    }
}

/// Represents an active account session that has already been authenticated.
#[derive(Clone, Debug)]
pub struct ActiveAccount {
    /// The ID of the account.
    pub account_id: String,
    
    /// The total permissions of this account.
    pub perms: AccountPermissions,
}

#[async_trait]
impl HasPermission<Database> for ActiveAccount {
    async fn has(&self, perm: &str, _db: &Option<&Database>) -> bool {
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
impl Authentication<ActiveAccount, String, Database> for ActiveAccount {
    async fn load_user(userid: String, db: Option<&Database>) -> anyhow::Result<Self> {
        let db = db.ok_or(anyhow!("database unavailable"))?;

        let mut res = db
            .query("type::record('account', $id).roles")
            .bind(("id", userid.as_str()))
            .await?;

        let roles: Option<Roles> = res.take(0)?;
        let roles = roles.unwrap();

        Ok(Self {
            account_id: userid,
            perms: roles.total_permissions(),
        })
    }

    fn is_authenticated(&self) -> bool {
        true
    }

    fn is_active(&self) -> bool {
        true
    }

    fn is_anonymous(&self) -> bool {
        false
    }
}

#[derive(SurrealValue, Debug)]
pub struct AuthAccount {
    pub id: String,
    pub email: Option<String>,
    pub email_verified: bool,
    pub game_id: Option<String>,
    pub game_id_verified: bool,
    pub passwd_hash: String,
    pub require_2fa: bool,
}

impl AuthAccount {
    pub fn valid_2fa_methods(&self, game_can_authenticate: bool) -> HashSet<TwoFactorMethod> {
        let mut output = HashSet::with_capacity(2);

        if self.email.is_some() && self.email_verified {
            output.insert(TwoFactorMethod::Email);
        }

        if game_can_authenticate && self.game_id.is_some() && self.game_id_verified {
            output.insert(TwoFactorMethod::Game);
        }

        output
    }
}