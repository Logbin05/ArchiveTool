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

if not exist "archive-tool.exe" (
    color 0C
    echo [ОШИБКА] Файл archive-tool.exe не найден!
    echo Убедитесь, что он находится рядом с run.bat
    pause
    exit /b
)

start "" "%~dp0archive-tool.exe"
timeout /t 1 >nul

exit
