use async_trait::async_trait;
use axum_session_auth::{Authentication, HasPermission};

use crate::backend::Database;

#[derive(Clone)]
pub struct UserAuth {
    // TODO
}

#[async_trait]
impl HasPermission<Database> for UserAuth {
    async fn has(&self, perm: &str, pool: &Option<&Database>) -> bool {
        todo!()
    }
}

#[async_trait]
impl Authentication<UserAuth, i64, Database> for UserAuth {
    async fn load_user(userid: i64, pool: Option<&Database>) -> anyhow::Result<UserAuth> {
        todo!()
    }

    fn is_authenticated(&self) -> bool {
        todo!()
    }

    fn is_active(&self) -> bool {
        todo!()
    }

    fn is_anonymous(&self) -> bool {
        todo!()
    }
}