@echo off

type NUL && "%CODEQL_EXTRACTOR_{{ config.name | upcase }}_ROOT%\tools\%CODEQL_PLATFORM%\extractor" autobuild

exit /b %ERRORLEVEL%