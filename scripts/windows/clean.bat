@echo off
REM Docker環境をクリーンアップするスクリプト（コンテナ・ボリューム削除）

echo ====================================
echo AI-Driven WebApp - Clean
echo ====================================
echo.
echo WARNING: This will remove all containers and volumes!
echo All data will be lost!
echo.

set /p confirm="Are you sure? (yes/no): "

if /i "%confirm%"=="yes" (
    cd /d "%~dp0..\.."

    echo.
    echo [1/3] Stopping containers...
    docker-compose down

    echo.
    echo [2/3] Removing volumes...
    docker-compose down -v

    echo.
    echo [3/3] Pruning system...
    docker system prune -f

    echo.
    echo ====================================
    echo Cleanup completed!
    echo ====================================
) else (
    echo.
    echo Cleanup cancelled.
)

echo.
pause
