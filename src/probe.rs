use std::{
    fmt::{self, Display, Formatter},
    os,
    sync::OnceLock,
};

use libmacchina::{
    traits::{PackageManager, ReadoutError, ShellFormat, ShellKind},
    BatteryReadout, GeneralReadout, KernelReadout, MemoryReadout, NetworkReadout, PackageReadout,
    ProductReadout,
};
use thiserror::Error;

use crate::config::{Config, ProbeConfig};

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
    Resolution(usize, usize),
    DE(String),
    WM(String),
    WMTheme(String),
    Theme(String),
    Icons(String),
    Cursor(String),
    Terminal(String),
    TerminalFont(String),
    CPU(String),
    GPU(String),
    Memory(String),
    Network(String),
    Bluetooth(String),
    BIOS(String),
    GPUDriver(String),
    Disk(String),         // TODO: CHECK
    Battery(String),      // TODO: CHECK
    PowerAdapter(String), // TODO: CHECK
    Font(String),
    Song(String),
    LocalIP(String),  // TODO: CHECK
    PublicIP(String), // TODO: CHECK
    Users(usize),     // TODO: CHECK
    Locale(String),
    Java(usize),
    Python(usize),
    Rust(usize),
}

impl ToString for ProbeValue {
    fn to_string(&self) -> String {
        match self {
            ProbeValue::OS(os) => os.to_string(),
            ProbeValue::Model(vendor, product) => format!("{} {}", vendor, product),
            ProbeValue::Kernel(kernel) => kernel.to_string(),
            ProbeValue::Uptime(uptime) => uptime.to_string(),
            ProbeValue::Packages(manager, count) => format!("{} ({})", count, manager),
            ProbeValue::Shell(shell) => shell.to_string(),
            ProbeValue::Editor(editor) => editor.to_string(),
            ProbeValue::Resolution(width, height) => format!("{}x{}", width, height),
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
            ProbeValue::Memory(memory) => memory.to_string(),
            ProbeValue::Network(network) => network.to_string(),
            ProbeValue::Bluetooth(bluetooth) => bluetooth.to_string(),
            ProbeValue::BIOS(bios) => bios.to_string(),
            ProbeValue::GPUDriver(gpu_driver) => gpu_driver.to_string(),
            ProbeValue::Disk(disk) => disk.to_string(),
            ProbeValue::Battery(battery) => battery.to_string(),
            ProbeValue::PowerAdapter(power_adapter) => power_adapter.to_string(),
            ProbeValue::Font(font) => font.to_string(),
            ProbeValue::Song(song) => song.to_string(),
            ProbeValue::LocalIP(local_ip) => local_ip.to_string(),
            ProbeValue::PublicIP(public_ip) => public_ip.to_string(),
            ProbeValue::Users(users) => users.to_string(),
            ProbeValue::Locale(locale) => locale.to_string(),
            ProbeValue::Java(java) => java.to_string(),
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
pub fn probe_metrics(config: &ProbeConfig) -> ProbeList {
    use libmacchina::traits::BatteryReadout as _;
    use libmacchina::traits::GeneralReadout as _;
    use libmacchina::traits::KernelReadout as _;
    use libmacchina::traits::MemoryReadout as _;
    use libmacchina::traits::NetworkReadout as _;
    use libmacchina::traits::PackageReadout as _;
    use libmacchina::traits::ProductReadout as _;

    let mut metrics: Vec<(String, Box<dyn Fn() -> ProbeResult>)> = Vec::new();

    if let Some(os) = &config.os {
        metrics.push((
            os.clone(),
            Box::new(|| {
                Ok(ProbeValue::OS(general_readout().distribution()?))
                // TODO: Doesn't work for all platforms ^^
            }),
        ));
    }
    if let Some(model) = &config.model {
        metrics.push((
            model.clone(),
            Box::new(|| {
                Ok(ProbeValue::Model(
                    product_readout().vendor()?,
                    product_readout().product()?,
                ))
            }),
        ));
    }
    if let Some(kernel) = &config.kernel {
        metrics.push((
            kernel.clone(),
            Box::new(|| Ok(ProbeValue::Kernel(kernel_readout().pretty_kernel()?))),
        ));
    }
    if let Some(uptime) = &config.uptime {
        metrics.push((
            uptime.clone(),
            Box::new(|| Ok(ProbeValue::Uptime(general_readout().uptime()?))),
        ));
    }
    // TODO: Use another library to independently count packages
    // TODO: support timeouts
    if let Some(packages) = &config.packages {
        let results = package_readout().count_pkgs();
        results.into_iter().for_each(|(name, count)| {
            metrics.push((
                packages.clone(),
                Box::new(move || Ok(ProbeValue::Packages(name.to_string(), count))),
            ))
        });
    }
    if let Some(shell) = &config.shell {
        metrics.push((
            shell.clone(),
            Box::new(|| {
                Ok(ProbeValue::Shell(
                    general_readout().shell(ShellFormat::Relative, ShellKind::Current)?,
                ))
            }),
        ));
    }
    if let Some(editor) = &config.editor {
        // TODO: Implement editor readout
        metrics.push((editor.clone(), Box::new(|| Err(ProbeError::Unimplemented))));
    }
    // TODO: Implement these
    // vv
    if let Some(resolution) = &config.resolution {
        metrics.push((
            resolution.clone(),
            Box::new(|| Ok(ProbeValue::Resolution(0, 0))),
        ));
    }
    if let Some(de) = &config.de {
        metrics.push((de.clone(), Box::new(|| Ok(ProbeValue::DE("".to_string())))));
    }
    if let Some(wm) = &config.wm {
        metrics.push((wm.clone(), Box::new(|| Ok(ProbeValue::WM("".to_string())))))
    }
    if let Some(wm_theme) = &config.wm_theme {
        metrics.push((
            wm_theme.clone(),
            Box::new(|| Ok(ProbeValue::WMTheme("".to_string()))),
        ));
    }
    if let Some(theme) = &config.theme {
        metrics.push((
            theme.clone(),
            Box::new(|| Ok(ProbeValue::Theme("".to_string()))),
        ));
    }
    if let Some(icons) = &config.icons {
        metrics.push((
            icons.clone(),
            Box::new(|| Ok(ProbeValue::Icons("".to_string()))),
        ));
    }
    if let Some(cursor) = &config.cursor {
        metrics.push((
            cursor.clone(),
            Box::new(|| Ok(ProbeValue::Cursor("".to_string()))),
        ));
    }
    if let Some(terminal) = &config.terminal {
        metrics.push((
            terminal.clone(),
            Box::new(|| Ok(ProbeValue::Terminal("".to_string()))),
        ));
    }
    if let Some(terminal_font) = &config.terminal_font {
        metrics.push((
            terminal_font.clone(),
            Box::new(|| Ok(ProbeValue::TerminalFont("".to_string()))),
        ));
    }
    if let Some(cpu) = &config.cpu {
        metrics.push((
            cpu.clone(),
            Box::new(|| Ok(ProbeValue::CPU("".to_string()))),
        ));
    }
    if let Some(gpu) = &config.gpu {
        metrics.push((
            gpu.clone(),
            Box::new(|| Ok(ProbeValue::GPU("".to_string()))),
        ));
    }
    if let Some(memory) = &config.memory {
        metrics.push((
            memory.clone(),
            Box::new(|| Ok(ProbeValue::Memory("".to_string()))),
        ));
    }
    if let Some(network) = &config.network {
        metrics.push((
            network.clone(),
            Box::new(|| Ok(ProbeValue::Network("".to_string()))),
        ));
    }
    if let Some(bluetooth) = &config.bluetooth {
        metrics.push((
            bluetooth.clone(),
            Box::new(|| Ok(ProbeValue::Bluetooth("".to_string()))),
        ));
    }
    if let Some(bios) = &config.bios {
        metrics.push((
            bios.clone(),
            Box::new(|| Ok(ProbeValue::BIOS("".to_string()))),
        ));
    }
    if let Some(gpu_driver) = &config.gpu_driver {
        metrics.push((
            gpu_driver.clone(),
            Box::new(|| Ok(ProbeValue::GPUDriver("".to_string()))),
        ));
    }
    if let Some(disk) = &config.disk {
        metrics.push((
            disk.clone(),
            Box::new(|| Ok(ProbeValue::Disk("".to_string()))),
        ));
    }
    if let Some(battery) = &config.battery {
        metrics.push((
            battery.clone(),
            Box::new(|| Ok(ProbeValue::Battery("".to_string()))),
        ));
    }
    if let Some(power_adapter) = &config.power_adapter {
        metrics.push((
            power_adapter.clone(),
            Box::new(|| Ok(ProbeValue::PowerAdapter("".to_string()))),
        ));
    }
    if let Some(font) = &config.font {
        metrics.push((
            font.clone(),
            Box::new(|| Ok(ProbeValue::Font("".to_string()))),
        ));
    }
    if let Some(song) = &config.song {
        metrics.push((
            song.clone(),
            Box::new(|| Ok(ProbeValue::Song("".to_string()))),
        ));
    }
    if let Some(local_ip) = &config.local_ip {
        metrics.push((
            local_ip.clone(),
            Box::new(|| Ok(ProbeValue::LocalIP("".to_string()))),
        ));
    }
    if let Some(public_ip) = &config.public_ip {
        metrics.push((
            public_ip.clone(),
            Box::new(|| Ok(ProbeValue::PublicIP("".to_string()))),
        ));
    }
    if let Some(users) = &config.users {
        metrics.push((users.clone(), Box::new(|| Ok(ProbeValue::Users(0)))));
    }
    if let Some(locale) = &config.locale {
        metrics.push((
            locale.clone(),
            Box::new(|| Ok(ProbeValue::Locale("".to_string()))),
        ));
    }
    if let Some(java) = &config.java {
        metrics.push((java.clone(), Box::new(|| Ok(ProbeValue::Java(0)))));
    }
    if let Some(python) = &config.python {
        metrics.push((python.clone(), Box::new(|| Ok(ProbeValue::Python(0)))));
    }
    if let Some(rust) = &config.rust {
        metrics.push((rust.clone(), Box::new(|| Ok(ProbeValue::Rust(0)))));
    }
    // TODO: Implement these ^^

    // TODO: Implement for everything in ProbeConfig
    // TODO: Then, make sure all capabilities in libmacchina are used

    metrics
}

pub type ProbeResult = Result<ProbeValue, ProbeError>;
pub type ProbeList = Vec<(String, impl Fn() -> ProbeResult)>;
