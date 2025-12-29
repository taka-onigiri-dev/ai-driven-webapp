@echo off
REM Docker環境を再起動するスクリプト

echo ====================================
echo AI-Driven WebApp - Restarting...
echo ====================================
echo.

cd /d "%~dp0..\.."

echo [1/2] Stopping containers...
docker-compose down

echo.
echo [2/2] Starting containers...
docker-compose up -d

echo.
echo ====================================
echo Services restarted successfully!
echo ====================================
echo.
echo Frontend: http://localhost:3000
echo Backend:  http://localhost:8080
echo Database: localhost:5432
echo.

pause
