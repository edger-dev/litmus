#!/usr/bin/env bash
# Run a scripted bash session that demonstrates prompt colors and common output.
set -euo pipefail

cd "$FIXTURE_WORK_DIR"

# Colors
BOLD='\e[1m'
GREEN='\e[1;32m'
BLUE='\e[1;34m'
YELLOW='\e[33m'
CYAN='\e[36m'
RED='\e[31m'
MAGENTA='\e[35m'
RESET='\e[0m'

# Prompt template: user@host:path (branch*) $
prompt() {
    local path="$1" branch="$2" dirty="${3:-}"
    printf "${GREEN}user@host${RESET}:${BLUE}%s${RESET} ${YELLOW}(%s%s)${RESET} \$ " "$path" "$branch" "$dirty"
}

# Command 1: echo
prompt "~/projects/myapp" "main"
echo -e "echo \"Hello, world!\""
echo "Hello, world!"
echo

# Command 2: failed command
prompt "~/projects/myapp" "main"
echo -e "cat nonexistent.txt"
echo -e "${RED}cat: nonexistent.txt: No such file or directory${RESET}"
echo

# Command 3: grep with TODO results (dirty branch)
prompt "~/projects/myapp" "main" "*"
echo -e "grep -rn TODO src/ | head -5"
printf "${MAGENTA}src/main.rs${RESET}${CYAN}:${RESET}${GREEN}2${RESET}${CYAN}:${RESET}    // ${YELLOW}TODO${RESET}: refactor this function\n"
printf "${MAGENTA}src/lib.rs${RESET}${CYAN}:${RESET}${GREEN}2${RESET}${CYAN}:${RESET}    // ${YELLOW}TODO${RESET}: add error handling\n"
printf "${MAGENTA}src/config.rs${RESET}${CYAN}:${RESET}${GREEN}2${RESET}${CYAN}:${RESET}    // ${YELLOW}TODO${RESET}: load from env\n"
echo

# Command 4: git status
prompt "~/projects/myapp" "main" "*"
echo -e "git status -s"
printf " ${RED}M${RESET} src/main.rs\n"
echo

# Final prompt (waiting for input)
prompt "~/projects/myapp" "main" "*"
