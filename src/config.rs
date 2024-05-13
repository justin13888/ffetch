use std::path::Path;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub renderer: RendererConfig,
    pub neofetch: NeofetchRendererConfig,
    pub probes: Vec<ProbeConfig>,
}

impl Config {
    /// Default config with all features enabled
    pub fn default_all() -> Self {
        Self {
            renderer: RendererConfig::default(),
            neofetch: NeofetchRendererConfig::default(),
            probes: ProbeConfig::default_all(),
        }
    }

    /// Default config replicating neofetch
    pub fn default_neofetch() -> Self {
        Self {
            renderer: RendererConfig::default(),
            neofetch: NeofetchRendererConfig::default(),
            probes: ProbeConfig::default_neofetch(),
        }
    }

    /// Default config replicating macchina CLI
    /// TODO: Implement all macchina CLI configs
    pub fn default_macchina() -> Self {
        Self {
            renderer: RendererConfig::default(), // TODO: Change to Macchina renderer when implemented
            neofetch: NeofetchRendererConfig::default(),
            probes: ProbeConfig::default_macchina(),
        }
    }

    /// Load config from a file
    // TODO: Support deserialization where undefined fields are set to default
    // TODO: When deserializing, check the probe config order to determine another field. Need to modify ProbeConfig structure
    pub fn from_file(path: &Path) -> Result<Self, ConfigParseError> {
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
}

impl Default for Config {
    fn default() -> Self {
        Self::default_all()
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
pub enum RendererConfig {
    Neofetch,
}

impl Default for RendererConfig {
    fn default() -> Self {
        Self::Neofetch
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NeofetchRendererConfig {
    /// Whether to display the title
    /// (e.g. "johndoe@myhostname\n------------------")
    pub title: bool,
    pub underline: bool,
    pub col: bool,
}

impl Default for NeofetchRendererConfig {
    fn default() -> Self {
        Self {
            title: true,
            underline: true,
            col: true,
        }
    }
}

// TODO: Find neofetch online and make sure it covers everything
// TODO: Figure out what other metadata is needed in the config (e.g. format of OS field)
/// Configuration for probing. Refer to `ProbeValue` for what each metric corresponds to.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ProbeConfig {
    OS(String),
    Model(String),
    Kernel(String),
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
        todo!("Refer to neofetch default config to implement")
    }

    /// Default config replicating macchina CLI
    pub fn default_macchina() -> Vec<Self> {
        todo!("Refer to macchina CLI default config to implement")
    }
}
