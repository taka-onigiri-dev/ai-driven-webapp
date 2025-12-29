@echo off
REM Dockerコンテナのログを表示するスクリプト

echo ====================================
echo AI-Driven WebApp - Logs
echo ====================================
echo.

cd /d "%~dp0..\.."

if "%1"=="" (
    echo Showing logs for all services...
    echo Press Ctrl+C to stop
    echo.
    docker-compose logs -f
) else (
    echo Showing logs for %1...
    echo Press Ctrl+C to stop
    echo.
    docker-compose logs -f %1
)
