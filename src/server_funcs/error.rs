use dioxus::{fullstack::{AsStatusCode, StatusCode}, server::ServerFnError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Errors used throughout several different server function types.
#[derive(Error, Serialize, Deserialize, Debug, Clone)]
pub enum CommonError {
    /// Generic error for cases where a request reaches
    /// an endpoint without an authorized session.
    #[error("Unauthorized")]
    Unauthorized,

    /// these are errors with stuff that dioxus handles internally.
    /// the functions would never need to directly return this, but all server function
    /// error types must implement From<ServerFnError>.
    #[error("Server function related error: {0}")]
    ServerFn(#[from] ServerFnError),

    /// An internal server error whose state should not be made public to the frontend.
    /// An error log should be printed to the server's terminal whenever one of these is thrown.
    #[error("An internal server error has occurred")]
    Internal,
}

impl AsStatusCode for CommonError {
    fn as_status_code(&self) -> StatusCode {
        match self {
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::ServerFn(e) => e.as_status_code(),
            Self::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[cfg(feature = "server")]
pub trait CommonErrorExt {
    fn new_internal(e: impl std::fmt::Display) -> Self;
    fn unauthorized() -> Self;
}

#[cfg(feature = "server")]
impl<T> CommonErrorExt for T
where
    T: From<CommonError>,
{
    fn new_internal(e: impl std::fmt::Display) -> Self {
        use dioxus::logger::tracing::error;

        error!("Encountered internal server error: {e}");
        Self::from(CommonError::Internal)
    }

    fn unauthorized() -> Self {
        Self::from(CommonError::Unauthorized)
    }
}

/// Helper macro to implement `From<E>` for types which become [`CommonError::Internal`]
macro_rules! impl_from_internal {
    ($E: ty) => {
        #[cfg(feature = "server")]
        impl From<$E> for CommonError {
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
    bcrypt::BcryptError,
    anyhow::Error
}

/// Helper macro to implement `From<E>` for types where `CommonError: From<E>` via a transitive [`CommonError`].
#[macro_export]
macro_rules! err_impl_from_common_child {
    ($T: ty, $E: ty) => {
        impl From<$E> for $T {
            fn from(e: $E) -> $T {
                <$T as From<CommonError>>::from(CommonError::from(e))
            }
        }
    };
    ($T: ty => $($E: ty),* ) => {
        $(crate::err_impl_from_common_child!($T, $E);)*
    };
}