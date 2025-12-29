# Windows用 Docker操作スクリプト

このディレクトリには、Docker環境を簡単に操作するためのWindowsバッチファイルが含まれています。

## 使用方法

### 基本操作

- **start.bat** - Docker環境を起動
  ```
  scripts\windows\start.bat
  ```

- **stop.bat** - Docker環境を停止
  ```
  scripts\windows\stop.bat
  ```

- **restart.bat** - Docker環境を再起動
  ```
  scripts\windows\restart.bat
  ```

### モニタリング

- **status.bat** - コンテナの状態確認
  ```
  scripts\windows\status.bat
  ```

- **logs.bat** - ログ表示
  ```
  # 全サービスのログ
  scripts\windows\logs.bat

  # 特定サービスのログ
  scripts\windows\logs.bat backend
  scripts\windows\logs.bat frontend
  scripts\windows\logs.bat postgres
  ```

### データベース

#### マイグレーションスクリプト

データベースのマイグレーションは、SQLファイルベースで管理されています。

- **migrate-all.bat** - 全てのマイグレーションを実行（DDL + マスターデータ + トランザクションデータ）
  ```
  scripts\windows\migrate-all.bat
  ```
  ※初回起動時はこちらを実行してください

- **migrate-ddl.bat** - DDL（データベーススキーマ）のみ実行
  ```
  scripts\windows\migrate-ddl.bat
  ```

- **migrate-master.bat** - マスターデータのみ投入
  ```
  scripts\windows\migrate-master.bat
  ```
  システムユーザー（admin、moderator、user）が作成されます

- **migrate-transaction.bat** - トランザクションデータのみ投入
  ```
  scripts\windows\migrate-transaction.bat
  ```
  テストユーザー（test@example.com など）が作成されます

- **migrate.bat** - レガシースクリプト（migrate-all.batにリダイレクトされます）
  ```
  scripts\windows\migrate.bat
  ```

#### SQLファイルの場所

- DDL: `backend/sql/ddl/*.sql`
- マスターデータ: `backend/sql/master_data/*.sql`
- トランザクションデータ: `backend/sql/transaction_data/*.sql`

#### デフォルトアカウント

マイグレーション実行後、以下のアカウントが利用可能です：

**システムユーザー（マスターデータ）:**
- admin@example.com （管理者）
- moderator@example.com （モデレーター）
- user@example.com （一般ユーザー）

**テストユーザー（トランザクションデータ）:**
- test@example.com
- test2@example.com
- test3@example.com
- inactive@example.com （非アクティブ）

全てのアカウントのパスワード: `Password123`

### メンテナンス

- **rebuild.bat** - Dockerイメージを再ビルドして起動
  ```
  scripts\windows\rebuild.bat
  ```

- **clean.bat** - コンテナとボリュームを削除（データも削除されます）
  ```
  scripts\windows\clean.bat
  ```

## サービスURL

- フロントエンド: http://localhost:3000
- バックエンドAPI: http://localhost:8080
- PostgreSQL: localhost:5432

## 注意事項

- `clean.bat` を実行すると、データベースのデータも削除されます
- 初回起動時は、イメージのダウンロードとビルドに時間がかかります
- エラーが発生した場合は、`logs.bat` でログを確認してください
