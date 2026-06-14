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

# Verify this branch's commits (vs origin/master) follow Conventional Commits.
# convco is invoked through mise so it resolves in git hooks regardless of shell activation.
commit-check:
    mise exec -- convco check origin/master..HEAD

# Lint a single commit message file against Conventional Commits (used by the commit-msg hook)
commit-msg-check file:
    mise exec -- convco check --from-stdin --strip < {{file}}
