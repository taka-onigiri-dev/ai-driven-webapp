# SQLマイグレーションファイル

このディレクトリには、データベースのDDLとデータ投入用のSQLファイルが含まれています。

## ディレクトリ構成

```
sql/
├── ddl/                  # DDL（Data Definition Language）
│   ├── 001_create_users_table.sql
│   └── 002_create_refresh_tokens_table.sql
├── master_data/          # マスターデータ（本番環境でも必要）
│   └── README.md
└── transaction_data/     # トランザクションデータ（開発・テスト環境専用）
    └── README.md
```

## テストデータの作成方法

テストデータは、**画面から実際に登録したアカウント**のパスワードハッシュを使用します。

### 手順

1. **アカウントを登録**
   - フロントエンド（http://localhost:3000/register）からアカウントを作成

2. **パスワードハッシュを取得**
   ```bash
   docker exec -it ai-webapp-postgres psql -U app_user -d ai_webapp -c "SELECT email, password_hash FROM users WHERE email = 'your-email@example.com';"
   ```

3. **SQLファイルを作成**
   - マスターデータ: `backend/sql/master_data/001_insert_admin.sql`
   - テストデータ: `backend/sql/transaction_data/001_insert_test_users.sql`

4. **SQLファイルの例**
   ```sql
   -- 画面から登録したユーザーのハッシュを使用
   INSERT INTO users (email, password_hash, name, role, is_active, created_at, updated_at)
   VALUES (
       'admin@example.com',
       '取得したパスワードハッシュをここに貼り付け',
       'Administrator',
       'admin',
       true,
       CURRENT_TIMESTAMP,
       CURRENT_TIMESTAMP
   )
   ON CONFLICT (email) DO NOTHING;
   ```

## マイグレーションの実行

### Windowsの場合

```batch
# DDLのみ
scripts\windows\migrate-ddl.bat

# マスターデータのみ
scripts\windows\migrate-master.bat

# トランザクションデータのみ
scripts\windows\migrate-transaction.bat

# 全て実行
scripts\windows\migrate-all.bat
```

## 重要な注意事項

- パスワードハッシュは**絶対に手動で作成しないでください**
- 必ず画面から登録したアカウントのハッシュを使用してください
- bcryptハッシュは毎回異なる値が生成されるため、実際に登録したハッシュのみが有効です
