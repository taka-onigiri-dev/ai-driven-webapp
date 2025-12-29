# Webアプリケーション 設計書

## ドキュメント管理情報

- 作成日: 2025-12-29
- 最終更新日: 2025-12-29
- ステータス: Draft

---

## 1. システム概要

### 1.1 システムの目的

本システムは、モダンなWebアプリケーション開発のベストプラクティスを実装した、スケーラブルで保守性の高いWebアプリケーションです。

### 1.2 システムの特徴

- **設計先行開発**: 実装前に設計を固め、設計書ベースで開発を進める
- **テスト駆動**: すべての機能にテストを用意し、高いカバレッジを維持する
- **AI駆動開発**: Claude Codeによる実装を前提とした構成
- **モノレポ構成**: フロントエンド・バックエンド・インフラを一元管理

### 1.3 ユースケース（初期想定）

**Phase 1（MVP）**:
- ユーザー登録・ログイン
- ユーザープロファイル管理
- 基本的なCRUD操作

**Phase 2（拡張）**:
- 権限管理（ロールベース）
- 検索機能
- 通知機能

**Phase 3（発展）**:
- リアルタイム機能
- データ分析
- 外部サービス連携

---

## 2. 技術スタック詳細

### 2.1 バックエンド

| 項目 | 技術 | バージョン | 選定理由 |
|------|------|-----------|---------|
| 言語 | Rust | 1.75+ | 型安全性、パフォーマンス、メモリ安全性 |
| Webフレームワーク | Actix Web | 4.x | 高性能、非同期処理、柔軟性 |
| ORM | SeaORM | 0.12+ | 型安全、async/await対応、マイグレーション管理 |
| DB | PostgreSQL | 15+ | ACID保証、拡張性、JSON対応 |
| 認証 | jsonwebtoken | 9.x | JWT標準、シンプル、自己完結型 |
| バリデーション | validator | 0.16+ | 宣言的バリデーション、カスタムルール対応 |
| シリアライズ | serde / serde_json | 1.x | 標準的、高性能、マクロベース |
| ログ | tracing | 0.1+ | 構造化ログ、非同期対応、分散トレーシング対応 |
| テスト | tokio-test, mockall | - | 非同期テスト、モック作成 |

### 2.2 フロントエンド

| 項目 | 技術 | バージョン | 選定理由 |
|------|------|-----------|---------|
| フレームワーク | Next.js | 14.x | App Router、SSR/SSG、最適化 |
| 言語 | TypeScript | 5.x | 型安全性、IDE支援 |
| UIライブラリ | React | 18.x | コンポーネント指向、エコシステム |
| スタイリング | Tailwind CSS | 3.x | ユーティリティファースト、カスタマイズ性 |
| 状態管理 | Zustand | 4.x | シンプル、TypeScript対応 |
| フォーム | React Hook Form | 7.x | パフォーマンス、バリデーション統合 |
| HTTPクライアント | fetch (native) | - | 標準API、軽量 |
| テスト | Vitest, Testing Library | - | 高速、React対応 |

### 2.3 インフラ・開発環境

| 項目 | 技術 | 用途 |
|------|------|------|
| コンテナ | Docker | 環境統一 |
| オーケストレーション | Docker Compose | ローカル開発環境 |
| CI/CD | GitHub Actions | 自動テスト、デプロイ |
| リンター | clippy, ESLint | コード品質 |
| フォーマッター | rustfmt, Prettier | コードスタイル統一 |

---

## 3. アーキテクチャ設計

### 3.1 全体アーキテクチャ

```
┌─────────────────────────────────────────────────────┐
│                   User (Browser)                    │
└─────────────────┬───────────────────────────────────┘
                  │
                  │ HTTPS
                  ▼
┌─────────────────────────────────────────────────────┐
│              Frontend (Next.js)                     │
│  ┌──────────────────────────────────────────────┐  │
│  │ Pages / Components                           │  │
│  ├──────────────────────────────────────────────┤  │
│  │ Hooks / State Management (Zustand)           │  │
│  ├──────────────────────────────────────────────┤  │
│  │ API Client Layer                             │  │
│  └──────────────────────────────────────────────┘  │
└─────────────────┬───────────────────────────────────┘
                  │
                  │ REST API (JSON)
                  ▼
┌─────────────────────────────────────────────────────┐
│           Backend (Actix Web / Rust)                │
│  ┌──────────────────────────────────────────────┐  │
│  │ Handler Layer (HTTP)                         │  │
│  │  - リクエスト受信、レスポンス返却            │  │
│  │  - 認証チェック、バリデーション              │  │
│  ├──────────────────────────────────────────────┤  │
│  │ Service / UseCase Layer                      │  │
│  │  - ビジネスロジック                          │  │
│  │  - トランザクション制御                      │  │
│  ├──────────────────────────────────────────────┤  │
│  │ Domain Layer                                 │  │
│  │  - ドメインモデル                            │  │
│  │  - バリデーションルール                      │  │
│  ├──────────────────────────────────────────────┤  │
│  │ Repository Layer (Trait)                     │  │
│  │  - データアクセスインターフェース            │  │
│  ├──────────────────────────────────────────────┤  │
│  │ Infrastructure Layer                         │  │
│  │  - DB実装 (SeaORM)                           │  │
│  │  - 外部サービス連携                          │  │
│  └──────────────────────────────────────────────┘  │
└─────────────────┬───────────────────────────────────┘
                  │
                  │ SQL
                  ▼
┌─────────────────────────────────────────────────────┐
│              PostgreSQL Database                    │
└─────────────────────────────────────────────────────┘
```

### 3.2 レイヤードアーキテクチャ詳細

#### 3.2.1 Handler Layer

**責務**:
- HTTPリクエストの受信とパース
- 認証・認可の確認
- 入力バリデーション（型レベル）
- Serviceレイヤーの呼び出し
- HTTPレスポンスの構築

**依存関係**:
- Service Layer を利用
- Domain Layer の型を利用

**テスト方針**:
- HTTPリクエスト/レスポンスの確認
- ステータスコードの確認
- 認証チェックの確認

#### 3.2.2 Service / UseCase Layer

**責務**:
- ビジネスロジックの実装
- 複数のRepositoryの協調制御
- トランザクション境界の定義
- ドメインルールの適用

**依存関係**:
- Repository Trait に依存（実装には依存しない）
- Domain Layer を利用

**テスト方針**:
- ビジネスロジックの単体テスト（100%カバレッジ目標）
- Repository は Mock を利用

#### 3.2.3 Domain Layer

**責務**:
- ドメインモデルの定義
- ドメイン固有のバリデーション
- ビジネスルールの表現

**依存関係**:
- 他レイヤーに依存しない（純粋なドメインロジック）

**テスト方針**:
- すべてのバリデーションロジックをテスト（100%カバレッジ必須）
- 副作用なし、純粋関数として実装

#### 3.2.4 Repository Layer

**責務**:
- データアクセスのインターフェース定義（Trait）
- CRUD操作の抽象化

**依存関係**:
- Domain Layer の型を利用

**テスト方針**:
- Trait定義のみのため、単体テストは不要
- 結合テストで実装を確認

#### 3.2.5 Infrastructure Layer

**責務**:
- Repository Trait の具体実装
- DBアクセス（SeaORM）
- 外部サービスとの通信

**依存関係**:
- Repository Trait を実装
- Domain Layer の型を利用
- 外部ライブラリ（SeaORM等）を利用

**テスト方針**:
- 結合テスト（実際のDBを使用）
- 主要なCRUD操作のみテスト

---

## 4. ディレクトリ構成

### 4.1 全体構成

```
ai-driven-webapp/
├── README.md                    # プロジェクト概要
├── docs/                        # ドキュメント
│   ├── DESIGN.md               # 本ドキュメント
│   ├── API.md                  # API仕様書
│   ├── DATABASE.md             # DB設計書
│   └── DEPLOYMENT.md           # デプロイ手順
├── docker/                      # Docker関連
│   ├── backend/
│   │   └── Dockerfile
│   ├── frontend/
│   │   └── Dockerfile
│   └── postgres/
│       └── init.sql
├── docker-compose.yml          # ローカル環境定義
├── backend/                     # バックエンド
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs             # エントリーポイント
│   │   ├── config/             # 設定管理
│   │   │   └── mod.rs
│   │   ├── shared/             # 共通機能
│   │   │   ├── error.rs        # エラー型定義
│   │   │   ├── middleware/     # ミドルウェア
│   │   │   └── utils/          # ユーティリティ
│   │   └── features/           # 機能別モジュール
│   │       ├── health/
│   │       │   ├── mod.rs
│   │       │   ├── DESIGN.md   # 機能別設計書
│   │       │   ├── handler.rs
│   │       │   └── tests/
│   │       ├── auth/
│   │       │   ├── mod.rs
│   │       │   ├── DESIGN.md
│   │       │   ├── handler.rs
│   │       │   ├── service.rs
│   │       │   ├── domain.rs
│   │       │   ├── repository.rs
│   │       │   ├── infra.rs
│   │       │   └── tests/
│   │       └── user/
│   │           ├── mod.rs
│   │           ├── DESIGN.md
│   │           ├── handler.rs
│   │           ├── service.rs
│   │           ├── domain.rs
│   │           ├── repository.rs
│   │           ├── infra.rs
│   │           └── tests/
│   └── tests/                   # 結合テスト
│       └── integration/
└── frontend/                    # フロントエンド
    ├── package.json
    ├── next.config.js
    ├── tsconfig.json
    ├── tailwind.config.js
    ├── src/
    │   ├── app/                # App Router
    │   │   ├── layout.tsx
    │   │   ├── page.tsx
    │   │   ├── login/
    │   │   └── dashboard/
    │   ├── components/         # 再利用可能コンポーネント
    │   │   ├── ui/             # UIコンポーネント
    │   │   └── features/       # 機能別コンポーネント
    │   ├── lib/                # ライブラリ・ユーティリティ
    │   │   ├── api/            # APIクライアント
    │   │   ├── hooks/          # カスタムフック
    │   │   ├── store/          # 状態管理
    │   │   └── utils/          # ユーティリティ関数
    │   └── types/              # 型定義
    └── tests/                  # テスト
        ├── unit/
        ├── component/
        └── e2e/
```

### 4.2 Feature構成の詳細

各featureは以下の構成を基本とする：

```
features/{feature_name}/
├── mod.rs              # モジュール定義、公開API
├── DESIGN.md           # 機能設計書
├── handler.rs          # HTTPハンドラ
├── service.rs          # ビジネスロジック
├── domain.rs           # ドメインモデル・バリデーション
├── repository.rs       # データアクセストレイト定義
├── infra.rs            # リポジトリ実装
└── tests/              # テストコード
    ├── handler_test.rs
    ├── service_test.rs
    └── domain_test.rs
```

---

## 5. データベース設計

### 5.1 設計方針

- **正規化**: 第3正規形を基本とする
- **命名規則**: スネークケース（例: `user_id`, `created_at`）
- **必須カラム**: すべてのテーブルに `id`, `created_at`, `updated_at` を含める
- **論理削除**: 必要に応じて `deleted_at` を使用
- **外部キー制約**: すべての外部キー参照に制約を設定
- **インデックス**: 検索条件・結合条件には適切にインデックスを設定

### 5.2 共通カラム定義

```sql
-- すべてのテーブルに含める基本カラム
id            BIGSERIAL PRIMARY KEY,
created_at    TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
updated_at    TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
```

### 5.3 初期テーブル設計（MVP）

#### 5.3.1 usersテーブル

```sql
CREATE TABLE users (
    id              BIGSERIAL PRIMARY KEY,
    email           VARCHAR(255) NOT NULL UNIQUE,
    password_hash   VARCHAR(255) NOT NULL,
    name            VARCHAR(100) NOT NULL,
    role            VARCHAR(50) NOT NULL DEFAULT 'user',
    is_active       BOOLEAN NOT NULL DEFAULT true,
    created_at      TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    deleted_at      TIMESTAMP WITH TIME ZONE
);

CREATE INDEX idx_users_email ON users(email) WHERE deleted_at IS NULL;
CREATE INDEX idx_users_role ON users(role);
```

**カラム説明**:
- `email`: ログインID（ユニーク制約）
- `password_hash`: bcryptでハッシュ化されたパスワード
- `name`: 表示名
- `role`: ロール（`admin`, `user`等）
- `is_active`: アカウント有効フラグ
- `deleted_at`: 論理削除用

#### 5.3.2 refresh_tokensテーブル

```sql
CREATE TABLE refresh_tokens (
    id              BIGSERIAL PRIMARY KEY,
    user_id         BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_hash      VARCHAR(255) NOT NULL UNIQUE,
    expires_at      TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at      TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_refresh_tokens_user_id ON refresh_tokens(user_id);
CREATE INDEX idx_refresh_tokens_expires_at ON refresh_tokens(expires_at);
```

**カラム説明**:
- `user_id`: ユーザーID（外部キー）
- `token_hash`: リフレッシュトークンのハッシュ
- `expires_at`: トークン有効期限

### 5.4 マイグレーション方針

- **ツール**: SeaORM CLI を使用
- **バージョン管理**: すべてのマイグレーションファイルをGit管理
- **命名規則**: `{timestamp}_{description}.sql`
- **ロールバック**: すべてのマイグレーションにdownスクリプトを用意
- **環境**: dev, staging, productionで同一のマイグレーションを使用

---

## 6. API設計

### 6.1 REST API設計方針

- **規約**: RESTful原則に従う
- **バージョニング**: `/api/v1/` プレフィックスを使用
- **HTTPメソッド**: GET, POST, PUT, PATCH, DELETE を適切に使い分け
- **ステータスコード**: 標準的なHTTPステータスコードを使用
- **レスポンス形式**: JSON形式
- **エラーレスポンス**: 統一されたエラー形式

### 6.2 エンドポイント命名規則

```
/api/v1/{resource}          # コレクション
/api/v1/{resource}/{id}     # 単一リソース
/api/v1/{resource}/{id}/{sub-resource}  # サブリソース
```

### 6.3 HTTPステータスコード使用方針

| コード | 用途 | 例 |
|-------|------|-----|
| 200 | 成功（GET, PUT, PATCH） | リソース取得成功 |
| 201 | 作成成功（POST） | リソース作成成功 |
| 204 | 成功（レスポンスボディなし）（DELETE） | 削除成功 |
| 400 | リクエスト不正 | バリデーションエラー |
| 401 | 未認証 | トークン未提供、無効 |
| 403 | 権限なし | アクセス権限不足 |
| 404 | リソース不存在 | 指定IDのリソースが存在しない |
| 409 | 競合 | 一意制約違反 |
| 422 | 処理不可 | ビジネスルール違反 |
| 500 | サーバーエラー | 予期しないエラー |

### 6.4 レスポンス形式

#### 6.4.1 成功レスポンス

```json
{
  "data": {
    "id": 1,
    "name": "John Doe",
    "email": "john@example.com"
  }
}
```

複数リソース:

```json
{
  "data": [
    { "id": 1, "name": "John" },
    { "id": 2, "name": "Jane" }
  ],
  "pagination": {
    "total": 100,
    "page": 1,
    "per_page": 20,
    "total_pages": 5
  }
}
```

#### 6.4.2 エラーレスポンス

```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "入力値が不正です",
    "details": [
      {
        "field": "email",
        "message": "メールアドレスの形式が不正です"
      }
    ]
  }
}
```

### 6.5 初期APIエンドポイント（MVP）

#### 6.5.1 ヘルスチェック

```
GET /api/v1/health
```

**レスポンス**:
```json
{
  "status": "ok",
  "version": "1.0.0",
  "timestamp": "2025-12-29T10:00:00Z"
}
```

#### 6.5.2 認証

```
POST /api/v1/auth/register
POST /api/v1/auth/login
POST /api/v1/auth/refresh
POST /api/v1/auth/logout
```

#### 6.5.3 ユーザー

```
GET    /api/v1/users          # ユーザー一覧（要認証）
GET    /api/v1/users/{id}     # ユーザー詳細（要認証）
GET    /api/v1/users/me       # 自分の情報（要認証）
PUT    /api/v1/users/me       # 自分の情報更新（要認証）
DELETE /api/v1/users/{id}     # ユーザー削除（要管理者権限）
```

詳細なAPI仕様は `docs/API.md` に記載する。

---

## 7. 認証・認可設計

### 7.1 認証方式

**JWT（JSON Web Token）を使用**:
- **Access Token**: 短期間有効（15分）、APIアクセスに使用
- **Refresh Token**: 長期間有効（7日）、Access Token再発行に使用

### 7.2 認証フロー

```
1. ユーザー登録 / ログイン
   ↓
2. サーバーがAccess Token + Refresh Tokenを発行
   ↓
3. クライアントがAccess TokenをAuthorizationヘッダーに付与してAPI呼び出し
   ↓
4. Access Token期限切れ時、Refresh Tokenで再発行
   ↓
5. Refresh Token期限切れ時、再ログイン
```

### 7.3 トークン構成

**Access Token（JWT）**:
```json
{
  "sub": "user_id",
  "email": "user@example.com",
  "role": "user",
  "exp": 1234567890,
  "iat": 1234567000
}
```

### 7.4 認可方式

**ロールベースアクセス制御（RBAC）**:

| ロール | 説明 | 権限 |
|-------|------|------|
| `admin` | 管理者 | すべての操作 |
| `user` | 一般ユーザー | 自分のリソースのみ操作 |

### 7.5 実装方針

- **パスワードハッシュ**: bcryptを使用（コスト: 12）
- **トークン保存**: Refresh TokenはDBに保存（ハッシュ化）
- **ミドルウェア**: 認証ミドルウェアでトークン検証
- **権限チェック**: ハンドラーまたはサービス層で実施

---

## 8. エラーハンドリング設計

### 8.1 エラー分類

```rust
// 共通エラー型（イメージ）
pub enum AppError {
    // バリデーションエラー
    ValidationError { field: String, message: String },

    // 認証エラー
    Unauthorized { message: String },

    // 認可エラー
    Forbidden { message: String },

    // リソース不存在
    NotFound { resource: String, id: String },

    // 競合エラー
    Conflict { message: String },

    // ビジネスルールエラー
    BusinessRuleViolation { message: String },

    // 内部エラー
    InternalError { message: String },

    // データベースエラー
    DatabaseError { message: String },
}
```

### 8.2 エラーハンドリング方針

- **Result型の徹底**: すべてのエラー可能性がある関数は `Result<T, E>` を返す
- **エラーの伝播**: `?` 演算子を活用し、適切にエラーを上位に伝播
- **エラーの変換**: 各レイヤーで適切なエラー型に変換
- **ログ記録**: すべてのエラーを構造化ログに記録
- **ユーザーへの情報**: セキュリティを考慮し、内部エラー詳細は隠蔽

### 8.3 エラーコード体系

```
{CATEGORY}_{SPECIFIC_ERROR}

例:
- AUTH_INVALID_TOKEN
- AUTH_TOKEN_EXPIRED
- VALIDATION_INVALID_EMAIL
- USER_NOT_FOUND
- USER_ALREADY_EXISTS
```

---

## 9. ログ設計

### 9.1 ログレベル

| レベル | 用途 | 例 |
|-------|------|-----|
| ERROR | エラー発生 | 予期しない例外、システムエラー |
| WARN | 警告 | リトライ、非推奨機能の使用 |
| INFO | 重要な情報 | リクエスト受信、処理完了 |
| DEBUG | デバッグ情報 | 中間処理の状態 |
| TRACE | 詳細トレース | 関数呼び出し詳細 |

### 9.2 ログ出力形式

**構造化ログ（JSON形式）**:

```json
{
  "timestamp": "2025-12-29T10:00:00Z",
  "level": "INFO",
  "target": "backend::features::auth::handler",
  "message": "User login successful",
  "fields": {
    "user_id": 123,
    "email": "user@example.com",
    "ip_address": "192.168.1.1"
  },
  "span": {
    "name": "login_request",
    "request_id": "abc-123-def"
  }
}
```

### 9.3 ログ方針

- **個人情報**: パスワード、トークンは絶対にログに出力しない
- **リクエストID**: すべてのリクエストに一意なIDを付与し、ログに含める
- **スパン**: 分散トレーシングを想定し、スパン情報を含める
- **本番環境**: INFO レベル以上を出力
- **開発環境**: DEBUG レベル以上を出力

---

## 10. テスト設計

### 10.1 テスト戦略

```
テストピラミッド:

        /\
       /  \      E2E Tests (少数)
      /____\
     /      \    Integration Tests (中程度)
    /________\
   /          \  Unit Tests (大多数)
  /____________\
```

### 10.2 バックエンドテスト

#### 10.2.1 単体テスト（Unit Tests）

**対象**:
- Domain層（100%カバレッジ必須）
- Service層（100%カバレッジ目標）
- ユーティリティ関数

**方針**:
- 純粋関数として実装し、副作用を排除
- Repository層はMockを使用
- すべてのエッジケースをテスト

**例**:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_email_valid() {
        let email = Email::new("test@example.com").unwrap();
        assert_eq!(email.value(), "test@example.com");
    }

    #[test]
    fn test_validate_email_invalid() {
        let result = Email::new("invalid-email");
        assert!(result.is_err());
    }
}
```

#### 10.2.2 統合テスト（Integration Tests）

**対象**:
- Handler層 + Service層 + Infrastructure層
- 実際のDB接続を使用

**方針**:
- テスト用DBを使用（Docker Composeで起動）
- 各テストで独立したトランザクション
- 主要なユースケースのみテスト

**例**:
```rust
#[actix_web::test]
async fn test_create_user_integration() {
    let pool = setup_test_db().await;
    let app = test::init_service(App::new().configure(configure_routes)).await;

    let req = test::TestRequest::post()
        .uri("/api/v1/auth/register")
        .set_json(&json!({
            "email": "test@example.com",
            "password": "SecurePass123!"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);
}
```

#### 10.2.3 E2Eテスト

**対象**:
- 主要なユーザーフロー（登録→ログイン→リソース操作）

**方針**:
- 実環境に近い構成でテスト
- CI環境で自動実行
- 最小限のシナリオに絞る

### 10.3 フロントエンドテスト

#### 10.3.1 単体テスト

**対象**:
- カスタムフック（100%カバレッジ必須）
- ユーティリティ関数（100%カバレッジ必須）
- ストア（状態管理）

**ツール**: Vitest

#### 10.3.2 コンポーネントテスト

**対象**:
- UIコンポーネント
- フォーム

**ツール**: Vitest + Testing Library

**方針**:
- ユーザー視点でのテスト（クリック、入力等）
- スナップショットテストは最小限

#### 10.3.3 E2Eテスト

**対象**:
- ユーザー登録フロー
- ログインフロー
- 主要機能の操作

**ツール**: Playwright

**方針**:
- クリティカルパスのみ
- CI環境で自動実行

### 10.4 カバレッジ目標

| 対象 | カバレッジ目標 |
|------|--------------|
| Domain層 | 100% |
| Service層 | 100% |
| Handler層 | 80-90% |
| Infrastructure層 | 対象外 |
| フロントエンド hooks/utils | 100% |
| フロントエンド components | 80% |

---

## 11. 開発ワークフロー

### 11.1 基本フロー

```
1. Issue作成（機能追加、バグ修正等）
   ↓
2. ブランチ作成（feature/xxx, fix/xxx）
   ↓
3. 設計書更新（必要に応じて）
   ↓
4. テスト作成（TDD）
   ↓
5. 実装
   ↓
6. テスト実行（ローカル）
   ↓
7. コミット、プッシュ
   ↓
8. Pull Request作成
   ↓
9. CI実行（自動テスト、リンター）
   ↓
10. レビュー
   ↓
11. マージ
```

### 11.2 ブランチ戦略

**Git Flow ベース**:

- `main`: 本番環境
- `develop`: 開発環境
- `feature/*`: 新機能開発
- `fix/*`: バグ修正
- `hotfix/*`: 緊急修正

### 11.3 コミットメッセージ規約

**Conventional Commits を採用**:

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Type**:
- `feat`: 新機能
- `fix`: バグ修正
- `docs`: ドキュメント変更
- `style`: コードスタイル変更（動作に影響なし）
- `refactor`: リファクタリング
- `test`: テスト追加・修正
- `chore`: ビルド、設定変更

**例**:
```
feat(auth): add JWT authentication

- Implement JWT token generation
- Add login/logout endpoints
- Add authentication middleware

Closes #123
```

### 11.4 CI/CD パイプライン

#### 11.4.1 CI（Continuous Integration）

**トリガー**: Pull Request作成、プッシュ

**ステップ**:
1. コードチェックアウト
2. 依存関係インストール
3. リンター実行（clippy, ESLint）
4. フォーマッターチェック（rustfmt, Prettier）
5. 単体テスト実行
6. 統合テスト実行
7. カバレッジ計測
8. ビルド確認

#### 11.4.2 CD（Continuous Deployment）

**トリガー**: `main` ブランチへのマージ

**ステップ**:
1. ビルド（Docker image）
2. イメージレジストリへプッシュ
3. ステージング環境へデプロイ
4. E2Eテスト実行
5. 本番環境へデプロイ（手動承認後）

---

## 12. セキュリティ設計

### 12.1 セキュリティ対策

| 脅威 | 対策 |
|------|------|
| SQL Injection | ORM使用、パラメータ化クエリ |
| XSS | 自動エスケープ、CSP設定 |
| CSRF | SameSite Cookie、トークン検証 |
| 認証突破 | JWT検証、有効期限管理 |
| 権限昇格 | ロールベース制御、厳密な権限チェック |
| 機密情報漏洩 | 環境変数管理、ログマスキング |
| DDoS | レート制限、WAF |

### 12.2 環境変数管理

**機密情報はすべて環境変数で管理**:

```env
# Database
DATABASE_URL=postgresql://user:pass@localhost/dbname

# JWT
JWT_SECRET=your-secret-key
JWT_ACCESS_TOKEN_EXPIRY=900  # 15分
JWT_REFRESH_TOKEN_EXPIRY=604800  # 7日

# Server
SERVER_HOST=0.0.0.0
SERVER_PORT=8080

# CORS
CORS_ALLOWED_ORIGINS=http://localhost:3000
```

### 12.3 HTTPS/TLS

- **本番環境**: HTTPS必須
- **証明書**: Let's Encryptまたはマネージド証明書
- **開発環境**: HTTPも許可（ローカルのみ）

### 12.4 依存関係管理

- **定期的な更新**: 週次で依存関係をチェック
- **脆弱性スキャン**: `cargo audit`, `npm audit` を使用
- **CI組み込み**: 脆弱性検出時にビルド失敗

---

## 13. パフォーマンス設計

### 13.1 バックエンド最適化

- **非同期処理**: Tokioベースの非同期処理を全面採用
- **コネクションプール**: DB接続プールを適切に設定
- **キャッシュ**: 頻繁にアクセスされるデータはRedis等でキャッシュ（Phase 2）
- **クエリ最適化**: N+1問題の回避、適切なインデックス設定
- **ページネーション**: 大量データ取得時は必ずページネーション

### 13.2 フロントエンド最適化

- **コード分割**: Next.jsの動的インポートを活用
- **画像最適化**: Next.js Image コンポーネント使用
- **SSR/SSG**: 適切にSSR、SSG、CSRを使い分け
- **バンドルサイズ**: 不要なライブラリを含めない、Tree Shaking活用

### 13.3 監視・計測

- **メトリクス**: レスポンスタイム、スループット、エラー率
- **ログ**: 構造化ログによる分析
- **APM**: Application Performance Monitoring導入（Phase 2）

---

## 14. スケーラビリティ設計

### 14.1 水平スケーリング

- **ステートレス**: バックエンドはステートレスに設計
- **セッション管理**: JWTによりサーバー側セッション不要
- **ロードバランサー**: 複数インスタンス間で負荷分散

### 14.2 データベーススケーリング

- **読み取りレプリカ**: 読み取り負荷分散（Phase 3）
- **シャーディング**: 必要に応じて検討（Phase 3）
- **インデックス最適化**: 適切なインデックス設計

---

## 15. 運用・保守設計

### 15.1 ログ収集

- **集約**: すべてのログを中央集約（Phase 2）
- **検索**: Elasticsearch等での検索環境（Phase 2）
- **可視化**: Kibana等でのダッシュボード（Phase 2）

### 15.2 監視・アラート

- **ヘルスチェック**: `/health` エンドポイントで監視
- **アラート**: エラー率、レスポンスタイム異常時にアラート
- **通知**: Slack等への通知

### 15.3 バックアップ

- **DB**: 日次自動バックアップ
- **保持期間**: 30日間
- **リストアテスト**: 月次でリストアテストを実施

---

## 16. Phase別実装計画

### Phase 1: MVP（最小実装）

**期間**: 初期リリースまで

**スコープ**:
- ✅ プロジェクト構成
- ✅ Docker環境構築
- ✅ バックエンド基盤（Actix Web, SeaORM）
- ✅ フロントエンド基盤（Next.js）
- ✅ 認証機能（登録、ログイン、JWT）
- ✅ ユーザー管理（CRUD）
- ✅ ヘルスチェック
- ✅ 基本的なテスト

**成果物**:
- 動作するWebアプリケーション
- ユーザー登録・ログイン可能
- テストカバレッジ80%以上

### Phase 2: 機能拡張

**スコープ**:
- ロールベース権限管理
- 検索機能
- ページネーション
- キャッシュ機能（Redis）
- ログ集約
- 監視ダッシュボード

### Phase 3: 発展機能

**スコープ**:
- リアルタイム機能（WebSocket）
- データ分析機能
- 外部サービス連携
- DB読み取りレプリカ
- APM導入

---

## 17. 開発ルール（Claude Code前提）

### 17.1 設計ファースト

- **実装前に設計**: すべての実装前に設計書を更新
- **設計書の場所**: 機能別に `DESIGN.md` を配置
- **設計レビュー**: 実装前に設計内容を確認

### 17.2 テスト駆動

- **テストファースト**: 実装前にテストを書く
- **カバレッジ確認**: 実装後に必ずカバレッジを確認
- **失敗は設計見直し**: テスト失敗時は設計を見直す

### 17.3 AI駆動開発ルール

- **コンテキスト提供**: 設計書を常に参照できる状態に
- **段階的実装**: 1機能ずつ実装、テスト
- **自動化**: CI/CDで品質を担保

---

## 18. 用語集

| 用語 | 説明 |
|------|------|
| Feature | 機能単位のモジュール（例: auth, user） |
| Handler | HTTPリクエストを受け取る層 |
| Service | ビジネスロジックを実装する層 |
| Domain | ドメインモデル、バリデーションロジック |
| Repository | データアクセスインターフェース |
| Infrastructure | Repositoryの具体実装、外部サービス連携 |
| JWT | JSON Web Token（認証トークン） |
| RBAC | Role-Based Access Control（ロールベースアクセス制御） |
| ORM | Object-Relational Mapping |

---

## 19. 参考資料

### 19.1 公式ドキュメント

- Rust: https://doc.rust-lang.org/
- Actix Web: https://actix.rs/
- SeaORM: https://www.sea-ql.org/SeaORM/
- Next.js: https://nextjs.org/docs
- PostgreSQL: https://www.postgresql.org/docs/

### 19.2 設計参考

- Clean Architecture
- Domain-Driven Design (DDD)
- The Twelve-Factor App

---

## 20. 変更履歴

| 日付 | バージョン | 変更内容 | 担当 |
|------|-----------|---------|------|
| 2025-12-29 | 0.1.0 | 初版作成 | Claude |

---

## 付録A: チェックリスト

### 新機能実装時のチェックリスト

- [ ] 設計書（DESIGN.md）を作成・更新
- [ ] ドメインモデルを定義
- [ ] バリデーションルールを定義
- [ ] Repository Traitを定義
- [ ] Serviceロジックを実装
- [ ] Handlerを実装
- [ ] Infrastructure実装
- [ ] 単体テストを作成（カバレッジ100%目標）
- [ ] 統合テストを作成
- [ ] API仕様書を更新（docs/API.md）
- [ ] マイグレーションファイル作成（必要に応じて）
- [ ] フロントエンド実装（必要に応じて）
- [ ] E2Eテスト作成（主要フローのみ）
- [ ] ドキュメント更新
- [ ] Pull Request作成
- [ ] レビュー対応
- [ ] マージ

---

**以上、設計書 v0.1.0**
