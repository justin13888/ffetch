use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::probe::{ProbeResultFunction, ProbeType, ProbeValue};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Config {
    Neofetch(NeofetchRendererConfig),
}

pub enum RendererOverride {
    Neofetch,
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
        directories::ProjectDirs::from("net", "justin13888", "purr")
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

// ─────────────────────────────────────────────────────────────────────────
// Per-probe options
//
// Each probe carries a small options struct: a `label` plus the neofetch-style
// tunables that apply to it. The `probe_options!` macro generates the struct,
// its neofetch defaults, and a `Deserialize` impl that accepts either a bare
// label string (`OS = "OS"`) or a full options table (`{ label = "OS", … }`),
// so the terse form stays available while rich options are opt-in.
// ─────────────────────────────────────────────────────────────────────────

macro_rules! probe_options {
    ($(#[$m:meta])* $name:ident { $($field:ident : $ty:ty = $default:expr),* $(,)? }) => {
        $(#[$m])*
        #[derive(Clone, Debug, Serialize)]
        pub struct $name {
            pub label: String,
            $(pub $field: $ty,)*
        }

        impl Default for $name {
            fn default() -> Self {
                Self { label: String::new(), $($field: $default,)* }
            }
        }

        impl $name {
            /// Construct with the given label and neofetch-default options.
            // `..Default::default()` is redundant for option-less probes (only `label`).
            #[allow(clippy::needless_update)]
            pub fn with_label(label: &str) -> Self {
                Self { label: label.to_string(), ..Default::default() }
            }
        }

        impl From<String> for $name {
            #[allow(clippy::needless_update)]
            fn from(label: String) -> Self {
                Self { label, ..Default::default() }
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: serde::Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
                // Mirror struct used for the table form; container `default` so a
                // partial table fills the rest from the neofetch defaults.
                #[derive(Deserialize)]
                #[serde(default)]
                struct Full { label: String, $($field: $ty,)* }
                impl Default for Full {
                    fn default() -> Self { Self { label: String::new(), $($field: $default,)* } }
                }

                struct OptVisitor;
                impl<'de> serde::de::Visitor<'de> for OptVisitor {
                    type Value = $name;
                    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                        f.write_str("a label string or an options table")
                    }
                    fn visit_str<E: serde::de::Error>(self, s: &str) -> Result<$name, E> {
                        Ok($name::with_label(s))
                    }
                    fn visit_map<A: serde::de::MapAccess<'de>>(self, map: A) -> Result<$name, A::Error> {
                        let Full { label, $($field,)* } =
                            Full::deserialize(serde::de::value::MapAccessDeserializer::new(map))?;
                        Ok($name { label, $($field,)* })
                    }
                }
                de.deserialize_any(OptVisitor)
            }
        }
    };
}

/// Memory size unit (`memory_unit`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MemoryUnit {
    Kib,
    #[default]
    Mib,
    Gib,
}

/// Uptime verbosity (`uptime_shorthand`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UptimeFormat {
    #[default]
    On,
    Tiny,
    Off,
}

/// CPU core-count style (`cpu_cores`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CoresMode {
    #[default]
    Logical,
    Physical,
    Off,
}

/// Which sysfs frequency to report (`speed_type`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpeedType {
    #[default]
    BiosLimit,
    ScalingCurFreq,
    ScalingMinFreq,
    ScalingMaxFreq,
}

/// GPU filter (`gpu_type`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GpuType {
    #[default]
    All,
    Dedicated,
    Integrated,
}

/// Package-list verbosity (`package_managers`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PackageDisplay {
    #[default]
    On,
    Tiny,
    Off,
}

/// Distro-name verbosity (`distro_shorthand`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Shorthand {
    #[default]
    On,
    Tiny,
    Off,
}

/// Disk line subtitle (`disk_subtitle`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DiskSubtitle {
    #[default]
    Mount,
    Name,
    Dir,
    None,
}

probe_options!(
    /// A probe with no neofetch options beyond its label.
    LabeledOptions {}
);
probe_options!(
    /// Options for the OS/distro line (`distro_shorthand`, `os_arch`).
    DistroOptions {
        shorthand: Shorthand = Shorthand::On,
        os_arch: bool = true,
    }
);
probe_options!(
    /// Options for the kernel line (`kernel_shorthand`).
    KernelOptions { shorthand: bool = true }
);
probe_options!(
    /// Options for the uptime line (`uptime_shorthand`).
    UptimeOptions { format: UptimeFormat = UptimeFormat::On }
);
probe_options!(
    /// Options for the packages line (`package_managers`).
    PackagesOptions { display: PackageDisplay = PackageDisplay::On }
);
probe_options!(
    /// Options for the shell line (`shell_path`, `shell_version`).
    ShellOptions {
        path: bool = false,
        version: bool = true,
    }
);
probe_options!(
    /// Options for the resolution line (`refresh_rate`).
    ResolutionOptions { refresh_rate: bool = false }
);
probe_options!(
    /// Options for the desktop-environment line (`de_version`).
    DeOptions { version: bool = true }
);
probe_options!(
    /// Options for the CPU line (`cpu_brand`, `cpu_cores`, `cpu_speed`, …).
    CpuOptions {
        brand: bool = true,
        cores: CoresMode = CoresMode::Logical,
        speed: bool = true,
        speed_type: SpeedType = SpeedType::BiosLimit,
        speed_shorthand: bool = false,
    }
);
probe_options!(
    /// Options for the GPU line (`gpu_brand`, `gpu_type`).
    GpuOptions {
        brand: bool = true,
        gpu_type: GpuType = GpuType::All,
    }
);
probe_options!(
    /// Options for the memory line (`memory_unit`, `memory_percent`).
    MemoryOptions {
        unit: MemoryUnit = MemoryUnit::Mib,
        percent: bool = false,
    }
);
probe_options!(
    /// Options for the disk line (`disk_show`, `disk_subtitle`, `disk_percent`).
    /// Empty `show` means every mount point (neofetch defaults to `/`).
    DiskOptions {
        show: Vec<String> = Vec::new(),
        subtitle: DiskSubtitle = DiskSubtitle::Mount,
        percent: bool = true,
    }
);
probe_options!(
    /// Options for the now-playing line (`music_player`, `song_format`, `song_shorthand`).
    SongOptions {
        player: String = "auto".to_string(),
        format: String = "%artist% - %album% - %title%".to_string(),
        shorthand: bool = false,
    }
);

/// Probe config. Each variant selects a metric and carries its display label
/// plus any neofetch-style options. Refer to [`ProbeValue`] for what each
/// metric corresponds to.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ProbeConfig {
    Host(LabeledOptions),
    OS(DistroOptions),
    Model(LabeledOptions),
    Kernel(KernelOptions),
    Distro(LabeledOptions),
    Uptime(UptimeOptions),
    Packages(PackagesOptions),
    Shell(ShellOptions),
    Editor(LabeledOptions),
    Resolution(ResolutionOptions),
    DE(DeOptions),
    WM(LabeledOptions),
    WMTheme(LabeledOptions),
    Theme(LabeledOptions),
    Icons(LabeledOptions),
    Cursor(LabeledOptions),
    Terminal(LabeledOptions),
    TerminalFont(LabeledOptions),
    CPU(CpuOptions),
    GPU(GpuOptions),
    Memory(MemoryOptions),
    Network(LabeledOptions),
    Bluetooth(LabeledOptions),
    BIOS(LabeledOptions),

    GPUDriver(LabeledOptions),
    CPUUsage(LabeledOptions),
    Disk(DiskOptions),
    Battery(LabeledOptions),
    PowerAdapter(LabeledOptions),
    Font(LabeledOptions),
    Song(SongOptions),
    LocalIP(LabeledOptions),
    PublicIP(LabeledOptions),
    Users(LabeledOptions),
    Locale(LabeledOptions),

    Java(LabeledOptions),
    Python(LabeledOptions),
    Node(LabeledOptions),
    Rust(LabeledOptions),
}

impl ProbeConfig {
    /// Default config enabling all probes (the `--all` preset).
    pub fn default_all() -> Vec<Self> {
        vec![
            Self::OS(DistroOptions::with_label("OS")),
            Self::Model(LabeledOptions::with_label("Host")),
            Self::Kernel(KernelOptions::with_label("Kernel")),
            Self::Uptime(UptimeOptions::with_label("Uptime")),
            Self::Packages(PackagesOptions::with_label("Packages")),
            Self::Shell(ShellOptions::with_label("Shell")),
            Self::Editor(LabeledOptions::with_label("Editor")),
            Self::Resolution(ResolutionOptions::with_label("Resolution")),
            Self::DE(DeOptions::with_label("DE")),
            Self::WM(LabeledOptions::with_label("WM")),
            Self::WMTheme(LabeledOptions::with_label("WM Theme")),
            Self::Theme(LabeledOptions::with_label("Theme")),
            Self::Icons(LabeledOptions::with_label("Icons")),
            Self::Cursor(LabeledOptions::with_label("Cursor")),
            Self::Terminal(LabeledOptions::with_label("Terminal")),
            Self::TerminalFont(LabeledOptions::with_label("Terminal Font")),
            Self::CPU(CpuOptions::with_label("CPU")),
            Self::GPU(GpuOptions::with_label("GPU")),
            Self::Memory(MemoryOptions::with_label("Memory")),
            Self::Network(LabeledOptions::with_label("Network")),
            Self::Bluetooth(LabeledOptions::with_label("Bluetooth")),
            Self::BIOS(LabeledOptions::with_label("BIOS")),
            Self::GPUDriver(LabeledOptions::with_label("GPU Driver")),
            Self::CPUUsage(LabeledOptions::with_label("CPU Usage")),
            Self::Disk(DiskOptions::with_label("Disk")),
            Self::Battery(LabeledOptions::with_label("Battery")),
            Self::PowerAdapter(LabeledOptions::with_label("Power Adapter")),
            Self::Font(LabeledOptions::with_label("Font")),
            Self::Song(SongOptions::with_label("Song")),
            Self::LocalIP(LabeledOptions::with_label("Local IP")),
            Self::PublicIP(LabeledOptions::with_label("Public IP")),
            Self::Users(LabeledOptions::with_label("Users")),
            Self::Locale(LabeledOptions::with_label("Locale")),
            Self::Java(LabeledOptions::with_label("Java")),
            Self::Python(LabeledOptions::with_label("Python")),
            Self::Node(LabeledOptions::with_label("Node")),
            Self::Rust(LabeledOptions::with_label("Rust")),
        ]
    }

    /// Default config replicating neofetch's default info block.
    pub fn default_neofetch() -> Vec<Self> {
        vec![
            Self::OS(DistroOptions::with_label("OS")),
            Self::Model(LabeledOptions::with_label("Host")),
            Self::Kernel(KernelOptions::with_label("Kernel")),
            Self::Uptime(UptimeOptions::with_label("Uptime")),
            Self::Packages(PackagesOptions::with_label("Packages")),
            Self::Shell(ShellOptions::with_label("Shell")),
            Self::Resolution(ResolutionOptions::with_label("Resolution")),
            Self::DE(DeOptions::with_label("DE")),
            Self::WM(LabeledOptions::with_label("WM")),
            Self::WMTheme(LabeledOptions::with_label("WM Theme")),
            Self::Theme(LabeledOptions::with_label("Theme")),
            Self::Icons(LabeledOptions::with_label("Icons")),
            Self::Terminal(LabeledOptions::with_label("Terminal")),
            Self::TerminalFont(LabeledOptions::with_label("Terminal Font")),
            Self::CPU(CpuOptions::with_label("CPU")),
            Self::GPU(GpuOptions::with_label("GPU")),
            Self::Memory(MemoryOptions::with_label("Memory")),
        ]
    }

    /// Display label for this probe.
    pub fn label(&self) -> &str {
        match self {
            Self::Host(o) => &o.label,
            Self::OS(o) => &o.label,
            Self::Model(o) => &o.label,
            Self::Kernel(o) => &o.label,
            Self::Distro(o) => &o.label,
            Self::Uptime(o) => &o.label,
            Self::Packages(o) => &o.label,
            Self::Shell(o) => &o.label,
            Self::Editor(o) => &o.label,
            Self::Resolution(o) => &o.label,
            Self::DE(o) => &o.label,
            Self::WM(o) => &o.label,
            Self::WMTheme(o) => &o.label,
            Self::Theme(o) => &o.label,
            Self::Icons(o) => &o.label,
            Self::Cursor(o) => &o.label,
            Self::Terminal(o) => &o.label,
            Self::TerminalFont(o) => &o.label,
            Self::CPU(o) => &o.label,
            Self::GPU(o) => &o.label,
            Self::Memory(o) => &o.label,
            Self::Network(o) => &o.label,
            Self::Bluetooth(o) => &o.label,
            Self::BIOS(o) => &o.label,
            Self::GPUDriver(o) => &o.label,
            Self::CPUUsage(o) => &o.label,
            Self::Disk(o) => &o.label,
            Self::Battery(o) => &o.label,
            Self::PowerAdapter(o) => &o.label,
            Self::Font(o) => &o.label,
            Self::Song(o) => &o.label,
            Self::LocalIP(o) => &o.label,
            Self::PublicIP(o) => &o.label,
            Self::Users(o) => &o.label,
            Self::Locale(o) => &o.label,
            Self::Java(o) => &o.label,
            Self::Python(o) => &o.label,
            Self::Node(o) => &o.label,
            Self::Rust(o) => &o.label,
        }
    }

    /// The underlying metric this probe gathers.
    fn probe_type(&self) -> ProbeType {
        match self {
            Self::Host(_) => ProbeType::Host,
            Self::OS(_) => ProbeType::OS,
            Self::Model(_) => ProbeType::Model,
            Self::Kernel(_) => ProbeType::Kernel,
            Self::Distro(_) => ProbeType::Distro,
            Self::Uptime(_) => ProbeType::Uptime,
            Self::Packages(_) => ProbeType::Packages,
            Self::Shell(_) => ProbeType::Shell,
            Self::Editor(_) => ProbeType::Editor,
            Self::Resolution(_) => ProbeType::Resolution,
            Self::DE(_) => ProbeType::DE,
            Self::WM(_) => ProbeType::WM,
            Self::WMTheme(_) => ProbeType::WMTheme,
            Self::Theme(_) => ProbeType::Theme,
            Self::Icons(_) => ProbeType::Icons,
            Self::Cursor(_) => ProbeType::Cursor,
            Self::Terminal(_) => ProbeType::Terminal,
            Self::TerminalFont(_) => ProbeType::TerminalFont,
            Self::CPU(_) => ProbeType::CPU,
            Self::GPU(_) => ProbeType::GPU,
            Self::Memory(_) => ProbeType::Memory,
            Self::Network(_) => ProbeType::Network,
            Self::Bluetooth(_) => ProbeType::Bluetooth,
            Self::BIOS(_) => ProbeType::BIOS,
            Self::GPUDriver(_) => ProbeType::GPUDriver,
            Self::CPUUsage(_) => ProbeType::CPUUsage,
            Self::Disk(_) => ProbeType::Disk,
            Self::Battery(_) => ProbeType::Battery,
            Self::PowerAdapter(_) => ProbeType::PowerAdapter,
            Self::Font(_) => ProbeType::Font,
            Self::Song(_) => ProbeType::Song,
            Self::LocalIP(_) => ProbeType::LocalIP,
            Self::PublicIP(_) => ProbeType::PublicIP,
            Self::Users(_) => ProbeType::Users,
            Self::Locale(_) => ProbeType::Locale,
            Self::Java(_) => ProbeType::Java,
            Self::Python(_) => ProbeType::Python,
            Self::Node(_) => ProbeType::Node,
            Self::Rust(_) => ProbeType::Rust,
        }
    }

    /// Build the `(label, probe function)` pair the renderer executes.
    pub fn get_funcs(&self) -> (String, ProbeResultFunction) {
        (self.label().to_string(), self.probe_type().into())
    }

    /// Render a probed value for this probe, honoring its options.
    ///
    /// Currently delegates to the option-free [`ProbeValue::format`]; per-probe
    /// option handling layers onto this in subsequent changes.
    pub fn format_value(&self, value: &ProbeValue) -> String {
        value.format()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_roundtrips_through_toml() {
        let cfg = Config::default();
        let serialized = toml::to_string(&cfg).expect("serialize");
        let back: Config = toml::from_str(&serialized).expect("deserialize");
        match back {
            Config::Neofetch(c) => assert!(!c.probes.is_empty()),
        }
    }

    #[test]
    fn probe_accepts_bare_label_and_table_forms() {
        let src = r#"
[Neofetch]
title = true
underline = true
col = true
probes = [
    { OS = "Operating System" },
    { CPU = { label = "Processor", cores = "physical" } },
]
"#;
        let cfg: Config = toml::from_str(src).expect("deserialize");
        match cfg {
            Config::Neofetch(c) => {
                assert_eq!(c.probes.len(), 2);
                assert_eq!(c.probes[0].label(), "Operating System");
                assert_eq!(c.probes[1].label(), "Processor");
                match &c.probes[1] {
                    ProbeConfig::CPU(o) => assert_eq!(o.cores, CoresMode::Physical),
                    other => panic!("expected CPU, got {other:?}"),
                }
            }
        }
    }
}
