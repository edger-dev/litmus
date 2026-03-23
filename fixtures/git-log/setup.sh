#!/usr/bin/env bash
# Setup: create a git repo with multiple commits, branches, merges, and tags.
set -euo pipefail

cd "$FIXTURE_WORK_DIR"

git init -b main -q
git config user.email "demo@litmus.dev"
git config user.name "Litmus Demo"

# Helper to commit with fixed timestamps
commit() {
    local msg="$1"
    local ts="$2"
    GIT_AUTHOR_DATE="$ts" GIT_COMMITTER_DATE="$ts" git commit -q -m "$msg"
}

# Main branch: initial commit
echo "v1.0.0" > VERSION
git add -A
commit "chore: initial project setup" "2024-01-10T09:00:00"

# feature/auth branch
git checkout -q -b feature/auth
echo "auth module" > auth.rs
git add -A
commit "feat(auth): add authentication module" "2024-01-11T11:30:00"

echo "jwt support" >> auth.rs
git add -A
commit "feat(auth): add JWT token support" "2024-01-12T14:00:00"

# Back to main, add another commit
git checkout -q main
echo "config.rs" > config.rs
git add -A
commit "feat: add configuration module" "2024-01-12T10:00:00"

# hotfix branch
git checkout -q -b hotfix/null-check
echo "// null check fix" > fix.rs
git add -A
commit "fix: prevent null pointer in request handler" "2024-01-13T09:15:00"

# Merge hotfix to main
git checkout -q main
GIT_AUTHOR_DATE="2024-01-13T12:00:00" GIT_COMMITTER_DATE="2024-01-13T12:00:00" \
git merge -q --no-ff hotfix/null-check -m "Merge branch 'hotfix/null-check'"

# Tag the release
git tag -a v1.1.0 -m "Release v1.1.0" HEAD

# Merge feature/auth to main
GIT_AUTHOR_DATE="2024-01-15T10:00:00" GIT_COMMITTER_DATE="2024-01-15T10:00:00" \
git merge -q --no-ff feature/auth -m "Merge branch 'feature/auth'"

# One more commit on main
echo "v1.2.0" > VERSION
git add -A
commit "chore: bump version to 1.2.0" "2024-01-15T16:00:00"

# develop branch for ongoing work
git checkout -q -b develop
echo "new feature" > wip.rs
git add -A
commit "wip: start new payment feature" "2024-01-16T10:00:00"

git checkout -q main
