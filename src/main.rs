#![feature(type_alias_impl_trait)]
#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
compile_error!("This crate is only supported on Linux, macOS, and Windows.");

use std::path::PathBuf;

use clap::{ArgGroup, Parser};
use tracing::{debug, Level};
use tracing_subscriber::field::debug;

use crate::{
    config::{Config, RendererConfig},
    renderer::{macchina::MacchinaRenderer, neofetch::NeofetchRenderer},
};

pub mod colour;
pub mod config;
pub mod probe;
pub mod renderer;

// TODO: Support adjusting configs via CLI.
// TODO: Include 'libmacchina' version in version command
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[clap(group = ArgGroup::new("preset").multiple(false).required(true))]
pub struct Args {
    /// Include verbose output or not.
    #[clap(long, default_value = "false")]
    verbose: bool,

    #[clap(short, long, group = "preset")]
    config: Option<PathBuf>,
    #[clap(short, long, group = "preset")]
    neofetch: bool,
    #[clap(short, long, group = "preset")]
    macchina: bool,
    // TODO: Support "generate" subcommand
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
    debug!("Args: {:?}", args);

    // Fetch config, otherwise use default
    let config = if let Some(config_path) = args.config {
        // Custom config file was provided
        debug!("Using custom config: {:?}", config_path);
        Config::from_file(&config_path)?
    } else if args.neofetch {
        // Neofetch preset
        debug!("Using neofetch preset");
        Config::default_neofetch()
    } else if args.macchina {
        // Macchina preset
        debug!("Using macchina preset");
        Config::default_macchina()
    } else {
        // No custom config file was provided
        debug!("Searching default config");
        let project_dirs = directories::ProjectDirs::from("net", "justin13888", "ffetch")
            .expect("BUG: Could not find project directories");
        let config_dir = project_dirs.config_dir();
        debug!("Default config dir: {:?}", config_dir);
        let config_path = config_dir.join("config.toml");
        debug!("Default config path: {:?}", config_path);

        match config_path.try_exists() {
            Ok(true) => Config::from_file(&config_path)?,
            Ok(false) => {
                debug!("No config dir found, using default config");
                Config::default()
            }
            Err(e) => {
                debug!("Error checking for config dir: {:?}", e);
                Config::default()
            }
        }
    };

    debug!("Config: {:?}", config);

    // TODO: Read config and determine render output
    let probe_list = config
        .probes
        .into_iter()
        .map(|p| p.get_funcs())
        .collect::<Vec<_>>();
    match config.renderer {
        RendererConfig::Neofetch => {
            NeofetchRenderer::new(config.neofetch).draw(&probe_list)?;
        }
        RendererConfig::Macchina => {
            MacchinaRenderer::new(config.macchina).draw(&probe_list)?;
        }
    };

    Ok(())
}
