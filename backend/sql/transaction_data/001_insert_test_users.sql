-- Transaction data: Test users for development/testing
-- Password for all users: Password123
-- Password hash generated with bcrypt cost 12

-- Test user 1
INSERT INTO users (email, password_hash, name, role, is_active, created_at, updated_at)
VALUES (
    'test@example.com',
    '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5eo5kXBq.JVKi',
    'Test User',
    'user',
    true,
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP
)
ON CONFLICT (email) DO NOTHING;

-- Test user 2
INSERT INTO users (email, password_hash, name, role, is_active, created_at, updated_at)
VALUES (
    'test2@example.com',
    '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5eo5kXBq.JVKi',
    'Test User 2',
    'user',
    true,
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP
)
ON CONFLICT (email) DO NOTHING;

-- Test user 3
INSERT INTO users (email, password_hash, name, role, is_active, created_at, updated_at)
VALUES (
    'test3@example.com',
    '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5eo5kXBq.JVKi',
    'Test User 3',
    'user',
    true,
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP
)
ON CONFLICT (email) DO NOTHING;

-- Inactive test user
INSERT INTO users (email, password_hash, name, role, is_active, created_at, updated_at)
VALUES (
    'inactive@example.com',
    '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5eo5kXBq.JVKi',
    'Inactive User',
    'user',
    false,
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP
)
ON CONFLICT (email) DO NOTHING;
