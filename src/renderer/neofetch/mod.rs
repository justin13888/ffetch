use console::style;
use tracing::debug;

use crate::{
    ascii::{get_ascii_art, get_distro_color, get_filler},
    config::NeofetchRendererConfig,
    probe::{ProbeList, ProbeResultValue, ProbeValue, general_readout},
};

use super::execute_probes_parallel;

use super::RendererError;

pub struct NeofetchRenderer {
    config: NeofetchRendererConfig,
    probe_list: ProbeList,
}

impl Default for NeofetchRenderer {
    fn default() -> Self {
        Self::new(NeofetchRendererConfig::default())
    }
}

impl NeofetchRenderer {
    pub fn new(config: NeofetchRendererConfig) -> Self {
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

        let max_title_len = self
            .probe_list
            .iter()
            .map(|(title, _)| title.len())
            .max()
            .unwrap_or(0);

        let mut art_iter = ascii_art.iter();

        // Render title (username@hostname)
        let mut title_len = 0;
        if self.config.title {
            let username = general_readout().username()?;
            let hostname = general_readout().hostname()?;
            title_len = username.len() + hostname.len() + 1;
            println!(
                "{}   {}@{}",
                style(art_iter.next().unwrap_or(&filler.as_str())).fg(primary_color),
                style(&username).fg(primary_color),
                style(&hostname).fg(primary_color),
            );
        }

        // Render underline
        if self.config.underline {
            let underline = "-".repeat(title_len);
            println!(
                "{}   {}",
                style(art_iter.next().unwrap_or(&filler.as_str())).fg(primary_color),
                underline
            );
        }

        // Run all probes in parallel, then render results in order
        let probe_results = execute_probes_parallel(&self.probe_list);
        for (title, result) in probe_results {
            let padded_title = format!("{:width$}:", title, width = max_title_len);
            let results = match result {
                Some(ProbeResultValue::Single(value)) => vec![Self::probe_config_to_string(&value)],
                Some(ProbeResultValue::Multiple(values)) => values
                    .iter()
                    .map(Self::probe_config_to_string)
                    .collect::<Vec<_>>(),
                None => {
                    debug!("Error while probing {}", title);
                    continue;
                }
            };
            results.into_iter().for_each(|result| {
                println!(
                    "{}   {} {}",
                    style(art_iter.next().unwrap_or(&filler.as_str())).fg(primary_color),
                    style(padded_title.clone()).fg(primary_color),
                    result
                );
            });
        }

        // Render neofetch colour blocks
        if self.config.col {
            // Empty spacer line between probes and colour blocks
            println!(
                "{}",
                style(art_iter.next().unwrap_or(&filler.as_str())).fg(primary_color)
            );

            // Build two rows of 8 coloured blocks (3 spaces each with ANSI background colour)
            let row1: String = (0u8..8)
                .map(|i| format!("\x1b[{}m   \x1b[0m", 40 + i))
                .collect();
            let row2: String = (0u8..8)
                .map(|i| format!("\x1b[{}m   \x1b[0m", 100 + i))
                .collect();

            println!(
                "{}   {}",
                style(art_iter.next().unwrap_or(&filler.as_str())).fg(primary_color),
                row1
            );
            println!(
                "{}   {}",
                style(art_iter.next().unwrap_or(&filler.as_str())).fg(primary_color),
                row2
            );
        }

        // Print remaining ASCII art lines
        for art_line in art_iter {
            println!("{}", style(art_line).fg(primary_color));
        }

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
                let seconds = (uptime % 60.0).floor() as i32;

                if days > 0 {
                    format!("{} days, {} hours, {} mins", days, hours, minutes)
                } else if hours > 0 {
                    format!("{} hours, {} mins", hours, minutes)
                } else if minutes > 0 {
                    format!("{} mins", minutes)
                } else {
                    format!("{} seconds", seconds)
                }
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
            ProbeValue::Memory(used, total) => {
                format!("{}MiB / {}MiB", *used / 1024, *total / 1024,)
            }
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
            ProbeValue::Battery(battery) => battery.to_string(),
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
