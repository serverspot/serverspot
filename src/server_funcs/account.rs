use dioxus::{fullstack::AsStatusCode, prelude::*};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::server_funcs::error::CommonError;

#[cfg(feature="server")]
use dioxus::server::axum::Extension;
#[cfg(feature="server")]
use crate::backend::AppState;

#[derive(Error, Serialize, Deserialize, Debug, Clone)]
pub enum AccountError {
    #[error("This username is taken.")]
    UsernameTaken,

    #[error("An account is already linked to the given game id.")]
    GameIdTaken,

    #[error("{0}")]
    Common(#[from] CommonError),
}

impl AsStatusCode for AccountError {
    fn as_status_code(&self) -> StatusCode {
        match self {
            Self::UsernameTaken | Self::GameIdTaken => StatusCode::BAD_REQUEST,
            Self::Common(e) => e.as_status_code(),
        }
    }
}

crate::err_impl_from_common_child! {AccountError=>
    dioxus::server::ServerFnError,
    bcrypt::BcryptError,
    surrealdb::Error
}

/// Create an account. Returns the account ID if successful.
/// Account/gameid must be verified by logging in via 2fa within 24 hours or the account will be deleted.
// TODO after game module is more implemented: Username param (Option<String>) is only usable if the game module allows custom usernames.
#[post("/api/account/create", state: Extension<AppState>)]
pub async fn create_account(game_id: String, password: String, username: String) -> Result<String, AccountError> {
    const AVAIL_QUERIES: &str = r#"
        fn::is_username_taken($username);
        fn::is_game_id_taken($game_id);
    "#;

    let mut res = state.db.query(AVAIL_QUERIES)
        .bind(("username", username.as_str()))
        .bind(("game_id", game_id.as_str()))
        .await?;

    // i'm not sure how safe these unwraps are, TODO investigate and test.
    let username_taken: Option<bool> = res.take(0)?;
    let username_taken = username_taken.unwrap();

    if username_taken {
        return Err(AccountError::UsernameTaken);
    }

    let game_id_taken: Option<bool> = res.take(1)?;
    let game_id_taken = game_id_taken.unwrap();

    // TODO allow overriding false accounts
    if game_id_taken {
        return Err(AccountError::GameIdTaken);
    }

    let passwd_hash = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;

    let mut res = state.db.query("fn::create_unverified_account($username, $passwd_hash, $game_id)")
        .bind(("username", username))
        .bind(("passwd_hash", passwd_hash))
        .bind(("game_id", game_id))
        .await?;

    let account_id: Option<String> = res.take(0)?;

    Ok(account_id.unwrap())
}