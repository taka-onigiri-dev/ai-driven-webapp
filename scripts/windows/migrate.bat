@echo off
REM データベースマイグレーションを実行するスクリプト（レガシー）
REM 新しいSQLベースのマイグレーションスクリプトを使用してください

echo ====================================
echo AI-Driven WebApp - Database Migration
echo ====================================
echo.
echo NOTICE: This script is deprecated.
echo Please use the new SQL-based migration scripts:
echo.
echo   migrate-ddl.bat          - Create database schema
echo   migrate-master.bat       - Insert master data
echo   migrate-transaction.bat  - Insert transaction data
echo   migrate-all.bat          - Run all migrations
echo.
echo Redirecting to migrate-all.bat...
echo.
pause

call migrate-all.bat
