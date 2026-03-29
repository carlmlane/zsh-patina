# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

zsh-patina is a Zsh syntax highlighting plugin written in Rust. It runs a background daemon that communicates with Zsh over a Unix socket, using syntect (Sublime Text syntax definitions) for tokenization. The daemon is shared across shell sessions.

## Build & Test Commands

```sh
cargo build                    # Debug build
cargo build --release          # Release build
cargo build --profile release-lto  # Optimized release (LTO, used in CI)
cargo test                     # Run all tests
cargo fmt -- --check           # Check formatting
cargo clippy                   # Lint
```

Smoke tests (after building release):
```sh
target/release/zsh-patina list-scopes
echo 'for i in 1 2 3; do echo $i; done' | target/release/zsh-patina tokenize
```

## Architecture

**Daemon (`src/daemon.rs`):** Manages the background process lifecycle, Unix socket communication, and caches syntax definitions and themes in memory.

**Highlighter (`src/highlighting/highlighter.rs`):** Core syntax highlighting engine. Tokenizes input line-by-line using syntect, tracks scope state, and mixes dynamic styles over static styles.

**Dynamic highlighting (`src/highlighting/dynamic.rs`):** Post-processing pass that validates commands (aliases, builtins, executables, functions) and paths (files/directories), coloring invalid commands red and underlining existing paths.

**Theme system (`src/theme.rs`, `themes/`):** TOML-based themes mapping Sublime Text scopes to colors/styles. Supports theme inheritance via `metadata.extends`.

**Configuration (`src/config.rs`):** TOML config at `~/.config/zsh-patina/config.toml`, parsed with figment.

**Build script (`build.rs`):** Generates scope definitions from `assets/Packages/ShellScript/Bash.sublime-syntax` and dumps syntax definitions to binary format for runtime use.

**Zsh integration (`templates/zsh-patina.zsh`):** Askama template with custom delimiters (`<%...%>` blocks, `<{...}>` expressions), configured in `askama.toml`.

## Key Conventions

- Rust edition 2024, MSRV 1.88.0
- Error handling uses `anyhow` with context via `?` operator
- CI enforces `cargo fmt` and `cargo clippy` on all PRs
