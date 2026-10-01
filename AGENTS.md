# AGENTS.md

`shed` is a Rust command-line tool. `CLAUDE.md` is a symlink to this file;
edit `AGENTS.md`.

## Commands

```sh
cargo build
cargo test
cargo fmt --check
cargo clippy --all-targets -- -D warnings
```

Run all four before finishing a change.

## Principles

- **Clean, strict Rust.** No `unsafe` (forbidden). No `unwrap`, `expect`, or
  `panic!` outside tests; return errors instead. Clippy pedantic is on; fix
  warnings rather than silencing them, and justify any `#[allow]` with a
  comment.
- **Few dependencies.** Prefer `std`. Add a crate only when it clearly pays
  for itself, keep default features off where possible, and mention why in
  the commit. Currently there are none.
- **Standard Unix behavior.**
  - Output goes to stdout, diagnostics to stderr, prefixed `shed:`.
  - Exit codes: `0` success, `1` runtime error, `2` usage error.
  - Support `-h/--help` and `-V/--version`; handle `BrokenPipe` quietly.
  - Config follows XDG paths with precedence flags > env (`SHED_*`) > file >
    defaults. See `docs/configuration.md`.
  - No color or prompts unless stdout is a TTY; respect `NO_COLOR`.
- **Testable core.** Keep logic in functions that take input and a
  `Write`r, and keep `main` thin.

## Layout

- `src/`: source; `main.rs` is the entry point.
- `docs/`: user and developer docs. Update them with behavior changes.

## Docs

When adding or changing flags, config keys, or exit codes, update
`docs/usage.md` or `docs/configuration.md` in the same change.
