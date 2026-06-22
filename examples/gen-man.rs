//! Generates `man/purr.1` from the `clap` CLI defined in `purr_lib::cli`.
//!
//! The man page is rendered from the exact same command that backs
//! `purr --help`, so the two can never disagree. Hand-written EXAMPLES,
//! ENVIRONMENT, and FILES sections are appended for the things clap can't know
//! about (the config file, `NO_COLOR`, `RUST_LOG`).
//!
//! Run via `mise run man` (or `cargo run --example gen-man`); the result is
//! committed at `man/purr.1`. `mise run man-check` verifies it is up to date.

use std::fs;
use std::path::Path;

use clap::CommandFactory;
use clap_mangen::Man;
use purr_lib::cli::Cli;

/// Extra DESCRIPTION prose. clap only knows the one-line `about`, so spell out
/// what purr does and how configuration is resolved. Injected into the man page
/// only — `purr --help` is left untouched.
const DESCRIPTION_EXTRA: &str = r#".PP
purr is a fast, neofetch\-compatible system information tool. With no arguments
it prints a neofetch\-style report \(em the same fields, styling, and
\fB${c1}\fR..\fB${c6}\fR ASCII logo format \(em then exits. Probes run in
parallel, so a typical run completes in a few milliseconds.
.PP
Output is controlled by a TOML configuration file and the options below;
precedence is defaults, then the config file, then command\-line flags. Use
\fB\-\-json\fR for machine\-readable output.
"#;

/// Sections clap can't derive from the CLI: usage examples, the runtime
/// environment variables purr honours, and the config file it reads.
const EXTRA_SECTIONS: &str = r#".SH EXAMPLES
.TP
.B purr
Print the default neofetch\-style report.
.TP
.B purr \-\-all
Show every available probe.
.TP
.B purr \-\-json
Emit the probe results as JSON for scripting.
.TP
.B purr \-L
Print only the logo, with no system information.
.TP
.B purr \-\-ascii_distro arch \-\-ascii_colors \(dq4 6 1\(dq
Force the Arch logo and recolour it.
.TP
.B purr generate \-\-neofetch
Write a starter neofetch\-style config file.
.SH ENVIRONMENT
.TP
.B NO_COLOR
When set to a non\-empty value, disables all colour output
(https://no\-color.org). The \fB\-\-stdout\fR flag sets this automatically.
.TP
.B RUST_LOG
Controls log verbosity, e.g. \fBRUST_LOG=debug\fR. Passing \fB\-\-verbose\fR
raises the level to trace.
.SH FILES
.TP
.I config.toml
Configuration file, read from the platform configuration directory
(precedence: defaults < config file < CLI flags). Run \fBpurr config\-path\fR
to print its resolved path, or \fBpurr generate\fR to write a starter file.
"#;

fn main() -> std::io::Result<()> {
    // Use the short crate version (not the verbose build-provenance string that
    // `purr -V` prints) so the committed page stays deterministic across builds.
    let cmd = Cli::command()
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"));
    let man = Man::new(cmd);

    let mut buf: Vec<u8> = Vec::new();
    man.render_title(&mut buf)?;
    man.render_name_section(&mut buf)?;
    man.render_synopsis_section(&mut buf)?;
    man.render_description_section(&mut buf)?;
    buf.extend_from_slice(DESCRIPTION_EXTRA.as_bytes());
    man.render_options_section(&mut buf)?;
    man.render_subcommands_section(&mut buf)?;
    buf.extend_from_slice(EXTRA_SECTIONS.as_bytes());
    man.render_version_section(&mut buf)?;
    man.render_authors_section(&mut buf)?;

    let out_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("man");
    fs::create_dir_all(&out_dir)?;
    let out_path = out_dir.join("purr.1");
    fs::write(&out_path, buf)?;
    println!("wrote {}", out_path.display());

    Ok(())
}
