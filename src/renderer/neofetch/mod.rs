use std::io::{IsTerminal, Write};

use crossterm::{
    cursor, execute, queue,
    style::{
        Attribute, Color, Print, ResetColor, SetAttribute, SetBackgroundColor, SetForegroundColor,
    },
    terminal,
};
use tracing::debug;

use crate::{
    ascii::{get_ascii_art, get_distro_color, get_filler},
    config::NeofetchRendererConfig,
    probe::{ProbeList, ProbeResultValue, general_readout},
};

use super::{RendererError, execute_probes_streaming};

pub struct NeofetchRenderer {
    config: NeofetchRendererConfig,
    probe_list: ProbeList,
}

impl Default for NeofetchRenderer {
    fn default() -> Self {
        Self::new(NeofetchRendererConfig::default())
    }
}

/// Resolved text colours for the six neofetch slots.
struct ResolvedColors {
    title: Color,
    at: Color,
    underline: Color,
    subtitle: Color,
    colon: Color,
    info: Color,
}

/// Resolve the configured `colors` slots against the distro `primary` colour.
/// Empty `colors` reproduces neofetch's distro defaults (everything the logo
/// colour, values in the terminal's default foreground).
fn resolve_colors(colors: &[u8], primary: Color) -> ResolvedColors {
    if colors.is_empty() {
        return ResolvedColors {
            title: primary,
            at: primary,
            underline: primary,
            subtitle: primary,
            colon: primary,
            info: Color::Reset,
        };
    }
    let slot = |i: usize| {
        colors
            .get(i)
            .map(|&c| Color::AnsiValue(c))
            .unwrap_or(primary)
    };
    ResolvedColors {
        title: slot(0),
        at: slot(1),
        underline: slot(2),
        subtitle: slot(3),
        colon: slot(4),
        info: slot(5),
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

    /// Write `text` in `color`, optionally bold, then reset. ANSI is zero-width
    /// so this never affects the renderer's column math.
    fn put<W: Write>(w: &mut W, color: Color, bold: bool, text: &str) -> std::io::Result<()> {
        if bold {
            queue!(
                w,
                SetAttribute(Attribute::Bold),
                SetForegroundColor(color),
                Print(text),
                SetAttribute(Attribute::Reset),
            )
        } else {
            queue!(w, SetForegroundColor(color), Print(text), ResetColor)
        }
    }

    pub fn draw(&self) -> Result<(), RendererError> {
        use libmacchina::traits::GeneralReadout as _;

        let distro = general_readout()
            .distribution()
            .or_else(|_| general_readout().os_name())
            .unwrap_or_else(|_| "Linux".to_string());

        debug!("Detected distro: {}", distro);

        let (ascii_art, ascii_width) = get_ascii_art(&distro);
        let primary_color = get_distro_color(&distro);
        let filler = get_filler(ascii_width);
        let get_art =
            |idx: usize| -> &str { ascii_art.get(idx).copied().unwrap_or(filler.as_str()) };

        let colors = resolve_colors(&self.config.colors, primary_color);
        let bold = self.config.bold;
        let sep = self.config.separator.as_str();
        let sep_width = sep.chars().count();

        let max_title_len = self
            .probe_list
            .iter()
            .map(|(title, _)| title.chars().count())
            .max()
            .unwrap_or(0);

        let stdout = std::io::stdout();
        let is_tty = stdout.is_terminal();
        let mut w = std::io::BufWriter::new(stdout.lock());

        let mut art_idx = 0usize;
        let mut title_len = 0;

        // Print title (username@hostname)
        if self.config.title {
            let username = general_readout().username()?;
            let mut hostname = general_readout().hostname()?;
            if !self.config.title_fqdn
                && let Some((short, _domain)) = hostname.split_once('.')
            {
                hostname = short.to_string();
            }
            title_len = username.chars().count() + hostname.chars().count() + 1;
            Self::put(&mut w, primary_color, false, get_art(art_idx))?;
            queue!(w, Print("   "))?;
            Self::put(&mut w, colors.title, bold, &username)?;
            Self::put(&mut w, colors.at, bold, "@")?;
            Self::put(&mut w, colors.title, bold, &hostname)?;
            queue!(w, Print("\n"))?;
            art_idx += 1;
        }

        // Print underline
        if self.config.underline {
            Self::put(&mut w, primary_color, false, get_art(art_idx))?;
            queue!(w, Print("   "))?;
            Self::put(
                &mut w,
                colors.underline,
                false,
                &self.config.underline_char.repeat(title_len),
            )?;
            queue!(w, Print("\n"))?;
            art_idx += 1;
        }

        let probe_art_start = art_idx;
        let n_probes = self.probe_list.len();
        // Column (0-based) where values start: ascii + "   " + padded label + sep + " ".
        let value_col = (ascii_width + 3 + max_title_len + sep_width + 1) as u16;
        // Per-probe config, aligned with `probe_list` by index, for option-aware formatting.
        let probes = &self.config.probes;

        // Emit one full probe line: art, label, separator, value.
        let put_line = |w: &mut std::io::BufWriter<_>, art: &str, label: &str, value: &str| {
            Self::put(w, primary_color, false, art)?;
            queue!(w, Print("   "))?;
            Self::put(w, colors.subtitle, bold, label)?;
            Self::put(w, colors.colon, bold, sep)?;
            queue!(w, Print(" "))?;
            Self::put(w, colors.info, false, value)?;
            queue!(w, Print("\n"))
        };

        if !is_tty {
            // Non-TTY: run probes in parallel, print results sequentially
            let mut all_results: Vec<Option<Vec<String>>> = vec![None; n_probes];
            execute_probes_streaming(&self.probe_list, |index, _, result| {
                all_results[index] = Some(match result {
                    Some(ProbeResultValue::Single(v)) => vec![probes[index].format_value(&v)],
                    Some(ProbeResultValue::Multiple(vs)) => {
                        vs.iter().map(|v| probes[index].format_value(v)).collect()
                    }
                    None => vec![],
                });
            });

            for (i, (title, _)) in self.probe_list.iter().enumerate() {
                let strings = match all_results[i].as_deref() {
                    Some([]) | None => {
                        debug!("Error while probing {}", title);
                        continue;
                    }
                    Some(ss) => ss.to_vec(),
                };
                let label = format!("{:width$}", title, width = max_title_len);
                for s in strings.iter() {
                    // Repeat the label on every line (e.g. one "GPU:" per GPU),
                    // matching neofetch rather than leaving orphaned values.
                    put_line(&mut w, get_art(art_idx), &label, s)?;
                    art_idx += 1;
                }
            }
        } else {
            // TTY: progressive rendering with cursor movement

            // Phase 1: print all placeholder lines immediately (label + separator).
            for (i, (title, _)) in self.probe_list.iter().enumerate() {
                let label = format!("{:width$}", title, width = max_title_len);
                Self::put(&mut w, primary_color, false, get_art(probe_art_start + i))?;
                queue!(w, Print("   "))?;
                Self::put(&mut w, colors.subtitle, bold, &label)?;
                Self::put(&mut w, colors.colon, bold, sep)?;
                queue!(w, Print(" \n"))?;
            }
            w.flush()?;
            // Save cursor position at the bottom of the probe section
            execute!(w, cursor::SavePosition)?;

            // Phase 2: fill in values as each probe completes
            let mut results: Vec<Option<Vec<String>>> = vec![None; n_probes];
            let mut needs_rerender = false;

            execute_probes_streaming(&self.probe_list, |index, _label, result| {
                let strings: Vec<String> = match result {
                    Some(ProbeResultValue::Single(v)) => vec![probes[index].format_value(&v)],
                    Some(ProbeResultValue::Multiple(vs)) => {
                        vs.iter().map(|v| probes[index].format_value(v)).collect()
                    }
                    None => vec![],
                };

                if strings.len() == 1 {
                    // Single value: move cursor to the right line and fill in
                    let lines_up = (n_probes - index) as u16;
                    let _ = execute!(
                        w,
                        cursor::RestorePosition,
                        cursor::MoveUp(lines_up),
                        cursor::MoveToColumn(value_col),
                    );
                    let _ = Self::put(&mut w, colors.info, false, &strings[0]);
                    let _ = execute!(w, cursor::RestorePosition);
                } else {
                    // Zero (failure) or multiple values: needs a re-render pass
                    needs_rerender = true;
                }

                results[index] = Some(strings);
            });

            // Phase 3: if any probe had 0 or multiple lines, re-render the probe section
            if needs_rerender {
                execute!(
                    w,
                    cursor::RestorePosition,
                    cursor::MoveUp(n_probes as u16),
                    cursor::MoveToColumn(0),
                    terminal::Clear(terminal::ClearType::FromCursorDown),
                )?;
                let mut ra_idx = probe_art_start;
                for (i, (title, _)) in self.probe_list.iter().enumerate() {
                    let strings = match results[i].as_deref() {
                        Some([]) | None => {
                            debug!("Error while probing {}", title);
                            continue;
                        }
                        Some(ss) => ss.to_vec(),
                    };
                    let label = format!("{:width$}", title, width = max_title_len);
                    for s in strings.iter() {
                        put_line(&mut w, get_art(ra_idx), &label, s)?;
                        ra_idx += 1;
                    }
                }
                art_idx = ra_idx;
                w.flush()?;
            } else {
                // Ensure cursor is at the bottom of the probe section
                execute!(w, cursor::RestorePosition)?;
                art_idx = probe_art_start + n_probes;
            }
        }

        // Print color blocks
        if self.config.col {
            // Spacer line between probes and color blocks
            Self::put(&mut w, primary_color, false, get_art(art_idx))?;
            queue!(w, Print("\n"))?;
            art_idx += 1;

            // Row 1: dark/standard colors (equivalent to ANSI background 40-47)
            Self::put(&mut w, primary_color, false, get_art(art_idx))?;
            queue!(w, Print("   "))?;
            for color in [
                Color::Black,
                Color::DarkRed,
                Color::DarkGreen,
                Color::DarkYellow,
                Color::DarkBlue,
                Color::DarkMagenta,
                Color::DarkCyan,
                Color::Grey,
            ] {
                queue!(w, SetBackgroundColor(color), Print("   "), ResetColor)?;
            }
            queue!(w, Print("\n"))?;
            art_idx += 1;

            // Row 2: bright colors (equivalent to ANSI background 100-107)
            Self::put(&mut w, primary_color, false, get_art(art_idx))?;
            queue!(w, Print("   "))?;
            for color in [
                Color::DarkGrey,
                Color::Red,
                Color::Green,
                Color::Yellow,
                Color::Blue,
                Color::Magenta,
                Color::Cyan,
                Color::White,
            ] {
                queue!(w, SetBackgroundColor(color), Print("   "), ResetColor)?;
            }
            queue!(w, Print("\n"))?;
            art_idx += 1;
        }

        // Print remaining ASCII art lines
        for line in ascii_art.iter().skip(art_idx) {
            Self::put(&mut w, primary_color, false, line)?;
            queue!(w, Print("\n"))?;
        }

        w.flush()?;
        Ok(())
    }
}
