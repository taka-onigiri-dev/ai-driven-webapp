# AI-Driven Web Application

モノレポ構成の Web アプリケーション（Rust + Next.js + PostgreSQL）

## 概要

- **バックエンド**: Rust + Actix Web + SeaORM
- **フロントエンド**: Next.js 14 + TypeScript + Tailwind CSS
- **データベース**: PostgreSQL 15
- **認証**: JWT (Access Token + Refresh Token)
- **開発環境**: Docker + Docker Compose

## 🚀 クイックスタート

### 1. 環境起動

```bash
# Dockerコンテナを起動
scripts\windows\start.bat

# データベーススキーマを作成
scripts\windows\migrate-ddl.bat
```

### 2. ユーザー登録

ブラウザで http://localhost:3000/register にアクセスして、最初のユーザーを登録します。

### 3. テストデータ追加（オプション）

登録したユーザーのパスワードハッシュを取得して、マスターデータ・テストデータに追加できます。

```bash
# パスワードハッシュを確認
docker exec -it ai-webapp-postgres psql -U app_user -d ai_webapp -c "SELECT email, password_hash FROM users WHERE email = 'your-email@example.com';"

# SQLファイルに追加して実行
scripts\windows\migrate-master.bat        # マスターデータ
scripts\windows\migrate-transaction.bat   # テストデータ
```

## 📁 プロジェクト構成

```
.
├── backend/                  # Rustバックエンド
│   ├── src/
│   │   ├── main.rs
│   │   ├── config.rs
│   │   ├── entities/        # SeaORM エンティティ
│   │   ├── features/        # 機能別モジュール
│   │   │   ├── health/      # ヘルスチェック
│   │   │   └── auth/        # 認証機能
│   │   │       ├── domain.rs      # ドメインモデル
│   │   │       ├── repository.rs  # リポジトリインターフェース
│   │   │       ├── infra.rs       # インフラ実装
│   │   │       ├── service.rs     # ビジネスロジック
│   │   │       └── handler.rs     # HTTPハンドラ
│   │   └── shared/          # 共有モジュール
│   ├── sql/                 # SQLマイグレーション
│   │   ├── ddl/             # DDL（テーブル定義）
│   │   ├── master_data/     # マスターデータ
│   │   └── transaction_data/ # テストデータ
│   └── Cargo.toml
├── frontend/                # Next.js フロントエンド
│   ├── src/
│   │   ├── app/             # App Router
│   │   │   ├── page.tsx     # トップページ
│   │   │   ├── login/       # ログインページ
│   │   │   ├── register/    # 登録ページ
│   │   │   └── dashboard/   # ダッシュボード
│   │   └── lib/
│   │       ├── api/         # API クライアント
│   │       └── store/       # Zustand ストア
│   └── package.json
├── docker/                  # Docker設定
│   ├── backend/Dockerfile
│   └── frontend/Dockerfile
├── scripts/                 # 運用スクリプト
│   └── windows/             # Windows用バッチファイル
└── docs/                    # ドキュメント
```

## 🛠️ 開発ツール

### Windows用バッチファイル

| スクリプト | 説明 |
|-----------|------|
| `start.bat` | Docker環境を起動 |
| `stop.bat` | Docker環境を停止 |
| `restart.bat` | Docker環境を再起動 |
| `status.bat` | コンテナの状態確認 |
| `logs.bat` | ログ表示 |
| `rebuild.bat` | イメージを再ビルドして起動 |
| `clean.bat` | コンテナとボリュームを削除 |
| `migrate-ddl.bat` | DDL実行 |
| `migrate-master.bat` | マスターデータ投入 |
| `migrate-transaction.bat` | テストデータ投入 |
| `migrate-all.bat` | 全マイグレーション実行 |

詳細は [scripts/windows/README.md](scripts/windows/README.md) を参照。

## 🗄️ データベースマイグレーション

SQLファイルベースのマイグレーションシステムを採用しています。

### マイグレーション方針

1. **DDL（スキーマ定義）**: `backend/sql/ddl/`
   - テーブル、インデックス、制約の定義

2. **マスターデータ**: `backend/sql/master_data/`
   - 本番環境でも必要なデータ
   - システムユーザー、設定値など

3. **トランザクションデータ**: `backend/sql/transaction_data/`
   - 開発・テスト環境専用
   - テストユーザー、サンプルデータなど

### テストデータの追加方法

**重要**: パスワードハッシュは画面から登録したアカウントのハッシュを使用してください。

1. フロントエンドから新規ユーザーを登録
2. データベースからパスワードハッシュを取得
   ```bash
   docker exec -it ai-webapp-postgres psql -U app_user -d ai_webapp -c "SELECT email, password_hash FROM users;"
   ```
3. 取得したハッシュを使ってSQLファイルを作成
4. マイグレーションを実行

詳細は [backend/sql/README.md](backend/sql/README.md) を参照。

## 🔐 認証システム

- JWT（JSON Web Token）ベースの認証
- Access Token（有効期限: 15分）
- Refresh Token（有効期限: 7日）
- bcryptによるパスワードハッシュ化
- パスワード要件: 8文字以上、大文字・小文字・数字を含む

## 📡 API エンドポイント

### 認証

- `POST /api/v1/auth/register` - ユーザー登録
- `POST /api/v1/auth/login` - ログイン
- `POST /api/v1/auth/logout` - ログアウト
- `POST /api/v1/auth/refresh` - トークンリフレッシュ

### ヘルスチェック

- `GET /health` - サービス稼働確認

## 🌐 アクセスURL

- **フロントエンド**: http://localhost:3000
- **バックエンドAPI**: http://localhost:8080
- **データベース**: localhost:5432

## 📋 環境変数

バックエンドの環境変数は `docker-compose.yml` で設定されています：

- `DATABASE_URL`: PostgreSQL接続文字列
- `JWT_SECRET`: JWT署名用シークレット
- `JWT_ACCESS_TOKEN_EXPIRY`: アクセストークンの有効期限（秒）
- `JWT_REFRESH_TOKEN_EXPIRY`: リフレッシュトークンの有効期限（秒）
- `RUST_LOG`: ログレベル

## 🏗️ アーキテクチャ

### バックエンド

レイヤードアーキテクチャを採用：

1. **Handler層**: HTTP リクエストの受付
2. **Service層**: ビジネスロジック
3. **Repository層**: データアクセスの抽象化
4. **Infrastructure層**: 具体的なDB操作

### フロントエンド

- Next.js 14 App Router
- TypeScript による型安全性
- Zustand による状態管理
- React Hook Form によるフォーム管理
- Tailwind CSS によるスタイリング

## 🧪 開発フロー

1. 設計書を作成・レビュー（`docs/DESIGN.md`）
2. バックエンドAPI実装
3. フロントエンド実装
4. 動作確認
5. テストデータ追加
6. ドキュメント更新

## 📝 ライセンス

This project is licensed under the MIT License.
