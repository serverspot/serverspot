use dioxus::{fullstack::{AsStatusCode, StatusCode}, server::ServerFnError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Serialize, Deserialize, Debug, Clone)]
pub enum CommonError {
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
            Self::ServerFn(e) => e.as_status_code(),
            Self::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[cfg(feature = "server")]
pub trait HasInternalError {
    fn new_internal(e: impl std::fmt::Display) -> Self;
}

#[cfg(feature = "server")]
impl<T> HasInternalError for T
where
    T: From<CommonError>,
{
    fn new_internal(e: impl std::fmt::Display) -> Self {
        use dioxus::logger::tracing::error;

        error!("Encountered internal server error: {e}");
        Self::from(CommonError::Internal)
    }
}

/// Helper macro to implement From<E> for types which become [`CommonError::Internal`]
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