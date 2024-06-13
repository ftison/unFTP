use async_trait::async_trait;
use serde::Deserialize;
use std::path::PathBuf;
use unftp_sbe_restrict::VfsOperations;

use crate::domain::user::{User, UserDetailError, UserDetailProvider};

/// A [`UserDetailProvider`] that gets user details from env vars.
#[derive(Debug, Deserialize)]
pub struct UniqueUserProvider {}

#[async_trait]
impl UserDetailProvider for UniqueUserProvider {
    async fn provide_user_detail(&self, username: &str) -> Result<User, UserDetailError> {
        Ok(User {
            username: username.to_string(),
            name: None,
            surname: None,
            account_enabled: true,
            vfs_permissions: VfsOperations::all(),
            allowed_mime_types: None,
            root: Some(PathBuf::from(username.to_string())),
        })
    }
}
