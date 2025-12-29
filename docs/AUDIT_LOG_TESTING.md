# 操作ログシステム 動作確認手順

## 🚀 セットアップ

### 1. Docker環境起動

```batch
scripts\windows\start.bat
```

### 2. DDLマイグレーション実行

```batch
scripts\windows\migrate-ddl.bat
```

これで`audit_logs`テーブルが作成されます。

## 🧪 テスト手順

### ステップ1: ユーザー登録テスト

#### 1-1. ユーザー登録

ブラウザで http://localhost:3000/register にアクセス

- Email: `test@example.com`
- Password: `Password123`
- Name: `Test User`

登録ボタンをクリック

#### 1-2. ログ確認

```bash
docker exec -it ai-webapp-postgres psql -U app_user -d ai_webapp -c "SELECT id, user_email, action, http_method, endpoint, status_code, success, response_time_ms, ip_address, created_at FROM audit_logs ORDER BY created_at DESC LIMIT 5;"
```

**期待される結果:**
```
 id | user_email        | action   | http_method | endpoint                | status_code | success | response_time_ms | ip_address  | created_at
----+-------------------+----------+-------------+-------------------------+-------------+---------+------------------+-------------+------------------------
  1 | test@example.com  | REGISTER | POST        | /api/v1/auth/register   | 201         | t       | 150              | 172.31.0.1  | 2025-12-29 10:00:00+00
```

### ステップ2: ログインテスト

#### 2-1. ログアウトしてログイン

ダッシュボードからログアウトし、再度ログイン

- Email: `test@example.com`
- Password: `Password123`

#### 2-2. ログ確認

```bash
docker exec -it ai-webapp-postgres psql -U app_user -d ai_webapp -c "SELECT id, user_email, action, http_method, endpoint, status_code, success, created_at FROM audit_logs WHERE action = 'LOGIN' ORDER BY created_at DESC LIMIT 3;"
```

**期待される結果:**
```
 id | user_email        | action | http_method | endpoint            | status_code | success | created_at
----+-------------------+--------+-------------+---------------------+-------------+---------+------------------------
  3 | test@example.com  | LOGIN  | POST        | /api/v1/auth/login  | 200         | t       | 2025-12-29 10:05:00+00
```

### ステップ3: ログイン失敗テスト

#### 3-1. 間違ったパスワードでログイン

- Email: `test@example.com`
- Password: `WrongPassword`

#### 3-2. エラーログ確認

```bash
docker exec -it ai-webapp-postgres psql -U app_user -d ai_webapp -c "SELECT id, user_email, action, status_code, success, error_message, created_at FROM audit_logs WHERE success = false ORDER BY created_at DESC LIMIT 3;"
```

**期待される結果:**
```
 id | user_email        | action | status_code | success | error_message                | created_at
----+-------------------+--------+-------------+---------+------------------------------+------------------------
  4 | NULL              | LOGIN  | 401         | f       | Invalid email or password    | 2025-12-29 10:10:00+00
```

### ステップ4: 詳細ログ確認

#### 4-1. 全フィールド確認

```bash
docker exec -it ai-webapp-postgres psql -U app_user -d ai_webapp -c "SELECT * FROM audit_logs ORDER BY created_at DESC LIMIT 1;"
```

#### 4-2. 統計情報確認

```bash
# アクションごとの集計
docker exec -it ai-webapp-postgres psql -U app_user -d ai_webapp -c "SELECT action, COUNT(*) as count, AVG(response_time_ms) as avg_time FROM audit_logs GROUP BY action;"

# 成功/失敗の集計
docker exec -it ai-webapp-postgres psql -U app_user -d ai_webapp -c "SELECT success, COUNT(*) as count FROM audit_logs GROUP BY success;"

# ユーザーごとの集計
docker exec -it ai-webapp-postgres psql -U app_user -d ai_webapp -c "SELECT user_email, COUNT(*) as request_count FROM audit_logs WHERE user_email IS NOT NULL GROUP BY user_email ORDER BY request_count DESC;"
```

## 🔍 確認ポイント

### ✅ チェックリスト

- [ ] ユーザー登録時にログが作成される
- [ ] `user_email`、`user_role`が正しく記録される
- [ ] `action`が`REGISTER`になっている
- [ ] `status_code`が201（Created）
- [ ] `success`がtrue
- [ ] `response_time_ms`が記録されている
- [ ] `ip_address`が記録されている
- [ ] ログイン成功時に`action`が`LOGIN`
- [ ] ログイン失敗時に`success`がfalse
- [ ] エラーメッセージが記録される
- [ ] ヘルスチェック（`/health`）はログに記録されない

## 🐛 トラブルシューティング

### ログが記録されない場合

1. **テーブルが作成されているか確認**
   ```bash
   docker exec -it ai-webapp-postgres psql -U app_user -d ai_webapp -c "\dt"
   ```
   `audit_logs`テーブルが表示されるはずです。

2. **バックエンドログを確認**
   ```bash
   docker logs ai-webapp-backend
   ```
   エラーメッセージがないか確認

3. **データベース接続確認**
   ```bash
   docker exec -it ai-webapp-postgres psql -U app_user -d ai_webapp -c "SELECT COUNT(*) FROM users;"
   ```

### パフォーマンス確認

```bash
# 平均レスポンス時間
docker exec -it ai-webapp-postgres psql -U app_user -d ai_webapp -c "SELECT endpoint, AVG(response_time_ms) as avg_ms, MAX(response_time_ms) as max_ms FROM audit_logs GROUP BY endpoint ORDER BY avg_ms DESC;"

# 遅いリクエスト
docker exec -it ai-webapp-postgres psql -U app_user -d ai_webapp -c "SELECT endpoint, response_time_ms, created_at FROM audit_logs WHERE response_time_ms > 500 ORDER BY response_time_ms DESC;"
```

## 📊 期待される動作

1. **自動記録**: 全APIリクエストが自動的に記録される
2. **非同期処理**: ログ記録がリクエストをブロックしない
3. **エラーハンドリング**: ログ記録失敗してもリクエストは成功する
4. **機密情報フィルタ**: パスワード等は記録されない
5. **パフォーマンス**: レスポンスタイムへの影響は最小限（数ms以下）
