//! Build-time version metadata surfaced by `purr -V` / `purr --version`.
//!
//! The `PURR_*` values referenced here are injected by `build.rs` via
//! `cargo:rustc-env`, so they're baked into the binary at compile time and
//! cost nothing to read at runtime.

/// Crate version, e.g. `0.1.0`.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Verbose, multi-line version string used as clap's `version` value so that
/// both `-V` and `--version` report the full build provenance.
///
/// Clap prefixes this with the command name, so the first line is just the
/// bare version number; the rendered output looks like:
/// ```text
/// purr 0.1.0
/// commit:  9ee5747 (dirty)
/// profile: release
/// target:  x86_64-unknown-linux-gnu
/// host:    x86_64-unknown-linux-gnu
/// built:   2026-06-14 17:30:00 UTC
/// rustc:   rustc 1.89.0 (stable)
/// ```
pub const LONG_VERSION: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    "\n",
    "commit:  ",
    env!("PURR_GIT_HASH"),
    "\n",
    "profile: ",
    env!("PURR_BUILD_PROFILE"),
    "\n",
    "target:  ",
    env!("PURR_BUILD_TARGET"),
    "\n",
    "host:    ",
    env!("PURR_BUILD_HOST"),
    "\n",
    "built:   ",
    env!("PURR_BUILD_TIMESTAMP"),
    "\n",
    "rustc:   ",
    env!("PURR_RUSTC_VERSION"),
);
