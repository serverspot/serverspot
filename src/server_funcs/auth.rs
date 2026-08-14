use std::collections::HashSet;

use dioxus::{fullstack::AsStatusCode, prelude::*};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::server_funcs::error::CommonError;

#[cfg(feature = "server")]
use dioxus::server::axum::Extension;
#[cfg(feature = "server")]
use crate::{server_funcs::error::CommonErrorExt as _, backend::{AuthSession, AppState, auth::{AuthAccount, PendingTwoFactor, generate_otp}}};
#[cfg(feature = "server")]
use surrealdb::types::SurrealValue;


// TODO move types like this to a separate common mod
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "server", derive(SurrealValue))]
pub enum TwoFactorMethod {
    Game,
    Email,
}

impl ToString for TwoFactorMethod {
    fn to_string(&self) -> String {
        match self {
            Self::Game => "game",
            Self::Email => "email",
        }.into()
    }
}

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum AuthenticationError {
    #[error("No account was found with the provided username")]
    AccountNotFound,

    #[error("The login/secret provided was invalid")]
    InvalidSecret,
    
    #[error("The 2fa method requested is not available for this account. Available methods include: {0:?}")]
    InvalidTwoFactor(HashSet<TwoFactorMethod>),

    #[error("The session state is not prepared for this request. This usually means the session expired or the request was sent out of order in a procedure.")]
    InvalidSession,

    #[error("The related resource has been invalidated.")]
    Expired,

    #[error("{0}")]
    Common(#[from] CommonError),
}

impl AsStatusCode for AuthenticationError {
    fn as_status_code(&self) -> StatusCode {
        match self {
            Self::AccountNotFound => StatusCode::NOT_FOUND,
            Self::InvalidSecret => StatusCode::UNAUTHORIZED,
            Self::InvalidTwoFactor(_) => StatusCode::BAD_REQUEST,
            Self::InvalidSession => StatusCode::CONFLICT,
            Self::Expired => StatusCode::GONE,
            Self::Common(e) => e.as_status_code(),
        }
    }
}

crate::err_impl_from_common_child! {AuthenticationError=>
    surrealdb::Error,
    bcrypt::BcryptError,
    dioxus::server::ServerFnError
}

/// The resulting status of a login request.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LoginStatus {
    /// Login has completed successfully.
    Complete,

    /// Proceed with two factor authentication.
    /// The code has been sent to the method selected,
    /// and must be submitted via submit_2fa within an hour.
    /// The field represents the 2fa method provided in the initial request
    TwoFactorRequired(TwoFactorMethod),
}

#[cfg(feature = "server")]
const PENDING_TWO_FACTOR_KEY: &str = "pending-2fa";
#[cfg(feature = "server")]
const REMEMBER_ME_KEY: &str = "remember-me";

/// Initiate a login by entering the username and password.
/// The two factor method may be required if require_2fa is set to true for the account.
#[post("/api/auth/login", auth: AuthSession, state: Extension<AppState>)]
pub async fn initiate_login(
    username: String,
    password: String,
    two_factor_method: Option<TwoFactorMethod>,
    remember_me: bool,
) -> Result<LoginStatus, AuthenticationError> {
    const QUERIES: &str = r#"
        SELECT id,email,email_verified,game_id,game_id_verified,passwd_hash,require_2fa FROM account WHERE username = $username LIMIT 1;
        SELECT VALUE val FROM global_setting:game_can_authenticate;
    "#; 

    let mut res = state.db
        .query(QUERIES)
        .bind(("username", username))
        .await?;

    let account: Option<AuthAccount> = res.take(0)?;
    let account = account.ok_or(AuthenticationError::AccountNotFound)?;

    let game_can_authenticate: Option<bool> = res.take(1)?;
    let game_can_authenticate = game_can_authenticate.ok_or_else(|| {
        AuthenticationError::new_internal("'game_can_authenticate' global setting not found, cannot proceed with authentication")
    })?;

    if !bcrypt::verify(password, &account.passwd_hash)? {
        return Err(AuthenticationError::InvalidSecret);
    }
    
    let valid_2fa_methods = account.valid_2fa_methods(game_can_authenticate);
    
    if !(two_factor_method.as_ref().is_some_and(|m| valid_2fa_methods.contains(m)) ||
        (two_factor_method.is_none() && !account.require_2fa)) {
        return Err(AuthenticationError::InvalidTwoFactor(valid_2fa_methods)); 
    }

    match two_factor_method {
        Some(method) => {
            // generate and hash a OTP, storing it in the session's state.
            let mut rng = rand::rng();
            let code = generate_otp(&mut rng);
            let otp_state = PendingTwoFactor::new(account.id, method, &code, &state.auth_secret);
            auth.session.set(PENDING_TWO_FACTOR_KEY, otp_state);

            // store the "remember me" choice so we can call remember_user later.
            auth.session.set(REMEMBER_ME_KEY, remember_me);

            todo!("dispatch code to 2fa")
        },
        None => {
            auth.login_user(account.id.clone());
            auth.remember_user(remember_me);
            Ok(LoginStatus::Complete)
        }
    }
}

#[post("/api/auth/verify", auth: AuthSession, state: Extension<AppState>)]
pub async fn submit_2fa(code: String) -> Result<(), AuthenticationError> {
    let mut otp_state: PendingTwoFactor = auth.session.get(PENDING_TWO_FACTOR_KEY).ok_or(AuthenticationError::InvalidSession)?;
    let remember_me: bool = auth.session.get(REMEMBER_ME_KEY).ok_or(AuthenticationError::InvalidSession)?;

    let expired = otp_state.is_expired();

    // run full validity check and update attempt count
    let is_valid = otp_state.validate(&code, &state.auth_secret);

    // save new attempt count
    auth.session.set(PENDING_TWO_FACTOR_KEY, &otp_state);

    // if attempts count was too high or took too long before validation even began
    if expired {
        // cleanup expired values since they will need
        // to be replaced by another call to initiate_login anyway
        auth.session.remove(PENDING_TWO_FACTOR_KEY);
        auth.session.remove(REMEMBER_ME_KEY);
        return Err(AuthenticationError::Expired);
    }

    // any other reason validation failed
    if !is_valid {
        return Err(AuthenticationError::InvalidSecret);
    }

    // 2fa challenge accepted and passed
    auth.session.remove(PENDING_TWO_FACTOR_KEY);
    auth.session.remove(REMEMBER_ME_KEY);

    // mark the method as verified
    state.db.query("fn::verify_account_connection($acc, $ty)")
        .bind(("acc", otp_state.account_id.as_str()))
        .bind(("ty", otp_state.method.to_string()))
        .await?;

    // login the session
    auth.login_user(otp_state.account_id);
    auth.remember_user(remember_me);

    Ok(())
}

/// End the current user session.
#[post("/api/auth/logout", auth: AuthSession)]
pub async fn logout() -> Result<()> {
    auth.logout_user();
    Ok(())
}