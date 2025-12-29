@echo off
echo ========================================
echo AI-Driven WebApp - DDL Migration
echo ========================================
echo.

echo [Step 1/2] Checking if Docker containers are running...
docker ps | findstr ai-webapp-postgres >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo Error: PostgreSQL container is not running.
    echo Please run start.bat first.
    pause
    exit /b 1
)
echo OK - PostgreSQL container is running.
echo.

echo [Step 2/2] Executing DDL scripts...
echo.

set SQL_DIR=../../backend/sql/ddl

for %%f in (%SQL_DIR%\*.sql) do (
    echo Executing: %%~nxf
    docker exec -i ai-webapp-postgres psql -U app_user -d ai_webapp < "%%f"
    if %ERRORLEVEL% NEQ 0 (
        echo Error: Failed to execute %%~nxf
        pause
        exit /b 1
    )
    echo OK - %%~nxf executed successfully.
    echo.
)

echo ========================================
echo DDL migration completed successfully!
echo ========================================
echo.
echo Database schema has been created.
echo.
pause
