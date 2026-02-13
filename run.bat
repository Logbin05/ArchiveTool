@echo off
chcp 65001 >nul
setlocal EnableExtensions
cd /d "%~dp0"

title Archive Tool Launcher
color 0A

echo.
echo ============================================
echo           ARCHIVE TOOL LAUNCHER
echo ============================================
echo.

if not exist "target\release\archive-tool.exe" (
    color 0C
    echo [ОШИБКА] Файл archive-tool.exe не найден!
    pause
    exit /b
)

start "" "%~dp0\target\release\archive-tool.exe"

timeout /t 2 >nul
exit
