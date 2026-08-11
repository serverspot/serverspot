use dioxus::{fullstack::AsStatusCode, prelude::*};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::server_funcs::error::CommonError;

#[derive(Error, Serialize, Deserialize, Debug, Clone)]
pub enum AccountError {
    #[error("{0}")]
    Common(#[from] CommonError),
}

impl AsStatusCode for AccountError {
    fn as_status_code(&self) -> StatusCode {
        match self {
            Self::Common(e) => e.as_status_code(),
        }
    }
}

#[post("/api/account/create")]
pub async fn create_account(username: String, password: String, game_id: String) -> Result<()> {

    Ok(())
}