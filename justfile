default:
    @just --list

# Format code in place
fmt:
    cargo fmt

# Verify formatting without modifying files
fmt-check:
    cargo fmt --check

# clippy --fix exits 0 even on failure, so re-run a strict check afterwards
# Auto-fix clippy lints, then verify nothing remains
lint:
    cargo clippy --fix --allow-dirty --allow-staged
    just lint-check

# Verify clippy lints without modifying files (CI-grade)
lint-check:
    cargo clippy -- -D warnings

# Run the test suite
test:
    cargo test
