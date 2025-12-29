@echo off
REM データベースマイグレーションを実行するスクリプト

echo ====================================
echo AI-Driven WebApp - Database Migration
echo ====================================
echo.

cd /d "%~dp0..\.."

echo [1/2] Running database migrations...
echo.

docker exec -it ai-webapp-backend cargo run --manifest-path migration/Cargo.toml -- up

echo.
echo [2/2] Creating test user...
echo.

docker exec -i ai-webapp-postgres psql -U app_user -d ai_webapp -c "INSERT INTO users (email, password_hash, name, role, is_active, created_at, updated_at) VALUES ('test@example.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5eo5kXBq.JVKi', 'Test User', 'user', true, NOW(), NOW()) ON CONFLICT (email) DO NOTHING;"

echo.
echo ====================================
echo Migration completed!
echo ====================================
echo.
echo Test user created:
echo   Email: test@example.com
echo   Password: Password123
echo.

pause
