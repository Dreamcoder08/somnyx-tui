# somnyx-tui — Skill-Gap Analysis Terminal UI

> Minimal terminal dashboard for the **Somnus × Nyx** workspace ecosystem.  
> Real-time system stats, workspace health, inbox tracking, and integrated launchers — all in one TUI.

## Quickstart

```bash
cargo build --release
./target/release/somnyx-tui
```

Or install system-wide:

```bash
./install.sh
```

## Usage

| Key | Action |
|-----|--------|
| `r` | Refresh all data |
| `j` | Open today's journal in `$EDITOR` |
| `y` | Open [yazi](https://yazi-rs.github.io/) file manager in workspace |
| `f` | Fuzzy-find files with `fd` + `fzf` |
| `?` | Toggle help overlay |
| `q` / `Esc` | Exit |

## Dashboard

Three-column layout:

- **Workspace panel** — directory tree with sizes for `dev/`, `archive/`, `vault/`, `notes/`, `media/`, `inbox/`
- **System panel** — CPU, RAM, disk usage with color-coded bars + uptime, systemd timer status
- **Inbox / Journal** — inbox file count (warns on >7d old), daily journal entry tracker

Data refreshes automatically every ~2s (workspace) / ~5s (system).

## Configuration

The workspace paths are configurable via environment variables:

| Variable | Default |
|----------|---------|
| `SOMNYX_WORKSPACE` | `~/somnyx` |
| `SOMNYX_ARCHIVE` | `~/archive` |
| `SOMNYX_VAULT` | `~/vault` |
| `SOMNYX_NOTES` | `~/notes` |
| `SOMNYX_MEDIA` | `~/media` |
| `SOMNYX_INBOX` | `~/inbox` |

## Stack

**Rust** · **Ratatui** · **Crossterm** · **Terminal UI**

Dependencies:
- [ratatui](https://ratatui.rs/) (0.29) — terminal UI framework
- [crossterm](https://github.com/crossterm-rs/crossterm) (0.28) — terminal manipulation
- [chrono](https://github.com/chronotope/chrono) (0.4) — date/time utilities

Runtime tools (external): `fd`, `fzf`, `yazi`, `systemctl --user`, `du`, `df`.

## Proyectos relacionados

- [somnyx-web](https://github.com/Dreamcoder08/somnyx-web) — Web platform in Python
- [Dreamcoder08](https://github.com/Dreamcoder08) — Profile

## License

MIT — see [LICENSE](LICENSE).
