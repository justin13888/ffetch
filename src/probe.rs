use std::{
    fmt::{self, Display, Formatter},
    sync::OnceLock,
};

use libmacchina::{
    traits::{BatteryState, ReadoutError, ShellFormat, ShellKind},
    BatteryReadout, GeneralReadout, KernelReadout, MemoryReadout, NetworkReadout, PackageReadout,
    ProductReadout,
};
use thiserror::Error;

use crate::config::ProbeConfig;

// TODO: Complete the rest of doc comments for this enum vv
pub enum ProbeValue {
    /// e.g. "Ubuntu 22.04.4 LTS x86_64"
    OS(String),
    // (Vendor, Product)
    /// e.g. ("Dell Inc.", "XPS 15 9510")
    Model(String, String),
    /// e.g. "6.8.4-cachyos"
    Kernel(String),
    /// Uptime in seconds
    /// E.g. 123
    Uptime(usize),
    /// Number of packages installed
    /// (package manager, count)
    /// E.g. ("dpkg", 123)
    Packages(String, usize), // TODO: CHECK
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
    LocalIP(Vec<String>), // TODO: CHECK
    PublicIP(String),     // TODO: CHECK
    Users(usize),         // TODO: CHECK
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

impl ToString for ProbeValue {
    fn to_string(&self) -> String {
        match self {
            ProbeValue::OS(os) => os.to_string(),
            ProbeValue::Model(vendor, product) => format!("{} {}", vendor, product),
            ProbeValue::Kernel(kernel) => kernel.to_string(),
            ProbeValue::Uptime(uptime) => {
                let uptime = *uptime as f64;
                let days = (uptime / (60.0 * 60.0 * 24.0)).round() as i32;
                let hours = ((uptime / (60.0 * 60.0)) % 24.0).round() as i32;
                let minutes = ((uptime / 60.0) % 60.0).round() as i32;
                let seconds = (uptime % 60.0).round() as i32;
                let _res = String::new();

                if days > 0 {
                    format!("{:.0} days, {:.0} hours, {:.0} mins", days, hours, minutes)
                } else if hours > 0 {
                    format!("{:.0} hours, {:.0} mins", hours, minutes)
                } else if minutes > 0 {
                    format!("{:.0} mins", minutes)
                } else {
                    format!("{:.0} seconds", seconds)
                }
            }
            ProbeValue::Packages(manager, count) => format!("{} ({})", count, manager),
            ProbeValue::Shell(shell) => shell.to_string(),
            ProbeValue::Editor(editor) => editor.to_string(),
            ProbeValue::Resolution(resolution) => resolution.to_string(),
            ProbeValue::DE(de) => de.to_string(),
            ProbeValue::WM(wm) => wm.to_string(),
            ProbeValue::WMTheme(wm_theme) => wm_theme.to_string(),
            ProbeValue::Theme(theme) => theme.to_string(),
            ProbeValue::Icons(icons) => icons.to_string(),
            ProbeValue::Cursor(cursor) => cursor.to_string(),
            ProbeValue::Terminal(terminal) => terminal.to_string(),
            ProbeValue::TerminalFont(terminal_font) => terminal_font.to_string(),
            ProbeValue::CPU(cpu) => cpu.to_string(),
            ProbeValue::GPU(gpu) => gpu.to_string(),
            ProbeValue::Memory(free, total) => format!(
                "{} MiB / {} MiB",
                (*free as f32 / (1024.0 * 1024.0)).round() as i32,
                (*total as f32 / (1024.0 * 1024.0)).round() as i32,
            ),
            ProbeValue::Network(network) => network.to_string(),
            ProbeValue::Bluetooth(bluetooth) => bluetooth.to_string(),
            ProbeValue::BIOS(bios) => bios.to_string(),
            ProbeValue::GPUDriver(gpu_driver) => gpu_driver.to_string(),
            ProbeValue::CPUUsage(cpu_usage) => cpu_usage.to_string(),
            ProbeValue::Disk(used, total) => format!(
                "{} G / {} G ({}%)",
                (*used as f32 / (1024.0 * 1024.0 * 1024.0)).round() as i32,
                (*total as f32 / (1024.0 * 1024.0 * 1024.0)).round() as i32,
                (*used as f32 / *total as f32 * 100.0).round() as i32,
            ),
            ProbeValue::Battery(battery) => battery.to_string(),
            ProbeValue::PowerAdapter(power_adapter) => power_adapter.to_string(),
            ProbeValue::Font(font) => font.to_string(),
            ProbeValue::Song(song) => song.to_string(),
            ProbeValue::LocalIP(local_ip) => local_ip.join(", "),
            ProbeValue::PublicIP(public_ip) => public_ip.to_string(),
            ProbeValue::Users(users) => users.to_string(),
            ProbeValue::Locale(locale) => locale.to_string(),
            ProbeValue::Java(java) => java.to_string(),
            ProbeValue::Node(node) => node.to_string(),
            ProbeValue::Python(python) => python.to_string(),
            ProbeValue::Rust(rust) => rust.to_string(),
        }
    }
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

/// Return a list of metrics to be probed from config
/// Note: ProbeValue that errors out will have placeholder values (e.g. "N/A")
///       This is different from some other fetch tools like neofetch, which omits the result entirely
pub fn probe_metrics(config: &ProbeConfig) -> (String, ProbeResultFunction) {
    use libmacchina::traits::BatteryReadout as _;
    use libmacchina::traits::GeneralReadout as _;
    use libmacchina::traits::KernelReadout as _;
    use libmacchina::traits::MemoryReadout as _;
    use libmacchina::traits::NetworkReadout as _;
    use libmacchina::traits::PackageReadout as _;
    use libmacchina::traits::ProductReadout as _;

    match config {
        ProbeConfig::OS(label) => (
            label.clone(),
            Box::new(|| {
                Ok(ProbeResultValue::Single(ProbeValue::OS(
                    general_readout().os_name()?,
                )))
            }),
        ),
        ProbeConfig::Model(label) => (
            label.clone(),
            Box::new(|| {
                Ok(ProbeResultValue::Single(ProbeValue::Model(
                    product_readout().vendor()?,
                    product_readout().product()?,
                )))
            }),
        ),
        ProbeConfig::Kernel(label) => (
            label.clone(),
            Box::new(|| {
                Ok(ProbeResultValue::Single(ProbeValue::Kernel(
                    kernel_readout().pretty_kernel()?,
                )))
            }),
        ),
        ProbeConfig::Uptime(label) => (
            label.clone(),
            Box::new(|| {
                Ok(ProbeResultValue::Single(ProbeValue::Uptime(
                    general_readout().uptime()?,
                )))
            }),
        ),
        // TODO: Test libmacchina packages() function for package manager hanging issues
        ProbeConfig::Packages(label) => (
            label.clone(),
            Box::new(|| {
                Ok(ProbeResultValue::Multiple(
                    package_readout()
                        .count_pkgs()
                        .into_iter()
                        .map(|(name, count)| ProbeValue::Packages(name.to_string(), count))
                        .collect::<Vec<_>>(),
                ))
            }),
        ),
        ProbeConfig::Shell(label) => (
            label.clone(),
            Box::new(|| {
                Ok(ProbeResultValue::Single(ProbeValue::Shell(
                    general_readout().shell(ShellFormat::Relative, ShellKind::Current)?,
                )))
            }),
        ),
        ProbeConfig::Editor(label) => (label.clone(), Box::new(|| Err(ProbeError::Unimplemented))), // TODO
        ProbeConfig::Resolution(label) => (
            label.clone(),
            Box::new(|| {
                Ok(ProbeResultValue::Single(ProbeValue::Resolution(
                    general_readout().resolution()?,
                )))
            }),
        ),
        ProbeConfig::DE(label) => (
            label.clone(),
            Box::new(|| {
                Ok(ProbeResultValue::Single(ProbeValue::DE(
                    general_readout().desktop_environment()?,
                )))
            }),
        ),
        ProbeConfig::WM(label) => (
            label.clone(),
            Box::new(|| {
                Ok(ProbeResultValue::Single(ProbeValue::WM(
                    general_readout().window_manager()?,
                )))
            }),
        ),
        ProbeConfig::WMTheme(label) => (
            label.clone(),
            Box::new(|| {
                Ok(ProbeResultValue::Single(ProbeValue::WMTheme(
                    "".to_string(), // TODO
                )))
            }),
        ),

        ProbeConfig::Theme(label) => (
            label.clone(),
            Box::new(|| Ok(ProbeResultValue::Single(ProbeValue::Theme("".to_string())))), // TODO
        ),
        ProbeConfig::Icons(label) => (
            label.clone(),
            Box::new(|| Ok(ProbeResultValue::Single(ProbeValue::Icons("".to_string())))), // TODO
        ),
        ProbeConfig::Cursor(label) => (
            label.clone(),
            Box::new(|| Ok(ProbeResultValue::Single(ProbeValue::Cursor("".to_string())))), // TODO
        ),
        ProbeConfig::Terminal(label) => (
            label.clone(),
            Box::new(|| {
                Ok(ProbeResultValue::Single(ProbeValue::Terminal(
                    general_readout().terminal()?,
                )))
            }),
        ),
        ProbeConfig::TerminalFont(label) => (
            label.clone(),
            Box::new(|| {
                Ok(ProbeResultValue::Single(ProbeValue::TerminalFont(
                    "".to_string(), // TODO
                )))
            }),
        ),
        ProbeConfig::CPU(label) => (
            label.clone(),
            Box::new(|| {
                Ok(ProbeResultValue::Single(ProbeValue::CPU(
                    general_readout().cpu_model_name()?,
                )))
            }),
        ),
        ProbeConfig::GPU(label) => (
            label.clone(),
            Box::new(|| {
                Ok(ProbeResultValue::Multiple(
                    general_readout()
                        .gpus()?
                        .into_iter()
                        .map(ProbeValue::GPU)
                        .collect::<Vec<_>>(),
                ))
            }),
        ),
        ProbeConfig::Memory(label) => (
            label.clone(),
            Box::new(|| {
                Ok(ProbeResultValue::Single(ProbeValue::Memory(
                    memory_readout().used()?,
                    memory_readout().total()?,
                )))
            }),
        ),
        ProbeConfig::Network(label) => (
            label.clone(),
            Box::new(|| {
                Ok(ProbeResultValue::Single(ProbeValue::Network(
                    "".to_string(), // TODO
                )))
            }),
        ),
        ProbeConfig::Bluetooth(label) => (
            label.clone(),
            Box::new(|| {
                Ok(ProbeResultValue::Single(ProbeValue::Bluetooth(
                    "".to_string(), // TODO
                )))
            }),
        ),
        ProbeConfig::BIOS(label) => (
            label.clone(),
            Box::new(|| Ok(ProbeResultValue::Single(ProbeValue::BIOS("".to_string())))),
        ),
        ProbeConfig::GPUDriver(label) => (
            label.clone(),
            Box::new(|| {
                Ok(ProbeResultValue::Single(ProbeValue::GPUDriver(
                    "".to_string(), // TODO
                )))
            }),
        ),
        ProbeConfig::CPUUsage(label) => (
            label.clone(),
            Box::new(|| {
                Ok(ProbeResultValue::Single(ProbeValue::CPUUsage(
                    general_readout().cpu_usage()?,
                )))
            }),
        ),
        ProbeConfig::Disk(label) => (
            label.clone(),
            Box::new(|| {
                let disk_readout = general_readout().disk_space()?;
                Ok(ProbeResultValue::Single(ProbeValue::Disk(
                    disk_readout.0,
                    disk_readout.1,
                )))
            }),
        ),
        ProbeConfig::Battery(label) => (
            label.clone(),
            Box::new(|| {
                Ok(ProbeResultValue::Single(ProbeValue::Battery(
                    battery_readout().percentage()?,
                )))
            }),
        ),
        // TODO: Check if it's correct and matches neofetch
        ProbeConfig::PowerAdapter(label) => (
            label.clone(),
            Box::new(|| {
                Ok(ProbeResultValue::Single(ProbeValue::PowerAdapter(
                    match battery_readout().status()? {
                        BatteryState::Charging => "Charging".to_string(),
                        BatteryState::Discharging => "Discharging".to_string(),
                    },
                )))
            }),
        ),
        ProbeConfig::Font(label) => (
            label.clone(),
            Box::new(|| Ok(ProbeResultValue::Single(ProbeValue::Font("".to_string())))), // TODO
        ),
        ProbeConfig::Song(label) => (
            label.clone(),
            Box::new(|| Ok(ProbeResultValue::Single(ProbeValue::Song("".to_string())))), // TODO
        ),
        ProbeConfig::LocalIP(label) => (
            label.clone(),
            Box::new(|| Ok(ProbeResultValue::Single(ProbeValue::LocalIP(vec![])))), // TODO
        ),
        ProbeConfig::PublicIP(label) => (
            label.clone(),
            Box::new(|| {
                Ok(ProbeResultValue::Single(ProbeValue::PublicIP(
                    "".to_string(), // TODO
                )))
            }),
        ),
        ProbeConfig::Users(label) => (
            label.clone(),
            Box::new(|| Ok(ProbeResultValue::Single(ProbeValue::Users(0)))), // TODO
        ),
        ProbeConfig::Locale(label) => (
            label.clone(),
            Box::new(|| Ok(ProbeResultValue::Single(ProbeValue::Locale("".to_string())))), // TODO
        ),
        ProbeConfig::Java(label) => (
            label.clone(),
            Box::new(|| {
                Ok(ProbeResultValue::Single(ProbeValue::Java(
                    "N/A".to_string(), // TODO
                )))
            }),
        ),
        ProbeConfig::Python(label) => (
            label.clone(),
            Box::new(|| {
                Ok(ProbeResultValue::Single(ProbeValue::Python(
                    "N/A".to_string(), // TODO
                )))
            }),
        ),
        ProbeConfig::Node(label) => (
            label.clone(),
            Box::new(|| {
                Ok(ProbeResultValue::Single(ProbeValue::Node(
                    "N/A".to_string(), // TODO
                )))
            }),
        ),
        ProbeConfig::Rust(label) => (
            label.clone(),
            Box::new(|| {
                Ok(ProbeResultValue::Single(ProbeValue::Rust(
                    "N/A".to_string(), // TODO
                )))
            }),
        ),
    }
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
