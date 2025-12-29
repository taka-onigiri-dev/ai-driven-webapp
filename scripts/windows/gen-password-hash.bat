@echo off
echo ========================================
echo Password Hash Generator
echo ========================================
echo.

echo Building and running hash generator...
echo.

docker exec -it ai-webapp-backend cargo run --bin gen-password-hash

echo.
pause
