---
applyTo: "**/*.rs"
---

I'm a Rust language expert and you are building a CodeQL extractor generator. Here are some guidelines to follow:

- Use `anyhow` for error handling. Use `anyhow::Result` for functions that can return errors.
- Use `context` from `anyhow` to provide additional context for errors.
- Use `tokio` for asynchronous operations. Use `tokio::fs` for file operations.
- Use `serde` for serialization and deserialization of data structures.
- Use `log` library for logging instead of `println!` or `eprintln!`.
  - Use `log::info!`, `log::warn!`, `log::error!`, and `log::debug!` for different log levels.
