#!/bin/sh

set -eu

exec "${CODEQL_EXTRACTOR_{{ config.name | upcase }}_ROOT}/tools/${CODEQL_PLATFORM}/extractor" \
        extract \
        --file-list "$1" \
        --source-archive-dir "$CODEQL_EXTRACTOR_{{ config.name | upcase }}_SOURCE_ARCHIVE_DIR" \
        --output-dir "$CODEQL_EXTRACTOR_{{ config.name | upcase }}_TRAP_DIR"