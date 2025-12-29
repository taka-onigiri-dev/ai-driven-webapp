-- Audit logs table
CREATE TABLE IF NOT EXISTS audit_logs (
    id BIGSERIAL PRIMARY KEY,

    -- ユーザー情報
    user_id BIGINT,  -- NULL可（未認証の場合）
    user_email VARCHAR(255),
    user_role VARCHAR(50),

    -- アクション情報
    action VARCHAR(50) NOT NULL,  -- LOGIN, LOGOUT, CREATE, UPDATE, DELETE, READ
    resource_type VARCHAR(50),    -- User, Post など
    resource_id VARCHAR(100),

    -- リクエスト情報
    http_method VARCHAR(10) NOT NULL,
    endpoint VARCHAR(500) NOT NULL,
    request_params TEXT,  -- JSON形式（機密情報除く）

    -- レスポンス情報（シンプル）
    status_code INTEGER NOT NULL,
    success BOOLEAN NOT NULL,
    error_message TEXT,  -- エラー時のみ
    response_time_ms INTEGER,

    -- クライアント情報
    ip_address VARCHAR(45),
    user_agent TEXT,

    -- 位置情報（オプション）
    country VARCHAR(2),
    timezone VARCHAR(50),

    -- タイムスタンプ
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT fk_audit_logs_user_id
        FOREIGN KEY (user_id)
        REFERENCES users(id)
        ON DELETE SET NULL
);

-- インデックス
CREATE INDEX IF NOT EXISTS idx_audit_logs_user_id ON audit_logs(user_id);
CREATE INDEX IF NOT EXISTS idx_audit_logs_created_at ON audit_logs(created_at);
CREATE INDEX IF NOT EXISTS idx_audit_logs_action ON audit_logs(action);
CREATE INDEX IF NOT EXISTS idx_audit_logs_success ON audit_logs(success);
CREATE INDEX IF NOT EXISTS idx_audit_logs_ip_address ON audit_logs(ip_address);
CREATE INDEX IF NOT EXISTS idx_audit_logs_endpoint ON audit_logs(endpoint);

-- コメント
COMMENT ON TABLE audit_logs IS 'Audit logs for tracking user actions and API requests';
COMMENT ON COLUMN audit_logs.id IS 'Primary key';
COMMENT ON COLUMN audit_logs.user_id IS 'Reference to users table (NULL for unauthenticated requests)';
COMMENT ON COLUMN audit_logs.user_email IS 'Email of the user at the time of action';
COMMENT ON COLUMN audit_logs.user_role IS 'Role of the user at the time of action';
COMMENT ON COLUMN audit_logs.action IS 'Action type (LOGIN, LOGOUT, CREATE, UPDATE, DELETE, READ)';
COMMENT ON COLUMN audit_logs.resource_type IS 'Type of resource being accessed';
COMMENT ON COLUMN audit_logs.resource_id IS 'ID of the resource being accessed';
COMMENT ON COLUMN audit_logs.http_method IS 'HTTP method (GET, POST, PUT, DELETE, etc.)';
COMMENT ON COLUMN audit_logs.endpoint IS 'API endpoint path';
COMMENT ON COLUMN audit_logs.request_params IS 'Request parameters in JSON format (sensitive data filtered)';
COMMENT ON COLUMN audit_logs.status_code IS 'HTTP response status code';
COMMENT ON COLUMN audit_logs.success IS 'Whether the request was successful';
COMMENT ON COLUMN audit_logs.error_message IS 'Error message if the request failed';
COMMENT ON COLUMN audit_logs.response_time_ms IS 'Response time in milliseconds';
COMMENT ON COLUMN audit_logs.ip_address IS 'Client IP address';
COMMENT ON COLUMN audit_logs.user_agent IS 'Client user agent string';
COMMENT ON COLUMN audit_logs.country IS 'Country code (ISO 3166-1 alpha-2)';
COMMENT ON COLUMN audit_logs.timezone IS 'Client timezone';
COMMENT ON COLUMN audit_logs.created_at IS 'Timestamp when the log was created';
