#!/bin/sh

exec "${CODEQL_EXTRACTOR_{{ config.name | upcase }}_ROOT}/tools/${CODEQL_PLATFORM}/extractor" autobuild