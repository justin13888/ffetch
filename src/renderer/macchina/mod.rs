use console::style;
use tracing::debug;

use crate::{
    config::MacchinaRendererConfig,
    probe::{ProbeList, ProbeResultValue, ProbeValue},
    renderer::macchina::ascii::ASCII_ART_FILLER,
};

use super::RendererError;

mod ascii;
use ascii::{ASCII_ART};

pub struct MacchinaRenderer {
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
        // TODO: Implement ASCII macchina logos

        let mut art_iter = ASCII_ART.iter();

        for (title, probe) in &self.probe_list {
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
                    match art_iter.next() {
                        Some(art) => style(art).blue().to_string(),
                        None => style(ASCII_ART_FILLER).blue().to_string(),
                    },
                    style(title.clone()).blue(),
                    style("-").yellow(),
                    result
                );
            });
        }

        // Print remaining ASCII art
        for art in art_iter {
            println!("{}", style(art).blue());
        }

        println!();

        Ok(())
    }

    // TODO: Tweak this function to match actual macchina
    /// Convert a probe value to a string
    fn probe_config_to_string(probe_value: &ProbeValue) -> String {
        match probe_value {
            ProbeValue::Host(username, hostname) => format!("{}@{}", username, hostname),
            ProbeValue::OS(os) => os.to_string(),
            ProbeValue::Distro(distro) => distro.to_string(),
            ProbeValue::Model(vendor, product) => format!("{} {}", vendor, product),
            ProbeValue::Kernel(kernel) => kernel.to_string(),
            ProbeValue::Uptime(uptime) => {
                // TODO: Check if this is correct
                let uptime = *uptime as f64;
                let days = (uptime / (60.0 * 60.0 * 24.0)).floor() as i32;
                let hours = ((uptime / (60.0 * 60.0)) % 24.0).floor() as i32;
                let minutes = ((uptime / 60.0) % 60.0).floor() as i32;
                let _res = String::new();

                format!(
                    "{}{}{}",
                    if days > 0 {
                        format!("{:.0}d ", days)
                    } else {
                        String::new()
                    },
                    if hours > 0 {
                        format!("{:.0}h ", hours)
                    } else {
                        String::new()
                    },
                    if minutes > 0 {
                        format!("{:.0}m ", minutes)
                    } else {
                        String::new()
                    },
                )
                // if days > 0 {
                //     format!("{:.0}d {:.0}h {:.0}m", days, hours, minutes)
                // } else if hours > 0 {
                //     format!("{:.0}h {:.0}m", hours, minutes)
                // } else if minutes > 0 {
                //     format!("{:.0}m", minutes)
                // } else {
                //     format!("{:.0}s", seconds)
                // }
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
            ProbeValue::Memory(free, total) => format!(
                "{} GB / {} GB",
                ((*free as f32 * 10.0 / (1000.0 * 1000.0)).round() / 10.0),
                ((*total as f32 * 10.0 / (1000.0 * 1000.0)).round() / 10.0),
            ),
            ProbeValue::Network(network) => network.to_string(),
            ProbeValue::Bluetooth(bluetooth) => bluetooth.to_string(),
            ProbeValue::BIOS(bios) => bios.to_string(),
            ProbeValue::GPUDriver(gpu_driver) => gpu_driver.to_string(),
            ProbeValue::CPUUsage(cpu_usage) => format!("{}%", cpu_usage),
            ProbeValue::Disk(used, total) => format!(
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
            ProbeValue::Users(users) => users.to_string(),
            ProbeValue::Locale(locale) => locale.to_string(),
            ProbeValue::Java(java) => java.to_string(),
            ProbeValue::Node(node) => node.to_string(),
            ProbeValue::Python(python) => python.to_string(),
            ProbeValue::Rust(rust) => rust.to_string(),
        }
    }
}
