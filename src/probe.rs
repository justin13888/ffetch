use std::{
    fmt::{self, Display, Formatter},
    sync::OnceLock,
};

use libmacchina::{
    traits::{BatteryState, ReadoutError, ShellFormat, ShellKind},
    BatteryReadout, GeneralReadout, KernelReadout, MemoryReadout, NetworkReadout, PackageReadout,
    ProductReadout,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;



pub fn battery_readout() -> &'static BatteryReadout {
    use libmacchina::traits::BatteryReadout as _;
    static COMPUTATION: OnceLock<BatteryReadout> = OnceLock::new();
    COMPUTATION.get_or_init(BatteryReadout::new)
}

pub fn kernel_readout() -> &'static KernelReadout {
    use libmacchina::traits::KernelReadout as _;
    static COMPUTATION: OnceLock<KernelReadout> = OnceLock::new();
    COMPUTATION.get_or_init(KernelReadout::new)
}

pub fn memory_readout() -> &'static MemoryReadout {
    use libmacchina::traits::MemoryReadout as _;
    static COMPUTATION: OnceLock<MemoryReadout> = OnceLock::new();
    COMPUTATION.get_or_init(MemoryReadout::new)
}

pub fn general_readout() -> &'static GeneralReadout {
    use libmacchina::traits::GeneralReadout as _;
    static COMPUTATION: OnceLock<GeneralReadout> = OnceLock::new();
    COMPUTATION.get_or_init(GeneralReadout::new)
}

pub fn product_readout() -> &'static ProductReadout {
    use libmacchina::traits::ProductReadout as _;
    static COMPUTATION: OnceLock<ProductReadout> = OnceLock::new();
    COMPUTATION.get_or_init(ProductReadout::new)
}

pub fn package_readout() -> &'static PackageReadout {
    use libmacchina::traits::PackageReadout as _;
    static COMPUTATION: OnceLock<PackageReadout> = OnceLock::new();
    COMPUTATION.get_or_init(PackageReadout::new)
}

pub fn network_readout() -> &'static NetworkReadout {
    use libmacchina::traits::NetworkReadout as _;
    static COMPUTATION: OnceLock<NetworkReadout> = OnceLock::new();
    COMPUTATION.get_or_init(NetworkReadout::new)
}

// TODO: Complete the rest of doc comments for this enum vv
pub enum ProbeValue {
    /// Hostname (username@hostname)
    /// e.g. ("justin13888", "ffetch")
    Host(String, String),
    /// e.g. "Ubuntu 22.04.4 LTS (Jammy Jellyfish)"
    OS(String),
    /// OS Distribution
    /// E.g.
    Distro(String),
    /// Model
    // (Vendor, Product)
    /// e.g. ("Dell Inc.", "XPS 15 9510")
    Model(String, String),
    /// e.g. "6.8.4-cachyos"
    Kernel(String),
    /// Uptime in seconds
    /// E.g. 123
    Uptime(usize),
    /// Number of packages installed
    /// Vec<(package manager, count)>
    /// E.g. [("dpkg", 123)]
    Packages(Vec<(String, usize)>), // TODO: CHECK
    /// E.g. "zsh 5.8.1"
    Shell(String),
    /// E.g. "vim 8.2" // TODO: CHECK THIS example
    Editor(String),
    /// E.g. "1920x1080"
    Resolution(String),
    /// E.g. "GNOME", "hyprland", "Fluent" (Windows)
    DE(String),
    /// E.g. "Mutter"
    WM(String),
    /// E.g. "Adwaita" // TODO: CHECK
    WMTheme(String),
    /// E.g. "Adwaita-dark" // TODO: CHECK
    Theme(String),
    Icons(String),
    Cursor(String),
    Terminal(String),
    TerminalFont(String),
    /// E.g. "Intel Core i7-11800H"
    CPU(String),
    /// E.g. "NVIDIA GeForce RTX 4090", "Intel(R) UHD Graphics"
    GPU(String),
    /// Amount of memory (in MiB)
    /// (used, total)
    /// E.g. (46863, 64290)
    Memory(u64, u64),
    Network(String),
    Bluetooth(String),
    BIOS(String),
    /// E.g. "bochs-drm"
    GPUDriver(String),
    /// CPU usage percentage
    /// E.g. 12
    CPUUsage(usize),
    /// Disk usage (in bytes)
    /// (used, total)
    /// E.g.
    Disk(u64, u64), // TODO: CHECK
    /// Battery percentage
    /// E.g. 86
    Battery(u8), // TODO: CHECK
    PowerAdapter(String), // TODO: CHECK
    Font(String),
    Song(String),
    LocalIP(String),  // TODO: CHECK
    PublicIP(String), // TODO: CHECK
    Users(usize),     // TODO: CHECK
    /// E.g. "en_US.UTF-8"
    Locale(String),
    /// Java version
    /// E.g. "OpenJDK 11.0.12"
    Java(String),
    /// Python version
    /// E.g. "Python 3.9.7"
    Python(String),
    /// NodeJS version
    /// E.g. "20.9.0"
    Node(String),
    /// Rust version
    /// E.g. "rustc 1.57.0"
    Rust(String),
}

#[derive(Error, Debug)]
pub enum ProbeError {
    /// Metric is unavailable on this platform
    /// e.g. "Battery percentage"
    MetricsUnavailable,
    /// Metric is unimplemented yet
    Unimplemented,
    /// Metric readout failed possibly because of missing dependencies or some criteria
    Other(String),
    /// Metric readout might be erroneous
    Warning(String),
}

impl Display for ProbeError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ProbeError::MetricsUnavailable => write!(f, "Metrics unavailable"),
            ProbeError::Unimplemented => write!(f, "Unimplemented"),
            ProbeError::Other(s) => write!(f, "{}", s),
            ProbeError::Warning(s) => write!(f, "{}", s),
        }
    }
}

impl From<ReadoutError> for ProbeError {
    fn from(err: ReadoutError) -> Self {
        match err {
            ReadoutError::MetricNotAvailable => ProbeError::MetricsUnavailable,
            ReadoutError::NotImplemented => ProbeError::Unimplemented,
            ReadoutError::Other(s) => ProbeError::Other(s),
            ReadoutError::Warning(s) => ProbeError::Warning(s),
        }
    }
}

impl From<ProbeType> for ProbeResultFunction {
    fn from(probe_type: ProbeType) -> Self {
        use libmacchina::traits::BatteryReadout as _;
        use libmacchina::traits::GeneralReadout as _;
        use libmacchina::traits::KernelReadout as _;
        use libmacchina::traits::MemoryReadout as _;
        use libmacchina::traits::NetworkReadout as _;
        use libmacchina::traits::PackageReadout as _;
        use libmacchina::traits::ProductReadout as _;

        match probe_type {
            ProbeType::Host => Box::new(|| {
                Ok(ProbeResultValue::Single(ProbeValue::Host(
                    general_readout().username()?,
                    general_readout().hostname()?,
                )))
            }),
            ProbeType::OS => Box::new(|| {
                Ok(ProbeResultValue::Single(ProbeValue::OS(
                    general_readout().os_name()?,
                )))
            }),
            ProbeType::Distro => Box::new(|| {
                Ok(ProbeResultValue::Single(ProbeValue::Distro(
                    general_readout().distribution()?,
                )))
            }),
            ProbeType::Model => Box::new(|| {
                Ok(ProbeResultValue::Single(ProbeValue::Model(
                    product_readout().vendor()?,
                    product_readout().product()?,
                )))
            }),
            ProbeType::Kernel => Box::new(|| {
                Ok(ProbeResultValue::Single(ProbeValue::Kernel(
                    kernel_readout().os_release()?,
                )))
            }),
            ProbeType::Uptime => Box::new(|| {
                Ok(ProbeResultValue::Single(ProbeValue::Uptime(
                    general_readout().uptime()?,
                )))
            }),
            // TODO: Test libmacchina packages() function for package manager hanging issues
            ProbeType::Packages => Box::new(|| {
                Ok(ProbeResultValue::Single(ProbeValue::Packages(
                    package_readout()
                        .count_pkgs()
                        .into_iter()
                        .map(|(name, count)| (name.to_string(), count))
                        .collect::<Vec<_>>(),
                )))
            }),
            ProbeType::Shell => Box::new(|| {
                Ok(ProbeResultValue::Single(ProbeValue::Shell(
                    general_readout()
                        .shell(ShellFormat::Relative, ShellKind::Current)?
                        .trim()
                        .to_string(),
                )))
            }),
            ProbeType::Editor => Box::new(|| Err(ProbeError::Unimplemented)), // TODO
            ProbeType::Resolution => Box::new(|| {
                Ok(ProbeResultValue::Single(ProbeValue::Resolution(
                    general_readout().resolution()?,
                )))
            }),
            ProbeType::DE => Box::new(|| {
                Ok(ProbeResultValue::Single(ProbeValue::DE(
                    general_readout().desktop_environment()?,
                )))
            }),
            ProbeType::WM => Box::new(|| {
                Ok(ProbeResultValue::Single(ProbeValue::WM(
                    general_readout().window_manager()?,
                )))
            }),
            ProbeType::WMTheme => Box::new(|| {
                Ok(ProbeResultValue::Single(ProbeValue::WMTheme(
                    "".to_string(), // TODO
                )))
            }),

            ProbeType::Theme => {
                Box::new(|| Ok(ProbeResultValue::Single(ProbeValue::Theme("".to_string()))))
            } // TODO
            ProbeType::Icons => {
                Box::new(|| Ok(ProbeResultValue::Single(ProbeValue::Icons("".to_string()))))
            } // TODO
            ProbeType::Cursor => {
                Box::new(|| Ok(ProbeResultValue::Single(ProbeValue::Cursor("".to_string()))))
            } // TODO
            ProbeType::Terminal => Box::new(|| {
                Ok(ProbeResultValue::Single(ProbeValue::Terminal(
                    general_readout().terminal()?.trim().to_string(),
                )))
            }),
            ProbeType::TerminalFont => Box::new(|| {
                Ok(ProbeResultValue::Single(ProbeValue::TerminalFont(
                    "".to_string(), // TODO
                )))
            }),
            ProbeType::CPU => Box::new(|| {
                Ok(ProbeResultValue::Single(ProbeValue::CPU(
                    general_readout().cpu_model_name()?,
                )))
            }),
            ProbeType::GPU => Box::new(|| {
                Ok(ProbeResultValue::Multiple(
                    general_readout()
                        .gpus()?
                        .into_iter()
                        .map(ProbeValue::GPU)
                        .collect::<Vec<_>>(),
                ))
            }),
            ProbeType::Memory => Box::new(|| {
                Ok(ProbeResultValue::Single(ProbeValue::Memory(
                    memory_readout().used()?,
                    memory_readout().total()?,
                )))
            }),
            ProbeType::Network => Box::new(|| {
                Ok(ProbeResultValue::Single(ProbeValue::Network(
                    "".to_string(), // TODO
                )))
            }),
            ProbeType::Bluetooth => Box::new(|| {
                Ok(ProbeResultValue::Single(ProbeValue::Bluetooth(
                    "".to_string(), // TODO
                )))
            }),
            ProbeType::BIOS => {
                Box::new(|| Ok(ProbeResultValue::Single(ProbeValue::BIOS("".to_string()))))
            }
            ProbeType::GPUDriver => Box::new(|| {
                Ok(ProbeResultValue::Single(ProbeValue::GPUDriver(
                    "".to_string(), // TODO
                )))
            }),
            ProbeType::CPUUsage => Box::new(|| {
                Ok(ProbeResultValue::Single(ProbeValue::CPUUsage(
                    general_readout().cpu_usage()?,
                )))
            }),
            ProbeType::Disk => Box::new(|| {
                let disk_readout = general_readout().disk_space()?;
                Ok(ProbeResultValue::Single(ProbeValue::Disk(
                    disk_readout.0,
                    disk_readout.1,
                )))
            }),
            ProbeType::Battery => Box::new(|| {
                Ok(ProbeResultValue::Single(ProbeValue::Battery(
                    battery_readout().percentage()?,
                )))
            }),
            // TODO: Check if it's correct and matches neofetch
            ProbeType::PowerAdapter => Box::new(|| {
                Ok(ProbeResultValue::Single(ProbeValue::PowerAdapter(
                    match battery_readout().status()? {
                        BatteryState::Charging => "Charging".to_string(),
                        BatteryState::Discharging => "Discharging".to_string(),
                    },
                )))
            }),
            ProbeType::Font => {
                Box::new(|| Ok(ProbeResultValue::Single(ProbeValue::Font("".to_string()))))
            } // TODO
            ProbeType::Song => {
                Box::new(|| Ok(ProbeResultValue::Single(ProbeValue::Song("".to_string()))))
            } // TODO
            ProbeType::LocalIP => Box::new(|| {
                Ok(ProbeResultValue::Single(ProbeValue::LocalIP(
                    network_readout().logical_address(None)?,
                )))
            }), // TODO
            ProbeType::PublicIP => Box::new(|| {
                Ok(ProbeResultValue::Single(ProbeValue::PublicIP(
                    "".to_string(), // TODO
                )))
            }),
            ProbeType::Users => Box::new(|| Ok(ProbeResultValue::Single(ProbeValue::Users(0)))), // TODO
            ProbeType::Locale => {
                Box::new(|| Ok(ProbeResultValue::Single(ProbeValue::Locale("".to_string()))))
            } // TODO
            ProbeType::Java => Box::new(|| {
                Ok(ProbeResultValue::Single(ProbeValue::Java(
                    "N/A".to_string(), // TODO
                )))
            }),
            ProbeType::Python => Box::new(|| {
                Ok(ProbeResultValue::Single(ProbeValue::Python(
                    "N/A".to_string(), // TODO
                )))
            }),
            ProbeType::Node => Box::new(|| {
                Ok(ProbeResultValue::Single(ProbeValue::Node(
                    "N/A".to_string(), // TODO
                )))
            }),
            ProbeType::Rust => Box::new(|| {
                Ok(ProbeResultValue::Single(ProbeValue::Rust(
                    "N/A".to_string(), // TODO
                )))
            }),
        }
    }
}

/// Probe type. Refer to `ProbeValue` for what each metric corresponds to.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ProbeType {
    Host,
    OS,
    Model,
    Kernel,
    Distro,
    Uptime,
    Packages,
    Shell,
    Editor,
    Resolution,
    DE,
    WM,
    WMTheme,
    Theme,
    Icons,
    Cursor,
    Terminal,
    TerminalFont,
    CPU,
    GPU,
    Memory,
    Network,
    Bluetooth,
    BIOS,

    GPUDriver,
    CPUUsage,
    Disk,
    Battery,
    // TODO: Figure out what this should be
    PowerAdapter,
    Font,
    Song,
    LocalIP,
    PublicIP,
    Users,
    Locale,

    Java,
    Python,
    Node,
    Rust,
}

pub enum ProbeResultValue {
    Single(ProbeValue),
    Multiple(Vec<ProbeValue>),
}

impl From<ProbeValue> for ProbeResultValue {
    fn from(value: ProbeValue) -> Self {
        ProbeResultValue::Single(value)
    }
}

impl From<Vec<ProbeValue>> for ProbeResultValue {
    fn from(values: Vec<ProbeValue>) -> Self {
        ProbeResultValue::Multiple(values)
    }
}

pub type ProbeResult = Result<ProbeResultValue, ProbeError>;
pub type ProbeResultFunction = Box<dyn Fn() -> ProbeResult>;
pub type ProbeList = Vec<(String, ProbeResultFunction)>;
