use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    probe::{ProbeResultFunction, ProbeType},
    renderer::macchina::MacchinaRenderer,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Config {
    Neofetch(NeofetchRendererConfig),
    Macchina(MacchinaRendererConfig),
}

pub enum RendererOverride {
    Neofetch,
    Macchina,
}

impl Config {
    /// Default config with all features enabled
    pub fn default_all() -> Self {
        Self::default_neofetch_all()
    }

    /// Default config replicating neofetch
    pub fn default_neofetch() -> Self {
        Self::Neofetch(NeofetchRendererConfig::default())
    }

    /// Default config replicating neofetch with all features enabled
    pub fn default_neofetch_all() -> Self {
        Self::Neofetch(NeofetchRendererConfig::default_all())
    }

    /// Default config replicating macchina CLI
    /// TODO: Implement all macchina CLI configs
    pub fn default_macchina() -> Self {
        Self::Macchina(MacchinaRendererConfig::default())
    }

    /// Default config replicating macchina with all features enabled
    pub fn default_macchina_all() -> Self {
        Self::Macchina(MacchinaRendererConfig::default_all())
    }

    /// Load config from a file
    pub fn from_file(
        path: &Path,
        _renderer_override: Option<RendererOverride>,
    ) -> Result<Self, ConfigParseError> {
        // TODO: Support "extending" default configs
        // TODO: Implement renderer override
        toml::from_str(&std::fs::read_to_string(path)?).map_err(|e| e.into())
    }

    /// Write config to a file
    pub fn to_file(&self, path: &Path) -> Result<(), ConfigWriteError> {
        // Serialize to toml
        let toml = toml::to_string(self)?;
        // Write to file
        Ok(std::fs::write(path, toml)?)
    }

    /// Generate a default config file
    /// If the file already exists, it will not be overwritten
    pub fn generate_default(path: &Path) -> Result<(), ConfigWriteError> {
        if path.exists() {
            return Err(ConfigWriteError::Io(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                "File already exists",
            )));
        }
        let config = Self::default();
        config.to_file(path)
        // TODO: Replace line above with custom serialization to include comments
    }

    fn get_project_dirs() -> Option<directories::ProjectDirs> {
        directories::ProjectDirs::from("net", "justin13888", "ffetch")
    }

    pub fn get_config_dir() -> Option<PathBuf> {
        Self::get_project_dirs().map(|dirs| dirs.config_dir().to_path_buf())
    }

    pub const CONFIG_FILE_NAME: &'static str = "config.toml";
}

impl Default for Config {
    fn default() -> Self {
        Self::default_neofetch()
    }
}

#[derive(Error, Debug)]
pub enum ConfigParseError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Deserialization error: {0}")]
    Deserialization(#[from] toml::de::Error),
}

#[derive(Error, Debug)]
pub enum ConfigWriteError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] toml::ser::Error),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NeofetchRendererConfig {
    /// Whether to display the title
    /// (e.g. "johndoe@myhostname\n------------------")
    pub title: bool,
    pub underline: bool,
    pub col: bool,

    pub probes: Vec<ProbeConfig>,
}

impl NeofetchRendererConfig {
    pub fn default_all() -> Self {
        Self {
            title: true,
            underline: true,
            col: true,
            probes: ProbeConfig::default_all(),
        }
    }
}

impl Default for NeofetchRendererConfig {
    fn default() -> Self {
        Self {
            title: true,
            underline: true,
            col: true,
            probes: ProbeConfig::default_neofetch(),
        }
    }
}

// TODO: Implement Macchina configs
// TODO: Consume config with renderer
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MacchinaRendererConfig {
    /// Specifies the network interface to use for the LocalIP readout
    pub interface: Option<String>,
    /// Lengthen uptime output
    pub long_uptime: bool,

    // Probe configs
    pub probes: Vec<ProbeConfig>,
}

impl MacchinaRendererConfig {
    pub fn default_all() -> Self {
        Self {
            interface: None,
            long_uptime: true,
            probes: ProbeConfig::default_all(),
        }
    }
}

impl Default for MacchinaRendererConfig {
    fn default() -> Self {
        Self {
            interface: None,
            long_uptime: true,
            probes: ProbeConfig::default_macchina(),
        }
    }
}

// TODO: Find neofetch online and make sure it covers everything
// TODO: Figure out what other metadata is needed in the config (e.g. format of OS field)
/// Probe config. Refer to `ProbeValue` for what each metric corresponds to.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ProbeConfig {
    Host(String),
    OS(String),
    Model(String),
    Kernel(String),
    Distro(String),
    Uptime(String),
    Packages(String),
    Shell(String),
    Editor(String),
    Resolution(String),
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
    CPUUsage(String),
    Disk(String),
    Battery(String),
    // TODO: Figure out what this should be
    PowerAdapter(String),
    Font(String),
    Song(String),
    LocalIP(String),
    PublicIP(String),
    Users(String),
    Locale(String),

    Java(String),
    Python(String),
    Node(String),
    Rust(String),
}

impl ProbeConfig {
    /// Default config enabling all features
    pub fn default_all() -> Vec<Self> {
        vec![
            Self::OS("OS".to_string()),
            Self::Model("Host".to_string()),
            Self::Kernel("Kernel".to_string()),
            Self::Uptime("Uptime".to_string()),
            Self::Packages("Packages".to_string()),
            Self::Shell("Shell".to_string()),
            Self::Editor("Editor".to_string()),
            Self::Resolution("Resolution".to_string()),
            Self::DE("DE".to_string()),
            Self::WM("WM".to_string()),
            Self::WMTheme("WM Theme".to_string()),
            Self::Theme("Theme".to_string()),
            Self::Icons("Icons".to_string()),
            Self::Cursor("Cursor".to_string()),
            Self::Terminal("Terminal".to_string()),
            Self::TerminalFont("Terminal Font".to_string()),
            Self::CPU("CPU".to_string()),
            Self::GPU("GPU".to_string()),
            Self::Memory("Memory".to_string()),
            Self::Network("Network".to_string()),
            Self::Bluetooth("Bluetooth".to_string()),
            Self::BIOS("BIOS".to_string()),
            Self::GPUDriver("GPU Driver".to_string()),
            Self::CPUUsage("CPU Usage".to_string()),
            Self::Disk("Disk".to_string()),
            Self::Battery("Battery".to_string()),
            Self::PowerAdapter("Power Adapter".to_string()),
            Self::Font("Font".to_string()),
            Self::Song("Song".to_string()),
            Self::LocalIP("Local IP".to_string()),
            Self::PublicIP("Public IP".to_string()),
            Self::Users("Users".to_string()),
            Self::Locale("Locale".to_string()),
            Self::Java("Java".to_string()),
            Self::Python("Python".to_string()),
            Self::Node("Node".to_string()),
            Self::Rust("Rust".to_string()),
        ]
    }

    /// Default config replicating Neofetch
    pub fn default_neofetch() -> Vec<Self> {
        vec![
            Self::OS("OS".to_string()),
            Self::Model("Host".to_string()),
            Self::Kernel("Kernel".to_string()),
            Self::Uptime("Uptime".to_string()),
            Self::Packages("Packages".to_string()),
            Self::Shell("Shell".to_string()),
            Self::Editor("Editor".to_string()),
            Self::Resolution("Resolution".to_string()),
            Self::DE("DE".to_string()),
            Self::WM("WM".to_string()),
            Self::WMTheme("WM Theme".to_string()),
            Self::Theme("Theme".to_string()),
            Self::Icons("Icons".to_string()),
            Self::Cursor("Cursor".to_string()),
            Self::Terminal("Terminal".to_string()),
            Self::TerminalFont("Terminal Font".to_string()),
            Self::CPU("CPU".to_string()),
            Self::GPU("GPU".to_string()),
            Self::Memory("Memory".to_string()),
            Self::Network("Network".to_string()),
            Self::Bluetooth("Bluetooth".to_string()),
            Self::BIOS("BIOS".to_string()),
        ]
    }

    /// Default config replicating macchina CLI
    pub fn default_macchina() -> Vec<Self> {
        vec![
            Self::Host("Host".to_string()),
            Self::Model("Machine".to_string()),
            Self::Kernel("Kernel".to_string()),
            #[cfg(target_os = "linux")]
            Self::Distro("Distro".to_string()),
            #[cfg(any(target_os = "macos", target_os = "windows"))]
            Self::OS("OS".to_string()),
            Self::Packages("Packages".to_string()),
            Self::Terminal("Terminal".to_string()),
            Self::LocalIP("Local IP".to_string()),
            Self::Shell("Shell".to_string()),
            Self::Uptime("Uptime".to_string()),
            Self::CPU("CPU".to_string()),
            Self::Resolution("Resolution".to_string()),
            Self::CPUUsage("CPU Load".to_string()),
            Self::Memory("Memory".to_string()),
            Self::Battery("Battery".to_string()),
        ]
    } // TODO: Double check this function with macchina CLI source code

    pub fn get_funcs(&self) -> (String, ProbeResultFunction) {
        match self {
            Self::Host(label) => (label.clone(), ProbeType::Host.into()),
            Self::OS(label) => (label.clone(), ProbeType::OS.into()),
            Self::Distro(label) => (label.clone(), ProbeType::Distro.into()),
            Self::Model(label) => (label.clone(), ProbeType::Model.into()),
            Self::Kernel(label) => (label.clone(), ProbeType::Kernel.into()),
            Self::Uptime(label) => (label.clone(), ProbeType::Uptime.into()),
            Self::Packages(label) => (label.clone(), ProbeType::Packages.into()),
            Self::Shell(label) => (label.clone(), ProbeType::Shell.into()),
            Self::Editor(label) => (label.clone(), ProbeType::Editor.into()),
            Self::Resolution(label) => (label.clone(), ProbeType::Resolution.into()),
            Self::DE(label) => (label.clone(), ProbeType::DE.into()),
            Self::WM(label) => (label.clone(), ProbeType::WM.into()),
            Self::WMTheme(label) => (label.clone(), ProbeType::WMTheme.into()),
            Self::Theme(label) => (label.clone(), ProbeType::Theme.into()),
            Self::Icons(label) => (label.clone(), ProbeType::Icons.into()),
            Self::Cursor(label) => (label.clone(), ProbeType::Cursor.into()),
            Self::Terminal(label) => (label.clone(), ProbeType::Terminal.into()),
            Self::TerminalFont(label) => (label.clone(), ProbeType::TerminalFont.into()),
            Self::CPU(label) => (label.clone(), ProbeType::CPU.into()),
            Self::GPU(label) => (label.clone(), ProbeType::GPU.into()),
            Self::Memory(label) => (label.clone(), ProbeType::Memory.into()),
            Self::Network(label) => (label.clone(), ProbeType::Network.into()),
            Self::Bluetooth(label) => (label.clone(), ProbeType::Bluetooth.into()),
            Self::BIOS(label) => (label.clone(), ProbeType::BIOS.into()),
            Self::GPUDriver(label) => (label.clone(), ProbeType::GPUDriver.into()),
            Self::CPUUsage(label) => (label.clone(), ProbeType::CPUUsage.into()),
            Self::Disk(label) => (label.clone(), ProbeType::Disk.into()),
            Self::Battery(label) => (label.clone(), ProbeType::Battery.into()),
            Self::PowerAdapter(label) => (label.clone(), ProbeType::PowerAdapter.into()),
            Self::Font(label) => (label.clone(), ProbeType::Font.into()),
            Self::Song(label) => (label.clone(), ProbeType::Song.into()),
            Self::LocalIP(label) => (label.clone(), ProbeType::LocalIP.into()),
            Self::PublicIP(label) => (label.clone(), ProbeType::PublicIP.into()),
            Self::Users(label) => (label.clone(), ProbeType::Users.into()),
            Self::Locale(label) => (label.clone(), ProbeType::Locale.into()),
            Self::Java(label) => (label.clone(), ProbeType::Java.into()),
            Self::Python(label) => (label.clone(), ProbeType::Python.into()),
            Self::Node(label) => (label.clone(), ProbeType::Node.into()),
            Self::Rust(label) => (label.clone(), ProbeType::Rust.into()),
        }
    }
}
