#![feature(type_alias_impl_trait)]
#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
compile_error!("This crate is only supported on Linux, macOS, and Windows.");

use clap::Parser;
use tracing::debug;

use crate::{config::Config, probe::probe_metrics, renderer::neofetch::NeofetchRenderer};

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
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // use crate::probe::{
    //     general_readout, kernel_readout, memory_readout, network_readout, package_readout,
    //     product_readout,
    // };
    // use libmacchina::traits::BatteryReadout as _;
    // use libmacchina::traits::GeneralReadout as _;
    // use libmacchina::traits::KernelReadout as _;
    // use libmacchina::traits::MemoryReadout as _;
    // use libmacchina::traits::NetworkReadout as _;
    // use libmacchina::traits::PackageReadout as _;
    // use libmacchina::traits::ProductReadout as _;

    // let battery_readout = battery_readout();
    // println!(
    //     "Battery percentage: {}",
    //     battery_readout
    //         .percentage()
    //         .map(|n| n.to_string())
    //         .unwrap_or("N/A".to_string())
    // );
    // println!(
    //     "Battery status: {}",
    //     battery_readout
    //         .status()
    //         .map(|n| n.to_string())
    //         .unwrap_or("N/A".to_string())
    // );
    // println!(
    //     "Battery health: {}",
    //     battery_readout
    //         .health()
    //         .map(|n| n.to_string())
    //         .unwrap_or("N/A".to_string())
    // );
    // let kernel_readout = kernel_readout();
    // println!(
    //     "OS Release: {}",
    //     kernel_readout
    //         .os_release()
    //         .map(|n| n.to_string())
    //         .unwrap_or("N/A".to_string())
    // );
    // println!(
    //     "OS type: {}",
    //     kernel_readout
    //         .os_type()
    //         .map(|n| n.to_string())
    //         .unwrap_or("N/A".to_string())
    // );
    // println!(
    //     "Pretty kernel: {}",
    //     kernel_readout
    //         .pretty_kernel()
    //         .map(|n| n.to_string())
    //         .unwrap_or("N/A".to_string())
    // );
    // let memory_readout = memory_readout();
    // println!(
    //     "Total memory: {} KiB",
    //     memory_readout
    //         .total()
    //         .map(|n| n.to_string())
    //         .unwrap_or("N/A".to_string())
    // );
    // println!(
    //     "Free memory: {} KiB",
    //     memory_readout
    //         .free()
    //         .map(|n| n.to_string())
    //         .unwrap_or("N/A".to_string())
    // );
    // println!(
    //     "Used memory: {} KiB",
    //     memory_readout
    //         .used()
    //         .map(|n| n.to_string())
    //         .unwrap_or("N/A".to_string())
    // );
    // println!(
    //     "Buffer memory: {} KiB",
    //     memory_readout
    //         .buffers()
    //         .map(|n| n.to_string())
    //         .unwrap_or("N/A".to_string())
    // );
    // println!(
    //     "Cached memory: {} KiB",
    //     memory_readout
    //         .cached()
    //         .map(|n| n.to_string())
    //         .unwrap_or("N/A".to_string())
    // );
    // println!(
    //     "Reclaimable memory: {} KiB",
    //     memory_readout
    //         .reclaimable()
    //         .map(|n| n.to_string())
    //         .unwrap_or("N/A".to_string())
    // );
    // println!(
    //     "Used memory: {} KiB",
    //     memory_readout
    //         .used()
    //         .map(|n| n.to_string())
    //         .unwrap_or("N/A".to_string())
    // );
    // let general_readout = general_readout();
    // let product_readout = product_readout();
    // println!("Product:");
    // let package_readout = package_readout();
    // println!("Packages:");
    // package_readout.count_pkgs().iter().for_each(|(k, v)| {
    //     println!("\t{}: {}", k.to_string(), v);
    // });

    // let network_readout = network_readout();
    // // TODO: Implement openwrt specific code?

    // // // There are many more metrics we can query
    // // // i.e. username, distribution, terminal, shell, etc.

    // // let cpu_cores = general_readout.cpu_cores(); // 8 [logical cores]
    // // let cpu = general_readout.cpu_model_name(); // Intel(R) Core(TM) i5-8265U CPU @ 1.60GHz
    // // let uptime = general_readout.uptime(); // 1500 [in seconds]

    // // let memory_readout = MemoryReadout::new();

    // // let total_mem = memory_readout.total(); // 20242204 [in kB]
    // // let used_mem = memory_readout.used(); // 3894880 [in kB]

    // // // Let's print out the information we've gathered.
    // // println!(
    // //     "CPU Cores: {}",
    // //     cpu_cores
    // //         .map(|n| n.to_string())
    // //         .unwrap_or("N/A".to_string())
    // // );
    // // println!(
    // //     "CPU Model: {}",
    // //     cpu.map(|n| n.to_string()).unwrap_or("N/A".to_string())
    // // );
    // // println!(
    // //     "Uptime: {} seconds",
    // //     uptime.map(|n| n.to_string()).unwrap_or("N/A".to_string())
    // // );
    // // println!(
    // //     "Total Memory: {} kB",
    // //     total_mem
    // //         .map(|n| n.to_string())
    // //         .unwrap_or("N/A".to_string())
    // // );
    // // println!(
    // //     "Used Memory: {} kB",
    // //     used_mem.map(|n| n.to_string()).unwrap_or("N/A".to_string())
    // // );

    // Parse CLI arguments
    let args = Args::parse();
    // let verbose = args.verbose;

    // // TODO: Initialize tracing (logging)
    // match verbose {
    //     true => {
    //         tracing_subscriber::fmt::init();
    //     }
    //     false => {
    //         tracing_subscriber::fmt::init();
    //     }
    // }

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
    let probe_list = probe_metrics(&config.probe);
    match config.renderer {
        config::RendererConfig::Neofetch => {
            NeofetchRenderer::new().draw(&config.neofetch, &probe_list)?;
        }
    };

    Ok(())
}
