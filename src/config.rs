use std::path::Path;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Config {
    pub renderer: RendererConfig,
    pub neofetch: NeofetchRendererConfig,
    pub probe: ProbeConfig,
}

impl Config {
    /// Default config with all features enabled
    pub fn default_all() -> Self {
        Self {
            renderer: RendererConfig::default(),
            neofetch: NeofetchRendererConfig::default(),
            probe: ProbeConfig::default_all(),
        }
    }

    /// Default config replicating neofetch
    pub fn default_neofetch() -> Self {
        Self {
            renderer: RendererConfig::default(),
            neofetch: NeofetchRendererConfig::default(),
            probe: ProbeConfig::default_neofetch(),
        }
    }

    /// Load config from a file
    // TODO: Support deserialization where undefined fields are set to default
    // TODO: When deserializing, check the probe config order to determine another field. Need to modify ProbeConfig structure
    pub fn from_file(path: &Path) -> Result<Self, ConfigParseError> {
        // Read the file
        let file = std::fs::File::open(path)?;
        // Deserialize to Config
        let config: Self = toml::from_str(&std::fs::read_to_string(path)?)?;
        Ok(config)
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
}

impl Default for NeofetchRendererConfig {
    fn default() -> Self {
        Self {
            title: true,
            underline: true,
        }
    }
}

// TODO: Find neofetch online and make sure it covers everything
// TODO: Figure out what other metadata is needed in the config (e.g. format of OS field)
/// Configuration for probing. Refer to `ProbeValue` for what each metric corresponds to.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProbeConfig {
    pub os: Option<String>,
    pub model: Option<String>,
    pub kernel: Option<String>,
    pub uptime: Option<String>,
    pub packages: Option<String>,
    pub shell: Option<String>,
    pub editor: Option<String>,
    pub resolution: Option<String>,
    pub de: Option<String>,
    pub wm: Option<String>,
    pub wm_theme: Option<String>,
    pub theme: Option<String>,
    pub icons: Option<String>,
    pub cursor: Option<String>,
    pub terminal: Option<String>,
    pub terminal_font: Option<String>,
    pub cpu: Option<String>,
    pub gpu: Option<String>,
    pub memory: Option<String>,
    pub network: Option<String>,
    pub bluetooth: Option<String>,
    pub bios: Option<String>,

    pub gpu_driver: Option<String>,
    pub cpu_usage: Option<String>,
    pub disk: Option<String>,
    pub battery: Option<String>,
    // TODO: Figure out what this should be
    pub power_adapter: Option<String>,
    pub font: Option<String>,
    pub song: Option<String>,
    pub local_ip: Option<String>,
    pub public_ip: Option<String>,
    pub users: Option<String>,
    pub locale: Option<String>,

    pub java: Option<String>,
    pub python: Option<String>,
    pub node: Option<String>,
    pub rust: Option<String>,

    pub cols: bool,
}

impl Default for ProbeConfig {
    fn default() -> Self {
        // TODO: Change to a more reasonable, faster default. this is for testing
        Self::default_all()
        // Self::default_neofetch()
    }
}

impl ProbeConfig {
    // TODO: Implement
    pub fn default_all() -> Self {
        Self {
            os: Some("OS".to_string()),
            model: Some("Host".to_string()),
            kernel: Some("Kernel".to_string()),
            uptime: Some("Uptime".to_string()),
            packages: Some("Packages".to_string()),
            shell: Some("Shell".to_string()),
            editor: Some("Editor".to_string()),
            resolution: Some("Resolution".to_string()),
            de: Some("DE".to_string()),
            wm: Some("WM".to_string()),
            wm_theme: Some("WM Theme".to_string()),
            theme: Some("Theme".to_string()),
            icons: Some("Icons".to_string()),
            cursor: Some("Cursor".to_string()),
            terminal: Some("Terminal".to_string()),
            terminal_font: Some("Terminal Font".to_string()),
            cpu: Some("CPU".to_string()),
            gpu: Some("GPU".to_string()),
            memory: Some("Memory".to_string()),
            network: Some("Network".to_string()),
            bluetooth: Some("Bluetooth".to_string()),
            bios: Some("BIOS".to_string()),
            gpu_driver: Some("GPU Driver".to_string()),
            cpu_usage: Some("CPU Usage".to_string()),
            disk: Some("Disk".to_string()),
            battery: Some("Battery".to_string()),
            power_adapter: Some("Power Adapter".to_string()),
            font: Some("Font".to_string()),
            song: Some("Song".to_string()),
            local_ip: Some("Local IP".to_string()),
            public_ip: Some("Public IP".to_string()),
            users: Some("Users".to_string()),
            locale: Some("Locale".to_string()),
            java: Some("Java".to_string()),
            python: Some("Python".to_string()),
            node: Some("Node".to_string()),
            rust: Some("Rust".to_string()),
            cols: true,
        }
    }

    pub fn default_neofetch() -> Self {
        todo!("Refer to neofetch default config to implement")
    }
}
