# Development

```sh
cargo build
cargo test
cargo fmt --check
cargo clippy --all-targets -- -D warnings
```

All four must pass before committing. Lint levels live in `Cargo.toml`
under `[lints]`; formatting in `rustfmt.toml`.

See [AGENTS.md](../AGENTS.md) for code conventions.
