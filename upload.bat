@echo off
cd /d "%~dp0"
powershell -ExecutionPolicy Bypass -File "%~dp0upload.ps1"
pause
