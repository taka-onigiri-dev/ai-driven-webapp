@echo off
echo ========================================
echo AI-Driven WebApp - Full Database Migration
echo ========================================
echo.
echo This script will execute:
echo   1. DDL (Database schema)
echo   2. Master data (System users)
echo   3. Transaction data (Test users)
echo.
pause

echo.
echo ========================================
echo [1/3] Executing DDL migration...
echo ========================================
call migrate-ddl.bat
if %ERRORLEVEL% NEQ 0 (
    echo Error: DDL migration failed.
    pause
    exit /b 1
)

echo.
echo ========================================
echo [2/3] Executing master data migration...
echo ========================================
call migrate-master.bat
if %ERRORLEVEL% NEQ 0 (
    echo Error: Master data migration failed.
    pause
    exit /b 1
)

echo.
echo ========================================
echo [3/3] Executing transaction data migration...
echo ========================================
call migrate-transaction.bat
if %ERRORLEVEL% NEQ 0 (
    echo Error: Transaction data migration failed.
    pause
    exit /b 1
)

echo.
echo ========================================
echo Full database migration completed!
echo ========================================
echo.
echo All database objects and data have been created.
echo.
echo Available accounts:
echo.
echo System Users (Master Data):
echo   - admin@example.com
echo   - moderator@example.com
echo   - user@example.com
echo.
echo Test Users (Transaction Data):
echo   - test@example.com
echo   - test2@example.com
echo   - test3@example.com
echo   - inactive@example.com (inactive)
echo.
echo Password for all users: Password123
echo.
pause
