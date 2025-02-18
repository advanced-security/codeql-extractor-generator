@echo off

type NUL && "%CODEQL_EXTRACTOR_{{ config.name | upcase }}_ROOT%\tools\win64\extractor.exe" ^
        --file-list "%1" ^
        --source-archive-dir "%CODEQL_EXTRACTOR_{{ config.name | upcase }}_SOURCE_ARCHIVE_DIR%" ^
        --output-dir "%CODEQL_EXTRACTOR_{{ config.name | upcase }}_TRAP_DIR%"

exit /b %ERRORLEVEL%
