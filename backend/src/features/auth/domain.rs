use crate::shared::AppError;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// ユーザー登録リクエスト
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email, length(max = 255))]
    pub email: String,

    #[validate(length(min = 8, max = 100), custom = "validate_password")]
    pub password: String,

    #[validate(length(min = 1, max = 100))]
    pub name: String,
}

/// ログインリクエスト
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 1))]
    pub password: String,
}

/// リフレッシュトークンリクエスト
#[derive(Debug, Clone, Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

/// 認証レスポンス
#[derive(Debug, Clone, Serialize)]
pub struct AuthResponse {
    pub user: UserResponse,
    pub access_token: String,
    pub refresh_token: String,
}

/// ユーザーレスポンス
#[derive(Debug, Clone, Serialize)]
pub struct UserResponse {
    pub id: i64,
    pub email: String,
    pub name: String,
    pub role: String,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

/// トークンリフレッシュレスポンス
#[derive(Debug, Clone, Serialize)]
pub struct RefreshResponse {
    pub access_token: String,
}

/// パスワードの複雑性検証
fn validate_password(password: &str) -> Result<(), validator::ValidationError> {
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_numeric());

    if !has_uppercase || !has_lowercase || !has_digit {
        return Err(validator::ValidationError::new(
            "password must contain uppercase, lowercase, and digit",
        ));
    }

    Ok(())
}

/// JWTクレーム
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,          // ユーザーID
    pub email: String,
    pub role: String,
    pub exp: i64,          // 有効期限（Unixタイムスタンプ）
    pub iat: i64,          // 発行時刻
}

impl Claims {
    pub fn new(user_id: i64, email: String, role: String, expiry_seconds: i64) -> Self {
        let now = chrono::Utc::now().timestamp();
        Self {
            sub: user_id,
            email,
            role,
            exp: now + expiry_seconds,
            iat: now,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_password_valid() {
        assert!(validate_password("Password123").is_ok());
    }

    #[test]
    fn test_validate_password_no_uppercase() {
        assert!(validate_password("password123").is_err());
    }

    #[test]
    fn test_validate_password_no_lowercase() {
        assert!(validate_password("PASSWORD123").is_err());
    }

    #[test]
    fn test_validate_password_no_digit() {
        assert!(validate_password("PasswordABC").is_err());
    }

    #[test]
    fn test_claims_creation() {
        let claims = Claims::new(
            1,
            "test@example.com".to_string(),
            "user".to_string(),
            900,
        );

        assert_eq!(claims.sub, 1);
        assert_eq!(claims.email, "test@example.com");
        assert_eq!(claims.role, "user");
        assert!(claims.exp > claims.iat);
    }
}
