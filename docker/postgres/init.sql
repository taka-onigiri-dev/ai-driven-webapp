-- PostgreSQL初期化スクリプト
-- データベース作成は環境変数で自動実行されるため、ここではテーブル作成のみ

-- 拡張機能の有効化（必要に応じて）
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- タイムゾーン設定
SET timezone = 'UTC';

-- ここには初期設定のみ記載
-- 実際のテーブル作成はマイグレーションファイルで管理
