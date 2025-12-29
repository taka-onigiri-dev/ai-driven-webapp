@echo off
REM Docker環境を起動するスクリプト

echo ====================================
echo AI-Driven WebApp - Starting...
echo ====================================
echo.

cd /d "%~dp0..\.."

echo [1/2] Starting Docker containers...
docker-compose up -d

echo.
echo [2/2] Waiting for services to be ready...
timeout /t 5 /nobreak >nul

echo.
echo ====================================
echo Services are starting!
echo ====================================
echo.
echo Frontend: http://localhost:3000
echo Backend:  http://localhost:8080
echo Database: localhost:5432
echo.
echo Use 'scripts\windows\logs.bat' to view logs
echo Use 'scripts\windows\stop.bat' to stop services
echo.

pause
