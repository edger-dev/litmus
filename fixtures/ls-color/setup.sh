#!/usr/bin/env bash
# Setup: create a directory tree with a variety of file types for ls to color.
set -euo pipefail

cd "$FIXTURE_WORK_DIR"

# Regular files
echo "# Project configuration" > README.md
echo "fn main() {}" > main.rs
echo "body { color: red; }" > style.css
echo '{"name": "demo"}' > package.json
echo "TODO: write tests" > TODO.txt

# Hidden files
echo "EDITOR=vim" > .env
echo "[core]" > .gitconfig

# Executable
cat > run.sh << 'SH'
#!/usr/bin/env bash
echo "Running..."
SH
chmod +x run.sh

# Subdirectories
mkdir -p src tests docs build

# Symlinks
ln -s src/  source_link
ln -s README.md readme_link
ln -s /nonexistent broken_link

# Files with various extensions
touch archive.tar.gz
touch image.png
touch data.csv
touch Makefile
