@echo off
REM Dockerコンテナの状態を確認するスクリプト

echo ====================================
echo AI-Driven WebApp - Status
echo ====================================
echo.

cd /d "%~dp0..\.."

echo Checking container status...
echo.
docker-compose ps

echo.
echo ====================================
echo.

pause
