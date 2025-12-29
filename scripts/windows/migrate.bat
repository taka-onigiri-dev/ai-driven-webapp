@echo off
REM データベースマイグレーションを実行するスクリプト

echo ====================================
echo AI-Driven WebApp - Database Migration
echo ====================================
echo.

cd /d "%~dp0..\.."

echo Running database migrations...
echo.

docker exec -it ai-webapp-backend cargo run --manifest-path migration/Cargo.toml -- up

echo.
echo ====================================
echo Migration completed!
echo ====================================
echo.

pause
