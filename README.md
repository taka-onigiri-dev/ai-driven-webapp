# Web Application Project

本リポジトリは、設計先行・テスト重視・AI 駆動開発（Cloud Code）を前提とした  
Web アプリケーション開発のためのモノレポ構成です。

実装に入る前に、アーキテクチャ・テスト方針・ディレクトリ構成を明確に定義します。

---

## 1. 目的・基本方針

- Web アプリケーションを新規開発する
- 実装前に設計を固める
- 設計・テスト・コードを一貫した思想で管理する
- Cloud Code（AI）に実装を任せやすい構成を採用する
- ローカル開発・CI・本番で構成差分を最小化する

---

## 2. 技術スタック

### バックエンド

- 言語: Rust
- Web フレームワーク: Actix Web
- DB: PostgreSQL

### フロントエンド

- フレームワーク: Next.js

### 環境管理

- Docker
- Docker Compose

---

## 3. リポジトリ構成方針

- モノレポ構成
- フロントエンド / バックエンド / インフラ設定を 1 リポジトリで管理
- 機能（feature）単位でコード・テスト・設計をまとめる

想定ディレクトリ構成（概略）：

.
├─ docs/
├─ backend/
├─ frontend/
├─ docker/
└─ README.md

---

## 4. アーキテクチャ方針（バックエンド）

- レイヤードアーキテクチャを採用
- 機能（feature）× レイヤーのハイブリッド構成

構成イメージ：

handler (HTTP)
↓
service / usecase
↓
domain / validation
↓
repository (trait)
↓
infra (DB)

- HTTP / DB / 時刻などの副作用は境界に閉じ込める
- core ロジックは純粋関数または mock 可能な形で実装する

---

## 5. 機能（Feature）分割方針

- すべての機能は feature 単位で管理する
- feature は原則として他 feature に直接依存しない
- 共通機能は shared に集約する

最低限想定される基盤 feature：

- auth（認証・認可）
- user（ユーザー管理）
- validation（入力検証）
- error（エラーハンドリング）
- config（設定管理）
- logging（ログ）
- health（ヘルスチェック）

---

## 6. テスト方針

### 基本思想

- すべての動きにテストを書く
- テストは仕様書であり、設計の一部とする
- テスト不能なコードは設計ミスと考える

### バックエンド

- 単体テスト（domain / service）を最重視する
- ハンドラテストはレスポンス確認に限定する
- API + DB の結合テストを実施（主要ユースケースのみ）

### フロントエンド

- hooks / utils の単体テスト
- コンポーネントテスト
- E2E テストは最小限（主要動線のみ）

---

## 7. カバレッジ方針

- 数値としてのカバレッジ 100% を目標とする
- ただし「意味のある範囲」に限定する

カバレッジ対象ルール：

- domain / service / validation: 100%必須
- hooks / utils: 100%必須
- handler / UI コンポーネント: 80〜90%目安
- infra / 起動コード / FW 依存部分: 対象外

---

## 8. 設計書の管理方針

- 設計書はコードの近くに配置する
- 全体設計は docs/ 配下に配置する
- feature 単位の設計は feature 配下に DESIGN.md を置く

例（バックエンド）：

backend/src/features/auth/

- DESIGN.md
- handler.rs
- service.rs
- domain.rs
- tests/

---

## 9. Cloud Code 利用前提ルール

- 実装は必ず設計書（DESIGN.md）を前提に行う
- 設計未記載のコード追加は禁止する
- 変更は「設計 → テスト → 実装」の順で行う
- テスト失敗・カバレッジ不足は設計見直しのサインとする

---

## 10. この README の位置づけ

- 本リポジトリの設計思想・制約条件の一次情報
- Cloud Code に与える前提条件
- 開発方針がブレないための合意文書
