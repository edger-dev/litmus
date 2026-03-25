#!/usr/bin/env bash
# Show simulated application logs with ANSI-colored severity levels.
# Mimics journalctl/structured log output with timestamps and levels.
set -euo pipefail

# ANSI color codes
RST='\033[0m'
BOLD='\033[1m'
DIM='\033[2m'
RED='\033[31m'
GREEN='\033[32m'
YELLOW='\033[33m'
BLUE='\033[34m'
MAGENTA='\033[35m'
CYAN='\033[36m'
WHITE='\033[37m'
BRED='\033[1;31m'
BGREEN='\033[1;32m'
BYELLOW='\033[1;33m'
BBLUE='\033[1;34m'
BMAGENTA='\033[1;35m'
BCYAN='\033[1;36m'

# Header
echo -e "${BOLD}${WHITE}── Application Logs ──────────────────────────────────────${RST}"
echo ""

# Log entries with mixed severity
echo -e "${DIM}2024-01-15 10:00:01${RST} ${BGREEN} INFO${RST} ${CYAN}server${RST}    Starting HTTP server on ${GREEN}0.0.0.0:8080${RST}"
echo -e "${DIM}2024-01-15 10:00:01${RST} ${BGREEN} INFO${RST} ${CYAN}db${RST}        Connected to ${BLUE}postgres://localhost:5432/app${RST}"
echo -e "${DIM}2024-01-15 10:00:02${RST} ${BGREEN} INFO${RST} ${CYAN}server${RST}    Ready to accept connections"
echo -e "${DIM}2024-01-15 10:00:05${RST} ${BBLUE}DEBUG${RST} ${CYAN}http${RST}      ${WHITE}GET${RST} /health → ${GREEN}200${RST} ${DIM}(2ms)${RST}"
echo -e "${DIM}2024-01-15 10:00:08${RST} ${BBLUE}DEBUG${RST} ${CYAN}http${RST}      ${WHITE}GET${RST} /api/users → ${GREEN}200${RST} ${DIM}(15ms)${RST}"
echo -e "${DIM}2024-01-15 10:00:12${RST} ${BYELLOW} WARN${RST} ${CYAN}db${RST}        Connection pool at ${YELLOW}80%${RST} capacity (${YELLOW}8/10${RST})"
echo -e "${DIM}2024-01-15 10:00:15${RST} ${BBLUE}DEBUG${RST} ${CYAN}http${RST}      ${WHITE}POST${RST} /api/deploy → ${GREEN}202${RST} ${DIM}(45ms)${RST}"
echo -e "${DIM}2024-01-15 10:00:18${RST} ${BGREEN} INFO${RST} ${CYAN}deploy${RST}    Build ${MAGENTA}#1847${RST} started for branch ${BMAGENTA}main${RST}"
echo -e "${DIM}2024-01-15 10:00:22${RST} ${BYELLOW} WARN${RST} ${CYAN}deploy${RST}    Retrying artifact download (attempt ${YELLOW}2/3${RST})"
echo -e "${DIM}2024-01-15 10:00:25${RST} ${BGREEN} INFO${RST} ${CYAN}deploy${RST}    Build ${MAGENTA}#1847${RST} ${BGREEN}succeeded${RST} in ${BCYAN}7.2s${RST}"
echo -e "${DIM}2024-01-15 10:00:30${RST} ${BRED}ERROR${RST} ${CYAN}http${RST}      ${WHITE}GET${RST} /api/reports → ${RED}500${RST} ${DIM}(102ms)${RST}"
echo -e "${DIM}2024-01-15 10:00:30${RST} ${BRED}ERROR${RST} ${CYAN}handler${RST}   ${RED}QueryError: relation \"reports\" does not exist${RST}"
echo -e "${DIM}2024-01-15 10:00:30${RST} ${BRED}ERROR${RST} ${CYAN}handler${RST}   ${DIM}  at src/db/queries.rs:42${RST}"
echo -e "${DIM}2024-01-15 10:00:35${RST} ${BBLUE}DEBUG${RST} ${CYAN}http${RST}      ${WHITE}GET${RST} /api/users?limit=50 → ${GREEN}200${RST} ${DIM}(12ms)${RST}"
echo -e "${DIM}2024-01-15 10:00:40${RST} ${BGREEN} INFO${RST} ${CYAN}metrics${RST}   Requests: ${GREEN}127${RST} | Errors: ${RED}1${RST} | P99: ${CYAN}45ms${RST}"
echo ""
echo -e "${DIM}── end of log ──${RST}"
