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
