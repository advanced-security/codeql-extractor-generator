# CodeQL Extractor Generator

A Rust command-line tool for bootstrapping CodeQL language extractor projects from Tree-sitter grammars.

## Features, scope, and status

The generator renders extractor and QL library templates for a language, reads
the language's Tree-sitter `node-types.json`, and groups syntax nodes into
CodeQL AST categories using configurable mappings. The template includes
extractor source, QL libraries, diagnostics, test scaffolding, and pack scripts.

- **Features:** initialize an extractor project, interactively regenerate AST
  internal files, and load the generated `codeql-extractor.yml`.
- **Limitations:** the `extractor` command loads the generated configuration but
  does not build the extractor. On a fresh target, all template files are
  generated even when their corresponding feature flags are false; those flags
  control whether existing generated files in feature areas are overwritten.
- **Goals and scope:** provide a starting point for building CodeQL language
  extractors from Tree-sitter grammars. The generator scaffolds files; it does
  not build or maintain a production extractor for a particular language.
- **Status:** in development. See the [open issues](https://github.com/advanced-security/codeql-extractor-generator/issues)
  for current work.
- **Demo or production instance:** none; the generator is run locally from the
  command line.

## Background

The generator uses the repository's `template/` directory and the project's
root `mapping.yml` as an example set of Tree-sitter node-to-AST mapping rules.
The `mappings` property in `.codeql/generator.yml` is a list, so copy the
individual rules from `mapping.yml` rather than the file's outer `mappings:`
key.

See the [open issues](https://github.com/advanced-security/codeql-extractor-generator/issues)
and [milestones](https://github.com/advanced-security/codeql-extractor-generator/milestones)
for planned and ongoing work. Contributions are welcome; read
[CONTRIBUTING.md](./CONTRIBUTING.md) for prerequisites and submission guidance.

## Requirements

- [Rust and Cargo](https://www.rust-lang.org/tools/install), using the stable
  toolchain, to build and run the generator. Cargo downloads the Rust
  dependencies declared in `Cargo.toml`.
- Git, which the generator uses to clone the configured Tree-sitter grammar.
- The [CodeQL CLI](https://github.com/github/codeql-cli-binaries) on `PATH`.
  Initialization runs `codeql pack create ql/lib` to create the QL library
  pack; the CodeQL CLI is also needed to run tests for a generated extractor.

### Create an extractor project

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
use `--config` to select another file.

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

## License

This project is licensed under the terms of the MIT open source license. Please
refer to the [LICENSE.txt](./LICENSE.txt) file for the full terms.

## Maintainers

Maintained by the [GitHub Advanced Security organization](https://github.com/advanced-security).

## Support

There is no dedicated support service or SLA. Please report bugs and request
project help through [GitHub Issues](https://github.com/advanced-security/codeql-extractor-generator/issues).

## Acknowledgement

Thanks to the [Tree-sitter](https://tree-sitter.github.io/tree-sitter/) and
[CodeQL](https://codeql.github.com/) projects, which provide the grammar and
code analysis tools this generator builds on.
