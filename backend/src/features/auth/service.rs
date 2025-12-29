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

    pub async fn register(&self, req: RegisterRequest) -> AppResult<AuthResponse> {
        // Check for existing email
        if let Some(_) = self.user_repo.find_by_email(&req.email).await? {
            return Err(AppError::Conflict {
                message: "Email already exists".to_string(),
            });
        }

        // Hash password
        let password_hash = self.hash_password(&req.password)?;

        // Create user
        let user = self
            .user_repo
            .create(&req.email, &password_hash, &req.name, "user")
            .await?;

        // Generate tokens
        let (access_token, refresh_token) = self.generate_tokens(&user).await?;

        Ok(AuthResponse {
            user: self.user_to_response(&user),
            access_token,
            refresh_token,
        })
    }

    pub async fn login(&self, req: LoginRequest) -> AppResult<AuthResponse> {
        // Find user
        let user = self
            .user_repo
            .find_by_email(&req.email)
            .await?
            .ok_or_else(|| AppError::Unauthorized {
                message: "Invalid email or password".to_string(),
            })?;

        // Check if active
        if !user.is_active {
            return Err(AppError::Forbidden {
                message: "Account is not active".to_string(),
            });
        }

        // Verify password
        if !self.verify_password(&req.password, &user.password_hash)? {
            return Err(AppError::Unauthorized {
                message: "Invalid email or password".to_string(),
            });
        }

        // Generate tokens
        let (access_token, refresh_token) = self.generate_tokens(&user).await?;

        Ok(AuthResponse {
            user: self.user_to_response(&user),
            access_token,
            refresh_token,
        })
    }

    pub async fn logout(&self, user_id: i64) -> AppResult<()> {
        self.token_repo.delete_by_user_id(user_id).await?;
        Ok(())
    }

    pub async fn refresh(&self, req: RefreshRequest) -> AppResult<RefreshResponse> {
        // Verify refresh token
        let claims = self.verify_token(&req.refresh_token)?;

        // Hash the token for lookup
        let token_hash = self.hash_token(&req.refresh_token);

        // Find token in database
        let token_record = self
            .token_repo
            .find_by_token_hash(&token_hash)
            .await?
            .ok_or_else(|| AppError::Unauthorized {
                message: "Invalid refresh token".to_string(),
            })?;

        // Check expiration
        if token_record.expires_at < chrono::Utc::now().into() {
            self.token_repo.delete(token_record.id).await?;
            return Err(AppError::Unauthorized {
                message: "Refresh token expired".to_string(),
            });
        }

        // Get user
        let user = self
            .user_repo
            .find_by_id(claims.sub)
            .await?
            .ok_or_else(|| AppError::NotFound {
                message: "User not found".to_string(),
            })?;

        // Generate new access token
        let access_token = self.generate_access_token(&user)?;

        Ok(RefreshResponse { access_token })
    }

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

    // Private helpers
    fn hash_password(&self, password: &str) -> AppResult<String> {
        hash(password, DEFAULT_COST).map_err(|_| AppError::InternalError {
            message: "Failed to hash password".to_string(),
        })
    }

    fn verify_password(&self, password: &str, hash: &str) -> AppResult<bool> {
        verify(password, hash).map_err(|_| AppError::InternalError {
            message: "Failed to verify password".to_string(),
        })
    }

    async fn generate_tokens(&self, user: &users::Model) -> AppResult<(String, String)> {
        let access_token = self.generate_access_token(user)?;
        let refresh_token = self.generate_refresh_token(user).await?;
        Ok((access_token, refresh_token))
    }

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

        // Store token hash
        let token_hash = self.hash_token(&token);
        let expires_at = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::seconds(
                self.config.jwt.refresh_token_expiry,
            ))
            .unwrap()
            .into();

        self.token_repo
            .create(user.id, &token_hash, expires_at)
            .await?;

        Ok(token)
    }

    fn hash_token(&self, token: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    fn user_to_response(&self, user: &users::Model) -> UserResponse {
        UserResponse {
            id: user.id,
            email: user.email.clone(),
            name: user.name.clone(),
            role: user.role.clone(),
            is_active: user.is_active,
        }
    }
}
