# マスターデータ

このディレクトリには、システムの運用に必要な基本データを含むSQLファイルが格納されています。

## 注意事項

- ユーザーが画面から登録したアカウントのパスワードハッシュを使用してください
- 新しいユーザーを登録後、データベースから `password_hash` を取得して追加してください

## パスワードハッシュの取得方法

```sql
-- 登録したユーザーのハッシュを確認
SELECT email, password_hash FROM users WHERE email = 'your-email@example.com';
```

## ファイルの追加方法

1. 画面から新しいユーザーを登録
2. 上記のSQLでパスワードハッシュを取得
3. `00X_insert_xxx.sql` という形式でファイルを作成
4. `ON CONFLICT (email) DO NOTHING` を使用して冪等性を確保
