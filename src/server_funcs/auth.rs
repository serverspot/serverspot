use std::collections::HashSet;

use dioxus::{fullstack::AsStatusCode, prelude::*};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[cfg(feature = "server")]
use dioxus::server::axum::Extension;
#[cfg(feature = "server")]
use crate::backend::AppState;
#[cfg(feature = "server")]
use crate::backend::AuthSession;
#[cfg(feature = "server")]
use crate::backend::auth::AuthAccount;
use crate::backend::auth::PendingTwoFactor;
use crate::backend::auth::generate_otp;
#[cfg(feature = "server")]
use surrealdb::types::SurrealValue;


// TODO move types like this to a separate common mod
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "server", derive(SurrealValue))]
pub enum TwoFactorMethod {
    Game,
    Email,
}

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum AuthenticationError {
    #[error("No account was found with the provided username")]
    AccountNotFound,

    #[error("The login provided was invalid")]
    InvalidLogin,
    
    #[error("The 2fa method requested is not available for this account. Available methods include: {0:?}")]
    InvalidTwoFactor(HashSet<TwoFactorMethod>),

    // these are errors with stuff that dioxus handles internally.
    // the functions would never need to directly return this, but all server function
    // error types must implement From<ServerFnError>.
    #[error("Server function related error: {0}")]
    ServerFn(#[from] ServerFnError),

    /// An internal server error whose state should not be made public to the frontend.
    /// An error log should be printed to the server's terminal whenever one of these is thrown.
    #[error("An internal server error has occurred")]
    Internal,
}

impl AsStatusCode for AuthenticationError {
    fn as_status_code(&self) -> StatusCode {
        match self {
            Self::AccountNotFound => StatusCode::NOT_FOUND,
            Self::InvalidLogin => StatusCode::UNAUTHORIZED,
            Self::InvalidTwoFactor(_) => StatusCode::BAD_REQUEST,
            Self::ServerFn(e) => e.as_status_code(),
            Self::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[cfg(feature = "server")]
impl AuthenticationError {
    pub fn new_internal(e: impl std::fmt::Display) -> Self {
        error!("Encountered internal server error: {e}");
        Self::Internal
    }
}

/// Helper macro to implement From<E> for types which become [`AuthenticationError::Internal`]
macro_rules! impl_from_internal {
    ($E: ty) => {
        #[cfg(feature = "server")]
        impl From<$E> for AuthenticationError {
            fn from(e: $E) -> Self {
                Self::new_internal(e)
            }
        }
    };
    ($($E: ty),*) => {
        $(impl_from_internal!($E);)*
    };
}

impl_from_internal! {
    surrealdb::Error,
    bcrypt::BcryptError
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
const PENDING_REMEMBER_ME: &str = "remember-me";

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
        error!("'game_can_authenticate' global setting not found, cannot proceed with authentication");
        AuthenticationError::Internal
    })?;

    if !bcrypt::verify(password, &account.passwd_hash)? {
        return Err(AuthenticationError::InvalidLogin);
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
            auth.session.set(PENDING_REMEMBER_ME, remember_me);

            todo!("dispatch code to 2fa")
        },
        None => {
            auth.login_user(account.id.clone());
            auth.remember_user(remember_me);
            Ok(LoginStatus::Complete)
        }
    }
}

#[post("/api/auth/verify")]
pub async fn submit_2fa() -> Result<(), AuthenticationError> {
    todo!()
}

/// End the current user session.
#[post("/api/auth/logout", auth: AuthSession)]
pub async fn logout() -> Result<()> {
    auth.logout_user();
    Ok(())
}