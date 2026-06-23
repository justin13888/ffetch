#!/usr/bin/env bash
# Regenerate assets/purr.svg — an SVG screenshot of purr's default output,
# themed Catppuccin Macchiato in JetBrains Mono.
#
# Idempotent: re-runs overwrite assets/purr.svg in place. The rendered text
# differs run-to-run because purr reports live system info (uptime, memory, …).
#
# The screenshot is meant to be shareable (e.g. r/UnixPORN), so the title line
# (`username@hostname`, read live from the host) is redacted to a neutral, on-
# theme handle — see DEMO_USER/DEMO_HOST below — keeping the maintainer's real
# identity out of the asset.
#
# Requirements:
#   - freeze (https://github.com/charmbracelet/freeze)
#       Provisioned by `mise install` (declared in mise.toml). To install it
#       manually instead, see https://github.com/charmbracelet/freeze#installation
#
# Usage:
#   mise run svg
#   bash scripts/gen-svg.sh
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# --- Prerequisites ---
if ! command -v freeze &>/dev/null; then
    echo "Error: freeze is not installed." >&2
    echo "  Install: mise install" >&2
    echo "  Or: https://github.com/charmbracelet/freeze#installation" >&2
    exit 1
fi

# --- Build purr in release mode so the screenshot reflects current output ---
echo "Building purr (release)..."
cargo build --release --manifest-path "$REPO_ROOT/Cargo.toml"
PURR="$REPO_ROOT/target/release/purr"

mkdir -p "$REPO_ROOT/assets"
OUT="$REPO_ROOT/assets/purr.svg"
TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT
RAW="$TMP_DIR/purr.svg"

# --- Identity shown in the screenshot ---
# The title is `username@hostname`, read live from the host. For a public
# screenshot we swap in a neutral, on-theme handle: `rice` is r/UnixPORN slang,
# `macchiato` matches the Catppuccin theme — no real identity, no hint of purr.
# Edit freely.
DEMO_USER="rice"
DEMO_HOST="macchiato"

# --- Capture purr's colored output ---
# Pipe purr (not `freeze --execute`) so purr uses its plain, newline-based
# layout: under a PTY it positions text with cursor-movement escapes that
# freeze's ANSI parser can't replay, collapsing everything onto huge lines.
# Piped, purr still emits ANSI colour (colour is gated on NO_COLOR, which we
# clear here, not on whether stdout is a TTY).
#
# `-c demo-config.toml` is the stock neofetch preset minus the Shell/Terminal/
# Terminal Font probes — through `mise run svg` the parent process is always
# bash/mise, so those fields would misreport. See scripts/demo-config.toml.
RAW_TXT="$TMP_DIR/purr.txt"
env -u NO_COLOR "$PURR" -c "$SCRIPT_DIR/demo-config.toml" > "$RAW_TXT"

# --- Redact the real username@hostname before freeze lays out the SVG ---
# Substitute in the plain text stream (not the SVG) so freeze positions every
# glyph and the title underline stays aligned. demo-config.toml puts the title
# on line 1 and its underline on line 2. Pull the real `user@host` off line 1
# (strip ANSI, take the trailing token) so we know exactly what to replace, then
# swap in DEMO_USER/DEMO_HOST and resize the underline (a run of `-`, per
# demo-config's underline_char) to the new title length. Assumes the real
# username/hostname are plain alphanumerics (true on the maintainer's host).
real_pair="$(sed -n '1{s/\x1b\[[0-9;]*m//g;p;q}' "$RAW_TXT" | awk '{print $NF}')"
real_user="${real_pair%@*}"
real_host="${real_pair##*@}"
if [[ "$real_pair" != *@* || -z "$real_user" || -z "$real_host" ]]; then
    echo "Error: could not parse username@hostname from purr output to redact it." >&2
    exit 1
fi

echo "Rendering $OUT (as $DEMO_USER@$DEMO_HOST)..."
awk -v ru="$real_user" -v rh="$real_host" -v fu="$DEMO_USER" -v fh="$DEMO_HOST" '
    NR == 1 { gsub(rh, fh); gsub(ru, fu); print; next }
    NR == 2 {
        n = length(fu) + length(fh) + 1
        d = ""
        while (length(d) < n) d = d "-"
        sub(/-+/, d)
        print
        next
    }
    { print }
' "$RAW_TXT" | freeze \
    --output "$RAW" \
    --theme catppuccin-macchiato \
    --font.family "JetBrains Mono" \
    --window \
    --border.radius 8 \
    --padding 20 \
    --margin 20

# --- Recolour to the Catppuccin Macchiato ANSI palette ---
# freeze's `--theme` only sets the background; it renders captured ANSI with a
# fixed VGA palette (pure #0000ff blue, etc.) that is harsh and low-contrast on
# a dark background. Remap those 16 palette colours (plus freeze's default
# foreground #C5C8C6) to Catppuccin Macchiato. The window-control and border
# chrome use distinct hexes and are left untouched. Tied to the palette emitted
# by the freeze version pinned in mise.toml.
sed -E '
    s/#000000/#494d64/Ig
    s/#800000/#ed8796/Ig
    s/#008000/#a6da95/Ig
    s/#808000/#eed49f/Ig
    s/#000080/#8aadf4/Ig
    s/#800080/#c6a0f6/Ig
    s/#008080/#8bd5ca/Ig
    s/#c0c0c0/#b8c0e0/Ig
    s/#808080/#5b6078/Ig
    s/#ff0000/#ed8796/Ig
    s/#00ff00/#a6da95/Ig
    s/#ffff00/#eed49f/Ig
    s/#0000ff/#8aadf4/Ig
    s/#ff00ff/#c6a0f6/Ig
    s/#00ffff/#8bd5ca/Ig
    s/#ffffff/#a5adcb/Ig
    s/#c5c8c6/#cad3f5/Ig
' "$RAW" > "$OUT"

echo "Wrote $OUT"
