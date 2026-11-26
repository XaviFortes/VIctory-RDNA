@echo off
setlocal
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0scripts\play-rdna.ps1" %*
if errorlevel 1 pause
