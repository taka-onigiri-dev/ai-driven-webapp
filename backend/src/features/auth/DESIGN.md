# Auth Feature

## 概要

JWT（JSON Web Token）ベースの認証機能を提供します。

## 機能

- ユーザー登録
- ログイン（Access Token + Refresh Token発行）
- ログアウト
- トークンリフレッシュ

## エンドポイント

### ユーザー登録
```
POST /api/v1/auth/register
```

**リクエスト**:
```json
{
  "email": "user@example.com",
  "password": "SecurePass123!",
  "name": "John Doe"
}
```

**レスポンス**:
```json
{
  "data": {
    "user": {
      "id": 1,
      "email": "user@example.com",
      "name": "John Doe",
      "role": "user"
    },
    "access_token": "eyJ...",
    "refresh_token": "eyJ..."
  }
}
```

### ログイン
```
POST /api/v1/auth/login
```

**リクエスト**:
```json
{
  "email": "user@example.com",
  "password": "SecurePass123!"
}
```

**レスポンス**: 登録と同じ

### ログアウト
```
POST /api/v1/auth/logout
```

**ヘッダー**: `Authorization: Bearer <access_token>`

### トークンリフレッシュ
```
POST /api/v1/auth/refresh
```

**リクエスト**:
```json
{
  "refresh_token": "eyJ..."
}
```

**レスポンス**:
```json
{
  "data": {
    "access_token": "eyJ..."
  }
}
```

## セキュリティ

- パスワードはbcryptでハッシュ化（コスト: 12）
- Access Token: 15分有効
- Refresh Token: 7日有効、DBに保存
- Refresh TokenはSHA-256でハッシュ化してDB保存

## バリデーション

### Email
- 必須
- メールアドレス形式
- 最大255文字

### Password
- 必須
- 最小8文字
- 最大100文字
- 英大文字、英小文字、数字を含む

### Name
- 必須
- 最小1文字
- 最大100文字

## エラーケース

- メールアドレス重複: 409 Conflict
- 認証情報不正: 401 Unauthorized
- バリデーションエラー: 400 Bad Request
