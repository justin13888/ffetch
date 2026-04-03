use console::style;
use tracing::{debug, info_span};

use crate::{
    ascii::{get_ascii_art, get_distro_color, get_filler},
    config::MacchinaRendererConfig,
    probe::{ProbeList, ProbeResultValue, ProbeValue, general_readout},
};

use super::RendererError;

pub struct MacchinaRenderer {
    #[allow(dead_code)]
    config: MacchinaRendererConfig,
    probe_list: ProbeList,
}

impl Default for MacchinaRenderer {
    fn default() -> Self {
        Self::new(MacchinaRendererConfig::default())
    }
}

impl MacchinaRenderer {
    pub fn new(config: MacchinaRendererConfig) -> Self {
        let probe_list = config
            .probes
            .iter()
            .map(|p| p.get_funcs())
            .collect::<Vec<_>>();
        Self { config, probe_list }
    }

    pub fn draw(&self) -> Result<(), RendererError> {
        use libmacchina::traits::GeneralReadout as _;

        // Detect the current distro
        let distro = general_readout()
            .distribution()
            .or_else(|_| general_readout().os_name())
            .unwrap_or_else(|_| "Linux".to_string());

        debug!("Detected distro: {}", distro);

        // Get ASCII art for this distro
        let (ascii_art, ascii_width) = get_ascii_art(&distro);
        let primary_color = get_distro_color(&distro);
        let filler = get_filler(ascii_width);

        let title_width = std::cmp::max(
            self.probe_list
                .iter()
                .map(|(title, _)| title.len())
                .max()
                .unwrap_or(0)
                + 2,
            12,
        );

        println!();

        let mut art_iter = ascii_art.iter();

        for (title, probe) in &self.probe_list {
            let _span = info_span!("probe", name = %title).entered();
            let results: Vec<String> = match probe() {
                Ok(result) => match result {
                    ProbeResultValue::Single(value) => vec![Self::probe_config_to_string(&value)],
                    ProbeResultValue::Multiple(values) => values
                        .into_iter()
                        .map(|value| Self::probe_config_to_string(&value))
                        .collect::<Vec<_>>(),
                },
                Err(err) => {
                    debug!("Error while probing {}: {}", title, err);
                    continue;
                }
            };
            results.into_iter().for_each(|result| {
                println!(
                    "{}    {:title_width$}{}  {}",
                    style(art_iter.next().unwrap_or(&filler.as_str())).fg(primary_color),
                    style(title.clone()).fg(primary_color),
                    style("-").yellow(),
                    result
                );
            });
        }

        // Print remaining ASCII art
        for art_line in art_iter {
            println!("{}", style(art_line).fg(primary_color));
        }

        println!();

        Ok(())
    }

    /// Convert a probe value to a string
    fn probe_config_to_string(probe_value: &ProbeValue) -> String {
        match probe_value {
            ProbeValue::Host(username, hostname) => format!("{}@{}", username, hostname),
            ProbeValue::OS(os) => os.to_string(),
            ProbeValue::Distro(distro) => distro.to_string(),
            ProbeValue::Model(vendor, product) => format!("{} {}", vendor, product),
            ProbeValue::Kernel(kernel) => kernel.to_string(),
            ProbeValue::Uptime(uptime) => {
                let uptime = *uptime as f64;
                let days = (uptime / (60.0 * 60.0 * 24.0)).floor() as i32;
                let hours = ((uptime / (60.0 * 60.0)) % 24.0).floor() as i32;
                let minutes = ((uptime / 60.0) % 60.0).floor() as i32;

                format!(
                    "{}{}{}",
                    if days > 0 {
                        format!("{}d ", days)
                    } else {
                        String::new()
                    },
                    if hours > 0 {
                        format!("{}h ", hours)
                    } else {
                        String::new()
                    },
                    if minutes > 0 {
                        format!("{}m", minutes)
                    } else {
                        String::new()
                    },
                )
            }
            ProbeValue::Packages(counts) => counts
                .iter()
                .map(|(manager, count)| format!("{} ({})", count, manager))
                .collect::<Vec<_>>()
                .join(", "),
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
            ProbeValue::Memory(used, total) => format!(
                "{} GB / {} GB",
                ((*used as f32 * 10.0 / (1000.0 * 1000.0)).round() / 10.0),
                ((*total as f32 * 10.0 / (1000.0 * 1000.0)).round() / 10.0),
            ),
            ProbeValue::Network(network) => network.to_string(),
            ProbeValue::Bluetooth(bluetooth) => bluetooth.to_string(),
            ProbeValue::BIOS(bios) => bios.to_string(),
            ProbeValue::GPUDriver(gpu_driver) => gpu_driver.to_string(),
            ProbeValue::CPUUsage(cpu_usage) => format!("{}%", cpu_usage),
            ProbeValue::Disk(_mountpoint, used, total) => format!(
                "{} G / {} G ({}%)",
                (*used as f32 / (1024.0 * 1024.0 * 1024.0)).round() as i32,
                (*total as f32 / (1024.0 * 1024.0 * 1024.0)).round() as i32,
                (*used as f32 / *total as f32 * 100.0).round() as i32,
            ),
            ProbeValue::Battery(battery) => {
                if *battery >= 100 {
                    "Full".to_string()
                } else {
                    battery.to_string()
                }
            }
            ProbeValue::PowerAdapter(power_adapter) => power_adapter.to_string(),
            ProbeValue::Font(font) => font.to_string(),
            ProbeValue::Song(song) => song.to_string(),
            ProbeValue::LocalIP(local_ip) => local_ip.to_string(),
            ProbeValue::PublicIP(public_ip) => public_ip.to_string(),
            ProbeValue::Users(users) => users.join(", "),
            ProbeValue::Locale(locale) => locale.to_string(),
            ProbeValue::Java(java) => java.to_string(),
            ProbeValue::Node(node) => node.to_string(),
            ProbeValue::Python(python) => python.to_string(),
            ProbeValue::Rust(rust) => rust.to_string(),
        }
    }
}
