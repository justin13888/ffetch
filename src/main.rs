#![feature(type_alias_impl_trait)]
#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
compile_error!("This crate is only supported on Linux, macOS, and Windows.");

use clap::Parser;
use tracing::{debug, Level};


use crate::{
    config::Config,
    probe::{general_readout, probe_metrics},
    renderer::neofetch::NeofetchRenderer,
};

pub mod colour;
pub mod config;
pub mod probe;
pub mod renderer;

/// TODO: Support adjusting configs via CLI.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Include verbose output or not.
    #[clap(long, default_value = "false")]
    verbose: bool,
    // TODO: Support "generate" subcommand
    // TODO: Allow "--neofetch, --pfetch" flags to override renderer
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse CLI arguments
    let args = Args::parse();
    let verbose = args.verbose;

    // TODO: Initialize tracing (logging)
    match verbose {
        true => {
            tracing_subscriber::fmt()
                .with_max_level(Level::TRACE)
                .init();
        }
        false => {
            tracing_subscriber::fmt::init();
        }
    }

    // Fetch config, otherwise use default
    let project_dirs = directories::ProjectDirs::from("net", "justin13888", "ffetch")
        .expect("Could not find project directories");
    let config_dir = project_dirs.config_dir();
    debug!("Config dir: {:?}", config_dir);
    let config_path = config_dir.join("config.toml");
    debug!("Config path: {:?}", config_path);

    let config = match config_path.try_exists() {
        Ok(true) => Config::from_file(&config_path)?,
        Ok(false) => {
            debug!("No config dir found, using default config");
            Config::default()
        }
        Err(e) => {
            debug!("Error checking for config dir: {:?}", e);
            Config::default()
        }
    };
    debug!("Config: {:?}", config);

    // TODO: Read config and determine render output
    let probe_list = config
        .probes
        .into_iter()
        .map(|p| probe_metrics(&p))
        .collect::<Vec<_>>();
    match config.renderer {
        config::RendererConfig::Neofetch => {
            NeofetchRenderer::new().draw(&config.neofetch, &probe_list)?;
        }
    };

    use libmacchina::traits::GeneralReadout as _;
    println!(
        "{}",
        general_readout()
            .distribution()
            .unwrap_or("N/A".to_string())
    );

    Ok(())
}
