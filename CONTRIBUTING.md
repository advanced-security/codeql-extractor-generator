# Contributing

[fork]: https://github.com/advanced-security/codeql-extractor-generator/fork
[pr]: https://github.com/advanced-security/codeql-extractor-generator/compare
[style]: https://doc.rust-lang.org/style-guide/

Thanks for your interest in contributing! Contributions help improve the generator and the extractor templates it creates.

Contributions to this project are released under the terms of the project's [MIT license](LICENSE.txt).

## Prerequisites

- [Rust and Cargo](https://www.rust-lang.org/tools/install) (the project uses the stable toolchain)
- Git

The [CodeQL CLI](https://github.com/github/codeql-cli-binaries) is also required to initialize a generated extractor project and run its CodeQL tests.

## Submitting a pull request

1. [Fork][fork] and clone the repository.
1. Create a branch for your change: `git checkout -b my-branch-name`.
1. Make your change and add or update tests as appropriate.
1. Run the Rust checks from the repository root:

   ```sh
   cargo fmt --check
   cargo clippy --all-targets --all-features -- -D warnings
   cargo test --all-targets
   ```

1. If your change affects the generated extractor or its CodeQL libraries, validate the generated output with the CodeQL CLI as appropriate.
1. Push your branch to your fork and [open a pull request][pr].

To help your pull request get reviewed:

- Follow the [Rust style guide][style] and the conventions used in the surrounding code.
- Keep each pull request focused; submit independent changes separately.
- Include a clear description of the change and its motivation.
- Write a concise, descriptive commit message.

## Resources

- [How to Contribute to Open Source](https://opensource.guide/how-to-contribute/)
- [About pull requests](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests)
