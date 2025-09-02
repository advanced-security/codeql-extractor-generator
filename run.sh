#!/bin/bash
set -e

EXTRACTORS=$(yq -r '.[].name' extractors.yml)

for extractor in $EXTRACTORS; do
    echo "Running $extractor"
    WORKING_DIR=../codeql-extractor-php
    mkdir -p $WORKING_DIR

    if [[ ! -d $WORKING_DIR/extractor/tree-sitter ]]; then
        pushd $WORKING_DIR/extractor/tree-sitter
        tree-sitter generate
        popd
    fi

    exts=$(yq -r ".[] | select(.name == \"$extractor\") | .extensions | join(\",\")" extractors.yml)
    echo "Extensions: $exts"

    repo=$(yq -r ".[] | select(.name == \"$extractor\") | .repository" extractors.yml)

    cargo run -- \
        -w $WORKING_DIR \
        init -n $extractor -e "$exts" \
        -t $repo

done


# WORKING_DIR=../codeql-extractor-bash
# mkdir -p $WORKING_DIR

# cargo run -- \
#     -w $WORKING_DIR \
#     init -n Lua -e .lua,.l \
#     -t https://github.com/tree-sitter-grammars/tree-sitter-lua
