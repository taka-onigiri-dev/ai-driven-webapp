@echo off
REM Docker環境を停止するスクリプト

echo ====================================
echo AI-Driven WebApp - Stopping...
echo ====================================
echo.

cd /d "%~dp0..\.."

echo Stopping Docker containers...
docker-compose down

echo.
echo ====================================
echo Services stopped successfully!
echo ====================================
echo.

pause
