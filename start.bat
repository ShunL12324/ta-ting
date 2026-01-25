@echo off
echo ========================================
echo   TaTing - AI Offline Dictation
echo ========================================
echo.
echo Starting TaTing development server...
echo.
echo NOTE: Keep this window open while developing
echo Press Ctrl+C to stop the server
echo.
echo ========================================
echo.

cd /d "%~dp0"
npm run tauri:dev

pause
