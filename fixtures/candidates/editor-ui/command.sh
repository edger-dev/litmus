#!/usr/bin/env bash
# Simulated text editor UI showing syntax-highlighted code with a status bar.
# Mimics a minimal editor like helix/kakoune with line numbers and mode indicator.
set -euo pipefail

RST='\033[0m'
BOLD='\033[1m'
DIM='\033[2m'
UL='\033[4m'
REV='\033[7m'
BLACK='\033[30m'
RED='\033[31m'
GREEN='\033[32m'
YELLOW='\033[33m'
BLUE='\033[34m'
MAGENTA='\033[35m'
CYAN='\033[36m'
WHITE='\033[37m'
BBLACK='\033[90m'
BRED='\033[91m'
BGREEN='\033[92m'
BYELLOW='\033[93m'
BBLUE='\033[94m'
BMAGENTA='\033[95m'
BCYAN='\033[96m'
BWHITE='\033[97m'
BG_BLUE='\033[44m'
BG_BLACK='\033[40m'

# Line numbers in dim, keywords in colors
echo -e "${BBLACK}  1${RST}  ${MAGENTA}use${RST} ${WHITE}std::collections::HashMap;${RST}"
echo -e "${BBLACK}  2${RST}  ${MAGENTA}use${RST} ${WHITE}std::io;${RST}"
echo -e "${BBLACK}  3${RST}"
echo -e "${BBLACK}  4${RST}  ${CYAN}/// Process incoming requests and return responses.${RST}"
echo -e "${BBLACK}  5${RST}  ${MAGENTA}pub fn${RST} ${BBLUE}handle_request${RST}(${WHITE}req${RST}: ${GREEN}&Request${RST}) -> ${GREEN}Response${RST} {"
echo -e "${BBLACK}  6${RST}      ${MAGENTA}let${RST} ${WHITE}method${RST} = req.method();${RST}"
echo -e "${BBLACK}  7${RST}      ${MAGENTA}let${RST} ${WHITE}path${RST} = req.path();${RST}"
echo -e "${BBLACK}  8${RST}"
echo -e "${BBLACK}  9${RST}      ${MAGENTA}match${RST} (method, path) {"
echo -e "${BBLACK} 10${RST}          (${YELLOW}\"GET\"${RST}, ${YELLOW}\"/health\"${RST}) => {"
echo -e "${BBLACK} 11${RST}              Response::ok(${YELLOW}\"healthy\"${RST})"
echo -e "${BBLACK} 12${RST}          }"
echo -e "${BBLACK} 13${RST}          (${YELLOW}\"POST\"${RST}, ${YELLOW}\"/api/data\"${RST}) => {"
echo -e "${BBLACK} 14${RST}              ${MAGENTA}let${RST} ${WHITE}body${RST} = req.body().unwrap_or_default();"
echo -e "${BBLACK} 15${RST}              ${MAGENTA}let${RST} ${WHITE}count${RST} = process_data(&body);"
echo -e "${BBLACK} 16${RST}              Response::json(${YELLOW}\"processed\"${RST}, count)"
echo -e "${BBLACK} 17${RST}          }"
echo -e "${BBLACK} 18${RST}          _ => Response::not_found(),"
echo -e "${BBLACK} 19${RST}      }"
echo -e "${BBLACK} 20${RST}  }"
echo ""
# Status bar (reverse video)
echo -e "${REV}${BOLD} NOR ${RST}${REV} src/handler.rs ${RST}${REV}${DIM} [+] utf-8 rust ${RST}${REV}                         ${RST}${REV}${DIM} 5:15 ${RST}${REV} 20 ln ${RST}"
# Command area
echo -e "${DIM}:w${RST}"
