use crate::entities::{refresh_tokens, users};
use crate::shared::AppResult;
use async_trait::async_trait;
use sea_orm::prelude::DateTimeWithTimeZone;

/// ユーザーリポジトリのトレイト
#[async_trait]
pub trait UserRepository: Send + Sync {
    /// メールアドレスでユーザーを検索
    async fn find_by_email(&self, email: &str) -> AppResult<Option<users::Model>>;

    /// ユーザーを作成
    async fn create(
        &self,
        email: &str,
        password_hash: &str,
        name: &str,
        role: &str,
    ) -> AppResult<users::Model>;

    /// ユーザーIDで検索
    async fn find_by_id(&self, id: i64) -> AppResult<Option<users::Model>>;
}

/// リフレッシュトークンリポジトリのトレイト
#[async_trait]
pub trait RefreshTokenRepository: Send + Sync {
    /// リフレッシュトークンを作成
    async fn create(
        &self,
        user_id: i64,
        token_hash: &str,
        expires_at: DateTimeWithTimeZone,
    ) -> AppResult<refresh_tokens::Model>;

    /// トークンハッシュで検索
    async fn find_by_token_hash(
        &self,
        token_hash: &str,
    ) -> AppResult<Option<refresh_tokens::Model>>;

    /// ユーザーIDでトークンを削除
    async fn delete_by_user_id(&self, user_id: i64) -> AppResult<()>;

    /// トークンハッシュで削除
    async fn delete_by_token_hash(&self, token_hash: &str) -> AppResult<()>;
}
