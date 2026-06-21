#!/usr/bin/env bash
# SOMNYX TUI — Build e instalacion
set -euo pipefail

BOLD='\033[1m'; GREEN='\033[0;32m'; CYAN='\033[0;36m'; RESET='\033[0m'

echo -e "${BOLD}SOMNYX TUI — Build${RESET}"
echo -e "${CYAN}[*]${RESET} Compilando en modo release..."

cargo build --release 2>&1

BIN="$(pwd)/target/release/somnyx-tui"
DEST="$HOME/.local/bin/somnyx-tui"

cp "$BIN" "$DEST"
chmod +x "$DEST"

echo -e "${GREEN}[✓]${RESET} Instalado en ~/.local/bin/somnyx-tui"
echo ""
echo -e "${BOLD}Ejecutar:${RESET}"
echo -e "  ${CYAN}somnyx-tui${RESET}"
