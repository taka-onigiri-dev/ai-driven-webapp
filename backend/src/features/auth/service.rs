use super::domain::{
    AuthResponse, Claims, LoginRequest, RefreshRequest, RefreshResponse, RegisterRequest,
    UserResponse,
};
use super::repository::{RefreshTokenRepository, UserRepository};
use crate::config::Config;
use crate::entities::users;
use crate::shared::{AppError, AppResult};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use sha2::{Digest, Sha256};
use std::sync::Arc;

pub struct AuthService {
    user_repo: Arc<dyn UserRepository>,
    token_repo: Arc<dyn RefreshTokenRepository>,
    config: Config,
}

impl AuthService {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        token_repo: Arc<dyn RefreshTokenRepository>,
        config: Config,
    ) -> Self {
        Self {
            user_repo,
            token_repo,
            config,
        }
    }

    /// ユーザー登録
    pub async fn register(&self, req: RegisterRequest) -> AppResult<AuthResponse> {
        // メールアドレスの重複チェック
        if let Some(_) = self.user_repo.find_by_email(&req.email).await? {
            return Err(AppError::Conflict {
                message: "Email already exists".to_string(),
            });
        }

        // パスワードのハッシュ化
        let password_hash = self.hash_password(&req.password)?;

        // ユーザー作成
        let user = self
            .user_repo
            .create(&req.email, &password_hash, &req.name, "user")
            .await?;

        // トークン生成
        let (access_token, refresh_token) = self.generate_tokens(&user).await?;

        Ok(AuthResponse {
            user: self.user_to_response(&user),
            access_token,
            refresh_token,
        })
    }

    /// ログイン
    pub async fn login(&self, req: LoginRequest) -> AppResult<AuthResponse> {
        // ユーザー検索
        let user = self
            .user_repo
            .find_by_email(&req.email)
            .await?
            .ok_or_else(|| AppError::Unauthorized {
                message: "Invalid email or password".to_string(),
            })?;

        // アクティブチェック
        if !user.is_active {
            return Err(AppError::Forbidden {
                message: "Account is not active".to_string(),
            });
        }

        // パスワード検証
        if !self.verify_password(&req.password, &user.password_hash)? {
            return Err(AppError::Unauthorized {
                message: "Invalid email or password".to_string(),
            });
        }

        // トークン生成
        let (access_token, refresh_token) = self.generate_tokens(&user).await?;

        Ok(AuthResponse {
            user: self.user_to_response(&user),
            access_token,
            refresh_token,
        })
    }

    /// ログアウト
    pub async fn logout(&self, user_id: i64) -> AppResult<()> {
        // ユーザーのすべてのリフレッシュトークンを削除
        self.token_repo.delete_by_user_id(user_id).await?;

        Ok(())
    }

    /// トークンリフレッシュ
    pub async fn refresh(&self, req: RefreshRequest) -> AppResult<RefreshResponse> {
        // リフレッシュトークンのハッシュを計算
        let token_hash = self.hash_token(&req.refresh_token);

        // トークン検索
        let token_record = self
            .token_repo
            .find_by_token_hash(&token_hash)
            .await?
            .ok_or_else(|| AppError::Unauthorized {
                message: "Invalid refresh token".to_string(),
            })?;

        // 有効期限チェック
        let now = chrono::Utc::now();
        if token_record.expires_at.naive_utc() < now.naive_utc() {
            // 期限切れトークンを削除
            self.token_repo
                .delete_by_token_hash(&token_hash)
                .await?;

            return Err(AppError::Unauthorized {
                message: "Refresh token expired".to_string(),
            });
        }

        // ユーザー検索
        let user = self
            .user_repo
            .find_by_id(token_record.user_id)
            .await?
            .ok_or_else(|| AppError::NotFound {
                resource: "User".to_string(),
                id: token_record.user_id.to_string(),
            })?;

        // 新しいアクセストークンを生成
        let access_token = self.generate_access_token(&user)?;

        Ok(RefreshResponse { access_token })
    }

    /// JWTトークンを検証
    pub fn verify_token(&self, token: &str) -> AppResult<Claims> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.config.jwt.secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| AppError::Unauthorized {
            message: "Invalid token".to_string(),
        })?;

        Ok(token_data.claims)
    }

    // --- プライベートヘルパーメソッド ---

    /// パスワードをハッシュ化
    fn hash_password(&self, password: &str) -> AppResult<String> {
        hash(password, DEFAULT_COST).map_err(|_| AppError::InternalError {
            message: "Failed to hash password".to_string(),
        })
    }

    /// パスワードを検証
    fn verify_password(&self, password: &str, hash: &str) -> AppResult<bool> {
        verify(password, hash).map_err(|_| AppError::InternalError {
            message: "Failed to verify password".to_string(),
        })
    }

    /// アクセストークンとリフレッシュトークンを生成
    async fn generate_tokens(&self, user: &users::Model) -> AppResult<(String, String)> {
        let access_token = self.generate_access_token(user)?;
        let refresh_token = self.generate_refresh_token(user).await?;

        Ok((access_token, refresh_token))
    }

    /// アクセストークンを生成
    fn generate_access_token(&self, user: &users::Model) -> AppResult<String> {
        let claims = Claims::new(
            user.id,
            user.email.clone(),
            user.role.clone(),
            self.config.jwt.access_token_expiry,
        );

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.config.jwt.secret.as_bytes()),
        )
        .map_err(|_| AppError::InternalError {
            message: "Failed to generate access token".to_string(),
        })
    }

    /// リフレッシュトークンを生成
    async fn generate_refresh_token(&self, user: &users::Model) -> AppResult<String> {
        let claims = Claims::new(
            user.id,
            user.email.clone(),
            user.role.clone(),
            self.config.jwt.refresh_token_expiry,
        );

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.config.jwt.secret.as_bytes()),
        )
        .map_err(|_| AppError::InternalError {
            message: "Failed to generate refresh token".to_string(),
        })?;

        // トークンのハッシュをDBに保存
        let token_hash = self.hash_token(&token);
        let expires_at = chrono::Utc::now()
            + chrono::Duration::seconds(self.config.jwt.refresh_token_expiry);

        self.token_repo
            .create(user.id, &token_hash, expires_at.into())
            .await?;

        Ok(token)
    }

    /// トークンをSHA-256でハッシュ化
    fn hash_token(&self, token: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Userモデルをレスポンスに変換
    fn user_to_response(&self, user: &users::Model) -> UserResponse {
        UserResponse {
            id: user.id,
            email: user.email.clone(),
            name: user.name.clone(),
            role: user.role.clone(),
            is_active: user.is_active,
            created_at: user.created_at.to_string(),
            updated_at: user.updated_at.to_string(),
        }
    }
}
