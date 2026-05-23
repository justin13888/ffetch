default:
    @just --list

format:
    cargo fmt --check

format-fix:
    cargo fmt

lint:
    cargo clippy

# clippy --fix exits 0 even on failure; re-run clippy to surface errors
lint-fix:
    cargo clippy --fix --allow-dirty --allow-staged
    cargo clippy
