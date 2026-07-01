//! Generates the static shell completions in `completions/` from the `clap` CLI
//! defined in `purr_lib::cli`.
//!
//! Like `examples/gen-man.rs`, the completions are rendered from the exact same
//! command that backs `purr --help`, so they can never drift from the CLI. The
//! results are committed under `completions/` and installed by the distribution
//! packages (deb/rpm/AUR/Alpine/Nix) into the shells' standard completion dirs.
//!
//! Run via `mise run completions` (or `cargo run --example gen-completions`);
//! `mise run completions-check` verifies they are up to date.

use std::fs;
use std::path::Path;

use clap::CommandFactory;
use clap_complete::{Shell, generate};
use purr_lib::cli::Cli;

/// The command/binary name. Must match `[[bin]] name` in Cargo.toml and the
/// `#[command(name = "purr")]` on the CLI, since completion scripts key off it.
const BIN_NAME: &str = "purr";

fn main() -> std::io::Result<()> {
    let out_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("completions");
    fs::create_dir_all(&out_dir)?;

    let mut cmd = Cli::command();

    // Conventional filenames per shell: bash sources `purr.bash`, zsh autoloads
    // `_purr` from an fpath dir, fish reads `purr.fish` from a completions dir.
    for (shell, file) in [
        (Shell::Bash, "purr.bash"),
        (Shell::Zsh, "_purr"),
        (Shell::Fish, "purr.fish"),
    ] {
        let mut buf: Vec<u8> = Vec::new();
        generate(shell, &mut cmd, BIN_NAME, &mut buf);
        let path = out_dir.join(file);
        fs::write(&path, buf)?;
        println!("wrote {}", path.display());
    }

    Ok(())
}
