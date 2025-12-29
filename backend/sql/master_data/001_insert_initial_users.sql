-- Master data: Initial system users
-- Password for all users: Password123
-- Password hash generated with bcrypt cost 12

-- Admin user
INSERT INTO users (email, password_hash, name, role, is_active, created_at, updated_at)
VALUES (
    'admin@example.com',
    '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5eo5kXBq.JVKi',
    'System Administrator',
    'admin',
    true,
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP
)
ON CONFLICT (email) DO NOTHING;

-- Moderator user
INSERT INTO users (email, password_hash, name, role, is_active, created_at, updated_at)
VALUES (
    'moderator@example.com',
    '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5eo5kXBq.JVKi',
    'System Moderator',
    'moderator',
    true,
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP
)
ON CONFLICT (email) DO NOTHING;

-- Regular user (for reference)
INSERT INTO users (email, password_hash, name, role, is_active, created_at, updated_at)
VALUES (
    'user@example.com',
    '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5eo5kXBq.JVKi',
    'Regular User',
    'user',
    true,
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP
)
ON CONFLICT (email) DO NOTHING;
