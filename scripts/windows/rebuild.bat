@echo off
REM Dockerイメージを再ビルドして起動するスクリプト

echo ====================================
echo AI-Driven WebApp - Rebuild
echo ====================================
echo.

cd /d "%~dp0..\.."

echo [1/3] Stopping containers...
docker-compose down

echo.
echo [2/3] Rebuilding images...
docker-compose build --no-cache

echo.
echo [3/3] Starting containers...
docker-compose up -d

echo.
echo ====================================
echo Rebuild completed!
echo ====================================
echo.
echo Frontend: http://localhost:3000
echo Backend:  http://localhost:8080
echo Database: localhost:5432
echo.

pause
