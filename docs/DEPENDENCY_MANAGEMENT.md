# 依存関係のバージョン管理

## 📦 方針

### メジャーバージョンのみ指定
依存関係は**メジャーバージョン**のみ指定し、マイナー・パッチバージョンは自動で最新を取得します。

### ロックファイルで固定
実際にインストールされる正確なバージョンは**ロックファイル**で固定されます。

## 🦀 Rust (Cargo.toml)

### バージョン指定方法

```toml
# メジャーバージョンのみ指定
actix-web = "4"         # 最新の 4.x.x を取得
serde = "1"             # 最新の 1.x.x を取得

# マイナーバージョンまで指定（必要な場合）
validator = "0.18"      # 最新の 0.18.x を取得
```

### ロックファイル: `Cargo.lock`

- `cargo build` または `cargo update` 実行時に自動生成
- 正確なバージョンとハッシュを記録
- **Gitにコミットする**（アプリケーションの場合）
- チーム全員が同じバージョンを使用

### 更新方法

```bash
# 全ての依存関係を最新に更新
cargo update

# 特定のパッケージのみ更新
cargo update -p actix-web

# Cargo.lockを削除して再生成
rm Cargo.lock
cargo build
```

## 📦 Node.js (package.json)

### バージョン指定方法

```json
{
  "dependencies": {
    "next": "^15",           // 15.x.x の最新（メジャーバージョン固定）
    "react": "^19",          // 19.x.x の最新
    "axios": "^1"            // 1.x.x の最新
  }
}
```

**`^` (キャレット)の意味:**
- `^15.0.0` → `15.x.x` の最新（16.0.0未満）
- マイナー・パッチバージョンは自動更新

### ロックファイル: `package-lock.json`

- `npm install` 実行時に自動生成
- 正確なバージョンと依存関係ツリーを記録
- **Gitにコミットする**
- チーム全員が同じバージョンを使用

### 更新方法

```bash
# package.jsonに従って最新に更新
npm update

# 全ての依存関係を最新に更新
npm install

# package-lock.jsonを削除して再生成
rm package-lock.json
npm install

# 特定のパッケージのみ更新
npm update next
```

## 🔄 初回セットアップ時の動作

### Backend (Rust)

```bash
cd backend
cargo build
```

1. `Cargo.toml` の指定に従って最新版を解決
2. `Cargo.lock` を生成
3. ビルド実行

### Frontend (Node.js)

```bash
cd frontend
npm install
```

1. `package.json` の指定に従って最新版を解決
2. `package-lock.json` を生成
3. `node_modules/` にインストール

## ✅ メリット

### セキュリティアップデート
- パッチバージョン（脆弱性修正）が自動で適用される
- `^1.2.3` → `1.2.4` (セキュリティパッチ) は自動適用

### 最新機能
- マイナーバージョンアップ（新機能）も取得可能
- `^1.2.0` → `1.3.0` (新機能追加) も適用

### チーム統一
- ロックファイルで全員が同じバージョンを使用
- "私の環境では動く"問題を防止

### メンテナンス容易
- 定期的に `cargo update` / `npm update` で最新化
- メジャーバージョンアップは手動で制御

## ⚠️ 注意事項

### メジャーバージョンアップ

破壊的変更が含まれる可能性があるため、手動で対応：

```toml
# Before
actix-web = "4"

# After (手動で変更)
actix-web = "5"
```

その後、動作確認とテストを実施。

### ロックファイルは必ずコミット

```bash
git add Cargo.lock package-lock.json
git commit -m "chore: update dependencies"
```

### CI/CDでの利用

```yaml
# Rust
- run: cargo build --locked  # Cargo.lockを尊重

# Node.js
- run: npm ci  # package-lock.jsonを尊重（npm install より厳密）
```

## 📊 バージョン管理の比較

| 項目 | 固定バージョン | メジャーのみ指定（今回） |
|------|--------------|------------------------|
| セキュリティパッチ | 手動更新 | 自動適用 |
| 新機能 | 手動更新 | 自動適用 |
| 安定性 | 非常に高い | 高い |
| メンテナンス | 手間がかかる | 容易 |
| 推奨用途 | 本番環境の長期運用 | 開発・短期運用 |

## 🚀 推奨ワークフロー

### 1. 開発開始時

```bash
# 最新の依存関係でスタート
git clone <repository>
cd backend && cargo build
cd ../frontend && npm install
```

### 2. 定期更新（月1回程度）

```bash
# バックエンド
cd backend
cargo update
cargo test  # テスト実行
git add Cargo.lock
git commit -m "chore: update rust dependencies"

# フロントエンド
cd frontend
npm update
npm test  # テスト実行（あれば）
git add package-lock.json
git commit -m "chore: update npm dependencies"
```

### 3. メジャーバージョンアップ時

1. リリースノート確認
2. `Cargo.toml` / `package.json` を手動更新
3. ビルド・テスト
4. 動作確認
5. コミット

この方針により、**セキュリティと最新性のバランス**を保ちつつ、**メンテナンスコストを削減**できます。
