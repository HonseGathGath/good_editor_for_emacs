# Good Editor for Emacs

## Overview

A terminal-based text editor written in Rust using the `crossterm` library for cross-platform terminal control. Implements cursor movement (arrow keys, PgUp/PgDn, Home/End), file loading and display, screen refresh with terminal raw-mode management, and panic-safe terminal cleanup. Enforces strict clippy linting (`clippy::pedantic`, `clippy::print_stdout`, `clippy::arithmetic_side_effects`).

## Prerequisites

- Rust toolchain (edition 2024)
- Cargo

## Installation & Setup

```bash
git clone <repo-url>
cd good_editor_for_emacs
cargo build --release
```

## Usage

```bash
cargo run -- <file-to-edit>
```

### Key Bindings

| Key | Action |
|-----|--------|
| Arrow keys | Move cursor |
| Page Up / Page Down | Scroll by page |
| Home | Move to line start |
| End | Move to line end |
| Ctrl + Q | Quit |

Note: This is an early-stage editor. Text insertion, deletion, and file saving are not yet implemented.

## Project Structure

```
good_editor_for_emacs/
  Cargo.toml        — Rust dependencies (crossterm)
  justfile          — Build/release commands
  test.txt          — Sample text file for testing
  src/
    main.rs         — Application entry point with lint configuration
    editor.rs       — Editor core: initialization, event loop, screen refresh
    editor/
      terminal.rs   — Terminal raw-mode, cursor, and screen management
      view.rs       — File view rendering and scroll management
      view/         — View rendering modules
```

## Contributing

Contributions, bug reports, and feature requests are welcome. This is an open-source project.

## License

Open-source software. Available under the MIT License.
