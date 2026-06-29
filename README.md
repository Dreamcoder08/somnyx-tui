# SOMNYX TUI

> Terminal dashboard for system monitoring — Rust + Ratatui under the Somnus × Nyx identity.

A terminal UI dashboard by Dreamcoder08 built with Ratatui and Crossterm, providing real-time system metrics in a lightweight, keyboard-navigable TUI.

---

## Installation

```bash
cargo install --path .
```

Or build from source:

```bash
cd somnyx-tui
cargo build --release
./target/release/somnyx-tui
```

---

## Usage

```bash
somnyx-tui
```

Navigate with arrow keys. Exit with `q` or `Ctrl+C`.

---

## Architecture

A single-binary TUI application using Ratatui for rendering and Crossterm for terminal interaction. Data collection runs on a background tick loop.

```
somnyx-tui/
├── src/
│   ├── main.rs      # Entry point and app loop
│   ├── app.rs       # Application state and layout
│   ├── ui.rs        # Ratatui rendering widgets
│   └── data.rs      # System data collection
├── install.sh       # Convenience install script
└── Cargo.toml       # Rust project manifest
```

---

## Tech Stack

| Layer | Tech | Purpose |
|-------|------|---------|
| TUI Framework | Ratatui 0.29 | Terminal UI rendering |
| Terminal Backend | Crossterm 0.28 | Cross-platform terminal control |
| Language | Rust (edition 2021) | Performance and safety |

---

## Project Status

**Status:** Active
**Version:** 0.1.0

---

## License

MIT

---

## SDD

This project sits within the [Dreamcoder08](https://github.com/Dreamcoder08) ecosystem. Documentation is maintained in the [SDD Maestro](../arkelythex/sdd/ecosystem-readme-sdd/00-README.md).
