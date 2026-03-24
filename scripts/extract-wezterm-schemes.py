#!/usr/bin/env python3
"""Extract wezterm color schemes from scheme_data.rs into individual TOML files."""
import re
import os
import sys

def sanitize_filename(name):
    """Convert a scheme name to a safe filename."""
    # Replace characters that aren't alphanumeric, dash, or underscore
    s = name.lower()
    s = re.sub(r'[^a-z0-9_\-]', '-', s)
    s = re.sub(r'-+', '-', s)
    s = s.strip('-')
    return s

def main():
    src = sys.argv[1]
    out_dir = sys.argv[2]

    with open(src, 'r') as f:
        content = f.read()

    # Match each ("name", "toml_content") tuple
    # The pattern: ("Name", "content"),
    pattern = re.compile(r'\("([^"]+)",\s*"((?:[^"\\]|\\.)*)"\)', re.DOTALL)

    os.makedirs(out_dir, exist_ok=True)
    count = 0

    for m in pattern.finditer(content):
        name = m.group(1)
        toml_escaped = m.group(2)

        # Unescape the Rust string literals
        toml_content = toml_escaped.replace('\\n', '\n').replace('\\t', '\t').replace('\\"', '"').replace('\\\\', '\\')

        filename = sanitize_filename(name) + '.toml'
        filepath = os.path.join(out_dir, filename)

        with open(filepath, 'w') as f:
            f.write(toml_content)

        count += 1

    print(f"Extracted {count} color schemes to {out_dir}")

if __name__ == '__main__':
    main()
