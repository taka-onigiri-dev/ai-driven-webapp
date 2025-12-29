@echo off
echo ========================================
echo AI-Driven WebApp - Master Data Migration
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

echo [Step 2/2] Inserting master data...
echo.

set SQL_DIR=../../backend/sql/master_data

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
echo Master data migration completed!
echo ========================================
echo.
echo System users have been created:
echo   - admin@example.com      (Admin)
echo   - moderator@example.com  (Moderator)
echo   - user@example.com       (User)
echo.
echo Password for all users: Password123
echo.
pause
