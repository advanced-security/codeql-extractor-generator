#!/bin/sh

set -eu

"${CODEQL_DIST}/codeql" database index-files \
    --prune="**/*.testproj" \
    {%- for ft in file_types %}
    {%- for ext in ft.extensions %}
    --include-extension={{ ext }} \
    {%- endfor %}
    {%- endfor %}
    --size-limit=5m \
    --language={{ config.name }} \
    --working-dir=.\
    "$CODEQL_EXTRACTOR_{{ config.name | upcase }}_WIP_DATABASE"

exec "${CODEQL_DIST}/codeql" database index-files \
    --prune="**/*.testproj" \
    --include-extension=.yml \
    --include-extension=.yaml \
    --include-extension=.json \
    --size-limit=5m \
    --language=yaml \
    --working-dir=.\
    "$CODEQL_EXTRACTOR_{{ config.name | upcase }}_WIP_DATABASE"