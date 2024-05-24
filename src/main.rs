#![feature(type_alias_impl_trait)]
#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
compile_error!("This crate is only supported on Linux, macOS, and Windows.");

use std::path::PathBuf;

use clap::{ArgGroup, Parser, Subcommand};
use tracing::{debug, info, Level};

use crate::{
    config::{Config, RendererOverride},
    renderer::{macchina::MacchinaRenderer, neofetch::NeofetchRenderer},
};

pub mod config;
pub mod probe;
pub mod renderer;

// TODO: Include 'libmacchina' version in version command
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[clap(group = ArgGroup::new("setting").multiple(false).required(false))]
#[clap(group = ArgGroup::new("renderer").multiple(false).required(false))]
struct Cli {
    /// Include verbose output or not.
    #[clap(long, global = true, default_value = "false")]
    verbose: bool,

    /// Path to a custom config file.
    #[clap(short, long, group = "setting")]
    config: Option<PathBuf>,
    /// Use all default presets.
    #[clap(long, group = "setting")]
    all: bool,

    /// Set to Neofetch renderer.
    #[clap(short, long, group = "renderer")]
    neofetch: bool,
    /// Set to Macchina renderer.
    #[clap(short, long, group = "renderer")]
    macchina: bool,

    // Command subcommands
    #[clap(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
#[clap(group = ArgGroup::new("preset").multiple(false).required(false))]
enum Command {
    /// Generate a new config file
    Generate(GenerateCommandArgs),
    /// Return default config file path
    ConfigPath,
}

#[derive(Parser, Debug)]
struct GenerateCommandArgs {
    /// Generate neofetch preset.
    #[clap(short, long, group = "preset")]
    neofetch: bool,
    /// Generate macchina preset.
    #[clap(short, long, group = "preset")]
    macchina: bool,

    /// Use all default presets.
    #[clap(long)]
    all: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse CLI arguments
    let args = Cli::parse();
    let verbose = args.verbose;

    // Initialize logger
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

    if let Some(command) = args.command {
        match command {
            Command::Generate(args) => {
                // Generate a new config file
                debug!("Generating new config file");
                let config_dir =
                    Config::get_config_dir().expect("Could not determine config directory");
                debug!("Default config dir: {:?}", config_dir);

                // Create config directory if it doesn't exist
                if !config_dir.exists() {
                    std::fs::create_dir_all(&config_dir)?;
                    println!("Config directory created successfully");
                }

                let config_path = config_dir.join(Config::CONFIG_FILE_NAME);
                debug!("Default config path: {:?}", config_path);

                if config_path.try_exists()? {
                    println!("Config file already exists, skipping generation");
                    return Ok(());
                }

                // Determine which preset to generate
                let default_config = if args.neofetch {
                    if args.all {
                        Config::default_neofetch_all()
                    } else {
                        Config::default_neofetch()
                    }
                } else if args.macchina {
                    if args.all {
                        Config::default_macchina_all()
                    } else {
                        Config::default_macchina()
                    }
                } else if args.all {
                    Config::default_all()
                } else {
                    Config::default()
                };

                default_config.to_file(&config_path)?;
                println!("Config file generated successfully");
                return Ok(());
            }
            Command::ConfigPath => {
                // Return default config file path
                debug!("Returning default config file path");
                let config_path = Config::get_config_dir()
                    .expect("Could not determine config directory")
                    .join(Config::CONFIG_FILE_NAME);
                println!("{}", config_path.display());
                return Ok(());
            }
        }
    }

    // Handle main command

    // Fetch config, otherwise use default
    // TODO: Fix so arguments only change the default renderer and not set the default config
    let config = if args.all {
        // Use default all presets
        if args.neofetch {
            debug!("Using neofetch all preset");
            Config::default_neofetch_all()
        } else if args.macchina {
            debug!("Using macchina all preset");
            Config::default_macchina_all()
        } else {
            debug!("Using default all preset");
            Config::default_all()
        }
    } else {
        // Determine config path
        let config_path = match args.config {
            Some(config_path) => {
                // Custom config file was provided
                debug!("Using custom config path: {:?}", config_path);
                config_path
            }
            None => {
                // Search at default config path
                let default_config_path = Config::get_config_dir()
                    .expect("Could not determine config directory")
                    .join(Config::CONFIG_FILE_NAME);
                debug!("Using default config path: {:?}", default_config_path);

                default_config_path
            }
        };

        // Verify config file exists
        match config_path.try_exists() {
            Ok(true) => {
                // Load config from file
                Config::from_file(
                    &config_path,
                    if args.neofetch {
                        debug!("Overriding neofetch renderer");
                        Some(RendererOverride::Neofetch)
                    } else if args.macchina {
                        debug!("Overriding macchina renderer");
                        Some(RendererOverride::Macchina)
                    } else {
                        debug!("Using config from default path");
                        None
                    },
                )?
            }
            Ok(false) => {
                debug!("No config dir found, using default config");
                Config::default()
            }
            Err(e) => {
                info!(
                    "Using default config. Error checking for config dir: {:?}",
                    e
                );
                Config::default()
            }
        }
    };

    debug!("Config: {:?}", config);

    match config {
        Config::Neofetch(neofetch_config) => {
            NeofetchRenderer::new(neofetch_config).draw()?;
        }
        Config::Macchina(macchina_config) => {
            MacchinaRenderer::new(macchina_config).draw()?;
        }
    };

    Ok(())
}
