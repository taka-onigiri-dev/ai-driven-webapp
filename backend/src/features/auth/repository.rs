use crate::entities::{refresh_tokens, users};
use crate::shared::AppResult;
use async_trait::async_trait;
use chrono::DateTime;
use sea_orm::prelude::DateTimeWithTimeZone;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_email(&self, email: &str) -> AppResult<Option<users::Model>>;
    async fn find_by_id(&self, id: i64) -> AppResult<Option<users::Model>>;
    async fn create(
        &self,
        email: &str,
        password_hash: &str,
        name: &str,
        role: &str,
    ) -> AppResult<users::Model>;
}

#[async_trait]
pub trait RefreshTokenRepository: Send + Sync {
    async fn create(
        &self,
        user_id: i64,
        token_hash: &str,
        expires_at: DateTimeWithTimeZone,
    ) -> AppResult<refresh_tokens::Model>;
    async fn find_by_token_hash(&self, token_hash: &str) -> AppResult<Option<refresh_tokens::Model>>;
    async fn delete(&self, id: i64) -> AppResult<()>;
    async fn delete_by_user_id(&self, user_id: i64) -> AppResult<()>;
}
