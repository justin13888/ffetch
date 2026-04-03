#!/usr/bin/env bash
# Compare ffetch against neofetch, macchina, and fastfetch using hyperfine.
#
# Requirements:
#   - hyperfine (https://github.com/sharkdp/hyperfine)
#   - One or more of: neofetch, macchina, fastfetch (at least one must be installed)
#
# Usage:
#   bash scripts/bench-compare.sh
#   bash scripts/bench-compare.sh --cold   # also run cold-cache benchmark (requires sudo)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# --- Prerequisites ---
if ! command -v hyperfine &>/dev/null; then
    echo "Error: hyperfine is not installed." >&2
    echo "  Install: cargo install hyperfine" >&2
    echo "  Or: https://github.com/sharkdp/hyperfine/releases" >&2
    exit 1
fi

# --- Build ffetch in release mode ---
echo "Building ffetch (release)..."
cargo build --release --manifest-path "$REPO_ROOT/Cargo.toml"
FFETCH="$REPO_ROOT/target/release/ffetch"

# --- Collect available tools ---
declare -a CMDS=()
declare -a LABELS=()

CMDS+=("$FFETCH --all")
LABELS+=("ffetch")

if command -v fastfetch &>/dev/null; then
    CMDS+=("fastfetch")
    LABELS+=("fastfetch")
fi

if command -v macchina &>/dev/null; then
    CMDS+=("macchina")
    LABELS+=("macchina")
fi

if command -v neofetch &>/dev/null; then
    CMDS+=("neofetch")
    LABELS+=("neofetch")
fi

if [ "${#CMDS[@]}" -lt 2 ]; then
    echo "Warning: no competitors found (neofetch, macchina, fastfetch)."
    echo "Running ffetch standalone benchmark only."
fi

echo ""
echo "Benchmarking: ${LABELS[*]}"
echo ""

# Build hyperfine command arrays
HYPERFINE_ARGS=()
for i in "${!CMDS[@]}"; do
    HYPERFINE_ARGS+=("--command-name" "${LABELS[$i]}" "${CMDS[$i]}")
done

# --- Warm benchmark ---
echo "=== Warm benchmark (warmup=3, min-runs=10) ==="
hyperfine \
    --warmup 3 \
    --min-runs 10 \
    --shell=none \
    --export-json "$REPO_ROOT/bench-results.json" \
    --export-markdown "$REPO_ROOT/bench-results.md" \
    "${HYPERFINE_ARGS[@]}"

echo ""
echo "Results saved to bench-results.json and bench-results.md"

# --- Cold-cache benchmark (optional, requires sudo) ---
if [[ "${1:-}" == "--cold" ]]; then
    echo ""
    echo "=== Cold-cache benchmark (min-runs=5) ==="
    if ! sudo -n true 2>/dev/null; then
        echo "Sudo access required for dropping caches. Skipping cold benchmark."
    else
        hyperfine \
            --warmup 0 \
            --min-runs 5 \
            --shell=none \
            --prepare 'sync; echo 3 | sudo tee /proc/sys/vm/drop_caches > /dev/null' \
            --export-json "$REPO_ROOT/bench-cold-results.json" \
            --export-markdown "$REPO_ROOT/bench-cold-results.md" \
            "${HYPERFINE_ARGS[@]}"
        echo ""
        echo "Cold results saved to bench-cold-results.json and bench-cold-results.md"
    fi
fi
