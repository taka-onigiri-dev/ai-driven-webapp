# SQLマイグレーションファイル

このディレクトリには、データベースのDDLとデータ投入用のSQLファイルが含まれています。

## ディレクトリ構成

```
sql/
├── ddl/                  # DDL（Data Definition Language）
│   ├── 001_create_users_table.sql
│   └── 002_create_refresh_tokens_table.sql
├── master_data/          # マスターデータ
│   └── 001_insert_initial_users.sql
└── transaction_data/     # トランザクションデータ
    └── 001_insert_test_users.sql
```

## ファイルの分類

### DDL（ddl/）
データベースのスキーマ定義（テーブル、インデックス、制約など）を含むSQLファイル。

- **命名規則**: `{番号}_{テーブル名}_table.sql`
- **実行タイミング**: データベース初期化時、スキーマ変更時
- **実行順序**: ファイル名の番号順に実行される
- **用途**:
  - CREATE TABLE文
  - CREATE INDEX文
  - ALTER TABLE文（制約追加など）
  - テーブルやカラムへのコメント追加

### マスターデータ（master_data/）
システムの運用に必要な基本データ。環境を問わず必要なデータ。

- **命名規則**: `{番号}_insert_{データ種別}.sql`
- **実行タイミング**: データベース初期化時
- **特徴**:
  - 本番環境でも必要なデータ
  - システムユーザー（admin、moderatorなど）
  - 権限マスタ、設定マスタなど
  - 変更頻度が低い
- **含まれるデータ**:
  - システム管理者アカウント
  - デフォルトの役割（ロール）
  - 初期設定値

### トランザクションデータ（transaction_data/）
開発・テスト用のサンプルデータ。本番環境では使用しない。

- **命名規則**: `{番号}_insert_test_{データ種別}.sql`
- **実行タイミング**: 開発環境のセットアップ時
- **特徴**:
  - 開発・テスト環境専用
  - 本番環境では実行しない
  - テストユーザー、サンプルデータ
  - 動作確認用のデータ
- **含まれるデータ**:
  - テストユーザーアカウント
  - サンプルの投稿データ
  - テスト用のトランザクション

## 実行方法

### Windows

```batch
# 全て実行
scripts\windows\migrate-all.bat

# 個別実行
scripts\windows\migrate-ddl.bat          # DDLのみ
scripts\windows\migrate-master.bat       # マスターデータのみ
scripts\windows\migrate-transaction.bat  # トランザクションデータのみ
```

### 手動実行（Dockerコンテナ内）

```bash
# DDL
docker exec -i ai-webapp-postgres psql -U app_user -d ai_webapp < backend/sql/ddl/001_create_users_table.sql

# マスターデータ
docker exec -i ai-webapp-postgres psql -U app_user -d ai_webapp < backend/sql/master_data/001_insert_initial_users.sql

# トランザクションデータ
docker exec -i ai-webapp-postgres psql -U app_user -d ai_webapp < backend/sql/transaction_data/001_insert_test_users.sql
```

## ファイル追加時の注意事項

1. **番号の付与**: ファイル名の先頭に3桁の連番を付与（例: 001, 002, 003）
2. **依存関係**: 外部キー制約がある場合、参照先のテーブルを先に作成
3. **冪等性**: 複数回実行しても安全なように、`IF NOT EXISTS`や`ON CONFLICT`を使用
4. **コメント**: 各SQLファイルの先頭にファイルの目的を記載
5. **文字コード**: UTF-8で保存

## SQLファイルのテンプレート

### DDL

```sql
-- テーブル名テーブル
CREATE TABLE IF NOT EXISTS table_name (
    id BIGSERIAL PRIMARY KEY,
    -- カラム定義
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- インデックス
CREATE INDEX IF NOT EXISTS idx_table_name_column ON table_name(column);

-- コメント
COMMENT ON TABLE table_name IS 'テーブルの説明';
COMMENT ON COLUMN table_name.id IS 'プライマリキー';
```

### データ投入

```sql
-- データの説明
INSERT INTO table_name (column1, column2)
VALUES ('value1', 'value2')
ON CONFLICT (unique_column) DO NOTHING;
```

## トラブルシューティング

### エラー: relation "table_name" does not exist
- DDLが実行されていない可能性があります
- `migrate-ddl.bat` を先に実行してください

### エラー: duplicate key value violates unique constraint
- 既にデータが存在しています
- SQLファイルに `ON CONFLICT DO NOTHING` が含まれているか確認してください

### 実行順序を変更したい
- ファイル名の番号を変更してください
- 外部キー制約に注意して順序を決定してください
