#!/usr/bin/env bash
# Setup: create a git repo and some files for the shell session to operate in.
set -euo pipefail

cd "$FIXTURE_WORK_DIR"

git init -b main -q
git config user.email "demo@litmus.dev"
git config user.name "Litmus Demo"

mkdir -p src tests

cat > src/main.rs << 'RUST'
// TODO: refactor this function
fn main() {
    println!("Hello, world!");
}
RUST

cat > src/lib.rs << 'RUST'
// TODO: add error handling
pub fn process(input: &str) -> String {
    input.to_uppercase()
}
RUST

cat > src/config.rs << 'RUST'
// TODO: load from env
pub const VERSION: &str = "0.1.0";
RUST

git add -A
GIT_AUTHOR_DATE="2024-01-15T10:00:00" \
GIT_COMMITTER_DATE="2024-01-15T10:00:00" \
git commit -q -m "Initial commit"

# Modify a file so git status shows dirty working tree
echo "// new feature" >> src/main.rs

# Write the prompt config script
cat > "$FIXTURE_WORK_DIR/.bashrc_demo" << 'BASHRC'
# Minimal colorful bash prompt: user@host:path (branch) $
__git_branch() {
    local branch
    branch=$(git symbolic-ref --short HEAD 2>/dev/null) || return
    local dirty=""
    [[ -n $(git status --porcelain 2>/dev/null) ]] && dirty="*"
    echo " (\[\e[33m\]${branch}${dirty}\[\e[0m\])"
}
PS1='\[\e[1;32m\]\u@\h\[\e[0m\]:\[\e[1;34m\]\w\[\e[0m\]$(__git_branch) \$ '
BASHRC
