use super::repository::{RefreshTokenRepository, UserRepository};
use crate::entities::{refresh_tokens, users};
use crate::shared::AppResult;
use async_trait::async_trait;
use chrono::DateTime;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use std::sync::Arc;

pub struct UserRepositoryImpl {
    db: Arc<DatabaseConnection>,
}

impl UserRepositoryImpl {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn find_by_email(&self, email: &str) -> AppResult<Option<users::Model>> {
        let user = users::Entity::find()
            .filter(users::Column::Email.eq(email))
            .filter(users::Column::DeletedAt.is_null())
            .one(self.db.as_ref())
            .await?;
        Ok(user)
    }

    async fn find_by_id(&self, id: i64) -> AppResult<Option<users::Model>> {
        let user = users::Entity::find_by_id(id)
            .filter(users::Column::DeletedAt.is_null())
            .one(self.db.as_ref())
            .await?;
        Ok(user)
    }

    async fn create(
        &self,
        email: &str,
        password_hash: &str,
        name: &str,
        role: &str,
    ) -> AppResult<users::Model> {
        let now = chrono::Utc::now().into();
        let user = users::ActiveModel {
            email: Set(email.to_string()),
            password_hash: Set(password_hash.to_string()),
            name: Set(name.to_string()),
            role: Set(role.to_string()),
            is_active: Set(true),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };

        let user = user.insert(self.db.as_ref()).await?;
        Ok(user)
    }
}

pub struct RefreshTokenRepositoryImpl {
    db: Arc<DatabaseConnection>,
}

impl RefreshTokenRepositoryImpl {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl RefreshTokenRepository for RefreshTokenRepositoryImpl {
    async fn create(
        &self,
        user_id: i64,
        token_hash: &str,
        expires_at: DateTimeWithTimeZone,
    ) -> AppResult<refresh_tokens::Model> {
        let now = chrono::Utc::now().into();
        let token = refresh_tokens::ActiveModel {
            user_id: Set(user_id),
            token_hash: Set(token_hash.to_string()),
            expires_at: Set(expires_at),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };

        let token = token.insert(self.db.as_ref()).await?;
        Ok(token)
    }

    async fn find_by_token_hash(
        &self,
        token_hash: &str,
    ) -> AppResult<Option<refresh_tokens::Model>> {
        let token = refresh_tokens::Entity::find()
            .filter(refresh_tokens::Column::TokenHash.eq(token_hash))
            .one(self.db.as_ref())
            .await?;
        Ok(token)
    }

    async fn delete(&self, id: i64) -> AppResult<()> {
        refresh_tokens::Entity::delete_by_id(id)
            .exec(self.db.as_ref())
            .await?;
        Ok(())
    }

    async fn delete_by_user_id(&self, user_id: i64) -> AppResult<()> {
        refresh_tokens::Entity::delete_many()
            .filter(refresh_tokens::Column::UserId.eq(user_id))
            .exec(self.db.as_ref())
            .await?;
        Ok(())
    }
}
