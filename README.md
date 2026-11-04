# CodeQL Extractor Generator

This Rust command-line tool bootstraps a CodeQL language extractor project from the
repository's `template/`. It renders the extractor and QL library templates for a
language, reads the language's Tree-sitter `node-types.json`, and groups syntax
nodes into CodeQL AST categories using configurable mappings. The template includes
extractor source, QL libraries, diagnostics, test scaffolding, and pack scripts.

## Requirements

- Rust and Cargo
- Git (used to clone the configured Tree-sitter grammar)
- The CodeQL CLI on `PATH` (initialization runs `codeql pack create ql/lib`)

## Create an extractor project

The generator loads its configuration before running any subcommand. Create the
target directory and its `.codeql/generator.yml` first:

```sh
mkdir -p ../my-language-extractor/.codeql
```

For example, save the following as
`../my-language-extractor/.codeql/generator.yml`, replacing the language details
and mapping rules with those for your grammar:

```yaml
extractor: my-language
language_name: My Language
language_extensions:
  - .ml
features:
  extractor: true
  ast: true
  ast_internal: true
  cfg: true
  cfg_internal: true
  data_flow: true
  data_flow_internal: true
tree-sitter-repo:
  url: https://github.com/example/tree-sitter-my-language
mappings:
  - name: Comments
    pattern: Match
    aliases:
      - Comment
      - CommentBlock
```

Then run the generator from this repository's root:

```sh
cargo run --release -- \
  --working-dir ../my-language-extractor \
  --template-dir ./template \
  init \
  --name "My Language" \
  --extensions .ml \
  --tree-sitter-repo https://github.com/example/tree-sitter-my-language
```

`init` clones the grammar into `extractor/tree-sitter` if it is not already
present, writes `codeql-extractor.yml`, renders the template into the working
directory, and creates the QL library pack with the CodeQL CLI. The default
configuration path is `.codeql/generator.yml` relative to the working directory;
use `--config` to select another file. The generator's root `mapping.yml` is an
example set of AST mapping rules; the `mappings` property in `generator.yml` is a
list, so copy the individual rules rather than the file's outer `mappings:` key.

On a fresh target, template files are generated even when their corresponding
feature flags are false. The feature flags control whether existing generated
files in those feature areas are overwritten.

## Commands

```sh
cargo run -- --help
cargo run -- --working-dir ../my-language-extractor update
cargo run -- --working-dir ../my-language-extractor extractor
```

- `init` generates the extractor project from the template.
- `update` interactively regenerates AST internal files.
- `extractor` loads the generated `codeql-extractor.yml`; it does not currently
  build the extractor.

Common options are `--working-dir`, `--template-dir`, and `--config`. Set
`--debug` for debug logging or `--disable-banner` to suppress the startup banner.

## Repository layout

- `src/` contains the Rust CLI, configuration handling, Tree-sitter node loading,
  mapping, and template generation.
- `template/` contains the scaffold copied and rendered for a language.
- `mapping.yml` contains example Tree-sitter node-to-AST mapping rules.
- `extractors.yml` and `run.sh` are examples for iterating over multiple language
  extractor configurations.
