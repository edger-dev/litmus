#!/usr/bin/env bash
# Simulate a Python REPL session using ANSI output.
# We use printf/echo to reproduce what a Python REPL session looks like,
# since interactive python3 cannot be scripted reproducibly in a terminal screenshot.
set -euo pipefail

GREEN='\e[32m'
BLUE='\e[34m'
CYAN='\e[36m'
RED='\e[31m'
YELLOW='\e[33m'
MAGENTA='\e[35m'
BOLD='\e[1m'
DIM='\e[2m'
RESET='\e[0m'

# REPL header
echo -e "${BOLD}Python 3.12.0${RESET} (main, Oct  2 2023, 00:00:00)"
echo -e "Type \"help\", \"copyright\", \"credits\" or \"license\" for more information."

# Session: working with data
printf "${GREEN}>>> ${RESET}"
echo 'data = {"users": [{"name": "Alice", "score": 95}, {"name": "Bob", "score": 87}]}'
printf "${GREEN}>>> ${RESET}"
echo 'import json; print(json.dumps(data, indent=2))'
echo -e "${CYAN}{"
echo -e "  ${MAGENTA}\"users\"${RESET}: ["
echo -e "    {"
echo -e "      ${MAGENTA}\"name\"${RESET}: ${GREEN}\"Alice\"${RESET},"
echo -e "      ${MAGENTA}\"score\"${RESET}: ${YELLOW}95${RESET}"
echo -e "    },"
echo -e "    {"
echo -e "      ${MAGENTA}\"name\"${RESET}: ${GREEN}\"Bob\"${RESET},"
echo -e "      ${MAGENTA}\"score\"${RESET}: ${YELLOW}87${RESET}"
echo -e "    }"
echo -e "  ]"
echo -e "}${RESET}"

# List comprehension
printf "${GREEN}>>> ${RESET}"
echo 'top = [u["name"] for u in data["users"] if u["score"] > 90]'
printf "${GREEN}>>> ${RESET}"
echo 'print(top)'
echo -e "${CYAN}['Alice']${RESET}"

# Traceback
printf "${GREEN}>>> ${RESET}"
echo 'data["missing"]["key"]'
echo -e "${RED}Traceback (most recent call last):${RESET}"
echo -e "  ${DIM}File \"<stdin>\", line 1, in <module>${RESET}"
echo -e "${RED}KeyError: ${RESET}${YELLOW}'missing'${RESET}"

# Final prompt
printf "${GREEN}>>> ${RESET}"
