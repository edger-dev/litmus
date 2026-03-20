beans-serve:
    beans-serve --cors-origin "*"

dev:
    cd crates/litmus-web && dx serve

build-web:
    cd crates/litmus-web && dx build --release

build-cli:
    cargo build --release --package litmus-cli

check:
    cargo check --workspace

fmt:
    cargo fmt --all
